//! The Rust port of `supplenum.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZonePlenum::CalcAirZoneSupplyPlenum` followed by `UpdateAirZoneSupplyPlenum`: on the first
//! pass the plenum zone's air goes to every outlet node; on the second the outlets' flows are
//! summed back to the inlet and zone nodes, and the inlet is flagged as changed when its flow moves
//! by more than 0.01 kg/s. Every sum keeps the C++ order. The oracle's guards (see its shim) return
//! NaN here too.

#![forbid(unsafe_code)]

/// Tolerance for mass flow rate convergence (kg/s).
const FLOW_RATE_TOLER: f64 = 0.01;

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The eight results the shim reports, for the inlet node's flows `(m, x, n)`, the outlets'
/// requested flows and the zone's `(t, w, h)`.
fn passes(
    first: bool,
    [inm, inmx, inmn]: [f64; 3],
    outlets: &[[f64; 3]],
    [tz, wz, hz]: [f64; 3],
) -> [f64; 8] {
    if first {
        return [inm, inmx, inmn, 0.0, 0.0, tz, wz, hz];
    }
    let mut flow = 0.0;
    let mut max_avail = 0.0;
    let mut min_avail = 0.0;
    for [m, x, n] in outlets {
        flow += m;
        max_avail += x;
        min_avail += n;
    }
    let changed = if (inm - flow).abs() > FLOW_RATE_TOLER {
        1.0
    } else {
        0.0
    };
    [flow, max_avail, min_avail, changed, flow, 0.0, 0.0, 0.0]
}

/// Port of `double SupPlenum(int which, int first, int nout, ...)` (`supplenum.c:9`):
/// `CalcAirZoneSupplyPlenum` and `UpdateAirZoneSupplyPlenum`, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn SupPlenum(
    which: std::ffi::c_int,
    first: std::ffi::c_int,
    nout: std::ffi::c_int,
    inm: f64,
    inmx: f64,
    inmn: f64,
    m1: f64,
    x1: f64,
    n1: f64,
    m2: f64,
    x2: f64,
    n2: f64,
    m3: f64,
    x3: f64,
    n3: f64,
    m4: f64,
    x4: f64,
    n4: f64,
    tz: f64,
    wz: f64,
    hz: f64,
) -> f64 {
    let outlets = [[m1, x1, n1], [m2, x2, n2], [m3, x3, n3], [m4, x4, n4]];
    let (Ok(which), Ok(nout)) = (usize::try_from(which), usize::try_from(nout)) else {
        return f64::NAN;
    };
    if which > 7
        || (first != 0 && first != 1)
        || !(1..=4).contains(&nout)
        || !within(inm, 0.0, 1000.0)
        || !within(inmx, 0.0, 1000.0)
        || !within(inmn, 0.0, 1000.0)
        || !within(tz, -60.0, 80.0)
        || !within(wz, 0.0, 0.1)
        || !within(hz, -1.0e5, 1.0e6)
        || outlets.iter().flatten().any(|&v| !within(v, 0.0, 1000.0))
    {
        return f64::NAN;
    }
    passes(
        first == 1,
        [inm, inmx, inmn],
        &outlets[..nout],
        [tz, wz, hz],
    )[which]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_pass_sums_outlets_back_to_the_inlet() {
        let f = |which, inm| {
            SupPlenum(
                which, 0, 2, inm, 0.0, 0.0, 1.0, 1.2, 0.1, 2.0, 2.5, 0.2, 9.0, 9.0, 9.0, 9.0, 9.0,
                9.0, 20.0, 0.007, 38000.0,
            )
        };
        assert_eq!(f(0, 0.0), 3.0);
        assert_eq!(f(1, 0.0), 1.2 + 2.5);
        assert_eq!(f(3, 0.0), 1.0);
        assert_eq!(f(3, 2.995), 0.0);
        assert_eq!(f(5, 0.0), 0.0);
    }

    #[test]
    fn first_pass_passes_the_zone_air_to_the_outlets() {
        let f = |which| {
            SupPlenum(
                which, 1, 3, 0.5, 0.6, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 20.0, 0.007, 38000.0,
            )
        };
        assert_eq!(f(0), 0.5);
        assert_eq!(f(5), 20.0);
        assert_eq!(f(7), 38000.0);
        assert!(f(8).is_nan());
    }
}
