//! C ABI for `halfloop.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double HalfLoop(int which, int side, int newstep, double tin, double mdot, double pump, double mass, double last, double prev, double tssec)` (`halfloop.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn HalfLoop(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    newstep: std::ffi::c_int,
    tin: f64,
    mdot: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    tssec: f64,
) -> f64 {
    port::HalfLoop(
        which, side, newstep, tin, mdot, pump, mass, last, prev, tssec,
    )
}

/// `HalfLoop` as `rust_anything_HalfLoop`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_HalfLoop(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    newstep: std::ffi::c_int,
    tin: f64,
    mdot: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    tssec: f64,
) -> f64 {
    port::HalfLoop(
        which, side, newstep, tin, mdot, pump, mass, last, prev, tssec,
    )
}
