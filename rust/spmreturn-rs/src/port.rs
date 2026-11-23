//! The Rust port of `spmreturn.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `SetPointManager`'s return water temperature reset (`SPMReturnWaterTemp`: the supply set point
//! that keeps the loop's temperature difference and returns at the target, limited; at the limit
//! with no flow or a reversed difference) and system node reset (`SPMSystemNode`: the set point
//! interpolated in a reference node's temperature or humidity ratio, through `interpSetPoint`).
//! `std::clamp` is written out as the C++ library does it. The oracle's guards (see its shim)
//! return NaN here too.

#![forbid(unsafe_code)]

use std::ffi::c_int;

/// `DataConvergParams::PlantFlowRateToler` (kg/s).
const PLANT_FLOW_RATE_TOLER: f64 = 0.001;

/// `std::clamp(v, lo, hi)`.
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    if v < lo {
        lo
    } else if hi < v {
        hi
    } else {
        v
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// A temperature, target, set point or reference the shim accepts.
fn temp(v: f64) -> bool {
    within(v, -100.0, 200.0)
}

/// `SetPointManager::interpSetPoint`.
fn interp_set_point(
    low_val: f64,
    high_val: f64,
    ref_val: f64,
    setpt_at_low_val: f64,
    setpt_at_high_val: f64,
) -> f64 {
    if low_val >= high_val {
        return 0.5 * (setpt_at_low_val + setpt_at_high_val);
    }
    if ref_val <= low_val {
        return setpt_at_low_val;
    }
    if ref_val >= high_val {
        return setpt_at_high_val;
    }
    setpt_at_low_val
        - ((ref_val - low_val) / (high_val - low_val)) * (setpt_at_low_val - setpt_at_high_val)
}

/// Port of `double RetWater(int hot, int ttype, double mdot, double tsup, double tret, double target,
/// double retsp, double mint, double maxt)` (`spmreturn.c:9`): `SPMReturnWaterTemp::calculate`.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn RetWater(
    hot: c_int,
    ttype: c_int,
    mdot: f64,
    tsup: f64,
    tret: f64,
    target: f64,
    retsp: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    if (hot != 0 && hot != 1)
        || !(0..=2).contains(&ttype)
        || !within(mdot, 0.0, 1000.0)
        || ![tsup, tret, target, retsp, mint, maxt]
            .iter()
            .all(|&t| temp(t))
        || mint > maxt
    {
        return f64::NAN;
    }
    let chilled = hot == 0;
    let delta_t = if chilled { tret - tsup } else { tsup - tret };
    let limit = if chilled { mint } else { maxt };
    if delta_t < 0.0 {
        return limit;
    }
    // ttype 0 scheduled (the schedule's value is target), 1 constant, 2 the return node's set point.
    let t_return_target = if ttype == 2 { retsp } else { target };
    let mut t_supply_setpoint = limit;
    if mdot > PLANT_FLOW_RATE_TOLER {
        t_supply_setpoint = t_return_target + if chilled { -delta_t } else { delta_t };
    }
    clamp(t_supply_setpoint, mint, maxt)
}

/// Port of `double SysNode(int ctrl, double ref, double lowref, double highref, double splow, double
/// sphigh)` (`spmreturn.c:14`): `SPMSystemNode::calculate`.
pub fn SysNode(ctrl: c_int, r#ref: f64, lowref: f64, highref: f64, splow: f64, sphigh: f64) -> f64 {
    if !(0..=6).contains(&ctrl)
        || ![r#ref, lowref, highref, splow, sphigh]
            .iter()
            .all(|&t| temp(t))
    {
        return f64::NAN;
    }
    // Temperatures (0 to 2) and humidity ratios (3 to 5) read the node; mass flow (6) reads nothing.
    let ref_value = if ctrl <= 5 { r#ref } else { 0.0 };
    interp_set_point(lowref, highref, ref_value, splow, sphigh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chilled_water_reset_keeps_the_difference() {
        assert_eq!(RetWater(0, 1, 10.0, 7.0, 12.0, 14.0, 0.0, 5.0, 10.0), 9.0);
        assert_eq!(RetWater(0, 1, 10.0, 7.0, 6.0, 14.0, 0.0, 5.0, 10.0), 5.0);
        assert_eq!(RetWater(1, 2, 0.0, 60.0, 50.0, 0.0, 45.0, 40.0, 70.0), 70.0);
        assert!(RetWater(1, 2, 0.0, 60.0, 50.0, 0.0, 45.0, 80.0, 70.0).is_nan());
    }

    #[test]
    fn system_node_interpolates() {
        assert_eq!(SysNode(0, 15.0, 10.0, 20.0, 30.0, 20.0), 25.0);
        assert_eq!(SysNode(6, 15.0, 10.0, 20.0, 30.0, 20.0), 30.0);
        assert_eq!(SysNode(3, 15.0, 20.0, 10.0, 30.0, 20.0), 25.0);
    }
}
