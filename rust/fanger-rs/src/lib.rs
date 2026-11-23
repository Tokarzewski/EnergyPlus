//! C ABI for `fanger.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double FangerPMV(int which, double at, double rt, double rh, double vel, double act, double clo, double work)` (`fanger.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FangerPMV(
    which: std::ffi::c_int,
    at: f64,
    rt: f64,
    rh: f64,
    vel: f64,
    act: f64,
    clo: f64,
    work: f64,
) -> f64 {
    port::FangerPMV(which, at, rt, rh, vel, act, clo, work)
}

/// `FangerPMV` as `rust_anything_FangerPMV`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FangerPMV(
    which: std::ffi::c_int,
    at: f64,
    rt: f64,
    rh: f64,
    vel: f64,
    act: f64,
    clo: f64,
    work: f64,
) -> f64 {
    port::FangerPMV(which, at, rt, rh, vel, act, clo, work)
}

/// `double StandardEffectiveTemp(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work)` (`fanger.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn StandardEffectiveTemp(
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
    port::StandardEffectiveTemp(which, outbaro, at, rt, rh, vel, act, clo, work)
}

/// `StandardEffectiveTemp` as `rust_anything_StandardEffectiveTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_StandardEffectiveTemp(
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
    port::StandardEffectiveTemp(which, outbaro, at, rt, rh, vel, act, clo, work)
}
