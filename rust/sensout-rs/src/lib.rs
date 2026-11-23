//! C ABI for `sensout.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double CompOutput(int which, double m, double t2, double w2, double t1, double w1)` (`sensout.c:10`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CompOutput(
    which: std::ffi::c_int,
    m: f64,
    t2: f64,
    w2: f64,
    t1: f64,
    w1: f64,
) -> f64 {
    port::CompOutput(which, m, t2, w2, t1, w1)
}

/// `CompOutput` as `rust_anything_CompOutput`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CompOutput(
    which: std::ffi::c_int,
    m: f64,
    t2: f64,
    w2: f64,
    t1: f64,
    w1: f64,
) -> f64 {
    port::CompOutput(which, m, t2, w2, t1, w1)
}

/// `double ZoneOutput(int which, double m, double te, double we, double tz, double wz)` (`sensout.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ZoneOutput(
    which: std::ffi::c_int,
    m: f64,
    te: f64,
    we: f64,
    tz: f64,
    wz: f64,
) -> f64 {
    port::ZoneOutput(which, m, te, we, tz, wz)
}

/// `ZoneOutput` as `rust_anything_ZoneOutput`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ZoneOutput(
    which: std::ffi::c_int,
    m: f64,
    te: f64,
    we: f64,
    tz: f64,
    wz: f64,
) -> f64 {
    port::ZoneOutput(which, m, te, we, tz, wz)
}

/// `double ZoneSensOutput(double m, double te, double tz, double wz)` (`sensout.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ZoneSensOutput(m: f64, te: f64, tz: f64, wz: f64) -> f64 {
    port::ZoneSensOutput(m, te, tz, wz)
}

/// `ZoneSensOutput` as `rust_anything_ZoneSensOutput`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ZoneSensOutput(m: f64, te: f64, tz: f64, wz: f64) -> f64 {
    port::ZoneSensOutput(m, te, tz, wz)
}
