//! The Rust port of `simplefan.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `Fans::FanComponent::simulateConstant`, `simulateVAV` and `simulateOnOff`: the fan power from the
//! design pressure rise and efficiencies (with the part load curve and its low flow linearisation
//! for variable volume fans, and the runtime fraction for on/off fans), the heat it adds to the
//! air and the outlet air state, with the EMS overrides and the TurnFansOn and TurnFansOff
//! globals. Every expression keeps the C++ order, and `min` and `max` are ObjexxFCL's. The
//! oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::{PsyHFnTdbW, PsyTdbFnHW};

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The fields of one `FanComponent` and the HVAC globals the simulate methods read and write.
struct Fan {
    avail: f64,
    turn_fans_on: bool,
    turn_fans_off: bool,
    on_off_fan_part_load_fraction: f64,
    inlet_air_mass_flow_rate: f64,
    max_air_mass_flow_rate: f64,
    min_air_mass_flow_rate: f64,
    max_air_flow_rate: f64,
    min_air_flow_rate: f64,
    delta_press: f64,
    total_eff: f64,
    motor_eff: f64,
    motor_in_air_frac: f64,
    rho_air_std_init: f64,
    inlet_air_temp: f64,
    inlet_air_hum_rat: f64,
    inlet_air_enthalpy: f64,
    coeffs: [f64; 5],
    ems_flow: Option<f64>,
    ems_press: Option<f64>,
    ems_eff: Option<f64>,
    // Results.
    total_power: f64,
    power_loss_to_air: f64,
    outlet_air_temp: f64,
    outlet_air_enthalpy: f64,
    outlet_air_mass_flow_rate: f64,
    outlet_air_hum_rat: f64,
    runtime_frac: f64,
    mass_flow_rate_max_avail: f64,
}

impl Fan {
    /// The fan runs this time step.
    fn operating(&self, mass_flow: f64) -> bool {
        (self.avail > 0.0 || self.turn_fans_on) && !self.turn_fans_off && mass_flow > 0.0
    }

    /// The shaft power, the heat to the air and the outlet state at total power `total_power`.
    fn heat_air(&mut self, mass_flow: f64, motor_eff: f64, motor_in_air_frac: f64) {
        let shaft_power = motor_eff * self.total_power;
        self.power_loss_to_air = shaft_power + (self.total_power - shaft_power) * motor_in_air_frac;
        self.outlet_air_enthalpy = self.inlet_air_enthalpy + self.power_loss_to_air / mass_flow;
        self.outlet_air_hum_rat = self.inlet_air_hum_rat;
        self.outlet_air_mass_flow_rate = mass_flow;
        self.outlet_air_temp = PsyTdbFnHW(self.outlet_air_enthalpy, self.outlet_air_hum_rat);
    }

    /// The fan is off: no power and no flow.
    fn off(&mut self) {
        self.total_power = 0.0;
        self.power_loss_to_air = 0.0;
        self.outlet_air_mass_flow_rate = 0.0;
        self.outlet_air_hum_rat = self.inlet_air_hum_rat;
        self.outlet_air_enthalpy = self.inlet_air_enthalpy;
        self.outlet_air_temp = self.inlet_air_temp;
        self.mass_flow_rate_max_avail = 0.0;
    }

    /// `simulateConstant`.
    fn simulate_constant(&mut self) {
        let mut delta_press = self.delta_press;
        let mut total_eff = self.total_eff;
        let rho_air = self.rho_air_std_init;
        let mut mass_flow = self.inlet_air_mass_flow_rate;
        if let Some(v) = self.ems_flow {
            mass_flow = v;
        }
        if let Some(v) = self.ems_press {
            delta_press = v;
        }
        if let Some(v) = self.ems_eff {
            total_eff = v;
        }
        mass_flow = min(mass_flow, self.max_air_mass_flow_rate);
        mass_flow = max(mass_flow, self.min_air_mass_flow_rate);
        if self.operating(mass_flow) {
            self.total_power = max(0.0, mass_flow * delta_press / (total_eff * rho_air));
            self.heat_air(mass_flow, self.motor_eff, self.motor_in_air_frac);
        } else {
            self.off();
        }
    }

    /// The part load fraction at flow fraction `x`.
    fn part_load_frac(&self, x: f64) -> f64 {
        let c = &self.coeffs;
        c[0] + c[1] * x + c[2] * (x * x) + c[3] * (x * x * x) + c[4] * pow_4(x)
    }

