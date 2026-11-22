//! C ABI for `comfort.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double CalcFangerPPD(double PMV)` (`comfort.c:11`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcFangerPPD(PMV: f64) -> f64 {
    port::CalcFangerPPD(PMV)
}

/// `CalcFangerPPD` as `rust_anything_CalcFangerPPD`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcFangerPPD(PMV: f64) -> f64 {
    port::CalcFangerPPD(PMV)
}

/// `double CalcRelativeAirVelocity(double AirVel, double ActMet)` (`comfort.c:16`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcRelativeAirVelocity(AirVel: f64, ActMet: f64) -> f64 {
    port::CalcRelativeAirVelocity(AirVel, ActMet)
}

/// `CalcRelativeAirVelocity` as `rust_anything_CalcRelativeAirVelocity`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcRelativeAirVelocity(AirVel: f64, ActMet: f64) -> f64 {
    port::CalcRelativeAirVelocity(AirVel, ActMet)
}

/// `double CalcSatVapPressFromTemp(double Temp)` (`comfort.c:21`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcSatVapPressFromTemp(Temp: f64) -> f64 {
    port::CalcSatVapPressFromTemp(Temp)
}

/// `CalcSatVapPressFromTemp` as `rust_anything_CalcSatVapPressFromTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcSatVapPressFromTemp(Temp: f64) -> f64 {
    port::CalcSatVapPressFromTemp(Temp)
}

/// `double CalcSatVapPressFromTempTorr(double Temp)` (`comfort.c:26`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcSatVapPressFromTempTorr(Temp: f64) -> f64 {
    port::CalcSatVapPressFromTempTorr(Temp)
}

/// `CalcSatVapPressFromTempTorr` as `rust_anything_CalcSatVapPressFromTempTorr`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcSatVapPressFromTempTorr(Temp: f64) -> f64 {
    port::CalcSatVapPressFromTempTorr(Temp)
}
