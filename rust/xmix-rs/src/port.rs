//! The Rust port of `xmix.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The cross mixing part of `ZoneEquipmentManager::CalcAirFlowSimple` for one ZoneCrossMixing
//! object that exchanges equal air flows between a zone and its source zone: the zone, source zone
//! and outdoor temperature limits, the delta temperature test, and both zones' mixing sums. The
//! psychrometric functions come from `psychrometrics-rs`. Every expression keeps the C++ order.
//! The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::{PsyCpAirFnW, PsyRhoAirFnPbTdbW};

/// The cross mixing object's inputs; a limit is `Some` when its schedule exists.
struct CrossMixing {
    desired_air_flow_rate: f64,
    delta_temperature: f64,
    min_indoor: Option<f64>,
    max_indoor: Option<f64>,
    min_source: Option<f64>,
    max_source: Option<f64>,
    min_outdoor: Option<f64>,
    max_outdoor: Option<f64>,
}

/// A zone's mixing sums: MCPM, MCPTM, MixingMassFlowZone and MixingMassFlowXHumRat.
#[derive(Clone, Copy, Default)]
struct Sums {
    mcpm: f64,
    mcptm: f64,
    mixing_mass_flow_zone: f64,
    mixing_mass_flow_x_hum_rat: f64,
}

/// True when `t` is outside the scheduled limits.
fn outside(t: f64, min: Option<f64>, max: Option<f64>) -> bool {
    min.is_some_and(|m| t < m) || max.is_some_and(|m| t > m)
}

/// The cross mixing loop of `CalcAirFlowSimple` for the one object: both zones' sums and the
/// report flag, `None` on a negative air density (a fatal error).
fn calc_cross_mixing(
    m: &CrossMixing,
    (tzn, hum_rat_zn): (f64, f64),
    (tzm, hum_rat_zm): (f64, f64),
    temp_ext: f64,
    out_baro_press: f64,
) -> Option<(Sums, Sums, bool)> {
    let td = m.delta_temperature;
    let mut report_flag = false;
    let mut this_mcpxm = 0.0;
    let mut this_mcptxm = 0.0;
    let mut this_x_mixing_mass_flow = 0.0;
    let mut this_x_mixing_mass_flow_x_hum_rat = 0.0;
    let mut from_mcpxm = 0.0;
    let mut from_mcptxm = 0.0;
    let mut from_x_mixing_mass_flow_x_hum_rat = 0.0;

    if td >= 0.0 {
        let mixing_limit_flag = outside(tzn, m.min_indoor, m.max_indoor)
            || outside(tzm, m.min_source, m.max_source)
            || outside(temp_ext, m.min_outdoor, m.max_outdoor);
        if mixing_limit_flag {
            return Some((Sums::default(), Sums::default(), false));
        }
        if td == 0.0 || (td > 0.0 && (tzm - tzn) >= td) {
            report_flag = true;
        }
        if td <= 0.0 || tzm - tzn >= td {
            let tavg = (tzn + tzm) / 2.0;
            let wavg = (hum_rat_zn + hum_rat_zm) / 2.0;
            let air_density = PsyRhoAirFnPbTdbW(out_baro_press, tavg, wavg);
            if air_density < 0.0 {
                return None;
            }
            let cp_air = PsyCpAirFnW(wavg);
            this_x_mixing_mass_flow = m.desired_air_flow_rate * air_density;
            this_mcpxm = this_x_mixing_mass_flow * cp_air;
            from_mcpxm = this_mcpxm;
            this_mcptxm = this_mcpxm * tzm;
            from_mcptxm = from_mcpxm * tzn;
            from_x_mixing_mass_flow_x_hum_rat = this_x_mixing_mass_flow * hum_rat_zn;
            this_x_mixing_mass_flow_x_hum_rat = this_x_mixing_mass_flow * hum_rat_zm;
        }
    }
    let mut this = Sums::default();
    this.mcpm += this_mcpxm;
    this.mcptm += this_mcptxm;
    this.mixing_mass_flow_zone += this_x_mixing_mass_flow;
    this.mixing_mass_flow_x_hum_rat += this_x_mixing_mass_flow_x_hum_rat;
    let mut from = Sums::default();
    from.mcpm += from_mcpxm;
    from.mcptm += from_mcptxm;
    from.mixing_mass_flow_zone += this_x_mixing_mass_flow;
    from.mixing_mass_flow_x_hum_rat += from_x_mixing_mass_flow_x_hum_rat;
    Some((this, from, report_flag))
}

/// A 0 or 1 flag as a bool, `None` for anything else.
fn flag(v: i32) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// MixingMAT and MixingHumRat of a zone for system time step loop `systs`.
fn mixing_air(systs: i32, mat: f64, hum: f64, xmpt: f64, wprev: f64) -> (f64, f64) {
    if systs == 1 {
        (xmpt, wprev)
    } else {
        (mat, hum)
    }
}

