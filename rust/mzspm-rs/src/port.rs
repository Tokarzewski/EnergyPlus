//! The Rust port of `mzspm.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `SetPointManager`'s multizone managers over an air loop's cooled zones:
//! `SPMMultiZoneTemp` (the flow-weighted zone temperature plus the heating or cooling zones' load
//! over their flow times the supply specific heat, limited, at the limit with no load) and
//! `SPMMultiZoneHum` (the average zone humidity ratio plus the moisture load over the flow, or the
//! highest or lowest zone requirement, limited). Uses `psychrometrics-rs`. Every sum keeps the C++
//! order; `std::clamp` and ObjexxFCL `min` and `max` are written out as the C++ library does them.
//! The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::PsyCpAirFnW;
use std::ffi::c_int;

/// `HVAC::SmallMassFlow` (kg/s).
const SMALL_MASS_FLOW: f64 = 0.001;
/// `HVAC::SmallLoad` (W).
const SMALL_LOAD: f64 = 1.0;
/// The multizone humidity manager's small moisture load (kg/s).
const SMALL_MOISTURE_LOAD: f64 = 0.00001;

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

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The zones the shim's `nc` asks for, or `None` outside 1 to 4.
fn zones<T>(nc: c_int, zs: &[T; 4]) -> Option<&[T]> {
    let n = usize::try_from(nc).ok().filter(|n| (1..=4).contains(n))?;
    Some(&zs[..n])
}

/// Port of `double MZTemp(int heating, int nc, double mint, double maxt, ...)` (`mzspm.c:9`):
/// `SPMMultiZoneTemp::calculate`, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per zone field the shim sets"
)]
pub fn MZTemp(
    heating: c_int,
    nc: c_int,
    mint: f64,
    maxt: f64,
    m1: f64,
    load1: f64,
    zt1: f64,
    zw1: f64,
    iw1: f64,
    m2: f64,
    load2: f64,
    zt2: f64,
    zw2: f64,
    iw2: f64,
    m3: f64,
    load3: f64,
    zt3: f64,
    zw3: f64,
    iw3: f64,
    m4: f64,
    load4: f64,
    zt4: f64,
    zw4: f64,
    iw4: f64,
) -> f64 {
    let zs = [
        [m1, load1, zt1, zw1, iw1],
        [m2, load2, zt2, zw2, iw2],
        [m3, load3, zt3, zw3, iw3],
        [m4, load4, zt4, zw4, iw4],
    ];
    let bad = |z: &[f64; 5]| {
        !within(z[0], 0.0, 1000.0)
            || !within(z[1], -1.0e7, 1.0e7)
            || !within(z[2], -60.0, 80.0)
            || !within(z[3], 0.0, 0.1)
            || !within(z[4], 0.0, 0.1)
    };
    let Some(cooled) = zones(nc, &zs) else {
        return f64::NAN;
    };
    if (heating != 0 && heating != 1)
        || !within(mint, -60.0, 80.0)
        || !within(maxt, mint, 80.0)
        || zs.iter().any(bad)
    {
        return f64::NAN;
    }
    let heat = heating == 1;
    let mut sum_load = 0.0;
    let mut sum_mdot_cp = 0.0;
    let mut sum_mdot_cp_tot = 0.0;
    let mut sum_mdot_cp_t_zone_tot = 0.0;
    for &[m, load, zt, zw, iw] in cooled {
        let cp_air = PsyCpAirFnW(zw);
        sum_mdot_cp_tot += m * cp_air;
        sum_mdot_cp_t_zone_tot += m * cp_air * zt;
        if (heat && load > 0.0) || (!heat && load < 0.0) {
            let cp_air = PsyCpAirFnW(iw);
            sum_load += load;
            sum_mdot_cp += m * cp_air;
        }
    }
    let limit = if heat { mint } else { maxt };
    let zone_average_temp = if sum_mdot_cp_tot > 0.0 {
        sum_mdot_cp_t_zone_tot / sum_mdot_cp_tot
    } else {
        0.0
    };
    let set_point = if sum_mdot_cp > 0.0 {
        zone_average_temp + sum_load / sum_mdot_cp
    } else {
        limit
    };
    let set_point = clamp(set_point, mint, maxt);
    if sum_load.abs() < SMALL_LOAD {
        limit
    } else {
        set_point
    }
}

