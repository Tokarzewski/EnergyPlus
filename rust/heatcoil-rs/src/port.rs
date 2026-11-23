//! The Rust port of `heatcoil.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatingCoils::CalcElectricHeatingCoil` and `CalcFuelHeatingCoil`: the load a simple electric or
//! fuel heating coil delivers, by load or to its set point and limited by its capacity, its
//! electric and fuel use with the efficiency and parasitics, the part load fraction curve's
//! runtime fraction (a quadratic, as `Curve::value` evaluates it), and the outlet air state. Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::{PsyCpAirFnW, PsyHFnTdbW};

/// `Node::SensedLoadFlagValue`.
const SENSED_LOAD_FLAG_VALUE: f64 = -999.0;
/// `HVAC::TempControlTol` (C).
const TEMP_CONTROL_TOL: f64 = 0.1;

/// One coil as `InitHeatingCoil` leaves it, and the results the calc routines set.
#[derive(Default)]
struct Coil {
    avail: f64,
    mdot: f64,
    tin: f64,
    win: f64,
    tset: f64,
    eff: f64,
    cap: f64,
    par_elec: f64,
    par_fuel: f64,
    /// The quadratic part load fraction curve's coefficients, if the coil has one.
    plf: Option<[f64; 3]>,
    // Results.
    load: f64,
    outlet_temp: f64,
    elec_use: f64,
    fuel_use: f64,
    rtf: f64,
    parasitic_fuel_rate: f64,
    on_off_fan_part_load_fraction: f64,
    elec_heating_coil_power: f64,
}

/// The capacity limited load and outlet temperature of a running coil: by load `q_req`, or to the
/// set point. `None` when the coil is not running.
fn coil_load(
    c: &Coil,
    air_mass_flow: f64,
    capacitance_air: f64,
    tset: f64,
    q_req: f64,
) -> Option<(f64, f64)> {
    let runs = (air_mass_flow > 0.0 && c.cap > 0.0) && (c.avail > 0.0);
    if runs && q_req > 0.0 {
        let q = if q_req > c.cap { c.cap } else { q_req };
        Some((q, c.tin + q / capacitance_air))
    } else if runs && q_req == SENSED_LOAD_FLAG_VALUE && (tset - c.tin).abs() > TEMP_CONTROL_TOL {
        let q = capacitance_air * (tset - c.tin);
        if q <= 0.0 {
            Some((0.0, c.tin))
        } else if q > c.cap {
            Some((c.cap, c.tin + c.cap / capacitance_air))
        } else {
            Some((q, tset))
        }
    } else {
        None
    }
}

/// `CalcElectricHeatingCoil`; `q_req` is changed as the C++ changes it.
fn calc_electric(c: &mut Coil, q_req: &mut f64, cycling: bool, plr: f64) {
    let air_mass_flow = if cycling {
        if plr > 0.0 {
            *q_req /= plr;
            c.mdot / plr
        } else {
            0.0
        }
    } else {
        c.mdot
    };
    let capacitance_air = PsyCpAirFnW(c.win) * air_mass_flow;
    let (mut load, temp_air_out) =
        match coil_load(c, air_mass_flow, capacitance_air, c.tset, *q_req) {
            Some((q, t)) => {
                c.elec_use = q / c.eff;
                (q, t)
            }
            None => {
                c.elec_use = 0.0;
                (0.0, c.tin)
            }
        };
    if cycling {
        c.elec_use *= plr;
        load *= plr;
    }
    c.load = load;
    c.rtf = if c.cap > 0.0 { load / c.cap } else { 0.0 };
    c.outlet_temp = temp_air_out;
}

