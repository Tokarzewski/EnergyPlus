//! C ABI for `twowaypipe.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double TwoWayPipe(int which, int side, int psp, int ssp, double tank, double other, double tpsp, double tssp, double tpin, double tsin, double msec, double mpri, double pts, double prc, double src)` (`twowaypipe.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn TwoWayPipe(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    psp: std::ffi::c_int,
    ssp: std::ffi::c_int,
    tank: f64,
    other: f64,
    tpsp: f64,
    tssp: f64,
    tpin: f64,
    tsin: f64,
    msec: f64,
    mpri: f64,
    pts: f64,
    prc: f64,
    src: f64,
) -> f64 {
    port::TwoWayPipe(
        which, side, psp, ssp, tank, other, tpsp, tssp, tpin, tsin, msec, mpri, pts, prc, src,
    )
}

/// `TwoWayPipe` as `rust_anything_TwoWayPipe`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_TwoWayPipe(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    psp: std::ffi::c_int,
    ssp: std::ffi::c_int,
    tank: f64,
    other: f64,
    tpsp: f64,
    tssp: f64,
    tpin: f64,
    tsin: f64,
    msec: f64,
    mpri: f64,
    pts: f64,
    prc: f64,
    src: f64,
) -> f64 {
    port::TwoWayPipe(
        which, side, psp, ssp, tank, other, tpsp, tssp, tpin, tsin, msec, mpri, pts, prc, src,
    )
}
