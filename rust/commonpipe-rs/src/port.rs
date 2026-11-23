//! The Rust port of `commonpipe.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HVACInterfaceManager::UpdateCommonPipe` with a single common pipe, and through it
//! `ManageSingleCommonPipe`, which primary-secondary plant loops run after simulating each half:
//! the other side's tank (a quarter of the loop's mass on the demand side's outlet, three quarters
//! on the supply side's) over the system time step, then the flow in the common pipe (primary or
//! secondary recirculation, or none) and the mixed inlet temperatures of both sides. The water
//! specific heat comes from `halfloop-rs`'s cached lookup. Every expression keeps the C++ order.
//! The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use halfloop_rs::port::water_cp;
use std::ffi::c_int;

/// `DataBranchAirLoopPlant::MassFlowTolerance` (kg/s).
const MASS_FLOW_TOLERANCE: f64 = 0.000000001;
/// No flow in the common pipe.
const NO_RECIRC_FLOW: c_int = 0;
/// Flow from the supply outlet to the supply inlet.
const PRIMARY_RECIRC: c_int = 1;
/// Flow from the demand outlet to the demand inlet.
const SECONDARY_RECIRC: c_int = 2;

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The tank's final and average temperatures (`UpdateCommonPipe`'s capacitance calculation).
fn tank(
    tank_inlet_temp: f64,
    mass_flow_rate: f64,
    pump_heat: f64,
    this_tank_mass: f64,
    last_tank_outlet_temp: f64,
    time_step_seconds: f64,
) -> (f64, f64) {
    let cp = water_cp(last_tank_outlet_temp);
    if this_tank_mass <= 0.0 {
        if mass_flow_rate > 0.0 {
            let fin = tank_inlet_temp + pump_heat / (mass_flow_rate * cp);
            (fin, (fin + last_tank_outlet_temp) / 2.0)
        } else {
            (last_tank_outlet_temp, last_tank_outlet_temp)
        }
    } else if mass_flow_rate > 0.0 {
        let fin = (last_tank_outlet_temp
            - (mass_flow_rate * cp * tank_inlet_temp + pump_heat) / (mass_flow_rate * cp))
            * (-(mass_flow_rate * cp) / (this_tank_mass * cp) * time_step_seconds).exp()
            + (mass_flow_rate * cp * tank_inlet_temp + pump_heat) / (mass_flow_rate * cp);
        let avg = (this_tank_mass * cp) / (mass_flow_rate * cp)
            * (last_tank_outlet_temp
                - (mass_flow_rate * cp * tank_inlet_temp + pump_heat) / (mass_flow_rate * cp))
            * (1.0 - (-(mass_flow_rate * cp) / (this_tank_mass * cp) * time_step_seconds).exp())
            / time_step_seconds
            + (mass_flow_rate * cp * tank_inlet_temp + pump_heat) / (mass_flow_rate * cp);
        (fin, avg)
    } else {
        let fin = pump_heat / (this_tank_mass * cp) * time_step_seconds + last_tank_outlet_temp;
        (fin, (fin + last_tank_outlet_temp) / 2.0)
    }
}