/// `CalcFuelHeatingCoil`.
fn calc_fuel(c: &mut Coil, q_req: f64, cycling: bool) {
    let capacitance_air = PsyCpAirFnW(c.win) * c.mdot;
    let (load, temp_air_out, part_load_rat) =
        match coil_load(c, c.mdot, capacitance_air, c.tset, q_req) {
            Some((q, t)) => {
                let plr = q / c.cap;
                c.fuel_use = q / c.eff;
                c.elec_use = c.par_elec * plr;
                c.parasitic_fuel_rate = c.par_fuel * (1.0 - plr);
                (q, t, plr)
            }
            None => {
                c.fuel_use = 0.0;
                c.elec_use = 0.0;
                c.parasitic_fuel_rate = c.par_fuel;
                (0.0, c.tin, 0.0)
            }
        };
    c.rtf = part_load_rat;
    if let Some(k) = c.plf {
        if part_load_rat == 0.0 {
            c.fuel_use = 0.0;
        } else {
            // `Curve::value` for a quadratic with input limits 0 to 1; the oracle keeps it at 0.7
            // or more, and the runtime fraction within 1.001, so neither warning runs.
            let x = part_load_rat.clamp(0.0, 1.0);
            let plf = k[0] + x * (k[1] + x * k[2]);
            c.rtf = part_load_rat / plf;
            if c.rtf > 1.0 {
                c.rtf = 1.0;
            }
            c.elec_use = c.par_elec * c.rtf;
            c.fuel_use = c.cap / c.eff * c.rtf;
            c.parasitic_fuel_rate = c.par_fuel * (1.0 - c.rtf);
            if cycling {
                c.on_off_fan_part_load_fraction = plf;
            }
        }
    }
    c.load = load;
    c.outlet_temp = temp_air_out;
    c.elec_heating_coil_power = c.elec_use;
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// `c0 + c1 x + c2 x^2 - k x` is at least `m` at 0, 1 and, when it lies between them, the vertex
/// (the oracle's guard, in its order).
fn quad_at_least(c0: f64, c1: f64, c2: f64, k: f64, m: f64) -> bool {
    let b = c1 - k;
    let f = |x: f64| c0 + x * (b + x * c2);
    if f(0.0) < m || f(1.0) < m {
        return false;
    }
    if c2 > 0.0 {
        let xv = -b / (2.0 * c2);
        if xv > 0.0 && xv < 1.0 && f(xv) < m {
            return false;
        }
    }
    true
}

/// Port of `double HeatCoil(int which, int fuel, int cycling, int sensed, int plf, double avail,
/// double mdot, double tin, double win, double tset, double qreq, double plr, double eff, double
/// cap, double parelec, double parfuel, double c0, double c1, double c2)` (`heatcoil.c:9`): see
/// the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn HeatCoil(
    which: std::ffi::c_int,
    fuel: std::ffi::c_int,
    cycling: std::ffi::c_int,
    sensed: std::ffi::c_int,
    plf: std::ffi::c_int,
    avail: f64,
    mdot: f64,
    tin: f64,
    win: f64,
    tset: f64,
    qreq: f64,
    plr: f64,
    eff: f64,
    cap: f64,
    parelec: f64,
    parfuel: f64,
    c0: f64,
    c1: f64,
    c2: f64,
) -> f64 {
    let flag = |v: std::ffi::c_int| v == 0 || v == 1;
    let ok = (0..=9).contains(&which)
        && flag(fuel)
        && flag(cycling)
        && flag(sensed)
        && flag(plf)
        && within(avail, 0.0, 1.0)
        && within(mdot, 0.0, 50.0)
        && within(tin, -40.0, 80.0)
        && within(win, 0.0, 0.04)
        && within(tset, -40.0, 80.0)
        && within(qreq, -1.0e6, 1.0e7)
        && within(plr, 0.0, 1.0)
        && within(eff, 0.05, 1.2)
        && within(cap, 0.0, 1.0e7)
        && within(parelec, 0.0, 1.0e4)
        && within(parfuel, 0.0, 1.0e4)
        && within(c0, -2.0, 2.0)
        && within(c1, -2.0, 2.0)
        && within(c2, -2.0, 2.0)
        && !(plf == 1
            && (!quad_at_least(c0, c1, c2, 0.0, 0.7005)
                || !quad_at_least(c0, c1, c2, 1.0 / 1.0005, 0.0)));
    if !ok {
        return f64::NAN;
    }
    let mut c = Coil {
        avail,
        mdot,
        tin,
        win,
        tset,
        eff,
        cap,
        par_elec: parelec,
        par_fuel: parfuel,
        plf: (plf == 1).then_some([c0, c1, c2]),
        on_off_fan_part_load_fraction: 1.0,
        ..Coil::default()
    };
    let mut q = if sensed == 1 {
        SENSED_LOAD_FLAG_VALUE
    } else {
        qreq
    };
    if fuel == 0 {
        calc_electric(&mut c, &mut q, cycling == 1, plr);
    } else {
        calc_fuel(&mut c, q, cycling == 1);
    }
    match which {
        0 => c.load,
        1 => c.outlet_temp,
        2 => PsyHFnTdbW(c.outlet_temp, c.win),
        3 => c.elec_use,
        4 => c.fuel_use,
        5 => c.rtf,
        6 => c.parasitic_fuel_rate,
        7 => c.on_off_fan_part_load_fraction,
        8 => c.elec_heating_coil_power,
        _ => q,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coils_heat_to_load_or_set_point() {
        let coil = |which, fuel, sensed, qreq| {
            HeatCoil(
                which, fuel, 0, sensed, 1, 1.0, 1.0, 10.0, 0.005, 30.0, qreq, 1.0, 0.8, 20000.0,
                100.0, 50.0, 0.8, 0.2, 0.0,
            )
        };
        assert_eq!(coil(0, 0, 0, 5000.0), 5000.0);
        assert_eq!(coil(0, 0, 0, 50000.0), 20000.0);
        let capped = coil(1, 1, 1, 0.0); // the set point needs more than the capacity
        assert!(capped > 29.0 && capped < 30.0);
        assert_eq!(coil(3, 0, 0, 8000.0), 10000.0);
        assert!(coil(5, 1, 0, 10000.0) < 1.0);
        assert_eq!(coil(0, 0, 0, -5.0), 0.0);
        assert!(
            HeatCoil(
                0, 1, 0, 0, 1, 1.0, 1.0, 10.0, 0.005, 30.0, 1.0, 1.0, 0.8, 1.0, 0.0, 0.0, 0.5, 0.0,
                0.0
            )
            .is_nan()
        );
    }
}
