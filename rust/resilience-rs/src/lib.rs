//! C ABI for `resilience.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double ThermalResilience(int which, int method, double ztav, double airhumratavg, double outbaro)` (`resilience.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ThermalResilience(
    which: std::ffi::c_int,
    method: std::ffi::c_int,
    ztav: f64,
    airhumratavg: f64,
    outbaro: f64,
) -> f64 {
    port::ThermalResilience(which, method, ztav, airhumratavg, outbaro)
}

/// `ThermalResilience` as `rust_anything_ThermalResilience`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ThermalResilience(
    which: std::ffi::c_int,
    method: std::ffi::c_int,
    ztav: f64,
    airhumratavg: f64,
    outbaro: f64,
) -> f64 {
    port::ThermalResilience(which, method, ztav, airhumratavg, outbaro)
}
