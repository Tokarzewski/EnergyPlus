//! C ABI for `ksu.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double KsuTsv(int which, double outbaro, double ztav, double w, double mrt, double qrad, double act, double eff, double clo, double vel)` (`ksu.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn KsuTsv(
    which: std::ffi::c_int,
    outbaro: f64,
    ztav: f64,
    w: f64,
    mrt: f64,
    qrad: f64,
    act: f64,
    eff: f64,
    clo: f64,
    vel: f64,
) -> f64 {
    port::KsuTsv(which, outbaro, ztav, w, mrt, qrad, act, eff, clo, vel)
}

/// `KsuTsv` as `rust_anything_KsuTsv`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_KsuTsv(
    which: std::ffi::c_int,
    outbaro: f64,
    ztav: f64,
    w: f64,
    mrt: f64,
    qrad: f64,
    act: f64,
    eff: f64,
    clo: f64,
    vel: f64,
) -> f64 {
    port::KsuTsv(which, outbaro, ztav, w, mrt, qrad, act, eff, clo, vel)
}
