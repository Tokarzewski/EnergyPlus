//! The Rust port of `sensout.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `GeneralRoutines`' sensible and latent output rates of an air stream: across a component
//! (`CalcComponentSensibleLatentOutput`, the sensible part at the lower of the two humidity
//! ratios) and into a zone (`CalcZoneSensibleLatentOutput` and `calcZoneSensibleOutput`, the
//! sensible part at the zone's humidity ratio), the total from the enthalpy difference and the
//! latent part the rest. Uses `psychrometrics-rs`. Every expression keeps the C++ order. The
//! oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::{PsyDeltaHSenFnTdb2Tdb1W, PsyDeltaHSenFnTdb2W2Tdb1W1, PsyHFnTdbW};

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The mass flow and two air states are in range.
fn air_ok(m: f64, ta: f64, wa: f64, tb: f64, wb: f64) -> bool {
    within(m, 0.0, 1000.0)
        && within(ta, -60.0, 80.0)
        && within(wa, 0.0, 0.1)
        && within(tb, -60.0, 80.0)
        && within(wb, 0.0, 0.1)
}

/// The sensible, latent and total rates (W), from the total and the sensible rate when there is
/// flow.
fn outputs(m: f64, tot: impl FnOnce() -> f64, sens: impl FnOnce() -> f64) -> [f64; 3] {
    if m > 0.0 {
        let total = m * tot();
        let sensible = m * sens();
        [sensible, total - sensible, total]
    } else {
        [0.0; 3]
    }
}

/// The result `which` picks (0 sensible, 1 latent, 2 total), or NaN outside them.
fn pick(which: std::ffi::c_int, r: [f64; 3]) -> f64 {
    usize::try_from(which)
        .ok()
        .and_then(|i| r.get(i).copied())
        .unwrap_or(f64::NAN)
}

/// Port of `double CompOutput(int which, double m, double t2, double w2, double t1, double w1)`
/// (`sensout.c:9`): `CalcComponentSensibleLatentOutput`, see the oracle's shim.
pub fn CompOutput(which: std::ffi::c_int, m: f64, t2: f64, w2: f64, t1: f64, w1: f64) -> f64 {
    if !air_ok(m, t2, w2, t1, w1) {
        return f64::NAN;
    }
    pick(
        which,
        outputs(
            m,
            || PsyHFnTdbW(t2, w2) - PsyHFnTdbW(t1, w1),
            || PsyDeltaHSenFnTdb2W2Tdb1W1(t2, w2, t1, w1),
        ),
    )
}

/// Port of `double ZoneOutput(int which, double m, double te, double we, double tz, double wz)`
/// (`sensout.c:14`): `CalcZoneSensibleLatentOutput`, see the oracle's shim.
pub fn ZoneOutput(which: std::ffi::c_int, m: f64, te: f64, we: f64, tz: f64, wz: f64) -> f64 {
    if !air_ok(m, te, we, tz, wz) {
        return f64::NAN;
    }
    pick(
        which,
        outputs(
            m,
            || PsyHFnTdbW(te, we) - PsyHFnTdbW(tz, wz),
            || PsyDeltaHSenFnTdb2Tdb1W(te, tz, wz),
        ),
    )
}

/// Port of `double ZoneSensOutput(double m, double te, double tz, double wz)` (`sensout.c:19`):
/// `calcZoneSensibleOutput`, see the oracle's shim.
pub fn ZoneSensOutput(m: f64, te: f64, tz: f64, wz: f64) -> f64 {
    if !air_ok(m, te, 0.0, tz, wz) {
        return f64::NAN;
    }
    if m > 0.0 {
        m * PsyDeltaHSenFnTdb2Tdb1W(te, tz, wz)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warm_dry_supply_heats_and_dries_the_zone() {
        assert!(ZoneOutput(0, 1.0, 30.0, 0.006, 21.0, 0.008) > 0.0);
        assert!(ZoneOutput(1, 1.0, 30.0, 0.006, 21.0, 0.008) < 0.0);
        assert_eq!(
            ZoneSensOutput(1.0, 30.0, 21.0, 0.008),
            ZoneOutput(0, 1.0, 30.0, 0.006, 21.0, 0.008)
        );
        assert_eq!(CompOutput(2, 0.0, 30.0, 0.006, 21.0, 0.008), 0.0);
        assert!(CompOutput(3, 1.0, 30.0, 0.006, 21.0, 0.008).is_nan());
    }
}
