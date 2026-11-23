//! The Rust port of `waterhc.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `WaterCoils::CalcSimpleHeatingCoil`: the simple hot water heating coil, an effectiveness-NTU
//! model of a cross flow heat exchanger with both streams unmixed at a fixed UA, with the cycling
//! fan's flows scaled by the part load ratio, the water specific heat from `fluidproperties-rs`
//! and the air's from `psychrometrics-rs`. Every expression keeps the C++ order, and `min` and
//! `max` are ObjexxFCL's. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use fluidproperties_rs::port::WaterSpecificHeat;
use psychrometrics_rs::port::{PsyCpAirFnW, PsyHFnTdbW};

/// `DataBranchAirLoopPlant::MassFlowTolerance` (kg/s).
const MASS_FLOW_TOLERANCE: f64 = 0.000000001;

thread_local! {
    /// The last temperature's water specific heat (bits of the temperature, value).
    static CP_CACHE: std::cell::Cell<(u64, f64)> = const { std::cell::Cell::new((u64::MAX, 0.0)) };
}

/// `GlycolProps::getSpecificHeat` for water, which caches its results by temperature; this keeps
/// the last one (the value is the table's either way).
fn water_cp(t: f64) -> f64 {
    let key = t.to_bits();
    let (k, v) = CP_CACHE.get();
    if k == key {
        return v;
    }
    let cp = WaterSpecificHeat(t);
    CP_CACHE.set((key, cp));
    cp
}

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// One coil as `InitWaterCoil` leaves it.
struct Coil {
    ua: f64,
    avail: f64,
    mdot_air: f64,
    t_air_in: f64,
    w_in: f64,
    mdot_water: f64,
    mdot_water_max: f64,
    t_water_in: f64,
    h_water_in: f64,
}

/// The results: heating rate (W), outlet air and water temperatures (C), outlet water enthalpy
/// (J/kg) and mass flow (kg/s), outlet air enthalpy (J/kg).
struct Out {
    load: f64,
    t_air_out: f64,
    t_water_out: f64,
    h_water_out: f64,
    mdot_water_out: f64,
    h_air_out: f64,
}

/// `CalcSimpleHeatingCoil` (`MySizeFlag` and `MyUAAndFlowCalcFlag` false, as after sizing).
fn calc_simple_heating_coil(c: &Coil, cycling: bool, plr: f64, design: bool) -> Out {
    let (air_mass_flow, water_mass_flow) = if cycling {
        if plr > 0.0 {
            (c.mdot_air / plr, min(c.mdot_water / plr, c.mdot_water_max))
        } else {
            (0.0, 0.0)
        }
    } else {
        (c.mdot_air, c.mdot_water)
    };
    let (mut cap_air, mut cap_water, mut cap_min, mut cap_max) = (0.0, 0.0, 0.0, 0.0);
    if water_mass_flow > MASS_FLOW_TOLERANCE {
        cap_air = PsyCpAirFnW(c.w_in) * air_mass_flow;
        cap_water = water_cp(c.t_water_in) * water_mass_flow;
        cap_min = min(cap_air, cap_water);
        cap_max = max(cap_air, cap_water);
    }
    let running = (cap_air > 0.0 && cap_water > 0.0) && (design || c.avail > 0.0);
    let (t_air_out, t_water_out, mut load, h_water_out, mdot_water_out) = if running {
        let ntu = c.ua / cap_min;
        let eta = ntu.powf(0.22);
        let cap_ratio = cap_min / cap_max;
        let mut a = cap_ratio * ntu / eta;
        if a > 20.0 {
            a = eta * 1.0 / cap_ratio;
        } else {
            a = eta * (1.0 - (-a).exp()) / cap_ratio;
        }
        let effec = if a > 20.0 { 1.0 } else { 1.0 - (-a).exp() };
        let t_air_out = c.t_air_in + effec * cap_min * (c.t_water_in - c.t_air_in) / cap_air;
        let t_water_out = c.t_water_in - cap_air * (t_air_out - c.t_air_in) / cap_water;
        let load = cap_water * (c.t_water_in - t_water_out);
        (
            t_air_out,
            t_water_out,
            load,
            c.h_water_in - load / c.mdot_water,
            c.mdot_water,
        )
    } else {
        (c.t_air_in, c.t_water_in, 0.0, c.h_water_in, 0.0)
    };
    if cycling {
        load *= plr;
    }
    Out {
        load,
        t_air_out,
        t_water_out,
        h_water_out,
        mdot_water_out,
        h_air_out: PsyHFnTdbW(t_air_out, c.w_in),
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double WaterHeatCoil(int which, int cycling, int design, double ua, double avail,
/// double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin,
/// double plr)` (`waterhc.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn WaterHeatCoil(
    which: std::ffi::c_int,
    cycling: std::ffi::c_int,
    design: std::ffi::c_int,
    ua: f64,
    avail: f64,
    mdota: f64,
    tain: f64,
    win: f64,
    mdotw: f64,
    mdotwmax: f64,
    twin: f64,
    hwin: f64,
    plr: f64,
) -> f64 {
    let ok = (0..=5).contains(&which)
        && (0..=1).contains(&cycling)
        && (0..=1).contains(&design)
        && within(ua, 1.0, 1.0e7)
        && within(avail, 0.0, 1.0)
        && within(mdota, 0.0, 50.0)
        && within(tain, -40.0, 60.0)
        && within(win, 0.0, 0.04)
        && within(mdotwmax, 0.0, 100.0)
        && within(mdotw, 0.0, mdotwmax)
        && within(twin, 1.0, 99.0)
        && within(hwin, -1.0e6, 1.0e6)
        && within(plr, 0.0, 1.0);
    if !ok {
        return f64::NAN;
    }
    let coil = Coil {
        ua,
        avail,
        mdot_air: mdota,
        t_air_in: tain,
        w_in: win,
        mdot_water: mdotw,
        mdot_water_max: mdotwmax,
        t_water_in: twin,
        h_water_in: hwin,
    };
    let o = calc_simple_heating_coil(&coil, cycling == 1, plr, design == 1);
    match which {
        0 => o.load,
        1 => o.t_air_out,
        2 => o.t_water_out,
        3 => o.h_water_out,
        4 => o.mdot_water_out,
        _ => o.h_air_out,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hot_water_heats_the_air() {
        let coil = |which, mdotw| {
            WaterHeatCoil(
                which, 0, 0, 500.0, 1.0, 1.0, 10.0, 0.005, mdotw, 0.5, 80.0, 334_000.0, 1.0,
            )
        };
        let t = coil(1, 0.2);
        assert!(t > 10.0 && t < 80.0);
        assert!(coil(1, 0.4) > t);
        assert!(coil(0, 0.2) > 0.0);
        assert_eq!(coil(1, 0.0), 10.0);
        assert!(coil(6, 0.2).is_nan());
    }
}
