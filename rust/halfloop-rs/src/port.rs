//! The Rust port of `halfloop.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HVACInterfaceManager::UpdateHalfLoopInletTemp`, which every plant loop runs each time one half
//! of it has been simulated: the half loop is a well-mixed tank of half the loop's mass, fed by the
//! simulated side's outlet and heated by the pumps; its final and average temperatures over the
//! system time step come from the exact solution (or the steady one when the exponent is 700 or
//! more), and give the loop side inlet's energy terms. The water specific heat comes from
//! `fluidproperties-rs`, cached by temperature as the C++ caches it. Every expression keeps the C++
//! order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use fluidproperties_rs::port::WaterSpecificHeat;
use std::ffi::c_int;

/// Fraction of the total loop mass assigned to the half loop.
const FRAC_TOT_LOOP_MASS: f64 = 0.5;

thread_local! {
    /// The last temperature's water specific heat (bits of the temperature, value).
    static CP_CACHE: std::cell::Cell<(u64, f64)> = const { std::cell::Cell::new((u64::MAX, 0.0)) };
}

/// `GlycolProps::getSpecificHeat` for water, which caches its results by temperature; this keeps
/// the last one (the value is the table's either way).
fn water_cp(t: f64) -> f64 {
    let key = t.to_bits();
    let (k, v) = CP_CACHE.get();
    if k == key {
        return v;
    }
    let cp = WaterSpecificHeat(t);
    CP_CACHE.set((key, cp));
    cp
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The tank's average and final temperatures, inlet m cp dT, M cp dT/dt and the last time step's
/// outlet temperature used.
fn half_loop(
    tank_inlet_temp: f64,
    mass_flow_rate: f64,
    pump_heat: f64,
    loop_mass: f64,
    last_tank_outlet_temp: f64,
    time_step_seconds: f64,
) -> [f64; 5] {
    let cp = water_cp(last_tank_outlet_temp);
    let this_tank_mass = FRAC_TOT_LOOP_MASS * loop_mass;
    let (tank_final_temp, tank_average_temp);
    if this_tank_mass <= 0.0 {
        if mass_flow_rate > 0.0 {
            tank_final_temp = tank_inlet_temp + pump_heat / (mass_flow_rate * cp);
            tank_average_temp = (tank_final_temp + last_tank_outlet_temp) / 2.0;
        } else {
            tank_final_temp = last_tank_outlet_temp;
            tank_average_temp = last_tank_outlet_temp;
        }
    } else if mass_flow_rate > 0.0 {
        let mdot_cp = mass_flow_rate * cp;
        let mdot_cp_temp_in = mdot_cp * tank_inlet_temp;
        let tank_mass_cp = this_tank_mass * cp;
        let exponent_term = mdot_cp / tank_mass_cp * time_step_seconds;
        if exponent_term >= 700.0 {
            tank_final_temp = (mdot_cp * tank_inlet_temp + pump_heat) / mdot_cp;
            tank_average_temp = tank_mass_cp / mdot_cp
                * (last_tank_outlet_temp - (mdot_cp_temp_in + pump_heat) / mdot_cp)
                / time_step_seconds
                + (mdot_cp_temp_in + pump_heat) / mdot_cp;
        } else {
            tank_final_temp = (last_tank_outlet_temp - (mdot_cp_temp_in + pump_heat) / mdot_cp)
                * (-exponent_term).exp()
                + (mdot_cp_temp_in + pump_heat) / (mass_flow_rate * cp);
            tank_average_temp = tank_mass_cp / mdot_cp
                * (last_tank_outlet_temp - (mdot_cp_temp_in + pump_heat) / mdot_cp)
                * (1.0 - (-exponent_term).exp())
                / time_step_seconds
                + (mdot_cp_temp_in + pump_heat) / mdot_cp;
        }
    } else {
        tank_final_temp =
            pump_heat / (this_tank_mass * cp) * time_step_seconds + last_tank_outlet_temp;
        tank_average_temp = (tank_final_temp + last_tank_outlet_temp) / 2.0;
    }
    [
        tank_average_temp,
        tank_final_temp,
        (tank_inlet_temp - tank_average_temp) * cp * mass_flow_rate,
        (this_tank_mass * cp * (tank_final_temp - last_tank_outlet_temp)) / time_step_seconds,
        last_tank_outlet_temp,
    ]
}

/// Port of `double HalfLoop(int which, int side, int newstep, double tin, double mdot, double pump,
/// double mass, double last, double prev, double tssec)` (`halfloop.c:8`): `UpdateHalfLoopInletTemp`,
/// see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn HalfLoop(
    which: c_int,
    side: c_int,
    newstep: c_int,
    tin: f64,
    mdot: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    tssec: f64,
) -> f64 {
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    if which > 4
        || (side != 0 && side != 1)
        || (newstep != 0 && newstep != 1)
        || !within(tin, -20.0, 150.0)
        || !within(mdot, 0.0, 1000.0)
        || !within(pump, 0.0, 1.0e6)
        || !within(mass, 0.0, 1.0e7)
        || !within(last, 1.0, 99.0)
        || !within(prev, 1.0, 99.0)
        || !within(tssec, 1.0, 3600.0)
    {
        return f64::NAN;
    }
    // A new time step makes the last call's tank outlet temperature the last time step's.
    let last_tank_outlet_temp = if newstep == 1 { prev } else { last };
    half_loop(tin, mdot, pump, mass, last_tank_outlet_temp, tssec)[which]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tank_relaxes_towards_the_inlet() {
        let avg = HalfLoop(0, 0, 0, 30.0, 5.0, 0.0, 2000.0, 20.0, 20.0, 600.0);
        let fin = HalfLoop(1, 0, 0, 30.0, 5.0, 0.0, 2000.0, 20.0, 20.0, 600.0);
        assert!(avg > 20.0 && avg < fin && fin < 30.0);
        assert_eq!(
            HalfLoop(0, 1, 0, 30.0, 0.0, 0.0, 0.0, 20.0, 25.0, 600.0),
            20.0
        );
        assert_eq!(
            HalfLoop(4, 1, 1, 30.0, 0.0, 0.0, 0.0, 20.0, 25.0, 600.0),
            25.0
        );
        assert!(HalfLoop(5, 1, 1, 30.0, 0.0, 0.0, 0.0, 20.0, 25.0, 600.0).is_nan());
    }
}