    /// `simulateVAV`, with the pressure rise passed in or not.
    fn simulate_vav(&mut self, pressure_rise: Option<f64>) {
        let mut delta_press = pressure_rise.unwrap_or(self.delta_press);
        let mut total_eff = self.total_eff;
        let max_air_mass_flow_rate = self.max_air_mass_flow_rate;
        let rho_air = self.rho_air_std_init;
        let mut mass_flow = self.inlet_air_mass_flow_rate;
        let max_air_flow_rate = self.max_air_flow_rate;
        if let Some(v) = self.ems_press {
            delta_press = v;
        }
        if let Some(v) = self.ems_eff {
            total_eff = v;
        }
        if let Some(v) = self.ems_flow {
            mass_flow = v;
        }
        mass_flow = min(mass_flow, max_air_mass_flow_rate);
        if !self.operating(mass_flow) {
            self.off();
            return;
        }
        let min_flow_frac = self.min_air_flow_rate / max_air_flow_rate;
        let flow_frac_actual = mass_flow / max_air_mass_flow_rate;
        let flow_frac_for_power = max(min_flow_frac, min(flow_frac_actual, 1.0));
        let part_load_frac = self.part_load_frac(flow_frac_for_power);
        self.total_power = max(
            0.0,
            part_load_frac * max_air_mass_flow_rate * delta_press / (total_eff * rho_air),
        );
        self.heat_air(mass_flow, self.motor_eff, self.motor_in_air_frac);

        // Below 10 % flow the power curve is linearised to 0 when the air would heat by over 20 C.
        let delta_t_across_fan = self.outlet_air_temp - self.inlet_air_temp;
        if delta_t_across_fan > 20.0 {
            let min_flow_frac_limit_fan_heat = 0.10;
            let low_min_power = || {
                let part_load_frac_at_low_min = self.part_load_frac(min_flow_frac_limit_fan_heat);
                part_load_frac_at_low_min * max_air_mass_flow_rate * delta_press
                    / (total_eff * rho_air)
            };
            if flow_frac_for_power < min_flow_frac_limit_fan_heat {
                let fan_power_at_low_minimum = low_min_power();
                self.total_power = max(
                    0.0,
                    flow_frac_for_power * fan_power_at_low_minimum / min_flow_frac_limit_fan_heat,
                );
            } else if flow_frac_actual < min_flow_frac_limit_fan_heat {
                let fan_power_at_low_minimum = low_min_power();
                self.total_power = max(
                    0.0,
                    flow_frac_actual * fan_power_at_low_minimum / min_flow_frac_limit_fan_heat,
                );
            }
            self.heat_air(mass_flow, self.motor_eff, self.motor_in_air_frac);
        }
    }

    /// `simulateOnOff` without a speed ratio.
    fn simulate_on_off(&mut self) {
        let mut mass_flow = self.inlet_air_mass_flow_rate;
        let max_air_mass_flow_rate = self.max_air_mass_flow_rate;
        let mut delta_press = self.delta_press;
        let mut total_eff = self.total_eff;
        let rho_air = self.rho_air_std_init;
        if let Some(v) = self.ems_flow {
            mass_flow = v;
        }
        if let Some(v) = self.ems_press {
            delta_press = v;
        }
        if let Some(v) = self.ems_eff {
            total_eff = v;
        }
        mass_flow = min(mass_flow, max_air_mass_flow_rate);
        mass_flow = max(mass_flow, self.min_air_mass_flow_rate);
        self.runtime_frac = 0.0;
        if !(self.operating(mass_flow) && self.max_air_mass_flow_rate > 0.0) {
            self.off();
            return;
        }
        let flow_frac = mass_flow / max_air_mass_flow_rate;
        let part_load_ratio = min(1.0, flow_frac);
        // The oracle keeps the part load fraction above 0, so its recurring warning never runs.
        if self.on_off_fan_part_load_fraction < 0.7 {
            self.on_off_fan_part_load_fraction = 0.7;
        }
        self.runtime_frac = if self.on_off_fan_part_load_fraction >= 1.0 {
            part_load_ratio
        } else {
            max(
                0.0,
                min(1.0, part_load_ratio / self.on_off_fan_part_load_fraction),
            )
        };
        self.total_power = max(
            0.0,
            max_air_mass_flow_rate * self.runtime_frac * delta_press / (total_eff * rho_air),
        );
        self.on_off_fan_part_load_fraction = 1.0;
        self.heat_air(mass_flow, self.motor_eff, self.motor_in_air_frac);
    }
}

