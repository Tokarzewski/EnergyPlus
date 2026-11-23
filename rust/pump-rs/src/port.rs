//! The Rust port of `pump.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `Pumps::CalcPumps`: a variable or constant speed pump, condensate pump or pump bank takes the
//! flow it is asked for within its limits (through `SetComponentFlowRate` from `setflow-rs`), the
//! bank's running pumps, the power from the cubic part load curve on the volume flow (water density
//! from `fluidproperties-rs`), a negative power set to 0, the pressure-based or EMS pressure power,
//! the shaft power and heat to the fluid, and the outlet node's state. Every expression keeps the
//! C++ order, and `min` and `max` are ObjexxFCL's. The oracle's guards (see its shim) return NaN
//! here too.

#![forbid(unsafe_code)]

use fluidproperties_rs::port::WaterDensity;
use setflow_rs::port::{Branch, MASS_FLOW_TOLERANCE, Node};

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The pump type, in `Pumps::PumpType` order.
#[derive(Clone, Copy, PartialEq)]
enum PumpType {
    VarSpeed,
    ConSpeed,
    Cond,
    BankVarSpeed,
    BankConSpeed,
}

/// One pump as `InitializePumps` leaves it.
struct Pump {
    pump_type: PumpType,
    num_pumps_in_bank: i32,
    mass_flow_rate_max: f64,
    mass_flow_rate_min: f64,
    nom_vol_flow_rate: f64,
    nom_power_use: f64,
    part_load_coef: [f64; 4],
    motor_effic: f64,
    pump_effic: f64,
    frac_motor_loss_to_fluid: f64,
    ems_pressure_override: Option<f64>,
    /// The loop's pressure drop when the loop uses pressure for the pump calculations.
    loop_pressure_drop: Option<f64>,
}

/// The `Pumps` globals and the pump's results.
#[derive(Default)]
struct Out {
    pump_mass_flow_rate: f64,
    power: f64,
    shaft_power: f64,
    pump_heat_to_fluid: f64,
    num_pumps_running: i32,
    running: bool,
    pump_power: f64,
}

impl Pump {
    /// The cubic part load curve.
    fn frac_full_load_power(&self, plr: f64) -> f64 {
        let c = &self.part_load_coef;
        c[0] + c[1] * plr + c[2] * (plr * plr) + c[3] * (plr * plr * plr)
    }

