//! The Rust port of `spmzone.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Two of `SetPointManager`'s set point managers: `SPMSingleZoneTemp::calculate`, the single zone
//! heating or cooling supply temperature that meets the zone's load to its set point at the zone
//! inlet flow (the limit itself with no flow), and `SPMOutsideAir::calculate`, the outdoor air
//! reset line (`interpSetPoint`) the optional schedule chooses. Uses `psychrometrics-rs`. Every
//! expression keeps the C++ order, and `clamp` is the standard library's. The oracle's guards (see
//! its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::PsyCpAirFnW;

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

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// `SetPointManager::interpSetPoint`: the set point on the line from `at_low` at `low` to
/// `at_high` at `high`, held at the ends (their mean when the line is empty).
fn interp_set_point(low: f64, high: f64, x: f64, at_low: f64, at_high: f64) -> f64 {
    if low >= high {
        return 0.5 * (at_low + at_high);
    }
    if x <= low {
        return at_low;
    }
    if x >= high {
        return at_high;
    }
    at_low - ((x - low) / (high - low)) * (at_low - at_high)
}

/// Port of `double SZTemp(int heating, double zonem, double inw, double toheat, double tocool,
/// double zonet, double mint, double maxt)` (`spmzone.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn SZTemp(
    heating: std::ffi::c_int,
    zonem: f64,
    inw: f64,
    toheat: f64,
    tocool: f64,
    zonet: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    let ok = (0..=1).contains(&heating)
        && within(zonem, 0.0, 1000.0)
        && within(inw, 0.0, 0.1)
        && within(toheat, -1.0e7, 1.0e7)
        && within(tocool, -1.0e7, 1.0e7)
        && within(zonet, -60.0, 80.0)
        && within(mint, -60.0, 80.0)
        && within(maxt, mint, 80.0);
    if !ok {
        return f64::NAN;
    }
    let heating = heating == 1;
    let load_to_sp = if heating { toheat } else { tocool };
    if zonem <= SMALL_MASS_FLOW {
        return if heating { mint } else { maxt };
    }
    let cp_air = PsyCpAirFnW(inw);
    let set_pt = zonet + load_to_sp / (cp_air * zonem);
    clamp(set_pt, mint, maxt)
}

/// Port of `double OAReset(int schedv, double oat, double low1, double high1, double lsp1, double
/// hsp1, double low2, double high2, double lsp2, double hsp2)` (`spmzone.c:14`): see the oracle's
/// shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn OAReset(
    schedv: std::ffi::c_int,
    oat: f64,
    low1: f64,
    high1: f64,
    lsp1: f64,
    hsp1: f64,
    low2: f64,
    high2: f64,
    lsp2: f64,
    hsp2: f64,
) -> f64 {
    let ts = [oat, low1, high1, lsp1, hsp1, low2, high2, lsp2, hsp2];
    if !(0..=2).contains(&schedv) || !ts.iter().all(|&t| within(t, -60.0, 80.0)) {
        return f64::NAN;
    }
    if schedv == 2 {
        interp_set_point(low2, high2, oat, lsp2, hsp2)
    } else {
        interp_set_point(low1, high1, oat, lsp1, hsp1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_points_follow_the_load_and_the_outdoor_air() {
        assert_eq!(
            SZTemp(1, 0.0, 0.008, 1000.0, -1000.0, 21.0, 10.0, 50.0),
            10.0
        );
        assert!(SZTemp(1, 0.5, 0.008, 1000.0, -1000.0, 21.0, 10.0, 50.0) > 21.0);
        assert!(SZTemp(0, 0.5, 0.008, 1000.0, -1000.0, 21.0, 10.0, 50.0) < 21.0);
        assert_eq!(
            OAReset(0, 5.0, 0.0, 10.0, 20.0, 10.0, 0.0, 20.0, 30.0, 10.0),
            15.0
        );
        assert_eq!(
            OAReset(2, 30.0, 0.0, 10.0, 20.0, 10.0, 0.0, 20.0, 30.0, 10.0),
            10.0
        );
        assert!(OAReset(3, 5.0, 0.0, 10.0, 20.0, 10.0, 0.0, 20.0, 30.0, 10.0).is_nan());
    }
}
