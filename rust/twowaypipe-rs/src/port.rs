//! The Rust port of `twowaypipe.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HVACInterfaceManager::ManageTwoWayCommonPipe` with a constant speed primary pump: the flows in
//! a two-way common pipe (primary to secondary, and the recirculation on each leg) and the inlet
//! temperatures of both sides, iterated eight times when the supply side's inlet set point leads
//! and four times when the demand side's does. Every expression keeps the C++ order. The oracle's
//! guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use std::ffi::c_int;

/// `DataBranchAirLoopPlant::MassFlowTolerance` (kg/s).
const MASS_FLOW_TOLERANCE: f64 = 0.000000001;
/// `DataPlant::DeltaTempTol` (K).
const DELTA_TEMP_TOL: f64 = 0.0001;
/// Iterations when the supply side leads.
const MAX_ITER_LIMIT_CASE_A: usize = 8;
/// Iterations when the demand side leads.
const MAX_ITER_LIMIT_CASE_B: usize = 4;

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Zero below the mass flow tolerance.
fn floor_flow(m: f64) -> f64 {
    if m < MASS_FLOW_TOLERANCE { 0.0 } else { m }
}

/// The mixed temperature of two streams, or `fallback` with no flow.
fn mix(m_a: f64, t_a: f64, m_b: f64, t_b: f64, fallback: f64) -> f64 {
    if (m_a + m_b) > MASS_FLOW_TOLERANCE {
        (m_a * t_a + m_b * t_b) / (m_a + m_b)
    } else {
        fallback
    }
}

/// The common pipe's state and both inlets' temperatures.
struct Pipe {
    pri_to_sec: f64,
    pri_rc_leg: f64,
    sec_rc_leg: f64,
    temp_sec_inlet: f64,
    temp_pri_inlet: f64,
}

/// `ManageTwoWayCommonPipe` with a constant speed primary pump, given the primary and secondary
/// tank outlet temperatures, inlet set points and outlet flows.
fn two_way(
    supply_leads: bool,
    mut p: Pipe,
    [t_pri_out, t_sec_out]: [f64; 2],
    [t_pri_sp, t_sec_sp]: [f64; 2],
    [mdot_pri, mdot_sec]: [f64; 2],
) -> Pipe {
    if supply_leads {
        for _ in 0..MAX_ITER_LIMIT_CASE_A {
            if (t_sec_out - t_pri_sp).abs() > DELTA_TEMP_TOL {
                p.pri_to_sec = p.pri_rc_leg * (t_pri_sp - t_pri_out) / (t_sec_out - t_pri_sp);
                if p.pri_to_sec < MASS_FLOW_TOLERANCE {
                    p.pri_to_sec = 0.0;
                }
                if p.pri_to_sec > mdot_sec {
                    p.pri_to_sec = mdot_sec;
                }
            } else {
                p.pri_to_sec = mdot_sec;
            }
            p.pri_rc_leg = floor_flow(mdot_pri - p.pri_to_sec);
            p.sec_rc_leg = floor_flow(mdot_sec - p.pri_to_sec);
            p.temp_sec_inlet = mix(p.pri_to_sec, t_pri_out, p.sec_rc_leg, t_sec_out, t_pri_out);
            p.temp_pri_inlet = mix(p.pri_to_sec, t_sec_out, p.pri_rc_leg, t_pri_out, t_sec_out);
        }
    } else {
        for _ in 0..MAX_ITER_LIMIT_CASE_B {
            if (t_pri_out - t_sec_out).abs() > DELTA_TEMP_TOL {
                p.pri_to_sec = mdot_sec * (t_sec_sp - t_sec_out) / (t_pri_out - t_sec_out);
                if p.pri_to_sec < MASS_FLOW_TOLERANCE {
                    p.pri_to_sec = 0.0;
                }
                if p.pri_to_sec > mdot_sec {
                    p.pri_to_sec = mdot_sec;
                }
            } else {
                p.pri_to_sec = mdot_sec;
            }
            p.temp_pri_inlet = mix(p.pri_to_sec, t_sec_out, p.pri_rc_leg, t_pri_out, t_sec_out);
            p.sec_rc_leg = floor_flow(mdot_sec - p.pri_to_sec);
            p.pri_rc_leg = floor_flow(mdot_pri - p.pri_to_sec);
            p.temp_sec_inlet = mix(p.pri_to_sec, t_pri_out, p.sec_rc_leg, t_sec_out, t_pri_out);
        }
    }
    p
}

/// Port of `double TwoWayPipe(int which, int side, int psp, int ssp, ...)` (`twowaypipe.c:8`):
/// `ManageTwoWayCommonPipe`, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn TwoWayPipe(
    which: c_int,
    side: c_int,
    psp: c_int,
    ssp: c_int,
    tank: f64,
    other: f64,
    tpsp: f64,
    tssp: f64,
    tpin: f64,
    tsin: f64,
    msec: f64,
    mpri: f64,
    pts: f64,
    prc: f64,
    src: f64,
) -> f64 {
    let flag = |v: c_int| v == 0 || v == 1;
    let temp = |v: f64| within(v, -20.0, 150.0);
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    if which > 5
        || !flag(side)
        || !flag(psp)
        || !flag(ssp)
        || ![tank, other, tpsp, tssp, tpin, tsin]
            .iter()
            .all(|&t| temp(t))
        || ![msec, mpri, pts, prc, src]
            .iter()
            .all(|&m| within(m, 0.0, 1000.0))
    {
        return f64::NAN;
    }
    let supply_side = side == 1;
    let (t_sec_out, t_pri_out) = if supply_side {
        (tank, other)
    } else {
        (other, tank)
    };
    // The demand side leads only when its inlet has a set point and the supply side's does not;
    // otherwise (both, neither, or only the supply side's) the supply side leads.
    let supply_leads = !(psp == 0 && ssp == 1);
    let start = Pipe {
        pri_to_sec: pts,
        pri_rc_leg: prc,
        sec_rc_leg: src,
        temp_sec_inlet: tsin,
        temp_pri_inlet: tpin,
    };
    let p = two_way(
        supply_leads,
        start,
        [t_pri_out, t_sec_out],
        [tpsp, tssp],
        [mpri, msec],
    );
    [
        p.pri_to_sec,
        p.pri_to_sec,
        p.pri_rc_leg,
        p.sec_rc_leg,
        p.temp_sec_inlet,
        p.temp_pri_inlet,
    ][which]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supply_led_pipe_reaches_the_primary_set_point() {
        // Primary set point 7 C, primary tank 6 C, secondary returns 12 C: the primary inlet mixes to 7 C.
        let f = |which| {
            TwoWayPipe(
                which, 1, 1, 0, 12.0, 6.0, 7.0, 7.0, 7.0, 7.0, 4.0, 5.0, 0.0, 1.0, 0.0,
            )
        };
        assert!((f(5) - 7.0).abs() < 1.0e-4);
        assert_eq!(f(0), f(1));
        assert!(f(6).is_nan());
    }

    #[test]
    fn demand_led_pipe_meets_the_secondary_set_point() {
        let f = |which| {
            TwoWayPipe(
                which, 0, 0, 1, 6.0, 12.0, 7.0, 8.0, 7.0, 7.0, 4.0, 5.0, 0.0, 1.0, 0.0,
            )
        };
        assert!((f(4) - 8.0).abs() < 1.0e-9);
    }
}
