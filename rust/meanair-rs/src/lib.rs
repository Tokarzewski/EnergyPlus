//! C ABI for `meanair.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double MeanAirTemps(int which, double ztav, double w, double mrt, double outbaro, int anyop, int controlled, int opctrl, double frac, int wbgt)` (`meanair.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn MeanAirTemps(
    which: std::ffi::c_int,
    ztav: f64,
    w: f64,
    mrt: f64,
    outbaro: f64,
    anyop: std::ffi::c_int,
    controlled: std::ffi::c_int,
    opctrl: std::ffi::c_int,
    frac: f64,
    wbgt: std::ffi::c_int,
) -> f64 {
    port::MeanAirTemps(
        which, ztav, w, mrt, outbaro, anyop, controlled, opctrl, frac, wbgt,
    )
}

/// `MeanAirTemps` as `rust_anything_MeanAirTemps`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_MeanAirTemps(
    which: std::ffi::c_int,
    ztav: f64,
    w: f64,
    mrt: f64,
    outbaro: f64,
    anyop: std::ffi::c_int,
    controlled: std::ffi::c_int,
    opctrl: std::ffi::c_int,
    frac: f64,
    wbgt: std::ffi::c_int,
) -> f64 {
    port::MeanAirTemps(
        which, ztav, w, mrt, outbaro, anyop, controlled, opctrl, frac, wbgt,
    )
}