/// Port of `double CrossMixing(...)` (`xmix.c:8`): see the oracle's shim for the arguments and
/// what `which` picks.
pub fn CrossMixing(
    which: std::ffi::c_int,
    nmix: std::ffi::c_int,
    desired: f64,
    td: f64,
    hastd: std::ffi::c_int,
    hmint: std::ffi::c_int,
    mint: f64,
    hmaxt: std::ffi::c_int,
    maxt: f64,
    hmins: std::ffi::c_int,
    mins: f64,
    hmaxs: std::ffi::c_int,
    maxs: f64,
    hmino: std::ffi::c_int,
    mino: f64,
    hmaxo: std::ffi::c_int,
    maxo: f64,
    systs: std::ffi::c_int,
    mat1: f64,
    hum1: f64,
    xmpt1: f64,
    wprev1: f64,
    mat2: f64,
    hum2: f64,
    xmpt2: f64,
    wprev2: f64,
    outdb: f64,
    outbaro: f64,
) -> f64 {
    let checked = (|| {
        let flags = [
            flag(nmix)?,
            flag(hastd)?,
            flag(hmint)?,
            flag(hmaxt)?,
            flag(hmins)?,
            flag(hmaxs)?,
            flag(hmino)?,
            flag(hmaxo)?,
        ];
        let [_, _, a, b, c, d, e, f] = flags;
        let crossing =
            (a && b && mint > maxt) || (c && d && mins > maxs) || (e && f && mino > maxo);
        ((0..=8).contains(&which) && !crossing).then_some(flags)
    })();
    let Some([nmix, _, hmint, hmaxt, hmins, hmaxs, hmino, hmaxo]) = checked else {
        return f64::NAN;
    };
    let (this, from, report_flag) = if nmix {
        let m = CrossMixing {
            desired_air_flow_rate: desired,
            // The delta temperature schedule holds the same value as the object.
            delta_temperature: td,
            min_indoor: hmint.then_some(mint),
            max_indoor: hmaxt.then_some(maxt),
            min_source: hmins.then_some(mins),
            max_source: hmaxs.then_some(maxs),
            min_outdoor: hmino.then_some(mino),
            max_outdoor: hmaxo.then_some(maxo),
        };
        let zn = mixing_air(systs, mat1, hum1, xmpt1, wprev1);
        let zm = mixing_air(systs, mat2, hum2, xmpt2, wprev2);
        let Some(r) = calc_cross_mixing(&m, zn, zm, outdb, outbaro) else {
            return f64::NAN;
        };
        r
    } else {
        (Sums::default(), Sums::default(), false)
    };
    let pick = |s: Sums, i: i32| match i {
        0 => s.mcpm,
        1 => s.mcptm,
        2 => s.mixing_mass_flow_zone,
        _ => s.mixing_mass_flow_x_hum_rat,
    };
    match which {
        0..=3 => pick(this, which),
        4..=7 => pick(from, which - 4),
        _ => {
            if report_flag {
                1.0
            } else {
                0.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 0.4 m3/s exchanged between a 21 C zone and a 25 C source zone, delta temperature `td`,
    /// later system time step, minimum zone temperature schedule at `mint` when `hmint` is 1.
    fn atrium(which: i32, td: f64, hmint: i32, mint: f64) -> f64 {
        CrossMixing(
            which, 1, 0.4, td, 0, hmint, mint, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 2, 21.0,
            0.008, 20.0, 0.008, 25.0, 0.010, 24.0, 0.010, 5.0, 101325.0,
        )
    }

    #[test]
    fn equal_exchange() {
        let rho = PsyRhoAirFnPbTdbW(101325.0, (21.0 + 25.0) / 2.0, (0.008 + 0.010) / 2.0);
        let cp = PsyCpAirFnW((0.008 + 0.010) / 2.0);
        assert_eq!(atrium(0, 0.0, 0, 0.0), 0.4 * rho * cp);
        assert_eq!(atrium(4, 0.0, 0, 0.0), 0.4 * rho * cp);
        assert_eq!(atrium(1, 0.0, 0, 0.0), 0.4 * rho * cp * 25.0);
        assert_eq!(atrium(5, 0.0, 0, 0.0), 0.4 * rho * cp * 21.0);
        assert_eq!(atrium(7, 0.0, 0, 0.0), 0.4 * rho * 0.008);
        assert_eq!(atrium(8, 0.0, 0, 0.0), 1.0);
    }

    #[test]
    fn delta_temperature_and_limits() {
        assert!(atrium(0, 3.0, 0, 0.0) > 0.0);
        assert_eq!(atrium(0, 5.0, 0, 0.0), 0.0);
        assert_eq!(atrium(0, -1.0, 0, 0.0), 0.0);
        assert_eq!(atrium(0, 0.0, 1, 22.0), 0.0);
    }

    #[test]
    fn guards_return_nan() {
        assert!(atrium(9, 0.0, 0, 0.0).is_nan());
        assert!(atrium(0, 0.0, 2, 0.0).is_nan());
    }
}
