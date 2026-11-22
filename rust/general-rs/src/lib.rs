//! C ABI for `general.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `int OrdinalDay(int Month, int Day, int LeapYearValue)` (`general.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn OrdinalDay(
    Month: std::ffi::c_int,
    Day: std::ffi::c_int,
    LeapYearValue: std::ffi::c_int,
) -> std::ffi::c_int {
    port::OrdinalDay(Month, Day, LeapYearValue)
}

/// `OrdinalDay` as `rust_anything_OrdinalDay`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_OrdinalDay(
    Month: std::ffi::c_int,
    Day: std::ffi::c_int,
    LeapYearValue: std::ffi::c_int,
) -> std::ffi::c_int {
    port::OrdinalDay(Month, Day, LeapYearValue)
}

/// `bool BetweenDateHoursLeftInclusive(int TestDate, int TestHour, int StartDate, int StartHour, int EndDate, int EndHour)` (`general.c:19`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BetweenDateHoursLeftInclusive(
    TestDate: std::ffi::c_int,
    TestHour: std::ffi::c_int,
    StartDate: std::ffi::c_int,
    StartHour: std::ffi::c_int,
    EndDate: std::ffi::c_int,
    EndHour: std::ffi::c_int,
) -> bool {
    port::BetweenDateHoursLeftInclusive(TestDate, TestHour, StartDate, StartHour, EndDate, EndHour)
}

/// `BetweenDateHoursLeftInclusive` as `rust_anything_BetweenDateHoursLeftInclusive`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BetweenDateHoursLeftInclusive(
    TestDate: std::ffi::c_int,
    TestHour: std::ffi::c_int,
    StartDate: std::ffi::c_int,
    StartHour: std::ffi::c_int,
    EndDate: std::ffi::c_int,
    EndHour: std::ffi::c_int,
) -> bool {
    port::BetweenDateHoursLeftInclusive(TestDate, TestHour, StartDate, StartHour, EndDate, EndHour)
}

/// `bool BetweenDates(int TestDate, int StartDate, int EndDate)` (`general.c:24`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BetweenDates(
    TestDate: std::ffi::c_int,
    StartDate: std::ffi::c_int,
    EndDate: std::ffi::c_int,
) -> bool {
    port::BetweenDates(TestDate, StartDate, EndDate)
}

/// `BetweenDates` as `rust_anything_BetweenDates`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BetweenDates(
    TestDate: std::ffi::c_int,
    StartDate: std::ffi::c_int,
    EndDate: std::ffi::c_int,
) -> bool {
    port::BetweenDates(TestDate, StartDate, EndDate)
}

/// `double SafeDivide(double a, double b)` (`general.c:29`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SafeDivide(a: f64, b: f64) -> f64 {
    port::SafeDivide(a, b)
}

/// `SafeDivide` as `rust_anything_SafeDivide`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SafeDivide(a: f64, b: f64) -> f64 {
    port::SafeDivide(a, b)
}

/// `double rotAzmDiffDeg(double AzmA, double AzmB)` (`general.c:34`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn rotAzmDiffDeg(AzmA: f64, AzmB: f64) -> f64 {
    port::rotAzmDiffDeg(AzmA, AzmB)
}

/// `rotAzmDiffDeg` as `rust_anything_rotAzmDiffDeg`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_rotAzmDiffDeg(AzmA: f64, AzmB: f64) -> f64 {
    port::rotAzmDiffDeg(AzmA, AzmB)
}

/// `double epexp(double numerator, double denominator)` (`general.c:39`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn epexp(numerator: f64, denominator: f64) -> f64 {
    port::epexp(numerator, denominator)
}

/// `epexp` as `rust_anything_epexp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_epexp(numerator: f64, denominator: f64) -> f64 {
    port::epexp(numerator, denominator)
}

/// `double Interp(double Lower, double Upper, double InterpFac)` (`general.c:44`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Interp(Lower: f64, Upper: f64, InterpFac: f64) -> f64 {
    port::Interp(Lower, Upper, InterpFac)
}

/// `Interp` as `rust_anything_Interp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Interp(Lower: f64, Upper: f64, InterpFac: f64) -> f64 {
    port::Interp(Lower, Upper, InterpFac)
}
