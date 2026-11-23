//! The Rust port of `airsplit.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `SplitterComponent`'s `InitAirLoopSplitter`, `CalcAirLoopSplitter` and `UpdateSplitter`, as
//! `SimAirLoopSplitter` runs them for a zone splitter: on the first call the inlet flow and
//! maximum available flow shared equally among the outlets on the first HVAC iteration, the
//! outlets shut when no flow is available, and the inlet state passed to every outlet; on the
//! second call the outlets' flows summed back onto the inlet, with whether the inlet flow changed
//! by more than the tolerance. Every expression keeps the C++ order. The oracle's guards (see its
//! shim) return NaN here too.

#![forbid(unsafe_code)]

/// `UpdateSplitter`'s tolerance for the inlet flow to have changed (kg/s).
const FLOW_RATE_TOLER: f64 = 0.01;

/// The fields of one node the splitter reads or writes.
#[derive(Clone, Copy)]
struct Node {
    mass_flow_rate: f64,
    max_avail: f64,
    min_avail: f64,
    temp: f64,
    hum_rat: f64,
    enthalpy: f64,
    press: f64,
}

/// `SimAirLoopSplitter` (init, calc, update) on the inlet `nodes[0]` and outlets `nodes[1..]`;
/// returns whether the splitter's inlet flow changed.
fn sim_air_loop_splitter(nodes: &mut [Node], first_hvac_iteration: bool, first_call: bool) -> bool {
    let (inlet, outlets) = nodes.split_at_mut(1);
    let inlet = &mut inlet[0];
    // The outlets' count is at most 4, so it is exact as an f64.
    let n = f64::from(u8::try_from(outlets.len()).unwrap_or(u8::MAX));
    if first_call {
        if first_hvac_iteration {
            if inlet.mass_flow_rate > 0.0 {
                for o in outlets.iter_mut() {
                    o.mass_flow_rate = inlet.mass_flow_rate / n;
                }
            }
            if inlet.max_avail > 0.0 {
                for o in outlets.iter_mut() {
                    o.max_avail = inlet.max_avail / n;
                }
            }
        }
        if inlet.max_avail == 0.0 {
            for o in outlets.iter_mut() {
                o.mass_flow_rate = 0.0;
                o.max_avail = 0.0;
                o.min_avail = 0.0;
            }
        }
        for o in outlets.iter_mut() {
            o.temp = inlet.temp;
            o.hum_rat = inlet.hum_rat;
            o.enthalpy = inlet.enthalpy;
            o.press = inlet.press;
        }
        false
    } else {
        let (mut m, mut max_avail, mut min_avail) = (0.0, 0.0, 0.0);
        for o in outlets.iter() {
            m += o.mass_flow_rate;
            max_avail += o.max_avail;
            min_avail += o.min_avail;
        }
        let changed = (inlet.mass_flow_rate - m).abs() > FLOW_RATE_TOLER;
        inlet.mass_flow_rate = m;
        inlet.max_avail = max_avail;
        inlet.min_avail = min_avail;
        changed
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The node's inputs are in range.
fn node_ok(n: &Node) -> bool {
    within(n.mass_flow_rate, 0.0, 1000.0)
        && within(n.max_avail, 0.0, 1000.0)
        && within(n.min_avail, 0.0, 1000.0)
        && within(n.temp, -60.0, 80.0)
        && within(n.hum_rat, 0.0, 0.1)
        && within(n.enthalpy, -2.0e4, 4.0e5)
        && within(n.press, 5.0e4, 1.2e5)
}

/// Port of `double AirSplit(int which, int n, int fhvac, int first, double m0, ...)`
/// (`airsplit.c:9`), with the inlet's and each of outlets 1 to 4's node flows and state: see the
/// oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn AirSplit(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    fhvac: std::ffi::c_int,
    first: std::ffi::c_int,
    m0: f64,
    maxav0: f64,
    minav0: f64,
    t0: f64,
    w0: f64,
    h0: f64,
    p0: f64,
    m1: f64,
    maxav1: f64,
    minav1: f64,
    t1: f64,
    w1: f64,
    h1: f64,
    p1: f64,
    m2: f64,
    maxav2: f64,
    minav2: f64,
    t2: f64,
    w2: f64,
    h2: f64,
    p2: f64,
    m3: f64,
    maxav3: f64,
    minav3: f64,
    t3: f64,
    w3: f64,
    h3: f64,
    p3: f64,
    m4: f64,
    maxav4: f64,
    minav4: f64,
    t4: f64,
    w4: f64,
    h4: f64,
    p4: f64,
) -> f64 {
    let raw = [
        [m0, maxav0, minav0, t0, w0, h0, p0],
        [m1, maxav1, minav1, t1, w1, h1, p1],
        [m2, maxav2, minav2, t2, w2, h2, p2],
        [m3, maxav3, minav3, t3, w3, h3, p3],
        [m4, maxav4, minav4, t4, w4, h4, p4],
    ];
    let mut nodes = raw.map(
        |[
            mass_flow_rate,
            max_avail,
            min_avail,
            temp,
            hum_rat,
            enthalpy,
            press,
        ]| Node {
            mass_flow_rate,
            max_avail,
            min_avail,
            temp,
            hum_rat,
            enthalpy,
            press,
        },
    );
    let flag = |v: std::ffi::c_int| v == 0 || v == 1;
    let (Ok(which), Ok(n)) = (usize::try_from(which), usize::try_from(n)) else {
        return f64::NAN;
    };
    if which > 31
        || !(1..=4).contains(&n)
        || !flag(fhvac)
        || !flag(first)
        || !nodes.iter().all(node_ok)
    {
        return f64::NAN;
    }
    let changed = sim_air_loop_splitter(&mut nodes[..=n], fhvac == 1, first == 1);
    match which {
        31 => f64::from(u8::from(changed)),
        28 => nodes[0].mass_flow_rate,
        29 => nodes[0].max_avail,
        30 => nodes[0].min_avail,
        _ => {
            let o = &nodes[which / 7 + 1];
            match which % 7 {
                0 => o.mass_flow_rate,
                1 => o.max_avail,
                2 => o.min_avail,
                3 => o.temp,
                4 => o.hum_rat,
                5 => o.enthalpy,
                _ => o.press,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_is_shared_and_summed_back() {
        let s = |which, first| {
            AirSplit(
                which, 2, 1, first, 3.0, 4.0, 0.0, 14.0, 0.008, 34_000.0, 101_000.0, 1.0, 1.0, 0.0,
                20.0, 0.007, 38_000.0, 101_325.0, 0.5, 1.5, 0.2, 20.0, 0.007, 38_000.0, 101_325.0,
                9.0, 9.0, 0.0, 20.0, 0.007, 38_000.0, 101_325.0, 0.0, 0.0, 0.0, 20.0, 0.007,
                38_000.0, 101_325.0,
            )
        };
        assert_eq!(s(0, 1), 1.5);
        assert_eq!(s(8, 1), 2.0);
        assert_eq!(s(3, 1), 14.0);
        assert_eq!(s(14, 1), 9.0);
        assert_eq!(s(28, 0), 1.5);
        assert_eq!(s(29, 0), 2.5);
        assert_eq!(s(31, 0), 1.0);
        assert!(s(32, 0).is_nan());
    }
}
