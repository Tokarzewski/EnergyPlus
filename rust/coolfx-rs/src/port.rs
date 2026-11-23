//! The Rust port of `coolfx.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ThermalComfort::CalcCoolingEffectAdjustedPMV` (ANSI/ASHRAE 55 Appendix D): the elevated air
//! speed cooling effect, the temperature drop in still air that gives the same standard effective
//! temperature, found by `General::SolveRoot` (`solveroot-rs`) over the standard effective
//! temperature (`fanger-rs`), and the PMV adjusted for it, with its PPD (`comfort-rs`). Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use comfort_rs::port::{CalcFangerPPD, CalcRelativeAirVelocity};
use fanger_rs::port::{Person, calc_fanger_pmv, calc_standard_effective_temp};
use solveroot_rs::port::{algo, solve_root};

/// `ThermalComfort::ActLevelConv` (W/m2 per met).
const ACT_LEVEL_CONV: f64 = 58.2;

/// `CalcCoolingEffectAdjustedPMV` at outdoor barometric pressure `out_baro_press`, for a person
/// with activity `act_level` (W/m2): the cooling effect and the adjusted PMV.
fn calc_cooling_effect_adjusted_pmv(out_baro_press: f64, p: Person, act_level: f64) -> (f64, f64) {
    let rel_air_vel = CalcRelativeAirVelocity(p.air_vel, p.act_met);
    let set = calc_standard_effective_temp(
        out_baro_press,
        Person {
            air_vel: rel_air_vel,
            ..p
        },
    )
    .set;
    let ashrae55_pmv = calc_fanger_pmv(
        p.air_temp,
        p.rad_temp,
        p.rel_hum,
        rel_air_vel,
        act_level,
        p.clo_unit,
        p.work_eff,
    )
    .pmv;

    let still_air_vel = 0.1;
    let ce_root_function = |x: f64| {
        let still = Person {
            air_temp: p.air_temp - x,
            rad_temp: p.rad_temp - x,
            air_vel: still_air_vel,
            ..p
        };
        calc_standard_effective_temp(out_baro_press, still).set - set
    };
    // state.dataRootFinder's defaults: regula falsi, 5 iterations before a switch.
    let (sol_fla, x_res) = solve_root(
        algo::REGULA_FALSI,
        5,
        0.001,
        100,
        ce_root_function,
        0.0,
        50.0,
    );
    let cooling_effect = if sol_fla < 0 { 0.0 } else { x_res };

    let adjusted = if cooling_effect > 0.0 {
        calc_fanger_pmv(
            p.air_temp - cooling_effect,
            p.rad_temp - cooling_effect,
            p.rel_hum,
            still_air_vel,
            act_level,
            p.clo_unit,
            p.work_eff,
        )
        .pmv
    } else {
        ashrae55_pmv
    };
    (cooling_effect, adjusted)
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double CoolingEffect(int which, double outbaro, double at, double rt, double rh,
/// double vel, double act, double clo, double work)` (`coolfx.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn CoolingEffect(
    which: std::ffi::c_int,
    outbaro: f64,
    at: f64,
    rt: f64,
    rh: f64,
    vel: f64,
    act: f64,
    clo: f64,
    work: f64,
) -> f64 {
    let ok = (0..=2).contains(&which)
        && within(outbaro, 60000.0, 110000.0)
        && within(at, 10.0, 40.0)
        && within(rt, 10.0, 40.0)
        && within(rh, 0.0, 1.0)
        && within(vel, 0.0, 5.0)
        && within(act, 0.7 * 58.2, 4.0 * 58.2)
        && within(clo, 0.0, 3.0)
        && within(work, 0.0, act / 2.0);
    if !ok {
        return f64::NAN;
    }
    let person = Person {
        air_temp: at,
        rad_temp: rt,
        rel_hum: rh,
        air_vel: vel,
        act_met: act / ACT_LEVEL_CONV,
        clo_unit: clo,
        work_eff: work,
    };
    let (ce, pmv) = calc_cooling_effect_adjusted_pmv(outbaro, person, act);
    match which {
        0 => ce,
        1 => pmv,
        _ => CalcFangerPPD(pmv),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faster_air_cools_more() {
        let args =
            |which, vel| CoolingEffect(which, 101325.0, 28.0, 28.0, 0.5, vel, 1.1 * 58.2, 0.5, 0.0);
        assert!(args(0, 0.8) > 0.5);
        assert!(args(0, 0.1) < args(0, 0.8));
        assert!(args(1, 0.8) < args(1, 0.1));
        assert!(args(3, 0.1).is_nan());
        assert!(CoolingEffect(0, 101325.0, 45.0, 28.0, 0.5, 0.8, 64.0, 0.5, 0.0).is_nan());
    }
}
