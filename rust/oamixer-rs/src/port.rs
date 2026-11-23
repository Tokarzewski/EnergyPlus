//! The Rust port of `oamixer.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `MixedAir::OAMixerProps::CalcOAMixer`: the outdoor air mixer splits the return air into relief
//! and recirculated air (no recirculation below zero) and mixes the recirculated and outdoor air by
//! mass flow, the mixed temperature from the enthalpy and humidity ratio, moved to the saturation
//! temperature at constant enthalpy when it lies below it (`psychrometrics-rs`). Every expression
//! keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::{PsyTdbFnHW, PsyTsatFnHPb, PsyWFnTdbH};

/// `HVAC::VerySmallMassFlow` (kg/s).
const VERY_SMALL_MASS_FLOW: f64 = 1.0e-30;

thread_local! {
    /// The last enthalpy and pressure's saturation temperature (their bits, value).
    static TSAT_CACHE: std::cell::Cell<(u64, u64, f64)> = const { std::cell::Cell::new((u64::MAX, u64::MAX, 0.0)) };
}

/// `Psychrometrics::PsyTsatFnHPb`, which the C++ caches in the EnergyPlus state; this keeps the
/// last exact inputs (the value is the function's either way).
fn tsat(h: f64, pb: f64) -> f64 {
    let key = (h.to_bits(), pb.to_bits());
    let (kh, kp, v) = TSAT_CACHE.get();
    if (kh, kp) == key {
        return v;
    }
    let t = PsyTsatFnHPb(h, pb);
    TSAT_CACHE.set((key.0, key.1, t));
    t
}

/// One air stream: temperature (C), humidity ratio (kg/kg), enthalpy (J/kg), pressure (Pa).
#[derive(Clone, Copy)]
struct Stream {
    temp: f64,
    hum_rat: f64,
    enthalpy: f64,
    pressure: f64,
}

/// The mixed air and its mass flow (kg/s), and the relief air mass flow after the call (kg/s).
struct Out {
    mix: Stream,
    mix_mass_flow_rate: f64,
    rel_mass_flow_rate: f64,
}

/// `CalcOAMixer` with the return and outdoor streams and their mass flows, and the relief mass flow.
fn calc_oa_mixer(ret: Stream, ret_m: f64, oa: Stream, oa_m: f64, rel_m: f64) -> Out {
    let mut rel_mass_flow_rate = rel_m;
    let mut recirc_mass_flow_rate = ret_m - rel_mass_flow_rate;
    if recirc_mass_flow_rate < 0.0 {
        recirc_mass_flow_rate = 0.0;
        rel_mass_flow_rate = ret_m;
    }
    let mix_mass_flow_rate = oa_m + recirc_mass_flow_rate;
    if mix_mass_flow_rate <= VERY_SMALL_MASS_FLOW {
        return Out {
            mix: ret,
            mix_mass_flow_rate,
            rel_mass_flow_rate,
        };
    }
    let enthalpy = (recirc_mass_flow_rate * ret.enthalpy + oa_m * oa.enthalpy) / mix_mass_flow_rate;
    let mut hum_rat =
        (recirc_mass_flow_rate * ret.hum_rat + oa_m * oa.hum_rat) / mix_mass_flow_rate;
    let pressure = (recirc_mass_flow_rate * ret.pressure + oa_m * oa.pressure) / mix_mass_flow_rate;
    let mut temp = PsyTdbFnHW(enthalpy, hum_rat);
    let t_sat = tsat(enthalpy, pressure);
    if temp < t_sat {
        temp = t_sat;
        hum_rat = PsyWFnTdbH(t_sat, enthalpy);
    }
    Out {
        mix: Stream {
            temp,
            hum_rat,
            enthalpy,
            pressure,
        },
        mix_mass_flow_rate,
        rel_mass_flow_rate,
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// One stream's temperature, humidity ratio, enthalpy, pressure and mass flow are in range.
fn stream_ok(s: Stream, m: f64) -> bool {
    within(s.temp, -60.0, 80.0)
        && within(s.hum_rat, 0.0, 0.1)
        && within(s.enthalpy, -2.0e4, 4.0e5)
        && within(s.pressure, 5.0e4, 1.2e5)
        && within(m, 0.0, 1000.0)
}

/// Port of `double OAMixer(int which, double rett, double retw, double reth, double retp, double
/// retm, double oat, double oaw, double oah, double oap, double oam, double relm)`
/// (`oamixer.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn OAMixer(
    which: std::ffi::c_int,
    rett: f64,
    retw: f64,
    reth: f64,
    retp: f64,
    retm: f64,
    oat: f64,
    oaw: f64,
    oah: f64,
    oap: f64,
    oam: f64,
    relm: f64,
) -> f64 {
    let ret = Stream {
        temp: rett,
        hum_rat: retw,
        enthalpy: reth,
        pressure: retp,
    };
    let oa = Stream {
        temp: oat,
        hum_rat: oaw,
        enthalpy: oah,
        pressure: oap,
    };
    if !(0..=5).contains(&which)
        || !stream_ok(ret, retm)
        || !stream_ok(oa, oam)
        || !within(relm, 0.0, 1000.0)
    {
        return f64::NAN;
    }
    let o = calc_oa_mixer(ret, retm, oa, oam, relm);
    match which {
        0 => o.mix.temp,
        1 => o.mix.hum_rat,
        2 => o.mix.enthalpy,
        3 => o.mix.pressure,
        4 => o.mix_mass_flow_rate,
        _ => o.rel_mass_flow_rate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_streams_mix_to_the_same_air() {
        let h = 1.00484e3 * 20.0 + 0.008 * (2.50094e6 + 1.85895e3 * 20.0);
        let mix = |which, relm| {
            OAMixer(
                which, 20.0, 0.008, h, 101_325.0, 2.0, 20.0, 0.008, h, 101_325.0, 1.0, relm,
            )
        };
        assert_eq!(mix(4, 1.0), 2.0);
        assert!((mix(0, 1.0) - 20.0).abs() < 1e-9);
        assert_eq!(mix(5, 3.0), 2.0);
        assert_eq!(mix(4, 3.0), 1.0);
        assert!(mix(6, 1.0).is_nan());
    }
}
