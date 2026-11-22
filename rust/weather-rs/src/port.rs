//! The Rust port of `weather.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Plain-number helpers from EnergyPlus `WeatherManager.cc` (namespace `Weather`): the water
//! boundary convection coefficient, wind direction interpolation, air mass, the standard time
//! meridian, and date checks. Integer arithmetic wraps where the C++ would overflow (undefined
//! there); nothing here panics.

#![forbid(unsafe_code)]

/// ObjexxFCL `max`, `a < b ? b : a`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// Port of `Weather::calculateWaterBoundaryConvectionCoefficient` (`WeatherManager.cc:225`):
/// forced convection along the hull, or free convection below it, whichever is larger [W/m2-K].
/// The C++ folds `pow` of its constants at compile time; `pow(6, 0.5)` is written as the square
/// root it is.
pub fn calculateWaterBoundaryConvectionCoefficient(
    curWaterTemp: f64,
    curWaterVelocity: f64,
    distanceFromLeadingEdge: f64,
) -> f64 {
    const KINEMATIC_VISCOSITY: f64 = 1e-6; // m2/s
    const PRANDTL: f64 = 6.0;
    const CONDUCTIVITY: f64 = 0.6; // W/m-K
    const DISTANCE_FROM_BOTTOM_OF_HULL: f64 = 12.0; // m
    let reynolds = curWaterVelocity * distanceFromLeadingEdge / KINEMATIC_VISCOSITY;
    let nusselt = 0.0296 * reynolds.powf(0.8) * PRANDTL.powf(1.0 / 3.0);
    let forced = nusselt * CONDUCTIVITY / distanceFromLeadingEdge;
    let sqrt_pr = PRANDTL.sqrt();
    let prandtl_correction =
        (0.75 * sqrt_pr) / (0.609 + 1.221 * sqrt_pr + 1.238 * PRANDTL).powf(0.25);
    let grashof =
        (9.81 * 0.000214 * (25.0 - curWaterTemp).abs() * DISTANCE_FROM_BOTTOM_OF_HULL.powf(3.0))
            / KINEMATIC_VISCOSITY.powf(2.0);
    let free = (grashof / 4.0).powf(0.25) * prandtl_correction * CONDUCTIVITY
        / DISTANCE_FROM_BOTTOM_OF_HULL;
    max(forced, free)
}

/// Port of `Weather::interpolateWindDirection` (`WeatherManager.cc:3171`): the direction a
/// fraction of the way from the previous hour's, turning the short way round.
pub fn interpolateWindDirection(prevHrWindDir: f64, curHrWindDir: f64, curHrWeight: f64) -> f64 {
    let (mut cur, mut prev) = (curHrWindDir, prevHrWindDir);
    if (cur - prev).abs() > 180. {
        if cur > prev {
            prev += 360.;
        } else {
            cur += 360.;
        }
    }
    (prev + (cur - prev) * curHrWeight) % 360. // `%` on f64 is C's fmod
}

/// Port of `Weather::AirMass` (`WeatherManager.cc:4025`): Kasten and Young's air mass.
pub fn AirMass(CosZen: f64) -> f64 {
    const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
    if CosZen <= 0.001 {
        37.07837343
    } else if CosZen >= 1.0 {
        1.0
    } else {
        let sun_alt_d = CosZen.asin() / DEG_TO_RAD;
        1.0 / (CosZen + 0.50572 * (6.07995 + sun_alt_d).powf(-1.6364))
    }
}

/// Port of `Weather::GetSTM` (`WeatherManager.cc:7429`): the time zone of the 15-degree band a
/// longitude falls in, 0 when it falls in none. The C++ builds the band edges by adding 15 to
/// -7.5 and 7.5, which is exact. It folds longitudes past 180 by subtracting 180, not 360.
pub fn GetSTM(Longitude: f64) -> f64 {
    let mut temp = Longitude % 360.0; // ObjexxFCL mod: fmod for a non-zero divisor
    if temp > 180.0 {
        temp -= 180.0;
    }
    (-12..=12)
        .find(|&i| temp > -7.5 + 15.0 * f64::from(i) && temp <= 7.5 + 15.0 * f64::from(i))
        .map_or(0.0, |i| f64::from(i) % 24.0)
}

/// Port of `Weather::isLeapYear` (`WeatherManager.cc:8507`).
pub fn isLeapYear(Year: i32) -> bool {
    Year % 4 == 0 && !(Year % 100 == 0 && Year % 400 != 0)
}

/// Port of `Weather::computeJulianDate` (`WeatherManager.cc:8519`): the Julian day number of a
/// Gregorian date, in the C++'s integer arithmetic.
pub fn computeJulianDate(gyyyy: i32, gmm: i32, gdd: i32) -> i32 {
    let l = gmm.wrapping_sub(14) / 12;
    let a = 1461i32.wrapping_mul(gyyyy.wrapping_add(4800).wrapping_add(l)) / 4;
    let b = 367i32.wrapping_mul(gmm.wrapping_sub(2).wrapping_sub(l.wrapping_mul(12))) / 12;
    let c = 3i32.wrapping_mul(gyyyy.wrapping_add(4900).wrapping_add(l) / 100) / 4;
    gdd.wrapping_sub(32075)
        .wrapping_add(a)
        .wrapping_add(b)
        .wrapping_sub(c)
}

/// Port of `Weather::calculateDayOfYear` (`WeatherManager.cc:8602`). A month outside 1 to 12
/// reads outside the C++'s table; the oracle and this port return 0 for it (ADR-009).
pub fn calculateDayOfYear(Month: i32, Day: i32, leapYear: bool) -> i32 {
    const BEFORE: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const BEFORE_LEAP: [i32; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    if !(1..=12).contains(&Month) {
        return 0;
    }
    let table = if leapYear { &BEFORE_LEAP } else { &BEFORE };
    table[(Month - 1) as usize].wrapping_add(Day)
}

/// Port of `Weather::validMonthDay` (`WeatherManager.cc:8622`): whether the day is not past the
/// end of the month (it does not check for days below 1).
pub fn validMonthDay(month: i32, day: i32, leapYearAdd: i32) -> bool {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => day <= 28i32.wrapping_add(leapYearAdd),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        assert_eq!((AirMass(0.0), AirMass(1.0)), (37.07837343, 1.0));
        assert_eq!(
            (GetSTM(0.0), GetSTM(-75.0), GetSTM(187.5)),
            (0.0, -5.0, 0.0)
        );
        assert!(isLeapYear(2000) && !isLeapYear(1900) && isLeapYear(2024));
        assert_eq!(computeJulianDate(2000, 1, 1), 2451545);
        assert_eq!(
            (
                calculateDayOfYear(3, 1, true),
                calculateDayOfYear(0, 1, true)
            ),
            (61, 0)
        );
        assert!(validMonthDay(2, 29, 1) && !validMonthDay(2, 29, 0) && !validMonthDay(13, 1, 0));
    }
}
