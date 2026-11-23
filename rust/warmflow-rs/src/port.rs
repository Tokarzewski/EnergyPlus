//! The Rust port of `warmflow.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `SetPointManager`'s warmest temperature and flow manager (`SPMWarmestTempFlow`: the highest
//! supply temperature and lowest turndown that meet every cooled zone's load, temperature first or
//! flow first, limited, with the critical zone) and return air bypass flow manager
//! (`SPMReturnAirBypassFlow`: the bypass flow that mixes the supply air to the scheduled set point
//! less the fan heat, limited to the total flow). Uses `psychrometrics-rs`. `std::clamp` and
//! ObjexxFCL `max` are written out as the C++ library does them. The oracle's guards (see its
//! shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::PsyCpAirFnW;
use std::ffi::c_int;

/// `HVAC::SmallMassFlow` (kg/s).
const SMALL_MASS_FLOW: f64 = 0.001;
/// `HVAC::SmallLoad` (W).
const SMALL_LOAD: f64 = 1.0;

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

/// A temperature the shim accepts (C).
fn temp(v: f64) -> bool {
    within(v, -60.0, 80.0)
}

/// The set point, turndown and critical zone for the cooled zones `(load, zt, mmax, w)`.
fn warmest(
    flow_first: bool,
    zones: &[[f64; 4]],
    min_t: f64,
    max_t: f64,
    min_frac: f64,
) -> [f64; 3] {
    let mut tot_cool_load = 0.0;
    let mut set_point = max_t;
    let mut frac_flow = min_frac;
    let mut crit_temp = 0;
    let mut crit_flow = 0;
    for (k, &[load, zt, mmax, w]) in zones.iter().enumerate() {
        let mut zone_set_point = max_t;
        let mut zone_frac_flow = min_frac;
        if load < 0.0 {
            tot_cool_load += load.abs();
            let cp_air = PsyCpAirFnW(w);
            if mmax > SMALL_MASS_FLOW {
                if flow_first {
                    zone_frac_flow = (load / (cp_air * (max_t - zt))) / mmax;
                    zone_set_point = if zone_frac_flow > 1.0 || zone_frac_flow < 0.0 {
                        zt + load / (cp_air * mmax)
                    } else {
                        max_t
                    };
                } else {
                    zone_set_point = zt + load / (cp_air * mmax * min_frac);
                    zone_frac_flow = if zone_set_point < min_t {
                        (load / (cp_air * (min_t - zt))) / mmax
                    } else {
                        min_frac
                    };
                }
            }
        }
        if zone_set_point < set_point {
            set_point = zone_set_point;
            crit_temp = k + 1;
        }
        if zone_frac_flow > frac_flow {
            frac_flow = zone_frac_flow;
            crit_flow = k + 1;
        }
    }
    set_point = clamp(set_point, min_t, max_t);
    frac_flow = clamp(frac_flow, min_frac, 1.0);
    if tot_cool_load < SMALL_LOAD {
        set_point = max_t;
        frac_flow = min_frac;
    }
    let crit = if flow_first {
        if crit_temp != 0 { crit_temp } else { crit_flow }
    } else if crit_flow != 0 {
        crit_flow
    } else {
        crit_temp
    };
    // INFALLIBLE: at most four zones, so the number fits exactly in an f64.
    [
        set_point,
        frac_flow,
        f64::from(u8::try_from(crit).unwrap_or(0)),
    ]
}

/// Port of `double WarmFlow(int which, int flowfirst, int nc, ...)` (`warmflow.c:9`):
/// `SPMWarmestTempFlow::calculate`, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per zone field the shim sets"
)]
pub fn WarmFlow(
    which: c_int,
    flowfirst: c_int,
    nc: c_int,
    mint: f64,
    maxt: f64,
    minturn: f64,
    load1: f64,
    zt1: f64,
    mmax1: f64,
    w1: f64,
    load2: f64,
    zt2: f64,
    mmax2: f64,
    w2: f64,
    load3: f64,
    zt3: f64,
    mmax3: f64,
    w3: f64,
    load4: f64,
    zt4: f64,
    mmax4: f64,
    w4: f64,
) -> f64 {
    let zs = [
        [load1, zt1, mmax1, w1],
        [load2, zt2, mmax2, w2],
        [load3, zt3, mmax3, w3],
        [load4, zt4, mmax4, w4],
    ];
    let bad = |z: &[f64; 4]| {
        !within(z[0], -1.0e7, 1.0e7)
            || !temp(z[1])
            || !within(z[2], 0.0, 1000.0)
            || !within(z[3], 0.0, 0.1)
    };
    let (Ok(which), Ok(nc)) = (usize::try_from(which), usize::try_from(nc)) else {
        return f64::NAN;
    };
    if which > 2
        || (flowfirst != 0 && flowfirst != 1)
        || !(1..=4).contains(&nc)
        || !temp(mint)
        || !temp(maxt)
        || mint > maxt
        || !within(minturn, 0.0, 1.0)
        || zs.iter().any(bad)
    {
        return f64::NAN;
    }
    warmest(flowfirst == 1, &zs[..nc], mint, maxt, minturn)[which]
}

/// Port of `double RABypass(double spt, double tloop, double tmix, double msup, double tsup, double
/// mtot, double trab)` (`warmflow.c:14`): `SPMReturnAirBypassFlow::calculate`.
pub fn RABypass(
    spt: f64,
    tloop: f64,
    tmix: f64,
    msup: f64,
    tsup: f64,
    mtot: f64,
    trab: f64,
) -> f64 {
    if !temp(spt)
        || !temp(tloop)
        || !temp(tmix)
        || !within(msup, 0.0, 1000.0)
        || !temp(tsup)
        || !within(mtot, 0.0, 1000.0)
        || !temp(trab)
    {
        return f64::NAN;
    }
    let temp_set_pt_mod = spt - (tloop - tmix);
    let rab_flow = (mtot * temp_set_pt_mod - msup * tsup) / max(trab, 1.0);
    clamp(rab_flow, 0.0, mtot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warmest_meets_the_hungriest_zone() {
        let f = |which, ff| {
            WarmFlow(
                which, ff, 2, 12.0, 18.0, 0.3, -3000.0, 24.0, 1.0, 0.008, 100.0, 22.0, 1.0, 0.008,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )
        };
        // Temperature first: zone 1 needs 24 C plus its load over the minimum flow, about 14 C, which
        // the limits allow, so the turndown stays at its minimum.
        let cp = PsyCpAirFnW(0.008);
        assert_eq!(f(0, 0), 24.0 + -3000.0 / (cp * 1.0 * 0.3));
        assert_eq!(f(1, 0), 0.3);
        assert_eq!(f(2, 0), 1.0);
        // Flow first: zone 1 gets enough flow at the 18 C maximum.
        assert_eq!(f(0, 1), 18.0);
        assert_eq!(f(1, 1), (-3000.0 / (cp * (18.0 - 24.0))) / 1.0);
        assert!(f(3, 1).is_nan());
    }

    #[test]
    fn bypass_is_limited_to_the_total_flow() {
        assert_eq!(
            RABypass(13.0, 14.0, 13.0, 1.0, 10.0, 2.0, 24.0),
            (2.0 * 12.0 - 10.0) / 24.0
        );
        assert_eq!(RABypass(10.0, 14.0, 13.0, 2.0, 10.0, 2.0, 24.0), 0.0);
        assert_eq!(RABypass(20.0, 13.0, 13.0, 0.0, 10.0, 2.0, 15.0), 2.0);
    }
}
