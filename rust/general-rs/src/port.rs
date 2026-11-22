//! The Rust port of `general.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Plain-number helpers from EnergyPlus `General.cc` and `General.hh` (namespace `General`, and
//! `Interp` in `EnergyPlus`): dates, safe division, angles and interpolation. Integer arithmetic
//! wraps where the C++ would overflow (undefined there, so any result is the C++'s); nothing
//! here panics.

#![forbid(unsafe_code)]

/// ObjexxFCL `sign(x, y)`: `|x|` with the sign of `y`, where `y >= 0` counts as positive (so a
/// NaN `y` gives `-|x|`).
fn sign(x: f64, y: f64) -> f64 {
    if y >= 0.0 { x.abs() } else { -x.abs() }
}

/// Port of `General::OrdinalDay` (`General.cc:706`): the day of the year of a month and day,
/// with `LeapYearValue` added from March; 0 for a month outside 1 to 12.
pub fn OrdinalDay(Month: i32, Day: i32, LeapYearValue: i32) -> i32 {
    const END_DAY_OF_MONTH: [i32; 12] = [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
    match Month {
        1 => Day,
        2 => Day.wrapping_add(END_DAY_OF_MONTH[0]),
        3..=12 => Day
            .wrapping_add(END_DAY_OF_MONTH[(Month - 2) as usize])
            .wrapping_add(LeapYearValue),
        _ => 0,
    }
}

/// Port of `General::BetweenDateHoursLeftInclusive` (`General.cc:782`): whether the day and hour
/// fall in the period, which may wrap past the end of the year.
pub fn BetweenDateHoursLeftInclusive(
    TestDate: i32,
    TestHour: i32,
    StartDate: i32,
    StartHour: i32,
    EndDate: i32,
    EndHour: i32,
) -> bool {
    let test = f64::from(TestDate) + f64::from(TestHour) / 24.0;
    let start = f64::from(StartDate) + f64::from(StartHour) / 24.0;
    let end = f64::from(EndDate) + f64::from(EndHour) / 24.0;
    if start <= end {
        (start..=end).contains(&test)
    } else {
        (end..=start).contains(&test)
    }
}

/// Port of `General::BetweenDates` (`General.cc:795`): whether the day falls in the period, which
/// may wrap past the end of the year.
pub fn BetweenDates(TestDate: i32, StartDate: i32, EndDate: i32) -> bool {
    if StartDate <= EndDate {
        (StartDate..=EndDate).contains(&TestDate)
    } else {
        TestDate <= EndDate || TestDate >= StartDate
    }
}

/// Port of `General::SafeDivide` (`General.cc:905`): `a / b`, with `b` kept at least 1e-10 away
/// from zero.
pub fn SafeDivide(a: f64, b: f64) -> f64 {
    const SMALL: f64 = 1e-10;
    if b.abs() >= SMALL {
        a / b
    } else {
        a / sign(SMALL, b)
    }
}

/// Port of `General::rotAzmDiffDeg` (`General.cc:1559`): the difference between two azimuths.
pub fn rotAzmDiffDeg(AzmA: f64, AzmB: f64) -> f64 {
    let mut diff = AzmB - AzmA;
    if diff > 180.0 {
        diff = 360.0 - diff;
    } else if diff < -180.0 {
        diff += 360.0; // `360 + diff` in the C++; IEEE addition commutes
    }
    diff.abs()
}

/// Port of `General::epexp` (`General.hh:253`): `exp(numerator / denominator)`, 0 for a zero
/// denominator.
pub fn epexp(numerator: f64, denominator: f64) -> f64 {
    if denominator == 0.0 {
        0.0
    } else {
        (numerator / denominator).exp()
    }
}

/// Port of `Interp` (`General.hh:262`): linear interpolation.
pub fn Interp(Lower: f64, Upper: f64, InterpFac: f64) -> f64 {
    Lower + InterpFac * (Upper - Lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        assert_eq!((OrdinalDay(3, 1, 1), OrdinalDay(13, 1, 0)), (61, 0));
        assert!(BetweenDates(1, 360, 5) && !BetweenDates(100, 360, 5));
        assert!(SafeDivide(1.0, 0.0) > 0.0 && SafeDivide(1.0, f64::NAN) < 0.0);
        assert_eq!((rotAzmDiffDeg(10.0, 350.0), epexp(1.0, 0.0)), (20.0, 0.0));
    }
}
