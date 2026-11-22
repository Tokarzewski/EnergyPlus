//! C ABI for `wingas.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double GapCon(int which, double tleft, double tright, double width, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad)` (`wingas.c:10`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GapCon(
    which: std::ffi::c_int,
    tleft: f64,
    tright: f64,
    width: f64,
    nmix: std::ffi::c_int,
    g1: std::ffi::c_int,
    g2: std::ffi::c_int,
    g3: std::ffi::c_int,
    g4: std::ffi::c_int,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    quad: f64,
) -> f64 {
    port::GapCon(
        which, tleft, tright, width, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad,
    )
}

/// `GapCon` as `rust_anything_GapCon`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GapCon(
    which: std::ffi::c_int,
    tleft: f64,
    tright: f64,
    width: f64,
    nmix: std::ffi::c_int,
    g1: std::ffi::c_int,
    g2: std::ffi::c_int,
    g3: std::ffi::c_int,
    g4: std::ffi::c_int,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    quad: f64,
) -> f64 {
    port::GapCon(
        which, tleft, tright, width, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad,
    )
}

/// `double GapProps(int which, double tmean, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad)` (`wingas.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GapProps(
    which: std::ffi::c_int,
    tmean: f64,
    nmix: std::ffi::c_int,
    g1: std::ffi::c_int,
    g2: std::ffi::c_int,
    g3: std::ffi::c_int,
    g4: std::ffi::c_int,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    quad: f64,
) -> f64 {
    port::GapProps(which, tmean, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad)
}

/// `GapProps` as `rust_anything_GapProps`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GapProps(
    which: std::ffi::c_int,
    tmean: f64,
    nmix: std::ffi::c_int,
    g1: std::ffi::c_int,
    g2: std::ffi::c_int,
    g3: std::ffi::c_int,
    g4: std::ffi::c_int,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    quad: f64,
) -> f64 {
    port::GapProps(which, tmean, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad)
}

/// `double GapNusselt(double tso, double tsi, double width, double height, double tilt, double gr, double pr)` (`wingas.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GapNusselt(
    tso: f64,
    tsi: f64,
    width: f64,
    height: f64,
    tilt: f64,
    gr: f64,
    pr: f64,
) -> f64 {
    port::GapNusselt(tso, tsi, width, height, tilt, gr, pr)
}

/// `GapNusselt` as `rust_anything_GapNusselt`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GapNusselt(
    tso: f64,
    tsi: f64,
    width: f64,
    height: f64,
    tilt: f64,
    gr: f64,
    pr: f64,
) -> f64 {
    port::GapNusselt(tso, tsi, width, height, tilt, gr, pr)
}
