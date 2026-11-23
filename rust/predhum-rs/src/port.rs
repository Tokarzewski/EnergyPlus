//! The Rust port of `predhum.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneTempPredictorCorrector::ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio` for one
//! zone: its humidistat set points (relative humidity, or dew points turned into relative humidity
//! at the zone air temperature, then EMS overrides, with a humidifying set point above the
//! dehumidifying one brought down to it), the zone moisture balance coefficients, the moisture
//! loads to both set points under the zone air solution algorithm, the load the zone needs, and
//! the demands with the zone multipliers (`ZoneSystemMoistureDemand::reportMoistLoadsZoneMultiplier`).
//! The psychrometric functions come from `psychrometrics-rs`. Every expression keeps the C++ order.
//! The oracle's guards (see its shim) return NaN here too, as do the states EnergyPlus stops on as
//! fatal errors.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::{
    PsyHgAirFnWTdb, PsyRhFnTdbWPb, PsyRhoAirFnPbTdbW, PsyWFnTdbRhPb, PsyWFnTdpPb,
};

/// `DataZoneControls::HumidityCtrlVarType::RelativeHumidity` (`DewPoint` is 0).
const RELATIVE_HUMIDITY: i32 = 1;

/// `DataHeatBalance::SolutionAlgo`.
const THIRD_ORDER: i32 = 0;
const ANALYTICAL_SOLUTION: i32 = 1;

/// ObjexxFCL `min` for doubles: `a < b ? a : b`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// The zone's humidistat, when it has one.
struct Humidistat {
    ctrl_type: i32,
    humidifying: f64,
    dehumidifying: f64,
    ems_humidify: Option<f64>,
    ems_dehumidify: Option<f64>,
}

/// The zone terms `calcPredictedHumidityRatio` reads.
struct Zone {
    humidistat: Option<Humidistat>,
    zt: f64,
    out_baro_press: f64,
    air_hum_rat: f64,
    latent_gain: f64,
    sum_latent_ht_rad_sys: f64,
    sum_latent_pool: f64,
    time_step_sys_sec: f64,
    oamfl: f64,
    vamfl: f64,
    ctmfl: f64,
    eamfl: f64,
    eamfl_x_hum_rat: f64,
    sum_hm_a_ra_w: f64,
    sum_hm_a_ra: f64,
    mixing_mass_flow_x_hum_rat: f64,
    mixing_mass_flow_zone: f64,
    mdot_oa: f64,
    out_hum_rat: f64,
    volume: f64,
    zone_vol_cap_multp_moist: f64,
    algo: i32,
    w_prev_zone_ts_temp: [f64; 3],
    w1: f64,
    rafn_frac: f64,
}

/// The loads: the total, and to the humidifying and dehumidifying set points.
struct MoistLoads {
    total: f64,
    humidify: f64,
    dehumidify: f64,
}

impl Zone {
    /// The relative humidity set points (%) and whether they are equal.
    fn set_points(&self, h: &Humidistat) -> (f64, f64, bool) {
        let (mut hum, mut dehum) = if h.ctrl_type == RELATIVE_HUMIDITY {
            (h.humidifying, h.dehumidifying)
        } else {
            let pb = self.out_baro_press;
            (
                100.0 * PsyRhFnTdbWPb(self.zt, PsyWFnTdpPb(h.humidifying, pb), pb),
                100.0 * PsyRhFnTdbWPb(self.zt, PsyWFnTdpPb(h.dehumidifying, pb), pb),
            )
        };
        if let Some(v) = h.ems_humidify {
            hum = v;
        }
        if let Some(v) = h.ems_dehumidify {
            dehum = v;
        }
        if hum > dehum {
            hum = dehum;
        }
        (hum, dehum, hum == dehum)
    }

