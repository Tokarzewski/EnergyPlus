//! The Rust port of `retplenum.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZonePlenum::CalcAirZoneReturnPlenum`: a return plenum's outlet flow is the sum of its inlet
//! flows plus the leaks of the air distribution units into it, less the induced air drawn from it;
//! its available flows are the inlets' plus the units' changes, the maximum at least the flow; the
//! outlet pressure is the flow-weighted mean of the inlets' (the first inlet's with no flow); the
//! outlet air is the plenum zone's. Every sum keeps the C++ order. The oracle's guards (see its
//! shim) return NaN here too.

#![forbid(unsafe_code)]

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// One inlet: mass flow, maximum and minimum available flows (kg/s) and pressure (Pa).
#[derive(Clone, Copy)]
struct Inlet {
    m: f64,
    x: f64,
    n: f64,
    p: f64,
}

/// One air distribution unit that can leak into the plenum.
#[derive(Clone, Copy)]
struct Adu {
    up: bool,
    dn: bool,
    lu: f64,
    ld: f64,
    lp: f64,
    dx: f64,
    dmin: f64,
}

/// The outlet's flow, maximum and minimum available flows and pressure.
fn calc(inlets: &[Inlet], adus: &[Adu], induced: &[f64]) -> [f64; 4] {
    let mut flow = 0.0;
    let mut max_avail = 0.0;
    let mut min_avail = 0.0;
    let mut pressure = 0.0;
    let mut tot_ind = 0.0;
    for i in inlets {
        flow += i.m;
        max_avail += i.x;
        min_avail += i.n;
    }
    if flow > 0.0 {
        for i in inlets {
            pressure += i.p * i.m / flow;
        }
    } else {
        pressure = inlets[0].p;
    }
    for a in adus {
        if a.up || a.dn || a.lp > 0.0 {
            flow += a.lu + a.ld + a.lp;
            max_avail += a.dx;
            min_avail += a.dmin;
        }
    }
    for m in induced {
        tot_ind += m;
    }
    flow -= tot_ind;
    // ObjexxFCL max: the first unless it is less than the second.
    let max_avail = if max_avail < flow { flow } else { max_avail };
    [flow, max_avail, min_avail, pressure]
}

/// Port of `double RetPlenum(int which, int nin, int nadu, int nind, ...)` (`retplenum.c:9`):
/// `CalcAirZoneReturnPlenum`, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn RetPlenum(
    which: std::ffi::c_int,
    nin: std::ffi::c_int,
    nadu: std::ffi::c_int,
    nind: std::ffi::c_int,
    m1: f64,
    x1: f64,
    n1: f64,
    p1: f64,
    m2: f64,
    x2: f64,
    n2: f64,
    p2: f64,
    m3: f64,
    x3: f64,
    n3: f64,
    p3: f64,
    m4: f64,
    x4: f64,
    n4: f64,
    p4: f64,
    u1: std::ffi::c_int,
    d1: std::ffi::c_int,
    lu1: f64,
    ld1: f64,
    lp1: f64,
    dx1: f64,
    dn1: f64,
    u2: std::ffi::c_int,
    d2: std::ffi::c_int,
    lu2: f64,
    ld2: f64,
    lp2: f64,
    dx2: f64,
    dn2: f64,
    i1: f64,
    i2: f64,
    tz: f64,
    wz: f64,
    hz: f64,
) -> f64 {
    let inlets = [
        Inlet {
            m: m1,
            x: x1,
            n: n1,
            p: p1,
        },
        Inlet {
            m: m2,
            x: x2,
            n: n2,
            p: p2,
        },
        Inlet {
            m: m3,
            x: x3,
            n: n3,
            p: p3,
        },
        Inlet {
            m: m4,
            x: x4,
            n: n4,
            p: p4,
        },
    ];
    let flags = [u1, d1, u2, d2];
    let adus = [
        Adu {
            up: u1 == 1,
            dn: d1 == 1,
            lu: lu1,
            ld: ld1,
            lp: lp1,
            dx: dx1,
            dmin: dn1,
        },
        Adu {
            up: u2 == 1,
            dn: d2 == 1,
            lu: lu2,
            ld: ld2,
            lp: lp2,
            dx: dx2,
            dmin: dn2,
        },
    ];
    let induced = [i1, i2];
    let (Ok(nin), Ok(nadu), Ok(nind)) = (
        usize::try_from(nin),
        usize::try_from(nadu),
        usize::try_from(nind),
    ) else {
        return f64::NAN;
    };
    if !(0..=6).contains(&which)
        || !(1..=4).contains(&nin)
        || nadu > 2
        || nind > 2
        || !within(tz, -60.0, 80.0)
        || !within(wz, 0.0, 0.1)
        || !within(hz, -1.0e5, 1.0e6)
        || inlets.iter().any(|i| {
            !within(i.m, 0.0, 1000.0)
                || !within(i.x, 0.0, 1000.0)
                || !within(i.n, 0.0, 1000.0)
                || !within(i.p, 0.0, 2.0e5)
        })
        || flags.iter().any(|&f| f != 0 && f != 1)
        || adus.iter().any(|a| {
            !within(a.lu, 0.0, 1000.0)
                || !within(a.ld, 0.0, 1000.0)
                || !within(a.lp, 0.0, 1000.0)
                || !within(a.dx, -1000.0, 1000.0)
                || !within(a.dmin, -1000.0, 1000.0)
        })
        || induced.iter().any(|&m| !within(m, 0.0, 1000.0))
    {
        return f64::NAN;
    }
    match which {
        3 => tz,
        4 => wz,
        6 => hz,
        _ => {
            let [flow, max_avail, min_avail, pressure] =
                calc(&inlets[..nin], &adus[..nadu], &induced[..nind]);
            match which {
                0 => flow,
                1 => max_avail,
                2 => min_avail,
                _ => pressure,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaks_add_and_induced_air_subtracts() {
        let f = |which| {
            RetPlenum(
                which, 2, 1, 1, 1.0, 1.5, 0.0, 100.0, 3.0, 3.5, 0.5, 200.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1, 0, 0.2, 0.0, 0.0, 0.3, 0.1, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.5, 0.0, 24.0, 0.008, 45000.0,
            )
        };
        assert_eq!(f(0), 1.0 + 3.0 + 0.2 - 0.5);
        assert_eq!(f(1), 1.5 + 3.5 + 0.3);
        assert_eq!(f(2), 0.5 + 0.1);
        assert_eq!(f(5), 100.0 * 1.0 / 4.0 + 200.0 * 3.0 / 4.0);
        assert_eq!(f(3), 24.0);
        assert!(f(7).is_nan());
    }

    #[test]
    fn no_flow_takes_the_first_inlet_pressure_and_max_is_at_least_the_flow() {
        let r = calc(
            &[Inlet {
                m: 0.0,
                x: 0.0,
                n: 0.0,
                p: 101325.0,
            }],
            &[Adu {
                up: false,
                dn: false,
                lu: 0.0,
                ld: 0.0,
                lp: 0.4,
                dx: 0.0,
                dmin: 0.0,
            }],
            &[],
        );
        assert_eq!(r, [0.4, 0.4, 0.0, 101325.0]);
    }
}