/// ObjexxFCL `pow_4`.
fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double SimpleFan(int which, int kind, int turn, int ems, double avail, double mdot,
/// double mdotmax, double mdotmin, double volmax, double volmin, double dp, double eff, double
/// moteff, double motinair, double rho, double tin, double win, double plf, double c0, double c1,
/// double c2, double c3, double c4, double pr, double emsflow, double emspress, double emseff)`
/// (`simplefan.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn SimpleFan(
    which: std::ffi::c_int,
    kind: std::ffi::c_int,
    turn: std::ffi::c_int,
    ems: std::ffi::c_int,
    avail: f64,
    mdot: f64,
    mdotmax: f64,
    mdotmin: f64,
    volmax: f64,
    volmin: f64,
    dp: f64,
    eff: f64,
    moteff: f64,
    motinair: f64,
    rho: f64,
    tin: f64,
    win: f64,
    plf: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    pr: f64,
    emsflow: f64,
    emspress: f64,
    emseff: f64,
) -> f64 {
    let ok = (0..=8).contains(&which)
        && (0..=3).contains(&kind)
        && (0..=2).contains(&turn)
        && (0..=7).contains(&ems)
        && within(avail, 0.0, 1.0)
        && within(mdot, 0.0, 100.0)
        && within(mdotmax, 0.01, 100.0)
        && within(mdotmin, 0.0, mdotmax)
        && within(volmax, 0.01, 100.0)
        && within(volmin, 0.0, volmax)
        && within(dp, 0.0, 5000.0)
        && within(eff, 0.05, 1.0)
        && within(moteff, 0.0, 1.0)
        && within(motinair, 0.0, 1.0)
        && within(rho, 0.5, 1.5)
        && within(tin, -40.0, 60.0)
        && within(win, 0.0, 0.04)
        && within(plf, 0.05, 1.0)
        && [c0, c1, c2, c3, c4].iter().all(|&c| within(c, -10.0, 10.0))
        && within(pr, 0.0, 5000.0)
        && within(emsflow, 0.0, 100.0)
        && within(emspress, 0.0, 5000.0)
        && within(emseff, 0.05, 1.0);
    if !ok {
        return f64::NAN;
    }
    let mut fan = Fan {
        avail,
        turn_fans_on: turn == 1,
        turn_fans_off: turn == 2,
        on_off_fan_part_load_fraction: plf,
        inlet_air_mass_flow_rate: mdot,
        max_air_mass_flow_rate: mdotmax,
        min_air_mass_flow_rate: mdotmin,
        max_air_flow_rate: volmax,
        min_air_flow_rate: volmin,
        delta_press: dp,
        total_eff: eff,
        motor_eff: moteff,
        motor_in_air_frac: motinair,
        rho_air_std_init: rho,
        inlet_air_temp: tin,
        inlet_air_hum_rat: win,
        inlet_air_enthalpy: PsyHFnTdbW(tin, win),
        coeffs: [c0, c1, c2, c3, c4],
        ems_flow: (ems & 1 != 0).then_some(emsflow),
        ems_press: (ems & 2 != 0).then_some(emspress),
        ems_eff: (ems & 4 != 0).then_some(emseff),
        total_power: 0.0,
        power_loss_to_air: 0.0,
        outlet_air_temp: 0.0,
        outlet_air_enthalpy: 0.0,
        outlet_air_mass_flow_rate: 0.0,
        outlet_air_hum_rat: 0.0,
        runtime_frac: 0.0,
        mass_flow_rate_max_avail: mdotmax,
    };
    match kind {
        0 => fan.simulate_constant(),
        1 => fan.simulate_vav(None),
        2 => fan.simulate_vav(Some(pr)),
        _ => fan.simulate_on_off(),
    }
    match which {
        0 => fan.total_power,
        1 => fan.power_loss_to_air,
        2 => fan.outlet_air_temp,
        3 => fan.outlet_air_enthalpy,
        4 => fan.outlet_air_mass_flow_rate,
        5 => fan.outlet_air_hum_rat,
        6 => fan.runtime_frac,
        7 => fan.mass_flow_rate_max_avail,
        _ => fan.on_off_fan_part_load_fraction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fan_heats_the_air() {
        let fan = |which, kind, mdot| {
            SimpleFan(
                which, kind, 0, 0, 1.0, mdot, 2.0, 0.0, 1.7, 0.5, 600.0, 0.6, 0.9, 1.0, 1.2, 20.0,
                0.008, 0.8, 0.0, 0.0, 0.0, 1.0, 0.0, 400.0, 0.0, 0.0, 0.5,
            )
        };
        assert!((fan(0, 0, 1.8) - 1.8 * 600.0 / 0.72).abs() < 1e-9);
        assert!(fan(2, 0, 1.8) > 20.0);
        assert!(fan(0, 1, 1.0) < fan(0, 1, 2.0));
        assert!((fan(6, 3, 1.0) - 0.625).abs() < 1e-12);
        assert_eq!(fan(4, 0, 0.0), 0.0);
        assert!(fan(9, 0, 1.0).is_nan());
    }
}
