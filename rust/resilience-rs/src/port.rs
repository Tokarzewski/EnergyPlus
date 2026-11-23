//! The Rust port of `resilience.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatBalanceSurfaceManager::CalcThermalResilience` for one zone: the heat index by the
//! simplified method (the NWS regression in Fahrenheit with its dry and humid adjustments) or the
//! extended method (`heatindex-rs`), and the humidex from the dew point. The psychrometric
//! functions come from `psychrometrics-rs`. Every expression keeps the C++ order. The oracle's
//! guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use heatindex_rs::port::heatindex;
use psychrometrics_rs::port::{PsyRhFnTdbWPb, PsyTdpFnWPb};

/// `Constant::Kelvin`.
const KELVIN: f64 = 273.15;

/// `DataHeatBalance::HeatIndexMethod::Simplified`.
const SIMPLIFIED: i32 = 0;

/// The simplified heat index (C) at zone air temperature `zone_t` (C) and relative humidity
/// `zone_rh` (%).
fn simplified_heat_index(zone_t: f64, zone_rh: f64) -> f64 {
    const C1: f64 = -42.379;
    const C2: f64 = 2.04901523;
    const C3: f64 = 10.14333127;
    const C4: f64 = -0.22475541;
    const C5: f64 = -0.00683783;
    const C6: f64 = -0.05481717;
    const C7: f64 = 0.00122874;
    const C8: f64 = 0.00085282;
    const C9: f64 = -0.00000199;
    let tf = zone_t * (9.0 / 5.0) + 32.0;
    let rh = zone_rh;
    let mut hi;
    if tf < 80.0 {
        hi = 0.5 * (tf + 61.0 + (tf - 68.0) * 1.2 + (rh * 0.094));
    } else {
        hi = C1
            + C2 * tf
            + C3 * rh
            + C4 * tf * rh
            + C5 * tf * tf
            + C6 * rh * rh
            + C7 * tf * tf * rh
            + C8 * tf * rh * rh
            + C9 * tf * tf * rh * rh;
        if rh < 13.0 && tf < 112.0 {
            hi -= (13.0 - rh) / 4.0 * ((17.0 - (tf - 95.0).abs()) / 17.0).sqrt();
        } else if rh > 85.0 && tf < 87.0 {
            hi += (rh - 85.0) / 10.0 * (87.0 - tf) / 5.0;
        }
    }
    (hi - 32.0) * (5.0 / 9.0)
}

/// One result of `CalcThermalResilience` for one zone (see the oracle's shim for the arguments and
/// `which`).
pub fn ThermalResilience(
    which: i32,
    method: i32,
    ztav: f64,
    airhumratavg: f64,
    outbaro: f64,
) -> f64 {
    if !(0..=1).contains(&which)
        || !(0..=1).contains(&method)
        || !(10000.0..=200000.0).contains(&outbaro)
    {
        return f64::NAN;
    }
    if which == 0 {
        let zone_rh = PsyRhFnTdbWPb(ztav, airhumratavg, outbaro) * 100.0;
        if method == SIMPLIFIED {
            simplified_heat_index(ztav, zone_rh)
        } else {
            heatindex(ztav + KELVIN, zone_rh / 100.0) - KELVIN
        }
    } else {
        let t_dew_point_k = PsyTdpFnWPb(airhumratavg, outbaro) + KELVIN;
        let e = 6.11 * (5417.7530 * ((1.0 / 273.16) - (1.0 / t_dew_point_k))).exp();
        let h = 5.0 / 9.0 * (e - 10.0);
        ztav + h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mild_zone_heat_index_is_near_the_air_temperature() {
        let hi = ThermalResilience(0, 0, 22.0, 0.008, 101325.0);
        assert!((hi - 22.0).abs() < 1.5, "{hi}");
    }

    #[test]
    fn hot_humid_zone_feels_hotter() {
        assert!(ThermalResilience(0, 0, 35.0, 0.02, 101325.0) > 40.0);
        assert!(ThermalResilience(1, 0, 35.0, 0.02, 101325.0) > 45.0);
    }

    #[test]
    fn guards_return_nan() {
        assert!(ThermalResilience(2, 0, 22.0, 0.008, 101325.0).is_nan());
        assert!(ThermalResilience(0, 2, 22.0, 0.008, 101325.0).is_nan());
        assert!(ThermalResilience(0, 0, 22.0, 0.008, 5000.0).is_nan());
    }
}