    /// The moisture load to reach humidity ratio `w_zone_set_point`.
    fn load_to(&self, w_zone_set_point: f64, a: f64, b: f64, c: f64, exp_700_a_c: f64) -> f64 {
        let w = &self.w_prev_zone_ts_temp;
        let load = match self.algo {
            THIRD_ORDER => {
                ((11.0 / 6.0) * c + a) * w_zone_set_point
                    - (b + c * (3.0 * w[0] - (3.0 / 2.0) * w[1] + (1.0 / 3.0) * w[2]))
            }
            ANALYTICAL_SOLUTION => {
                if a == 0.0 {
                    c * (w_zone_set_point - self.w1) - b
                } else {
                    a * (w_zone_set_point - self.w1 * exp_700_a_c) / (1.0 - exp_700_a_c) - b
                }
            }
            // EulerMethod.
            _ => c * (w_zone_set_point - self.w1) + a * w_zone_set_point - b,
        };
        if self.rafn_frac > 0.0 {
            load / self.rafn_frac
        } else {
            load
        }
    }

    /// `calcPredictedHumidityRatio`; `None` where EnergyPlus stops on a fatal error, and all loads
    /// 0 for a zone with no humidistat.
    fn calc_predicted_humidity_ratio(&self) -> Option<MoistLoads> {
        let Some(h) = &self.humidistat else {
            return Some(MoistLoads {
                total: 0.0,
                humidify: 0.0,
                dehumidify: 0.0,
            });
        };
        let (hum_set_point, dehum_set_point, single_set_point) = self.set_points(h);
        let pb = self.out_baro_press;

        let latent_gain = self.latent_gain + self.sum_latent_ht_rad_sys + self.sum_latent_pool;
        let rho_air = PsyRhoAirFnPbTdbW(pb, self.zt, self.air_hum_rat);
        // A negative air density is a fatal error.
        if rho_air < 0.0 {
            return None;
        }
        let h2o_ht_of_vap = PsyHgAirFnWTdb(self.air_hum_rat, self.zt);
        let b = (latent_gain / h2o_ht_of_vap)
            + ((self.oamfl + self.vamfl + self.ctmfl) * self.out_hum_rat)
            + self.eamfl_x_hum_rat
            + self.sum_hm_a_ra_w
            + self.mixing_mass_flow_x_hum_rat
            + self.mdot_oa * self.out_hum_rat;
        let a = self.oamfl
            + self.vamfl
            + self.eamfl
            + self.ctmfl
            + self.sum_hm_a_ra
            + self.mixing_mass_flow_zone
            + self.mdot_oa;
        let c = rho_air * self.volume * self.zone_vol_cap_multp_moist / self.time_step_sys_sec;

        let exp_700_a_c = if self.algo == ANALYTICAL_SOLUTION && a != 0.0 {
            min(700.0, -a / c).exp()
        } else {
            0.0
        };
        let humidify = self.load_to(
            PsyWFnTdbRhPb(self.zt, hum_set_point / 100.0, pb),
            a,
            b,
            c,
            exp_700_a_c,
        );
        let dehumidify = self.load_to(
            PsyWFnTdbRhPb(self.zt, dehum_set_point / 100.0, pb),
            a,
            b,
            c,
            exp_700_a_c,
        );

        // The C++ tests the single set point first, then both loads above 0; both give the
        // humidifying load.
        let total = if single_set_point || (humidify > 0.0 && dehumidify > 0.0) {
            humidify
        } else if humidify < 0.0 && dehumidify < 0.0 {
            dehumidify
        } else if humidify <= 0.0 && dehumidify >= 0.0 {
            0.0
        } else {
            // Loads that compare as none of these: a fatal error.
            return None;
        };
        Some(MoistLoads {
            total,
            humidify,
            dehumidify,
        })
    }
}

