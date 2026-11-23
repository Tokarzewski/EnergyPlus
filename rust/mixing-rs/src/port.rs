//! The Rust port of `mixing.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The zone mixing part of `ZoneEquipmentManager::CalcAirFlowSimple` for one ZoneMixing object
//! that brings air from a source zone into a receiving zone: hybrid ventilation closing, the
//! zone, source zone and outdoor temperature limits, the delta temperature test, and the
//! receiving zone's mixing sums. The psychrometric functions come from `psychrometrics-rs`. Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::{PsyCpAirFnW, PsyRhoAirFnPbTdbW};

/// A zone's air: MAT and humidity ratio now and at the previous system time step.
#[derive(Clone, Copy)]
struct ZoneAir {
    mat: f64,
    air_hum_rat: f64,
    xmpt: f64,
    w_time_minus_p: f64,
}

impl ZoneAir {
    /// MixingMAT and MixingHumRat for system time step loop `systs`.
    fn mixing(self, systs: i32) -> (f64, f64) {
        if systs == 1 {
            (self.xmpt, self.w_time_minus_p)
        } else {
            (self.mat, self.air_hum_rat)
        }
    }
}

/// The mixing object's inputs; a limit is `Some` when its schedule exists.
struct Mixing {
    hybrid_close: bool,
    desired_air_flow_rate_saved: f64,
    delta_temperature: f64,
    min_indoor: Option<f64>,
    max_indoor: Option<f64>,
    min_source: Option<f64>,
    max_source: Option<f64>,
    min_outdoor: Option<f64>,
    max_outdoor: Option<f64>,
}

/// What the calculation sets: the receiving zone's MCPM, MCPTM, MixingMassFlowZone and
/// MixingMassFlowXHumRat, and the object's DesiredAirFlowRate, MixingMassFlowRate and ReportFlag.
#[derive(Default)]
struct Results {
    mcpm: f64,
    mcptm: f64,
    mixing_mass_flow_zone: f64,
    mixing_mass_flow_x_hum_rat: f64,
    desired_air_flow_rate: f64,
    mixing_mass_flow_rate: f64,
    report_flag: bool,
}

/// True when `t` is outside the scheduled limits.
fn outside(t: f64, min: Option<f64>, max: Option<f64>) -> bool {
    min.is_some_and(|m| t < m) || max.is_some_and(|m| t > m)
}

/// The mixing loop of `CalcAirFlowSimple` for the one object, `None` on a negative air density (a
/// fatal error).
fn calc_mixing(
    m: &Mixing,
    tzn: f64,
    hum_rat_zn: f64,
    tzm: f64,
    hum_rat_zm: f64,
    temp_ext: f64,
    out_baro_press: f64,
) -> Option<Results> {
    let mut r = Results::default();
    let td = m.delta_temperature;
    if m.hybrid_close {
        return Some(r);
    }
    let mixing_limit_flag = outside(tzn, m.min_indoor, m.max_indoor)
        || outside(tzm, m.min_source, m.max_source)
        || outside(temp_ext, m.min_outdoor, m.max_outdoor);
    if mixing_limit_flag {
        return Some(r);
    }

    let air_density = PsyRhoAirFnPbTdbW(
        out_baro_press,
        (tzn + tzm) / 2.0,
        (hum_rat_zn + hum_rat_zm) / 2.0,
    );
    if air_density < 0.0 {
        return None;
    }
    let cp_air = PsyCpAirFnW((hum_rat_zn + hum_rat_zm) / 2.0);

    let mut this_mcpm = 0.0;
    let mut this_mcptm = 0.0;
    let mut this_mixing_mass_flow = 0.0;
    let mut this_mixing_mass_flow_x_hum_rat = 0.0;
    if td < 0.0 {
        if tzm < tzn + td {
            r.desired_air_flow_rate = m.desired_air_flow_rate_saved;
            r.mixing_mass_flow_rate = r.desired_air_flow_rate * air_density;
            this_mcpm = r.mixing_mass_flow_rate * cp_air;
            this_mcptm = this_mcpm * tzn;
            this_mixing_mass_flow = r.desired_air_flow_rate * air_density;
            this_mixing_mass_flow_x_hum_rat = r.desired_air_flow_rate * air_density * hum_rat_zm;
            r.report_flag = true;
        }
    } else if (td > 0.0 && tzm > tzn + td) || td == 0.0 {
        r.desired_air_flow_rate = m.desired_air_flow_rate_saved;
        r.mixing_mass_flow_rate = r.desired_air_flow_rate * air_density;
        this_mcpm = r.mixing_mass_flow_rate * cp_air;
        this_mcptm = this_mcpm * tzm;
        this_mixing_mass_flow = r.mixing_mass_flow_rate;
        this_mixing_mass_flow_x_hum_rat = r.mixing_mass_flow_rate * hum_rat_zm;
        r.report_flag = true;
    }
    r.mcpm += this_mcpm;
    r.mcptm += this_mcptm;
    r.mixing_mass_flow_zone += this_mixing_mass_flow;
    r.mixing_mass_flow_x_hum_rat += this_mixing_mass_flow_x_hum_rat;
    Some(r)
}

