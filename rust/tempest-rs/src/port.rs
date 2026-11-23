//! The Rust port of `tempest.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `SetPointManager::SPMTempest::calculate`: the warmest manager's supply temperature is the lowest
//! any cooled zone needs at its maximum inlet flow, the coldest manager's the highest any heated
//! zone needs (the cooled zones when the loop heats none), limited to the manager's range and held
//! at the limit when the zones' total load is below 1 W. Uses `psychrometrics-rs`. Every
//! expression keeps the C++ order, `min` and `max` are ObjexxFCL's and `clamp` the standard
//! library's. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::PsyCpAirFnW;

/// `HVAC::SmallMassFlow` (kg/s).
const SMALL_MASS_FLOW: f64 = 0.001;
/// `HVAC::SmallLoad` (W).
const SMALL_LOAD: f64 = 1.0;

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

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

/// One zone: its total load (W), temperature (C), and its inlet's maximum flow (kg/s) and humidity
/// ratio (kg/kg).
#[derive(Clone, Copy)]
struct Zone {
    load: f64,
    temp: f64,
    mass_flow_max: f64,
    hum_rat: f64,
}

impl Zone {
    /// The supply temperature that meets the zone's load at its maximum flow, or `default`.
    fn set_point(&self, default: f64) -> f64 {
        let cp_air = PsyCpAirFnW(self.hum_rat);
        if self.mass_flow_max > SMALL_MASS_FLOW {
            self.temp + self.load / (cp_air * self.mass_flow_max)
        } else {
            default
        }
    }
}

/// `SPMTempest::calculate` over the cooled zones and the heated zones.
fn calculate(warmest: bool, cooled: &[Zone], heated: &[Zone], min_t: f64, max_t: f64) -> f64 {
    if warmest {
        let mut tot_cool_load = 0.0;
        let mut set_point = max_t;
        for z in cooled {
            let mut zone_set_point = max_t;
            if z.load < 0.0 {
                tot_cool_load += z.load.abs();
                zone_set_point = z.set_point(zone_set_point);
            }
            set_point = min(set_point, zone_set_point);
        }
        set_point = clamp(set_point, min_t, max_t);
        if tot_cool_load < SMALL_LOAD {
            set_point = max_t;
        }
        set_point
    } else {
        let mut tot_heat_load = 0.0;
        let mut set_point = min_t;
        let zones = if heated.is_empty() { cooled } else { heated };
        for z in zones {
            let mut zone_set_point = min_t;
            if z.load > 0.0 {
                tot_heat_load += z.load;
                zone_set_point = z.set_point(zone_set_point);
            }
            set_point = max(set_point, zone_set_point);
        }
        set_point = clamp(set_point, min_t, max_t);
        if tot_heat_load < SMALL_LOAD {
            set_point = min_t;
        }
        set_point
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double Tempest(int warmest, int nc, int nh, double mint, double maxt, double load1, ...)`
/// (`tempest.c:9`), with each of zones 1 to 4's load, temperature, maximum inlet flow and inlet
/// humidity ratio: see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn Tempest(
    warmest: std::ffi::c_int,
    nc: std::ffi::c_int,
    nh: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    load1: f64,
    zt1: f64,
    mmax1: f64,
    w1: f64,
    load2: f64,
    zt2: f64,
    mmax2: f64,
    w2: f64,
    load3: f64,
    zt3: f64,
    mmax3: f64,
    w3: f64,
    load4: f64,
    zt4: f64,
    mmax4: f64,
    w4: f64,
) -> f64 {
    let zones = [
        [load1, zt1, mmax1, w1],
        [load2, zt2, mmax2, w2],
        [load3, zt3, mmax3, w3],
        [load4, zt4, mmax4, w4],
    ]
    .map(|[load, temp, mass_flow_max, hum_rat]| Zone {
        load,
        temp,
        mass_flow_max,
        hum_rat,
    });
    let zone_ok = |z: &Zone| {
        within(z.load, -1.0e7, 1.0e7)
            && within(z.temp, -60.0, 80.0)
            && within(z.mass_flow_max, 0.0, 1000.0)
            && within(z.hum_rat, 0.0, 0.1)
    };
    let (Ok(nc), Ok(nh)) = (usize::try_from(nc), usize::try_from(nh)) else {
        return f64::NAN;
    };
    let ok = (0..=1).contains(&warmest)
        && (1..=4).contains(&nc)
        && nh <= nc
        && within(mint, -60.0, 80.0)
        && within(maxt, mint, 80.0)
        && zones.iter().all(zone_ok);
    if !ok {
        return f64::NAN;
    }
    calculate(warmest == 1, &zones[..nc], &zones[..nh], mint, maxt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_neediest_zone_sets_the_supply_temperature() {
        let t = |warmest, l1, l2| {
            Tempest(
                warmest, 2, 0, 12.0, 18.0, l1, 24.0, 1.0, 0.008, l2, 24.0, 1.0, 0.008, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )
        };
        let one = t(1, -5000.0, -1000.0);
        assert!(one > 12.0 && one < 24.0);
        assert_eq!(t(1, 0.0, 0.5), 18.0);
        assert_eq!(t(0, 0.0, 0.0), 12.0);
        assert_eq!(t(0, 20000.0, 0.0), 18.0);
        assert!(
            Tempest(
                1, 5, 0, 12.0, 18.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0
            )
            .is_nan()
        );
    }
}
