//! The Rust port of `szreheat.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `SetPointManager::SPMSingleZoneReheat::calculate`: the supply air temperature that meets the
//! control zone's load at the zone inlet flow, starting from the air the system delivers with no
//! heating or cooling (the mixed air at the minimum outdoor air fraction, or the loop inlet, plus
//! the fan's temperature rise), with the dead band and the loads to the heating and cooling set
//! points, limited to the manager's minimum and maximum. Uses `psychrometrics-rs`. Every
//! expression keeps the C++ order, and `clamp` is the standard library's. The oracle's guards (see
//! its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::{PsyCpAirFnW, PsyTdbFnHW};

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

/// The control zone and the air loop as the set point manager reads them.
struct Inputs {
    /// The return and outdoor air humidity ratios and enthalpies, if the loop has a mixer.
    mixer: Option<(f64, f64, f64, f64)>,
    /// The fan inlet and outlet temperatures, if the manager names the fan's nodes.
    fan: Option<(f64, f64)>,
    dead_band: bool,
    oa_frac: f64,
    zone_mass_flow: f64,
    inlet_hum_rat: f64,
    zone_load: f64,
    to_cool: f64,
    to_heat: f64,
    zone_temp: f64,
    loop_in_temp: f64,
    min_set_temp: f64,
    max_set_temp: f64,
}

/// `SPMSingleZoneReheat::calculate`.
fn calculate(x: &Inputs) -> f64 {
    let t_mix_at_min_oa = match x.mixer {
        Some((ret_w, ret_h, oa_w, oa_h)) => {
            let hum_rat_mix = (1.0 - x.oa_frac) * ret_w + x.oa_frac * oa_w;
            let enth_mix = (1.0 - x.oa_frac) * ret_h + x.oa_frac * oa_h;
            PsyTdbFnHW(enth_mix, hum_rat_mix)
        }
        None => x.loop_in_temp,
    };
    let fan_delta_t = match x.fan {
        Some((fan_in, fan_out)) => fan_out - fan_in,
        None => 0.0,
    };
    let t_sup_no_hc = t_mix_at_min_oa + fan_delta_t;
    let cp_air = PsyCpAirFnW(x.inlet_hum_rat);
    let m = x.zone_mass_flow;
    let extr_rate_no_hc = cp_air * m * (t_sup_no_hc - x.zone_temp);
    let t_set_pt = if m <= SMALL_MASS_FLOW {
        t_sup_no_hc
    } else if x.dead_band || x.zone_load.abs() < SMALL_LOAD {
        if extr_rate_no_hc < 0.0 {
            if extr_rate_no_hc >= x.to_heat {
                t_sup_no_hc
            } else {
                x.zone_temp + x.to_heat / (cp_air * m)
            }
        } else if extr_rate_no_hc > 0.0 {
            if extr_rate_no_hc <= x.to_cool {
                t_sup_no_hc
            } else {
                x.zone_temp + x.to_cool / (cp_air * m)
            }
        } else {
            t_sup_no_hc
        }
    } else if x.zone_load < -SMALL_LOAD {
        let t1 = x.zone_temp + x.zone_load / (cp_air * m);
        let t2 = x.zone_temp + x.to_heat / (cp_air * m);
        if t1 <= t_sup_no_hc {
            t1
        } else if t2 > t_sup_no_hc {
            t2
        } else {
            t_sup_no_hc
        }
    } else if x.zone_load > SMALL_LOAD {
        let t1 = x.zone_temp + x.zone_load / (cp_air * m);
        let t2 = x.zone_temp + x.to_cool / (cp_air * m);
        if t1 >= t_sup_no_hc {
            t1
        } else if t2 < t_sup_no_hc {
            t2
        } else {
            t_sup_no_hc
        }
    } else {
        t_sup_no_hc
    };
    clamp(t_set_pt, x.min_set_temp, x.max_set_temp)
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double SZReheat(int hasoa, int hasfan, int deadband, double oafrac, ...)`
/// (`szreheat.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn SZReheat(
    hasoa: std::ffi::c_int,
    hasfan: std::ffi::c_int,
    deadband: std::ffi::c_int,
    oafrac: f64,
    zonem: f64,
    inw: f64,
    load: f64,
    tocool: f64,
    toheat: f64,
    zonet: f64,
    retw: f64,
    reth: f64,
    oaw: f64,
    oah: f64,
    loopt: f64,
    fanin: f64,
    fanout: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    let flag = |v: std::ffi::c_int| v == 0 || v == 1;
    let ok = flag(hasoa)
        && flag(hasfan)
        && flag(deadband)
        && within(oafrac, 0.0, 1.0)
        && within(zonem, 0.0, 1000.0)
        && [inw, retw, oaw].iter().all(|&w| within(w, 0.0, 0.1))
        && within(reth, -2.0e4, 4.0e5)
        && within(oah, -2.0e4, 4.0e5)
        && [load, tocool, toheat]
            .iter()
            .all(|&q| within(q, -1.0e7, 1.0e7))
        && [zonet, loopt, fanin, fanout, mint]
            .iter()
            .all(|&t| within(t, -60.0, 80.0))
        && within(maxt, mint, 80.0);
    if !ok {
        return f64::NAN;
    }
    calculate(&Inputs {
        mixer: (hasoa == 1).then_some((retw, reth, oaw, oah)),
        fan: (hasfan == 1).then_some((fanin, fanout)),
        dead_band: deadband == 1,
        oa_frac: oafrac,
        zone_mass_flow: zonem,
        inlet_hum_rat: inw,
        zone_load: load,
        to_cool: tocool,
        to_heat: toheat,
        zone_temp: zonet,
        loop_in_temp: loopt,
        min_set_temp: mint,
        max_set_temp: maxt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heating_load_raises_the_supply_temperature() {
        let sp = |load, zonem| {
            SZReheat(
                0, 0, 0, 0.2, zonem, 0.008, load, -500.0, load, 21.0, 0.008, 40_000.0, 0.004,
                10_000.0, 13.0, 0.0, 0.0, 10.0, 50.0,
            )
        };
        assert!(sp(2000.0, 0.5) > 21.0);
        assert_eq!(sp(2000.0, 0.0), 13.0);
        assert_eq!(sp(200_000.0, 0.5), 50.0);
        assert!(
            SZReheat(
                2, 0, 0, 0.2, 0.5, 0.008, 0.0, 0.0, 0.0, 21.0, 0.0, 0.0, 0.0, 0.0, 13.0, 0.0, 0.0,
                10.0, 50.0
            )
            .is_nan()
        );
    }
}