/// A 0 or 1 flag as a bool, `None` for anything else.
fn flag(v: i32) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// Port of `double Mixing(...)` (`mixing.c:8`): see the oracle's shim for the arguments and what
/// `which` picks.
pub fn Mixing(
    which: std::ffi::c_int,
    nmix: std::ffi::c_int,
    hybrid: std::ffi::c_int,
    saved: f64,
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
            flag(hybrid)?,
            flag(hastd)?,
            flag(hmint)?,
            flag(hmaxt)?,
            flag(hmins)?,
            flag(hmaxs)?,
            flag(hmino)?,
            flag(hmaxo)?,
        ];
        let [_, _, _, a, b, c, d, e, f] = flags;
        let crossing =
            (a && b && mint > maxt) || (c && d && mins > maxs) || (e && f && mino > maxo);
        ((0..=6).contains(&which) && !crossing).then_some(flags)
    })();
    let Some(
        [
            nmix,
            hybrid_close,
            _,
            hmint,
            hmaxt,
            hmins,
            hmaxs,
            hmino,
            hmaxo,
        ],
    ) = checked
    else {
        return f64::NAN;
    };
    let r = if nmix {
        let m = Mixing {
            hybrid_close,
            desired_air_flow_rate_saved: saved,
            // The delta temperature schedule holds the same value as the object.
            delta_temperature: td,
            min_indoor: hmint.then_some(mint),
            max_indoor: hmaxt.then_some(maxt),
            min_source: hmins.then_some(mins),
            max_source: hmaxs.then_some(maxs),
            min_outdoor: hmino.then_some(mino),
            max_outdoor: hmaxo.then_some(maxo),
        };
        let air = |mat, air_hum_rat, xmpt, w_time_minus_p| ZoneAir {
            mat,
            air_hum_rat,
            xmpt,
            w_time_minus_p,
        };
        let (tzn, hum_rat_zn) = air(mat1, hum1, xmpt1, wprev1).mixing(systs);
        let (tzm, hum_rat_zm) = air(mat2, hum2, xmpt2, wprev2).mixing(systs);
        let Some(r) = calc_mixing(&m, tzn, hum_rat_zn, tzm, hum_rat_zm, outdb, outbaro) else {
            return f64::NAN;
        };
        r
    } else {
        Results::default()
    };
    match which {
        0 => r.mcpm,
        1 => r.mcptm,
        2 => r.mixing_mass_flow_zone,
        3 => r.mixing_mass_flow_x_hum_rat,
        4 => r.desired_air_flow_rate,
        5 => r.mixing_mass_flow_rate,
        _ => {
            if r.report_flag {
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

    /// 0.5 m3/s from a 26 C source zone into a 21 C zone, delta temperature `td`, later system
    /// time step, minimum zone temperature schedule at `mint` when `hmint` is 1.
    fn sunspace(which: i32, td: f64, hmint: i32, mint: f64) -> f64 {
        Mixing(
            which, 1, 0, 0.5, td, 0, hmint, mint, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 2, 21.0,
            0.008, 20.0, 0.008, 26.0, 0.010, 25.0, 0.010, 5.0, 101325.0,
        )
    }

    #[test]
    fn full_mixing() {
        let rho = PsyRhoAirFnPbTdbW(101325.0, (21.0 + 26.0) / 2.0, (0.008 + 0.010) / 2.0);
        let cp = PsyCpAirFnW((0.008 + 0.010) / 2.0);
        assert_eq!(sunspace(5, 0.0, 0, 0.0), 0.5 * rho);
        assert_eq!(sunspace(0, 0.0, 0, 0.0), 0.5 * rho * cp);
        assert_eq!(sunspace(1, 0.0, 0, 0.0), 0.5 * rho * cp * 26.0);
        assert_eq!(sunspace(3, 0.0, 0, 0.0), 0.5 * rho * 0.010);
        assert_eq!(sunspace(6, 0.0, 0, 0.0), 1.0);
    }

    #[test]
    fn delta_temperature_test() {
        assert!(sunspace(0, 2.0, 0, 0.0) > 0.0);
        assert_eq!(sunspace(0, 6.0, 0, 0.0), 0.0);
        assert_eq!(sunspace(6, -2.0, 0, 0.0), 0.0);
    }

    #[test]
    fn limits_stop_mixing() {
        assert_eq!(sunspace(0, 0.0, 1, 22.0), 0.0);
        assert!(sunspace(0, 0.0, 1, 20.0) > 0.0);
    }

    #[test]
    fn guards_return_nan() {
        assert!(sunspace(7, 0.0, 0, 0.0).is_nan());
        assert!(sunspace(0, 0.0, 2, 0.0).is_nan());
    }
}
