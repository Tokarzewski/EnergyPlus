//! The Rust port of `airmixer.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `MixerComponent::CalcAirMixer`: the zone mixer sums its inlets' flows and available flows,
//! sets each inlet's flow fraction and mixes the humidity ratio, pressure and enthalpy by mass flow,
//! the outlet temperature from the enthalpy and humidity ratio (`psychrometrics-rs`); with no flow
//! it mixes by the last flow fractions instead. Every expression keeps the C++ order, and `max` is
//! ObjexxFCL's. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::PsyTdbFnHW;

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// One inlet as `InitAirMixer` leaves it, with the flow fraction the last call left.
#[derive(Clone, Copy)]
struct Inlet {
    mass_flow_rate: f64,
    max_avail: f64,
    min_avail: f64,
    temp: f64,
    hum_rat: f64,
    enthalpy: f64,
    pressure: f64,
    flow_fraction: f64,
}

/// The outlet's state and flows.
#[derive(Default)]
struct Out {
    temp: f64,
    hum_rat: f64,
    enthalpy: f64,
    pressure: f64,
    mass_flow_rate: f64,
    max_avail: f64,
    min_avail: f64,
}

/// `CalcAirMixer` over the inlets (no parallel PIU leakage); updates their flow fractions.
fn calc_air_mixer(inlets: &mut [Inlet]) -> Out {
    let mut o = Out::default();
    for i in inlets.iter() {
        o.mass_flow_rate += i.mass_flow_rate;
        o.max_avail += i.max_avail;
        o.min_avail += i.min_avail;
    }
    if o.mass_flow_rate > 0.0 {
        for i in inlets.iter_mut() {
            i.flow_fraction = i.mass_flow_rate / o.mass_flow_rate;
        }
        for i in inlets.iter() {
            o.hum_rat += i.mass_flow_rate * i.hum_rat / o.mass_flow_rate;
        }
        for i in inlets.iter() {
            o.pressure += i.pressure * i.mass_flow_rate / o.mass_flow_rate;
        }
        for i in inlets.iter() {
            o.enthalpy += i.enthalpy * i.mass_flow_rate / o.mass_flow_rate;
        }
        o.temp = PsyTdbFnHW(o.enthalpy, o.hum_rat);
    } else {
        for i in inlets.iter() {
            o.hum_rat += i.flow_fraction * i.hum_rat;
            o.pressure += i.flow_fraction * i.pressure;
            o.enthalpy += i.flow_fraction * i.enthalpy;
        }
        o.temp = if inlets.len() == 1 {
            inlets[0].temp
        } else {
            PsyTdbFnHW(o.enthalpy, o.hum_rat)
        };
    }
    o.max_avail = max(o.max_avail, o.mass_flow_rate);
    o
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The inlet's inputs are in range.
fn inlet_ok(i: &Inlet) -> bool {
    within(i.mass_flow_rate, 0.0, 1000.0)
        && within(i.max_avail, 0.0, 1000.0)
        && within(i.min_avail, 0.0, 1000.0)
        && within(i.temp, -60.0, 80.0)
        && within(i.hum_rat, 0.0, 0.1)
        && within(i.enthalpy, -2.0e4, 4.0e5)
        && within(i.pressure, 5.0e4, 1.2e5)
        && within(i.flow_fraction, 0.0, 1.0)
}

/// Port of `double AirMixer(int which, int n, double m1, ...)` (`airmixer.c:9`), with each of inlets
/// 1 to 4's flow, available flows, temperature, humidity ratio, enthalpy, pressure and flow
/// fraction: see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn AirMixer(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: f64,
    maxav1: f64,
    minav1: f64,
    t1: f64,
    w1: f64,
    h1: f64,
    p1: f64,
    f1: f64,
    m2: f64,
    maxav2: f64,
    minav2: f64,
    t2: f64,
    w2: f64,
    h2: f64,
    p2: f64,
    f2: f64,
    m3: f64,
    maxav3: f64,
    minav3: f64,
    t3: f64,
    w3: f64,
    h3: f64,
    p3: f64,
    f3: f64,
    m4: f64,
    maxav4: f64,
    minav4: f64,
    t4: f64,
    w4: f64,
    h4: f64,
    p4: f64,
    f4: f64,
) -> f64 {
    let raw = [
        [m1, maxav1, minav1, t1, w1, h1, p1, f1],
        [m2, maxav2, minav2, t2, w2, h2, p2, f2],
        [m3, maxav3, minav3, t3, w3, h3, p3, f3],
        [m4, maxav4, minav4, t4, w4, h4, p4, f4],
    ];
    let mut inlets = raw.map(
        |[
            mass_flow_rate,
            max_avail,
            min_avail,
            temp,
            hum_rat,
            enthalpy,
            pressure,
            flow_fraction,
        ]| Inlet {
            mass_flow_rate,
            max_avail,
            min_avail,
            temp,
            hum_rat,
            enthalpy,
            pressure,
            flow_fraction,
        },
    );
    let Ok(n) = usize::try_from(n) else {
        return f64::NAN;
    };
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    if which > 10 || !(1..=4).contains(&n) || !inlets.iter().all(inlet_ok) {
        return f64::NAN;
    }
    let o = calc_air_mixer(&mut inlets[..n]);
    match which {
        0 => o.temp,
        1 => o.hum_rat,
        2 => o.enthalpy,
        3 => o.pressure,
        4 => o.mass_flow_rate,
        5 => o.max_avail,
        6 => o.min_avail,
        _ => inlets[which - 7].flow_fraction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_equal_inlets_share_the_flow() {
        let h = 1.00484e3 * 24.0 + 0.009 * (2.50094e6 + 1.85895e3 * 24.0);
        let mix = |which, m2| {
            AirMixer(
                which, 2, 1.0, 1.5, 0.0, 24.0, 0.009, h, 101_325.0, 0.5, m2, 1.0, 0.0, 24.0, 0.009,
                h, 101_325.0, 0.5, 0.0, 0.0, 0.0, 20.0, 0.008, h, 101_325.0, 0.0, 0.0, 0.0, 0.0,
                20.0, 0.008, h, 101_325.0, 0.0,
            )
        };
        assert_eq!(mix(4, 1.0), 2.0);
        assert_eq!(mix(7, 3.0), 0.25);
        assert!((mix(0, 1.0) - 24.0).abs() < 1e-9);
        assert_eq!(mix(5, 0.0), 2.5);
        assert_eq!(mix(8, 0.0), 0.0);
        assert!(mix(11, 1.0).is_nan());
    }
}
