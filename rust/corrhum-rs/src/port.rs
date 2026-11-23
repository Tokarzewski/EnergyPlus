//! The Rust port of `corrhum.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneTempPredictorCorrector::ZoneSpaceHeatBalanceData::correctHumRat` for one zone: the
//! moisture and air mass flows from its supply air inlets (divided by the zone multipliers), the
//! zone moisture balance coefficients, the new zone air humidity ratio under the zone air solution
//! algorithm (at least 0, at most saturation at the zone air temperature), and the system node's
//! humidity ratio and enthalpy. The psychrometric functions come from `psychrometrics-rs`. Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too, as does
//! the negative air density EnergyPlus stops on as a fatal error.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::{PsyHFnTdbW, PsyHgAirFnWTdb, PsyRhoAirFnPbTdbW, PsyWFnTdbRhPb};

/// `DataHeatBalance::SolutionAlgo`.
const THIRD_ORDER: i32 = 0;
const ANALYTICAL_SOLUTION: i32 = 1;

/// ObjexxFCL `min` for doubles: `a < b ? a : b`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// The zone terms `correctHumRat` reads.
pub struct Zone<'a> {
    /// The supply air inlets (mass flow, humidity ratio) of a controlled zone.
    pub inlets: &'a [(f64, f64)],
    pub zone_mult: i32,
    pub zt: f64,
    pub out_baro_press: f64,
    pub air_hum_rat: f64,
    pub latent_gain: f64,
    pub sum_latent_ht_rad_sys: f64,
    pub sum_latent_pool: f64,
    pub time_step_sys_sec: f64,
    pub oamfl: f64,
    pub vamfl: f64,
    pub ctmfl: f64,
    pub eamfl: f64,
    pub eamfl_x_hum_rat: f64,
    pub sum_hm_a_ra_w: f64,
    pub sum_hm_a_ra: f64,
    pub mixing_mass_flow_x_hum_rat: f64,
    pub mixing_mass_flow_zone: f64,
    pub mdot_oa: f64,
    pub out_hum_rat: f64,
    pub volume: f64,
    pub zone_vol_cap_multp_moist: f64,
    pub algo: i32,
    pub w_prev_zone_ts_temp: [f64; 3],
    pub w1: f64,
}

impl Zone<'_> {
    /// `correctHumRat`'s new `airHumRatTemp`; `None` where EnergyPlus stops on a fatal error.
    pub fn correct_hum_rat(&self) -> Option<f64> {
        let zone_mult = f64::from(self.zone_mult);
        let mut moisture_mass_flow_rate = 0.0;
        let mut zone_mass_flow_rate = 0.0;
        for &(mass_flow_rate, hum_rat) in self.inlets {
            moisture_mass_flow_rate += (mass_flow_rate * hum_rat) / zone_mult;
            zone_mass_flow_rate += mass_flow_rate / zone_mult;
        }

        let latent_gain = self.latent_gain + self.sum_latent_ht_rad_sys + self.sum_latent_pool;
        let rho_air = PsyRhoAirFnPbTdbW(self.out_baro_press, self.zt, self.air_hum_rat);
        // A negative air density is a fatal error.
        if rho_air < 0.0 {
            return None;
        }
        let h2o_ht_of_vap = PsyHgAirFnWTdb(self.air_hum_rat, self.zt);
        let b = (latent_gain / h2o_ht_of_vap)
            + ((self.oamfl + self.vamfl + self.ctmfl) * self.out_hum_rat)
            + self.eamfl_x_hum_rat
            + moisture_mass_flow_rate
            + self.sum_hm_a_ra_w
            + self.mixing_mass_flow_x_hum_rat
            + self.mdot_oa * self.out_hum_rat;
        let a = zone_mass_flow_rate
            + self.oamfl
            + self.vamfl
            + self.eamfl
            + self.ctmfl
            + self.sum_hm_a_ra
            + self.mixing_mass_flow_zone
            + self.mdot_oa;
        let c = rho_air * self.volume * self.zone_vol_cap_multp_moist / self.time_step_sys_sec;

        let w = &self.w_prev_zone_ts_temp;
        let mut air_hum_rat_temp = match self.algo {
            THIRD_ORDER => {
                (b + c * (3.0 * w[0] - (3.0 / 2.0) * w[1] + (1.0 / 3.0) * w[2]))
                    / ((11.0 / 6.0) * c + a)
            }
            ANALYTICAL_SOLUTION => {
                if a == 0.0 {
                    self.w1 + b / c
                } else {
                    (self.w1 - b / a) * min(700.0, -a / c).exp() + b / a
                }
            }
            // EulerMethod.
            _ => (c * self.w1 + b) / (c + a),
        };

        // Dried out.
        if air_hum_rat_temp < 0.0 {
            air_hum_rat_temp = 0.0;
        }
        // Condensation: back to saturation.
        let wz_sat = PsyWFnTdbRhPb(self.zt, 1.0, self.out_baro_press);
        if air_hum_rat_temp > wz_sat {
            air_hum_rat_temp = wz_sat;
        }
        Some(air_hum_rat_temp)
    }
}

