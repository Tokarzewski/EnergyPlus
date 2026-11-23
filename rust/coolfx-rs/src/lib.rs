//! C ABI for `coolfx.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double CoolingEffect(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work)` (`coolfx.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CoolingEffect(
    which: std::ffi::c_int,
    outbaro: f64,
    at: f64,
    rt: f64,
    rh: f64,
    vel: f64,
    act: f64,
    clo: f64,
    work: f64,
) -> f64 {
    port::CoolingEffect(which, outbaro, at, rt, rh, vel, act, clo, work)
}

/// `CoolingEffect` as `rust_anything_CoolingEffect`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CoolingEffect(
    which: std::ffi::c_int,
    outbaro: f64,
    at: f64,
    rt: f64,
    rh: f64,
    vel: f64,
    act: f64,
    clo: f64,
    work: f64,
) -> f64 {
    port::CoolingEffect(which, outbaro, at, rt, rh, vel, act, clo, work)
}
