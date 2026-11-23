//! The Rust port of `plantnode.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `PlantUtilities`' node helpers, which plant components call when they initialise, pass flow
//! through or bound a flow: `InitComponentNodes` (flow limits floored at zero, flows reset),
//! `SafeCopyPlantNode` (the inlet's state copied to the outlet, the available flows narrowed to the
//! hard limits, the pressure only on a loop without pressure simulation),
//! `BoundValueToNodeMinMaxAvail`, `TightenNodeMinMaxAvails` and `BoundValueToWithinTwoValues`.
//! `min` and `max` are ObjexxFCL's, which keep the C++ result for signed zeros. The oracle's
//! guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use std::ffi::c_int;

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// A flow the shim accepts (kg/s).
fn flow(v: f64) -> bool {
    within(v, -1000.0, 1000.0)
}

/// `r[which]`, or NaN outside it.
fn pick<const N: usize>(which: c_int, r: [f64; N]) -> f64 {
    usize::try_from(which)
        .ok()
        .and_then(|i| r.get(i).copied())
        .unwrap_or(f64::NAN)
}

/// Port of `double InitNodes(int which, double minm, double maxm)` (`plantnode.c:12`):
/// `InitComponentNodes`, see the oracle's shim.
pub fn InitNodes(which: c_int, minm: f64, maxm: f64) -> f64 {
    if !flow(minm) || !flow(maxm) {
        return f64::NAN;
    }
    let tmp_min = if minm < 0.0 { 0.0 } else { minm };
    let tmp_max = if maxm < 0.0 { 0.0 } else { maxm };
    pick(which, [0.0, tmp_min, tmp_min, tmp_max, tmp_max, 0.0, 0.0])
}

/// Port of `double SafeCopy(int which, int lp, ...)` (`plantnode.c:17`): `SafeCopyPlantNode`,
/// see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per node field the shim sets"
)]
pub fn SafeCopy(
    which: c_int,
    lp: c_int,
    t: f64,
    m: f64,
    q: f64,
    h: f64,
    tmin: f64,
    tmax: f64,
    mmin: f64,
    mminav: f64,
    mmax: f64,
    mmaxav: f64,
    w: f64,
    p: f64,
    op: f64,
) -> f64 {
    if !(0..=2).contains(&lp)
        || !within(t, -100.0, 500.0)
        || !within(q, 0.0, 1.0)
        || !within(h, -1.0e7, 1.0e7)
        || !within(tmin, -100.0, 500.0)
        || !within(tmax, -100.0, 500.0)
        || !within(w, 0.0, 0.1)
        || !within(p, 0.0, 1.0e7)
        || !within(op, 0.0, 1.0e7)
        || [m, mmin, mminav, mmax, mmaxav].iter().any(|&f| !flow(f))
    {
        return f64::NAN;
    }
    // Loop 1 simulates no pressure, so the pressure passes through; loop 2 corrects pump power.
    let press = if lp == 1 { p } else { op };
    pick(
        which,
        [
            t,
            m,
            q,
            h,
            tmin,
            tmax,
            max(mmin, mminav),
            min(mmax, mmaxav),
            w,
            press,
        ],
    )
}

/// Port of `double BoundNode(double v, double minav, double maxav)` (`plantnode.c:22`):
/// `BoundValueToNodeMinMaxAvail`.
pub fn BoundNode(v: f64, minav: f64, maxav: f64) -> f64 {
    BoundTwo(v, minav, maxav)
}

/// Port of `double Tighten(int which, double oldmin, double oldmax, double newmin, double newmax)`
/// (`plantnode.c:27`): `TightenNodeMinMaxAvails`.
pub fn Tighten(which: c_int, oldmin: f64, oldmax: f64, newmin: f64, newmax: f64) -> f64 {
    if [oldmin, oldmax, newmin, newmax].iter().any(|&f| !flow(f)) {
        return f64::NAN;
    }
    let mut min_avail = oldmin;
    let mut max_avail = oldmax;
    if newmin > oldmin && newmin <= oldmax {
        min_avail = newmin;
    }
    if newmax < oldmax && newmax >= oldmin {
        max_avail = newmax;
    }
    pick(which, [min_avail, max_avail])
}

/// Port of `double BoundTwo(double v, double lo, double hi)` (`plantnode.c:32`):
/// `BoundValueToWithinTwoValues`.
pub fn BoundTwo(v: f64, lo: f64, hi: f64) -> f64 {
    if !flow(v) || !flow(lo) || !flow(hi) {
        return f64::NAN;
    }
    min(max(v, lo), hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers_bound_and_tighten() {
        assert_eq!(InitNodes(1, -2.0, 3.0), 0.0);
        assert_eq!(InitNodes(4, -2.0, 3.0), 3.0);
        assert_eq!(BoundNode(5.0, 1.0, 4.0), 4.0);
        assert_eq!(BoundTwo(-1.0, 0.0, 4.0), 0.0);
        assert_eq!(Tighten(0, 1.0, 4.0, 2.0, 3.0), 2.0);
        assert_eq!(Tighten(1, 1.0, 4.0, 2.0, 0.5), 4.0);
        assert_eq!(
            SafeCopy(
                9, 2, 20.0, 1.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 2.0, 1.5, 0.0, 2.0e5, 1.0e5
            ),
            1.0e5
        );
        assert_eq!(
            SafeCopy(
                7, 1, 20.0, 1.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 2.0, 1.5, 0.0, 2.0e5, 1.0e5
            ),
            1.5
        );
        assert!(
            SafeCopy(
                10, 1, 20.0, 1.0, 0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 2.0, 1.5, 0.0, 2.0e5, 1.0e5
            )
            .is_nan()
        );
    }

    #[test]
    fn signed_zeros_follow_objexx() {
        assert!(BoundTwo(0.0, -0.0, 0.0).is_sign_positive());
        assert!(min(0.0, -0.0).is_sign_negative());
        assert!(max(-0.0, 0.0).is_sign_negative());
    }
}