/// One result of `correctHumRat` for one zone (see the oracle's shim for the arguments and
/// `which`).
pub fn CorrectHumRat(
    which: i32,
    controlled: i32,
    ninlets: i32,
    m1: f64,
    w1in: f64,
    m2: f64,
    w2in: f64,
    m3: f64,
    w3in: f64,
    hasnode: i32,
    mult: i32,
    listmult: i32,
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
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    if !(0..=2).contains(&which)
        || !flag(controlled)
        || !flag(hasnode)
        || !(0..=3).contains(&ninlets)
        || !(0..=2).contains(&algo)
        || !(1..=10000).contains(&mult)
        || !(1..=10000).contains(&listmult)
    {
        return f64::NAN;
    }
    let all_inlets = [(m1, w1in), (m2, w2in), (m3, w3in)];
    let n = if controlled == 1 {
        ninlets.unsigned_abs() as usize
    } else {
        0
    };
    let z = Zone {
        inlets: &all_inlets[..n],
        zone_mult: mult * listmult,
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
    };
    let Some(w) = z.correct_hum_rat() else {
        return f64::NAN;
    };
    match which {
        0 => w,
        1 => {
            if hasnode == 1 {
                w
            } else {
                0.0
            }
        }
        _ => {
            if hasnode == 1 {
                PsyHFnTdbW(zt, w)
            } else {
                0.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 200 m3 controlled zone at 22 C and 0.008 under the Euler method, with one inlet of
    /// `m` kg/s at 0.006 and no other moisture terms.
    fn zone(which: i32, m: f64, latentgain: f64) -> f64 {
        CorrectHumRat(
            which, 1, 1, m, 0.006, 0.0, 0.0, 0.0, 0.0, 1, 1, 1, 22.0, 101325.0, 0.008, latentgain,
            0.0, 0.0, 900.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.006, 200.0, 1.0,
            2, 0.008, 0.008, 0.008, 0.008,
        )
    }

    #[test]
    fn no_flow_and_no_gain_keeps_the_humidity_ratio() {
        assert_eq!(zone(0, 0.0, 0.0), 0.008);
        assert_eq!(zone(1, 0.0, 0.0), 0.008);
    }

    #[test]
    fn dry_supply_air_dries_the_zone() {
        let w = zone(0, 1.0, 0.0);
        assert!(w < 0.008 && w > 0.006);
        assert_eq!(zone(2, 1.0, 0.0), PsyHFnTdbW(22.0, w));
    }

    #[test]
    fn limits_and_guards() {
        // A huge humidifier is held at saturation.
        assert_eq!(zone(0, 0.0, 1.0e9), PsyWFnTdbRhPb(22.0, 1.0, 101325.0));
        // A huge drier dries the zone out to 0.
        assert_eq!(zone(0, 0.0, -1.0e9), 0.0);
        assert!(zone(3, 0.0, 0.0).is_nan());
    }
}
