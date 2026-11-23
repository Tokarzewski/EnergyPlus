//! The Rust port of `spmfollow.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Five `SetPointManager` calculations: the single zone one stage cooling and heating managers
//! (on or off set point by the zone's stage), the single zone minimum and maximum humidity managers
//! (the zone humidity ratio plus the moisture load over the flow, floored), the outdoor air
//! pretreat manager (the outdoor set point that mixes with the return air to the reference set
//! point, limited), and the managers that follow the outdoor or a node's dry or wet bulb plus an
//! offset, limited. `std::clamp` and ObjexxFCL `max` are written out as the C++ library does them.
//! The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use std::ffi::c_int;

/// `HVAC::SmallMassFlow` (kg/s).
const SMALL_MASS_FLOW: f64 = 0.001;

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

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// A 0 or 1 flag.
fn flag(v: c_int) -> bool {
    v == 0 || v == 1
}

/// A temperature, offset or set point the shim accepts.
fn temp(v: f64) -> bool {
    within(v, -100.0, 100.0)
}

/// Set point limits the shim accepts.
fn limits(lo: f64, hi: f64) -> bool {
    temp(lo) && temp(hi) && lo <= hi
}

/// Port of `double OneStage(int heating, int stage, double on, double off)` (`spmfollow.c:12`):
/// `SPMSingleZoneOneStageCooling` and `SPMSingleZoneOneStageHeating`.
pub fn OneStage(heating: c_int, stage: c_int, on: f64, off: f64) -> f64 {
    if !flag(heating) || !(-10..=10).contains(&stage) || !temp(on) || !temp(off) {
        return f64::NAN;
    }
    let off_now = if heating == 1 { stage <= 0 } else { stage >= 0 };
    if off_now { off } else { on }
}

/// Port of `double SZHum(int maxhum, double zonem, double zonew, double hum, double dehum)`
/// (`spmfollow.c:17`): `SPMSingleZoneHum`.
pub fn SZHum(maxhum: c_int, zonem: f64, zonew: f64, hum: f64, dehum: f64) -> f64 {
    if !flag(maxhum)
        || !within(zonem, 0.0, 1000.0)
        || !within(zonew, 0.0, 0.1)
        || !within(hum, -10.0, 10.0)
        || !within(dehum, -10.0, 10.0)
    {
        return f64::NAN;
    }
    if zonem > SMALL_MASS_FLOW {
        let (moisture_load, max_hum) = if maxhum == 1 {
            (dehum, 0.00001)
        } else {
            (hum, 0.0)
        };
        max(max_hum, zonew + moisture_load / zonem)
    } else {
        0.0
    }
}

/// Port of `double Pretreat(int ctrl, double refsp, double retv, double mixm, double oam, double
/// minsp, double maxsp)` (`spmfollow.c:22`): `SPMOutsideAirPretreat`.
pub fn Pretreat(
    ctrl: c_int,
    refsp: f64,
    retv: f64,
    mixm: f64,
    oam: f64,
    minsp: f64,
    maxsp: f64,
) -> f64 {
    if !(0..=3).contains(&ctrl)
        || !temp(refsp)
        || !temp(retv)
        || !within(mixm, 0.0, 1000.0)
        || !within(oam, 0.0, 1000.0)
        || !limits(minsp, maxsp)
    {
        return f64::NAN;
    }
    let is_humidity_set_point = ctrl != 0;
    if mixm <= 0.0 || oam <= 0.0 {
        refsp
    } else if is_humidity_set_point && refsp == 0.0 {
        0.0
    } else {
        let oa_fraction = oam / mixm;
        clamp(retv + (refsp - retv) / oa_fraction, minsp, maxsp)
    }
}

/// Port of `double FollowOA(int wet, double oadb, double oawb, double offset, double mint, double
/// maxt)` (`spmfollow.c:27`): `SPMFollowOutsideAirTemp`.
pub fn FollowOA(wet: c_int, oadb: f64, oawb: f64, offset: f64, mint: f64, maxt: f64) -> f64 {
    if !flag(wet) || !temp(oadb) || !temp(oawb) || !temp(offset) || !limits(mint, maxt) {
        return f64::NAN;
    }
    clamp((if wet == 1 { oawb } else { oadb }) + offset, mint, maxt)
}

/// Port of `double FollowNode(int wet, int more, double nodet, double nodewb, double offset, double
/// mint, double maxt)` (`spmfollow.c:32`): `SPMFollowSysNodeTemp`.
pub fn FollowNode(
    wet: c_int,
    more: c_int,
    nodet: f64,
    nodewb: f64,
    offset: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    if !flag(wet)
        || !flag(more)
        || !temp(nodet)
        || !temp(nodewb)
        || !temp(offset)
        || !limits(mint, maxt)
    {
        return f64::NAN;
    }
    let ref_node_temp = if wet == 0 {
        nodet
    } else if more == 1 {
        nodewb
    } else {
        0.0
    };
    clamp(ref_node_temp + offset, mint, maxt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn managers_pick_and_limit() {
        assert_eq!(OneStage(0, -1, 12.0, 30.0), 12.0);
        assert_eq!(OneStage(1, 0, 40.0, 10.0), 10.0);
        assert_eq!(SZHum(1, 1.0, 0.01, 0.0, -0.02), 0.00001);
        assert_eq!(SZHum(0, 0.0005, 0.01, 0.1, 0.0), 0.0);
        assert_eq!(Pretreat(0, 13.0, 24.0, 2.0, 1.0, 2.0, 30.0), 2.0);
        assert_eq!(Pretreat(1, 0.0, 0.01, 2.0, 1.0, 0.001, 0.02), 0.0);
        assert_eq!(FollowOA(1, 30.0, 20.0, 2.0, 0.0, 21.0), 21.0);
        assert_eq!(FollowNode(1, 0, 30.0, 20.0, 2.0, 0.0, 40.0), 2.0);
        assert!(FollowNode(1, 0, 30.0, 20.0, 2.0, 5.0, 4.0).is_nan());
    }
}
