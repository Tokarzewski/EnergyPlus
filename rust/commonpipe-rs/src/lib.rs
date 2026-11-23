//! C ABI for `commonpipe.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double CommonPipe(int which, int side, int newstep, double tin, double msec, double mpri, double pump, double mass, double last, double prev, double other, double tssec)` (`commonpipe.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CommonPipe(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    newstep: std::ffi::c_int,
    tin: f64,
    msec: f64,
    mpri: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    other: f64,
    tssec: f64,
) -> f64 {
    port::CommonPipe(
        which, side, newstep, tin, msec, mpri, pump, mass, last, prev, other, tssec,
    )
}

/// `CommonPipe` as `rust_anything_CommonPipe`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CommonPipe(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    newstep: std::ffi::c_int,
    tin: f64,
    msec: f64,
    mpri: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    other: f64,
    tssec: f64,
) -> f64 {
    port::CommonPipe(
        which, side, newstep, tin, msec, mpri, pump, mass, last, prev, other, tssec,
    )
}