/// `ManageSingleCommonPipe`: the mixed outlet, secondary and primary inlet temperatures, and the
/// common pipe's flow, temperature and direction.
fn single_common_pipe(
    outlet_is_supply: bool,
    tank_outlet_temp: f64,
    other_tank_temp: f64,
    mdot_sec: f64,
    mdot_pri: f64,
) -> [f64; 6] {
    let (temp_sec_out_tank_out, temp_pri_out_tank_out) = if outlet_is_supply {
        (tank_outlet_temp, other_tank_temp)
    } else {
        (other_tank_temp, tank_outlet_temp)
    };
    let (mut mdot_pri_rc_leg, mut mdot_sec_rc_leg) = (0.0, 0.0);
    let cp_flow_dir;
    let common_pipe_temp;
    if mdot_pri > mdot_sec {
        mdot_pri_rc_leg = mdot_pri - mdot_sec;
        if mdot_pri_rc_leg < MASS_FLOW_TOLERANCE {
            mdot_pri_rc_leg = 0.0;
            cp_flow_dir = NO_RECIRC_FLOW;
        } else {
            cp_flow_dir = PRIMARY_RECIRC;
        }
        common_pipe_temp = temp_pri_out_tank_out;
    } else if mdot_pri < mdot_sec {
        mdot_sec_rc_leg = mdot_sec - mdot_pri;
        if mdot_sec_rc_leg < MASS_FLOW_TOLERANCE {
            mdot_sec_rc_leg = 0.0;
            cp_flow_dir = NO_RECIRC_FLOW;
        } else {
            cp_flow_dir = SECONDARY_RECIRC;
        }
        common_pipe_temp = temp_sec_out_tank_out;
    } else {
        cp_flow_dir = NO_RECIRC_FLOW;
        common_pipe_temp = (temp_pri_out_tank_out + temp_sec_out_tank_out) / 2.0;
    }
    let temp_sec_inlet = if mdot_sec > 0.0 {
        (mdot_pri * temp_pri_out_tank_out + mdot_sec_rc_leg * temp_sec_out_tank_out
            - mdot_pri_rc_leg * temp_pri_out_tank_out)
            / mdot_sec
    } else {
        temp_pri_out_tank_out
    };
    let temp_pri_inlet = if mdot_pri > 0.0 {
        (mdot_sec * temp_sec_out_tank_out + mdot_pri_rc_leg * temp_pri_out_tank_out
            - mdot_sec_rc_leg * temp_sec_out_tank_out)
            / mdot_pri
    } else {
        temp_sec_out_tank_out
    };
    let mixed = if outlet_is_supply {
        temp_pri_inlet
    } else {
        temp_sec_inlet
    };
    [
        mixed,
        temp_sec_inlet,
        temp_pri_inlet,
        max(mdot_pri_rc_leg, mdot_sec_rc_leg),
        common_pipe_temp,
        f64::from(cp_flow_dir),
    ]
}

/// Port of `double CommonPipe(int which, int side, int newstep, double tin, double msec, double
/// mpri, double pump, double mass, double last, double prev, double other, double tssec)`
/// (`commonpipe.c:8`): `UpdateCommonPipe` with a single common pipe, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn CommonPipe(
    which: c_int,
    side: c_int,
    newstep: c_int,
    tin: f64,
    msec: f64,
    mpri: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    other: f64,
    tssec: f64,
) -> f64 {
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    if which > 7
        || (side != 0 && side != 1)
        || (newstep != 0 && newstep != 1)
        || !within(tin, -20.0, 150.0)
        || !within(msec, 0.0, 1000.0)
        || !within(mpri, 0.0, 1000.0)
        || !within(pump, 0.0, 1.0e6)
        || !within(mass, 0.0, 1.0e7)
        || !within(last, 1.0, 99.0)
        || !within(prev, 1.0, 99.0)
        || !within(other, -20.0, 150.0)
        || !within(tssec, 1.0, 3600.0)
    {
        return f64::NAN;
    }
    let inlet_is_supply = side == 1;
    let frac_tot_loop_mass = if inlet_is_supply { 0.75 } else { 0.25 };
    // A new time step makes the last call's tank outlet temperature the last time step's.
    let last_tank_outlet_temp = if newstep == 1 { prev } else { last };
    let mass_flow_rate = if inlet_is_supply { mpri } else { msec };
    let (fin, avg) = tank(
        tin,
        mass_flow_rate,
        pump,
        frac_tot_loop_mass * mass,
        last_tank_outlet_temp,
        tssec,
    );
    let pipe = single_common_pipe(!inlet_is_supply, avg, other, msec, mpri);
    match which {
        6 => fin,
        7 => avg,
        k => pipe[k],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_excess_recirculates() {
        let f = |which| CommonPipe(which, 1, 0, 7.0, 4.0, 6.0, 0.0, 0.0, 7.0, 7.0, 12.0, 600.0);
        assert_eq!(f(5), 1.0);
        assert_eq!(f(3), 2.0);
        assert_eq!(f(4), 7.0);
        // Secondary inlet: all primary flow at 7 C, less the 2 kg/s recirculating.
        assert_eq!(f(1), (6.0 * 7.0 - 2.0 * 7.0) / 4.0);
        assert_eq!(f(0), f(1));
        assert!(f(8).is_nan());
    }

    #[test]
    fn equal_flows_do_not_recirculate() {
        assert_eq!(
            CommonPipe(5, 0, 1, 12.0, 3.0, 3.0, 0.0, 0.0, 7.0, 7.0, 7.0, 600.0),
            0.0
        );
    }
}
