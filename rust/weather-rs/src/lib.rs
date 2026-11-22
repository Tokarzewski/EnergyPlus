//! C ABI for `weather.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double calculateWaterBoundaryConvectionCoefficient(double curWaterTemp, double curWaterVelocity, double distanceFromLeadingEdge)` (`weather.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn calculateWaterBoundaryConvectionCoefficient(
    curWaterTemp: f64,
    curWaterVelocity: f64,
    distanceFromLeadingEdge: f64,
) -> f64 {
    port::calculateWaterBoundaryConvectionCoefficient(
        curWaterTemp,
        curWaterVelocity,
        distanceFromLeadingEdge,
    )
}

/// `calculateWaterBoundaryConvectionCoefficient` as `rust_anything_calculateWaterBoundaryConvectionCoefficient`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_calculateWaterBoundaryConvectionCoefficient(
    curWaterTemp: f64,
    curWaterVelocity: f64,
    distanceFromLeadingEdge: f64,
) -> f64 {
    port::calculateWaterBoundaryConvectionCoefficient(
        curWaterTemp,
        curWaterVelocity,
        distanceFromLeadingEdge,
    )
}

/// `double interpolateWindDirection(double prevHrWindDir, double curHrWindDir, double curHrWeight)` (`weather.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn interpolateWindDirection(
    prevHrWindDir: f64,
    curHrWindDir: f64,
    curHrWeight: f64,
) -> f64 {
    port::interpolateWindDirection(prevHrWindDir, curHrWindDir, curHrWeight)
}

/// `interpolateWindDirection` as `rust_anything_interpolateWindDirection`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_interpolateWindDirection(
    prevHrWindDir: f64,
    curHrWindDir: f64,
    curHrWeight: f64,
) -> f64 {
    port::interpolateWindDirection(prevHrWindDir, curHrWindDir, curHrWeight)
}

/// `double AirMass(double CosZen)` (`weather.c:25`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn AirMass(CosZen: f64) -> f64 {
    port::AirMass(CosZen)
}

/// `AirMass` as `rust_anything_AirMass`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_AirMass(CosZen: f64) -> f64 {
    port::AirMass(CosZen)
}

/// `double GetSTM(double Longitude)` (`weather.c:30`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GetSTM(Longitude: f64) -> f64 {
    port::GetSTM(Longitude)
}

/// `GetSTM` as `rust_anything_GetSTM`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GetSTM(Longitude: f64) -> f64 {
    port::GetSTM(Longitude)
}

/// `bool isLeapYear(int Year)` (`weather.c:35`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn isLeapYear(Year: std::ffi::c_int) -> bool {
    port::isLeapYear(Year)
}

/// `isLeapYear` as `rust_anything_isLeapYear`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_isLeapYear(Year: std::ffi::c_int) -> bool {
    port::isLeapYear(Year)
}

/// `int computeJulianDate(int gyyyy, int gmm, int gdd)` (`weather.c:40`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn computeJulianDate(
    gyyyy: std::ffi::c_int,
    gmm: std::ffi::c_int,
    gdd: std::ffi::c_int,
) -> std::ffi::c_int {
    port::computeJulianDate(gyyyy, gmm, gdd)
}

/// `computeJulianDate` as `rust_anything_computeJulianDate`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_computeJulianDate(
    gyyyy: std::ffi::c_int,
    gmm: std::ffi::c_int,
    gdd: std::ffi::c_int,
) -> std::ffi::c_int {
    port::computeJulianDate(gyyyy, gmm, gdd)
}

/// `int calculateDayOfYear(int Month, int Day, bool leapYear)` (`weather.c:45`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn calculateDayOfYear(
    Month: std::ffi::c_int,
    Day: std::ffi::c_int,
    leapYear: bool,
) -> std::ffi::c_int {
    port::calculateDayOfYear(Month, Day, leapYear)
}

/// `calculateDayOfYear` as `rust_anything_calculateDayOfYear`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_calculateDayOfYear(
    Month: std::ffi::c_int,
    Day: std::ffi::c_int,
    leapYear: bool,
) -> std::ffi::c_int {
    port::calculateDayOfYear(Month, Day, leapYear)
}

/// `bool validMonthDay(int month, int day, int leapYearAdd)` (`weather.c:50`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn validMonthDay(
    month: std::ffi::c_int,
    day: std::ffi::c_int,
    leapYearAdd: std::ffi::c_int,
) -> bool {
    port::validMonthDay(month, day, leapYearAdd)
}

/// `validMonthDay` as `rust_anything_validMonthDay`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_validMonthDay(
    month: std::ffi::c_int,
    day: std::ffi::c_int,
    leapYearAdd: std::ffi::c_int,
) -> bool {
    port::validMonthDay(month, day, leapYearAdd)
}
