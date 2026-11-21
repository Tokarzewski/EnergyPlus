//! C ABI for `fluidproperties.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double WaterSpecificHeat(double Temperature)` (`fluidproperties.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterSpecificHeat(Temperature: f64) -> f64 {
    port::WaterSpecificHeat(Temperature)
}

/// `WaterSpecificHeat` as `rust_anything_WaterSpecificHeat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterSpecificHeat(Temperature: f64) -> f64 {
    port::WaterSpecificHeat(Temperature)
}

/// `double WaterSpecificHeat_raw(double Temperature)` (`fluidproperties.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterSpecificHeat_raw(Temperature: f64) -> f64 {
    port::WaterSpecificHeat_raw(Temperature)
}

/// `WaterSpecificHeat_raw` as `rust_anything_WaterSpecificHeat_raw`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterSpecificHeat_raw(Temperature: f64) -> f64 {
    port::WaterSpecificHeat_raw(Temperature)
}

/// `double WaterDensity(double Temperature)` (`fluidproperties.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterDensity(Temperature: f64) -> f64 {
    port::WaterDensity(Temperature)
}

/// `WaterDensity` as `rust_anything_WaterDensity`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterDensity(Temperature: f64) -> f64 {
    port::WaterDensity(Temperature)
}

/// `double WaterConductivity(double Temperature)` (`fluidproperties.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterConductivity(Temperature: f64) -> f64 {
    port::WaterConductivity(Temperature)
}

/// `WaterConductivity` as `rust_anything_WaterConductivity`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterConductivity(Temperature: f64) -> f64 {
    port::WaterConductivity(Temperature)
}

/// `double WaterViscosity(double Temperature)` (`fluidproperties.c:37`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterViscosity(Temperature: f64) -> f64 {
    port::WaterViscosity(Temperature)
}

/// `WaterViscosity` as `rust_anything_WaterViscosity`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterViscosity(Temperature: f64) -> f64 {
    port::WaterViscosity(Temperature)
}

/// `double SteamSatPressure(double Temperature)` (`fluidproperties.c:42`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SteamSatPressure(Temperature: f64) -> f64 {
    port::SteamSatPressure(Temperature)
}

/// `SteamSatPressure` as `rust_anything_SteamSatPressure`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SteamSatPressure(Temperature: f64) -> f64 {
    port::SteamSatPressure(Temperature)
}

/// `double SteamSatTemperature(double Pressure)` (`fluidproperties.c:47`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SteamSatTemperature(Pressure: f64) -> f64 {
    port::SteamSatTemperature(Pressure)
}

/// `SteamSatTemperature` as `rust_anything_SteamSatTemperature`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SteamSatTemperature(Pressure: f64) -> f64 {
    port::SteamSatTemperature(Pressure)
}

/// `double SteamSatEnthalpy(double Temperature, double Quality)` (`fluidproperties.c:52`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SteamSatEnthalpy(Temperature: f64, Quality: f64) -> f64 {
    port::SteamSatEnthalpy(Temperature, Quality)
}

/// `SteamSatEnthalpy` as `rust_anything_SteamSatEnthalpy`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SteamSatEnthalpy(Temperature: f64, Quality: f64) -> f64 {
    port::SteamSatEnthalpy(Temperature, Quality)
}

/// `double SteamSatDensity(double Temperature, double Quality)` (`fluidproperties.c:57`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SteamSatDensity(Temperature: f64, Quality: f64) -> f64 {
    port::SteamSatDensity(Temperature, Quality)
}

/// `SteamSatDensity` as `rust_anything_SteamSatDensity`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SteamSatDensity(Temperature: f64, Quality: f64) -> f64 {
    port::SteamSatDensity(Temperature, Quality)
}

/// `double SteamSatSpecificHeat(double Temperature, double Quality)` (`fluidproperties.c:62`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SteamSatSpecificHeat(Temperature: f64, Quality: f64) -> f64 {
    port::SteamSatSpecificHeat(Temperature, Quality)
}

/// `SteamSatSpecificHeat` as `rust_anything_SteamSatSpecificHeat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SteamSatSpecificHeat(Temperature: f64, Quality: f64) -> f64 {
    port::SteamSatSpecificHeat(Temperature, Quality)
}