/// Port of `double MZHum(int kind, int nc, double minh, double maxh, ...)` (`mzspm.c:14`):
/// `SPMMultiZoneHum::calculate`, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per zone field the shim sets"
)]
pub fn MZHum(
    kind: c_int,
    nc: c_int,
    minh: f64,
    maxh: f64,
    m1: f64,
    hum1: f64,
    dehum1: f64,
    zw1: f64,
    m2: f64,
    hum2: f64,
    dehum2: f64,
    zw2: f64,
    m3: f64,
    hum3: f64,
    dehum3: f64,
    zw3: f64,
    m4: f64,
    hum4: f64,
    dehum4: f64,
    zw4: f64,
) -> f64 {
    let zs = [
        [m1, hum1, dehum1, zw1],
        [m2, hum2, dehum2, zw2],
        [m3, hum3, dehum3, zw3],
        [m4, hum4, dehum4, zw4],
    ];
    let bad = |z: &[f64; 4]| {
        !within(z[0], 0.0, 1000.0)
            || !within(z[1], -10.0, 10.0)
            || !within(z[2], -10.0, 10.0)
            || !within(z[3], 0.0, 0.1)
    };
    let Some(cooled) = zones(nc, &zs) else {
        return f64::NAN;
    };
    if !(0..=3).contains(&kind)
        || !within(minh, 0.0, 0.1)
        || !within(maxh, minh, 0.1)
        || zs.iter().any(bad)
    {
        return f64::NAN;
    }
    // kind: 0 minimum average, 1 maximum average, 2 minimum, 3 maximum.
    let minimum = kind == 0 || kind == 2;
    let average = kind <= 1;
    let mut sum_mdot = 0.0;
    let mut sum_mdot_tot = 0.0;
    let mut sum_moisture_load = 0.0;
    let mut sum_mdot_hum_tot = 0.0;
    let mut set_point = if minimum { minh } else { maxh };
    for &[m, hum, dehum, zone_hum] in cooled {
        let moisture_load = if minimum { hum } else { dehum };
        let wanted = if minimum {
            moisture_load > 0.0
        } else {
            moisture_load < 0.0
        };
        if average {
            sum_mdot_tot += m;
            sum_mdot_hum_tot += m * zone_hum;
            if wanted {
                sum_mdot += m;
                sum_moisture_load += moisture_load;
            }
        } else {
            let mut zone_set_point = if minimum { minh } else { maxh };
            if wanted {
                sum_moisture_load += moisture_load;
                if m > SMALL_MASS_FLOW {
                    zone_set_point = max(0.0, zone_hum + moisture_load / m);
                }
            }
            set_point = if minimum {
                max(set_point, zone_set_point)
            } else {
                min(set_point, zone_set_point)
            };
        }
    }
    if average {
        let average_zone_hum = if sum_mdot_tot > SMALL_MASS_FLOW {
            sum_mdot_hum_tot / sum_mdot_tot
        } else {
            0.0
        };
        if sum_mdot > SMALL_MASS_FLOW {
            set_point = max(0.0, average_zone_hum + sum_moisture_load / sum_mdot);
        }
    } else if sum_moisture_load.abs() < SMALL_MOISTURE_LOAD {
        set_point = if kind == 2 { minh } else { maxh };
    }
    clamp(set_point, minh, maxh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooling_average_lowers_the_supply_temperature() {
        let t = |maxt| {
            MZTemp(
                0, 2, 10.0, maxt, 1.0, -2000.0, 24.0, 0.008, 0.008, 1.0, 500.0, 22.0, 0.008, 0.008,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )
        };
        // The zones average 23 C and the cooling zone needs about 2 K lower supply.
        assert_eq!(t(20.0), 20.0);
        assert!(t(25.0) > 20.0 && t(25.0) < 22.0);
        let none = MZTemp(
            1, 1, 10.0, 40.0, 1.0, -0.5, 24.0, 0.008, 0.008, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        );
        assert_eq!(none, 10.0);
    }

    #[test]
    fn humidity_managers_pick_and_limit() {
        let z = |kind| {
            MZHum(
                kind, 2, 0.004, 0.012, 1.0, 0.001, -0.002, 0.008, 2.0, 0.0, 0.0, 0.009, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )
        };
        assert_eq!(z(2), 0.008 + 0.001);
        assert_eq!(z(3), 0.006);
        assert!(z(4).is_nan());
        assert!(
            MZHum(
                0, 0, 0.0, 0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0
            )
            .is_nan()
        );
    }
}