/// One result of `calcPredictedHumidityRatio` for one zone (see the oracle's shim for the
/// arguments and `which`).
pub fn PredictedHumidityRatio(
    which: i32,
    ctrltype: i32,
    humsp: f64,
    dehumsp: f64,
    emshumon: i32,
    emshumval: f64,
    emsdehumon: i32,
    emsdehumval: f64,
    zt: f64,
    outbaro: f64,
    airhumrat: f64,
    latentgain: f64,
    latentradsys: f64,
    latentpool: f64,
    tsyssec: f64,
    oamfl: f64,
    vamfl: f64,
    ctmfl: f64,
    eamfl: f64,
    eamflxw: f64,
    sumhmaraw: f64,
    sumhmara: f64,
    mixw: f64,
    mixm: f64,
    mdotoa: f64,
    outhumrat: f64,
    volume: f64,
    volcapmult: f64,
    algo: i32,
    wprev0: f64,
    wprev1: f64,
    wprev2: f64,
    w1: f64,
    rafnfrac: f64,
    mult: i32,
    listmult: i32,
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    if !(0..=5).contains(&which)
        || !flag(emshumon)
        || !flag(emsdehumon)
        || !(-1..=1).contains(&ctrltype)
        || !(0..=2).contains(&algo)
        || !(1..=10000).contains(&mult)
        || !(1..=10000).contains(&listmult)
        || !(10000.0..=200000.0).contains(&outbaro)
        || (ctrltype == 0 && !(humsp.is_finite() && dehumsp.is_finite()))
    {
        return f64::NAN;
    }
    let z = Zone {
        humidistat: (ctrltype != -1).then_some(Humidistat {
            ctrl_type: ctrltype,
            humidifying: humsp,
            dehumidifying: dehumsp,
            ems_humidify: (emshumon == 1).then_some(emshumval),
            ems_dehumidify: (emsdehumon == 1).then_some(emsdehumval),
        }),
        zt,
        out_baro_press: outbaro,
        air_hum_rat: airhumrat,
        latent_gain: latentgain,
        sum_latent_ht_rad_sys: latentradsys,
        sum_latent_pool: latentpool,
        time_step_sys_sec: tsyssec,
        oamfl,
        vamfl,
        ctmfl,
        eamfl,
        eamfl_x_hum_rat: eamflxw,
        sum_hm_a_ra_w: sumhmaraw,
        sum_hm_a_ra: sumhmara,
        mixing_mass_flow_x_hum_rat: mixw,
        mixing_mass_flow_zone: mixm,
        mdot_oa: mdotoa,
        out_hum_rat: outhumrat,
        volume,
        zone_vol_cap_multp_moist: volcapmult,
        algo,
        w_prev_zone_ts_temp: [wprev0, wprev1, wprev2],
        w1,
        rafn_frac: rafnfrac,
    };
    let Some(l) = z.calc_predicted_humidity_ratio() else {
        return f64::NAN;
    };
    let zone_mult_fac = f64::from(mult * listmult);
    match which {
        0 => l.total,
        1 => l.humidify,
        2 => l.dehumidify,
        3 => l.total * zone_mult_fac,
        4 => l.humidify * zone_mult_fac,
        _ => l.dehumidify * zone_mult_fac,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 200 m3 zone at 22 C and 0.008 with 500 W of latent gain and 0.1 kg/s of infiltration at
    /// 0.006, humidistat type `ct` with set points `h` and `dh`, under algorithm `algo`.
    fn zone(which: i32, ct: i32, h: f64, dh: f64, algo: i32) -> f64 {
        PredictedHumidityRatio(
            which, ct, h, dh, 0, 0.0, 0, 0.0, 22.0, 101325.0, 0.008, 500.0, 0.0, 0.0, 900.0, 0.1,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.006, 200.0, 1.0, algo, 0.008, 0.008,
            0.008, 0.008, 0.0, 2, 1,
        )
    }

    #[test]
    fn deadband_needs_no_load() {
        assert_eq!(zone(0, 1, 30.0, 60.0, 0), 0.0);
        assert!(zone(1, 1, 30.0, 60.0, 0) < 0.0);
        assert!(zone(2, 1, 30.0, 60.0, 0) > 0.0);
        assert_eq!(zone(5, 1, 30.0, 60.0, 0), 2.0 * zone(2, 1, 30.0, 60.0, 0));
    }

    #[test]
    fn crossed_set_points_become_one() {
        // A humidifying set point above the dehumidifying one is brought down to it.
        assert_eq!(zone(0, 1, 70.0, 60.0, 0), zone(2, 1, 70.0, 60.0, 0));
        assert!(zone(0, 1, 70.0, 60.0, 0) > 0.0);
    }

    #[test]
    fn no_humidistat_and_guards() {
        assert_eq!(zone(3, -1, 0.0, 0.0, 0), 0.0);
        assert!(zone(6, 1, 30.0, 60.0, 0).is_nan());
        assert!(zone(0, 0, f64::INFINITY, 12.0, 0).is_nan());
        assert!(zone(0, 1, 30.0, 60.0, 3).is_nan());
    }
}
