//! The Rust port of `meanair.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatBalanceAirManager::calcMeanAirTemps` for one zone: the mean air temperature and humidity
//! ratio, the operative temperature, the dew point, the thermostat's operative temperature with
//! its radiative fraction, and the wet bulb globe temperature. The psychrometric functions come
//! from `psychrometrics-rs`. Every expression keeps the C++ order. The oracle's guards (see its
//! shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::{PsyTdpFnWPb, PsyTwbFnTdbWPb};

/// `DataHeatBalance::AirReportVars`: the values calcMeanAirTemps sets, starting at 0.
#[derive(Default)]
struct AirReportVars {
    mean_air_temp: f64,
    mean_air_hum_rat: f64,
    operative_temp: f64,
    mean_air_dew_point_temp: f64,
    therm_operative_temp: f64,
    wetbulb_globe_temp: f64,
}

/// `calcMeanAirTemps`. `op_frac` is the thermostat's radiative fraction when the model has
/// operative temperature control, the zone is controlled and its thermostat uses it.
fn calc_mean_air_temps(
    ztav: f64,
    air_hum_rat_avg: f64,
    mrt: f64,
    out_baro_press: f64,
    op_frac: Option<f64>,
    report_wbgt: bool,
) -> AirReportVars {
    let mut r = AirReportVars {
        mean_air_temp: ztav,
        mean_air_hum_rat: air_hum_rat_avg,
        operative_temp: 0.5 * (ztav + mrt),
        ..AirReportVars::default()
    };
    r.mean_air_dew_point_temp = PsyTdpFnWPb(r.mean_air_hum_rat, out_baro_press);
    if let Some(this_mrt_fraction) = op_frac {
        r.operative_temp = 0.5 * (ztav + mrt);
        r.therm_operative_temp = (1.0 - this_mrt_fraction) * ztav + this_mrt_fraction * mrt;
    }
    if report_wbgt {
        r.wetbulb_globe_temp =
            0.7 * PsyTwbFnTdbWPb(ztav, air_hum_rat_avg, out_baro_press) + 0.3 * r.operative_temp;
    }
    r
}

/// A 0 or 1 flag as a bool, `None` for anything else.
fn flag(v: i32) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// Port of `double MeanAirTemps(int which, double ztav, double w, double mrt, double outbaro, int
/// anyop, int controlled, int opctrl, double frac, int wbgt)` (`meanair.c:8`): see the oracle's
/// shim for the arguments and what `which` picks.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn MeanAirTemps(
    which: std::ffi::c_int,
    ztav: f64,
    w: f64,
    mrt: f64,
    outbaro: f64,
    anyop: std::ffi::c_int,
    controlled: std::ffi::c_int,
    opctrl: std::ffi::c_int,
    frac: f64,
    wbgt: std::ffi::c_int,
) -> f64 {
    let checked = (|| {
        let flags = (flag(anyop)?, flag(controlled)?, flag(wbgt)?);
        ((0..=5).contains(&which) && (0..=2).contains(&opctrl)).then_some(flags)
    })();
    let Some((anyop, controlled, wbgt)) = checked else {
        return f64::NAN;
    };
    // TempCtrl::None is 0; the constant and scheduled fractions hold the same value.
    let op_frac = (anyop && controlled && opctrl != 0).then_some(frac);
    let r = calc_mean_air_temps(ztav, w, mrt, outbaro, op_frac, wbgt);
    match which {
        0 => r.mean_air_temp,
        1 => r.mean_air_hum_rat,
        2 => r.operative_temp,
        3 => r.mean_air_dew_point_temp,
        4 => r.therm_operative_temp,
        _ => r.wetbulb_globe_temp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operative_and_dew_point() {
        assert_eq!(
            MeanAirTemps(2, 22.0, 0.008, 26.0, 101325.0, 0, 0, 0, 0.5, 0),
            24.0
        );
        assert_eq!(
            MeanAirTemps(3, 22.0, 0.008, 26.0, 101325.0, 0, 0, 0, 0.5, 0),
            PsyTdpFnWPb(0.008, 101325.0)
        );
        assert_eq!(
            MeanAirTemps(4, 22.0, 0.008, 26.0, 101325.0, 0, 0, 0, 0.5, 0),
            0.0
        );
    }

    #[test]
    fn thermostat_operative_temperature() {
        assert_eq!(
            MeanAirTemps(4, 22.0, 0.008, 26.0, 101325.0, 1, 1, 1, 0.3, 0),
            0.7 * 22.0 + 0.3 * 26.0
        );
        assert_eq!(
            MeanAirTemps(4, 22.0, 0.008, 26.0, 101325.0, 1, 0, 1, 0.3, 0),
            0.0
        );
    }

    #[test]
    fn wet_bulb_globe() {
        let t = MeanAirTemps(5, 30.0, 0.012, 32.0, 101325.0, 0, 0, 0, 0.5, 1);
        assert_eq!(t, 0.7 * PsyTwbFnTdbWPb(30.0, 0.012, 101325.0) + 0.3 * 31.0);
        assert_eq!(
            MeanAirTemps(5, 30.0, 0.012, 32.0, 101325.0, 0, 0, 0, 0.5, 0),
            0.0
        );
    }

    #[test]
    fn guards_return_nan() {
        assert!(MeanAirTemps(6, 22.0, 0.008, 26.0, 101325.0, 0, 0, 0, 0.5, 0).is_nan());
        assert!(MeanAirTemps(0, 22.0, 0.008, 26.0, 101325.0, 0, 0, 3, 0.5, 0).is_nan());
    }
}