    /// `CalcPumps` on the branch (node 0 inlet, node 1 outlet) with inlet temperature `tin`.
    fn calc(&self, branch: &mut Branch, flow_request: f64, tin: f64) -> Out {
        let mut o = Out {
            pump_mass_flow_rate: if flow_request > MASS_FLOW_TOLERANCE {
                flow_request
            } else {
                0.0
            },
            ..Out::default()
        };
        if matches!(
            self.pump_type,
            PumpType::VarSpeed | PumpType::BankVarSpeed | PumpType::Cond
        ) && branch.series_active
        {
            o.pump_mass_flow_rate = 0.0;
        }
        o.pump_mass_flow_rate = min(self.mass_flow_rate_max, o.pump_mass_flow_rate);
        o.pump_mass_flow_rate = max(self.mass_flow_rate_min, o.pump_mass_flow_rate);
        branch.set_component_flow_rate(&mut o.pump_mass_flow_rate, 0, 1);

        o.running = o.pump_mass_flow_rate > MASS_FLOW_TOLERANCE;
        let bank = matches!(
            self.pump_type,
            PumpType::BankVarSpeed | PumpType::BankConSpeed
        );
        if bank {
            if o.pump_mass_flow_rate >= self.mass_flow_rate_max {
                o.num_pumps_running = self.num_pumps_in_bank;
            } else {
                // `CEILING` of a ratio within 0 to the bank size, then truncated to an int.
                let n = (o.pump_mass_flow_rate / (self.mass_flow_rate_max)
                    * f64::from(self.num_pumps_in_bank))
                .ceil();
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "the C++ assigns the whole number to an int"
                )]
                let n = n as i32;
                o.num_pumps_running = n.min(self.num_pumps_in_bank);
            }
        }
        if o.pump_mass_flow_rate <= MASS_FLOW_TOLERANCE {
            return o;
        }

        let loop_density = WaterDensity(tin);
        let mut vol_flow_rate = 0.0;
        if bank {
            let num_pumps_full_load = f64::from(o.num_pumps_running - 1);
            let n = f64::from(self.num_pumps_in_bank);
            let full_load_vol_flow_rate = self.nom_vol_flow_rate / n;
            let part_load_vol_flow_rate = o.pump_mass_flow_rate / loop_density
                - full_load_vol_flow_rate * num_pumps_full_load;
            let full_load_power = self.nom_power_use / n;
            let c = &self.part_load_coef;
            let full_load_power_ratio = c[0] + c[1] + c[2] + c[3];
            let plr = min(1.0, part_load_vol_flow_rate / full_load_vol_flow_rate);
            o.power = (full_load_power_ratio * num_pumps_full_load
                + self.frac_full_load_power(plr))
                * full_load_power;
            if self.ems_pressure_override.is_some() {
                vol_flow_rate = part_load_vol_flow_rate;
            }
        } else {
            vol_flow_rate = o.pump_mass_flow_rate / loop_density;
            let plr = min(1.0, vol_flow_rate / self.nom_vol_flow_rate);
            o.power = self.frac_full_load_power(plr) * self.nom_power_use;
        }
        if o.power < 0.0 {
            o.power = 0.0;
        }
        if let Some(dp) = self.loop_pressure_drop {
            let total_effic = self.pump_effic * self.motor_effic;
            o.power = vol_flow_rate * dp / total_effic;
        }
        if let Some(p) = self.ems_pressure_override {
            let total_effic = self.pump_effic * self.motor_effic;
            o.power = vol_flow_rate * p / total_effic;
        }
        o.shaft_power = o.power * self.motor_effic;
        o.pump_heat_to_fluid =
            o.shaft_power + (o.power - o.shaft_power) * self.frac_motor_loss_to_fluid;
        o.pump_power = o.power;
        o
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double Pump(int which, int ptype, int lock, int series, int press, int ems, int nbank,
/// double flowreq, ...)` (`pump.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn Pump(
    which: std::ffi::c_int,
    ptype: std::ffi::c_int,
    lock: std::ffi::c_int,
    series: std::ffi::c_int,
    press: std::ffi::c_int,
    ems: std::ffi::c_int,
    nbank: std::ffi::c_int,
    flowreq: f64,
    mmax: f64,
    mmin: f64,
    nomvol: f64,
    nompow: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    motor: f64,
    peff: f64,
    frac: f64,
    pdrop: f64,
    emsp: f64,
    tin: f64,
    pin: f64,
    mdotin: f64,
    hwmax: f64,
    minav: f64,
    maxav: f64,
) -> f64 {
    let flag = |v: std::ffi::c_int| v == 0 || v == 1;
    let ok = (0..=10).contains(&which)
        && (0..=4).contains(&ptype)
        && flag(lock)
        && flag(series)
        && flag(press)
        && flag(ems)
        && (1..=20).contains(&nbank)
        && within(flowreq, 0.0, 1000.0)
        && within(mmax, 0.0, 1000.0)
        && within(mmin, 0.0, mmax)
        && within(nomvol, 1.0e-6, 10.0)
        && within(nompow, 0.0, 1.0e7)
        && [c0, c1, c2, c3].iter().all(|&c| within(c, -5.0, 5.0))
        && within(motor, 0.1, 1.0)
        && within(peff, 0.1, 1.0)
        && within(frac, 0.0, 1.0)
        && within(pdrop, 0.0, 1.0e7)
        && within(emsp, 0.0, 1.0e7)
        && within(tin, 1.0, 99.0)
        && within(pin, 0.0, 1.0e7)
        && [mdotin, hwmax, minav, maxav]
            .iter()
            .all(|&v| within(v, 0.0, 1000.0));
    if !ok {
        return f64::NAN;
    }
    let pump = Pump {
        pump_type: match ptype {
            0 => PumpType::VarSpeed,
            1 => PumpType::ConSpeed,
            2 => PumpType::Cond,
            3 => PumpType::BankVarSpeed,
            _ => PumpType::BankConSpeed,
        },
        num_pumps_in_bank: nbank,
        mass_flow_rate_max: mmax,
        mass_flow_rate_min: mmin,
        nom_vol_flow_rate: nomvol,
        nom_power_use: nompow,
        part_load_coef: [c0, c1, c2, c3],
        motor_effic: motor,
        pump_effic: peff,
        frac_motor_loss_to_fluid: frac,
        ems_pressure_override: (ems == 1).then_some(emsp),
        loop_pressure_drop: (press == 1).then_some(pdrop),
    };
    let inlet = Node {
        mass_flow_rate: mdotin,
        min: 0.0,
        max: hwmax,
        min_avail: minav,
        max_avail: maxav,
        request: 0.0,
    };
    let mut branch = Branch {
        nodes: [inlet, Node::default(), Node::default(), Node::default()],
        ncomp: 1,
        ems_off: [false; 3],
        locked: lock == 1,
        autosize: false,
        series_active: series == 1,
        demand: false,
        sim_loop_side_needed: false,
    };
    let o = pump.calc(&mut branch, flowreq, tin);
    match which {
        0 => o.pump_mass_flow_rate,
        1 => o.power,
        2 => o.shaft_power,
        3 => o.pump_heat_to_fluid,
        4 => f64::from(o.num_pumps_running),
        5 => f64::from(u8::from(o.running)),
        6 => o.pump_power,
        // The outlet node takes the inlet's temperature and pressure.
        7 => tin,
        8 => pin,
        9 => branch.nodes[0].mass_flow_rate,
        _ => branch.nodes[1].mass_flow_rate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pump_power_follows_the_flow() {
        let p = |which, ptype, flowreq| {
            Pump(
                which, ptype, 0, 0, 0, 0, 3, flowreq, 3.0, 0.0, 0.003, 1000.0, 0.0, 1.0, 0.0, 0.0,
                0.9, 0.7, 0.0, 0.0, 0.0, 20.0, 101_325.0, 0.0, 3.0, 0.0, 3.0,
            )
        };
        assert_eq!(p(0, 0, 1.5), 1.5);
        assert!((p(1, 0, 1.5) - 500.0).abs() < 2.0);
        assert_eq!(p(4, 3, 1.5), 2.0);
        assert_eq!(p(0, 0, 5.0), 3.0);
        assert_eq!(p(1, 1, 0.0), 0.0);
        assert_eq!(p(5, 1, 0.0), 0.0);
        assert!(p(11, 0, 1.0).is_nan());
    }
}
