//! The Rust port of `plantiface.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HVACInterfaceManager::UpdatePlantLoopInterface` for a loop without a common pipe, which every
//! plant and condenser loop runs after simulating each half: the other side's inlet enthalpy from
//! its old temperature, its new temperature from the half-loop tank (`halfloop-rs`, unit 84), the
//! flow and temperature differences pushed onto the convergence histories (rotated, newest first)
//! against their tolerances, and the outlet's flows, quality and pressure passed across. The water
//! specific heat comes from `halfloop-rs`'s cached lookup. `std::min` and `std::max` are written
//! out as the C++ library does them. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use halfloop_rs::port::{half_loop, water_cp};
use std::ffi::c_int;

/// `DataConvergParams::PlantFlowRateToler` (kg/s).
const PLANT_FLOW_RATE_TOLER: f64 = 0.001;
/// `DataConvergParams::PlantFlowRateRelToler` (fraction of flow).
const PLANT_FLOW_RATE_REL_TOLER: f64 = 0.01;
/// `DataEnvironment::StdPressureSeaLevel` (Pa), a fresh node's pressure.
const STD_PRESSURE_SEA_LEVEL: f64 = 101325.0;
/// `DataConvergParams::PlantTemperatureToler` (K).
const PLANT_TEMPERATURE_TOLER: f64 = 0.01;

/// `std::max(a, b)`.
fn std_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `std::min(a, b)`.
fn std_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The flow difference and whether it is out of tolerance (`isPlantFlowOutOfTolerance`).
fn flow_out_of_tolerance(first: f64, second: f64) -> (f64, bool) {
    let difference = (first - second).abs();
    let max_mass_flow = std_max(first.abs(), second.abs());
    let tolerance = std_min(
        PLANT_FLOW_RATE_TOLER,
        PLANT_FLOW_RATE_REL_TOLER * max_mass_flow,
    );
    (difference, difference > tolerance)
}

/// Port of `double PlantIface(int which, int side, int newstep, int haspress, int oot0, ...)`
/// (`plantiface.c:8`): `UpdatePlantLoopInterface` without a common pipe, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn PlantIface(
    which: c_int,
    side: c_int,
    newstep: c_int,
    haspress: c_int,
    oot0: c_int,
    tin: f64,
    mdot: f64,
    minav: f64,
    maxav: f64,
    q: f64,
    p: f64,
    otemp: f64,
    omdot: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    tssec: f64,
    hist: f64,
) -> f64 {
    let flag = |v: c_int| v == 0 || v == 1;
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    if which > 13
        || !flag(side)
        || !flag(newstep)
        || !flag(haspress)
        || !flag(oot0)
        || !within(tin, -20.0, 150.0)
        || ![mdot, minav, maxav, omdot]
            .iter()
            .all(|&m| within(m, 0.0, 1000.0))
        || !within(q, 0.0, 1.0)
        || !within(p, 0.0, 1.0e7)
        || ![otemp, last, prev].iter().all(|&t| within(t, 1.0, 99.0))
        || !within(pump, 0.0, 1.0e6)
        || !within(mass, 0.0, 1.0e7)
        || !within(tssec, 1.0, 3600.0)
        || !within(hist, -1000.0, 1000.0)
    {
        return f64::NAN;
    }
    let demand = side == 0;
    let old_tank_outlet_temp = otemp;
    let cp = water_cp(old_tank_outlet_temp);
    let enthalpy = cp * otemp;
    // A new time step makes the last call's tank outlet temperature the last time step's.
    let last_tank_outlet_temp = if newstep == 1 { prev } else { last };
    let tank_outlet_temp = half_loop(tin, mdot, pump, mass, last_tank_outlet_temp, tssec)[0];
    // Every history slot holds `hist`, so the rotation leaves `hist` next after the newest entry.
    let (flow_difference, mass_flow_not_converged) = flow_out_of_tolerance(mdot, omdot);
    let temp_difference = (old_tank_outlet_temp - tank_outlet_temp).abs();
    let temp_not_converged = temp_difference > PLANT_TEMPERATURE_TOLER;
    let out_of_tolerance = oot0 == 1 || mass_flow_not_converged || (demand && temp_not_converged);
    // With pressure components the pressure is left as the shim's fresh node has it.
    let press = if haspress == 1 {
        STD_PRESSURE_SEA_LEVEL
    } else {
        p
    };
    let r = [
        f64::from(u8::from(out_of_tolerance)),
        f64::from(u8::from(mass_flow_not_converged)),
        f64::from(u8::from(temp_not_converged)),
        tank_outlet_temp,
        mdot,
        minav,
        maxav,
        q,
        press,
        enthalpy,
        flow_difference,
        temp_difference,
        hist,
        hist,
    ];
    r[which]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converged_interface_raises_nothing() {
        let f = |which, omdot, otemp| {
            PlantIface(
                which, 1, 0, 0, 0, 20.0, 5.0, 0.0, 6.0, 0.0, 2.0e5, otemp, omdot, 0.0, 0.0, 20.0,
                20.0, 600.0, 7.0,
            )
        };
        assert_eq!(f(0, 5.0, 20.0), 0.0);
        assert_eq!(f(3, 5.0, 20.0), 20.0);
        assert_eq!(f(0, 4.0, 20.0), 1.0);
        assert_eq!(f(2, 5.0, 25.0), 1.0);
        assert_eq!(f(0, 5.0, 25.0), 0.0);
        assert_eq!(f(12, 5.0, 25.0), 7.0);
        assert!(f(14, 5.0, 25.0).is_nan());
    }
}
