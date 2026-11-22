//! C ABI for `intrad.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double IRView(int fixed, int i, int j, int mask, double L, double W, double H, double mass, double rot)` (`intrad.c:10`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IRView(
    fixed: std::ffi::c_int,
    i: std::ffi::c_int,
    j: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
) -> f64 {
    port::IRView(fixed, i, j, mask, L, W, H, mass, rot)
}

/// `IRView` as `rust_anything_IRView`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IRView(
    fixed: std::ffi::c_int,
    i: std::ffi::c_int,
    j: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
) -> f64 {
    port::IRView(fixed, i, j, mask, L, W, H, mass, rot)
}

/// `double IRScriptF(int i, int j, int mask, double L, double W, double H, double mass, double rot, double e0, double de)` (`intrad.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IRScriptF(
    i: std::ffi::c_int,
    j: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
    e0: f64,
    de: f64,
) -> f64 {
    port::IRScriptF(i, j, mask, L, W, H, mass, rot, e0, de)
}

/// `IRScriptF` as `rust_anything_IRScriptF`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IRScriptF(
    i: std::ffi::c_int,
    j: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
    e0: f64,
    de: f64,
) -> f64 {
    port::IRScriptF(i, j, mask, L, W, H, mass, rot, e0, de)
}

/// `double IRFmrt(int fp, int i, int mask, double L, double W, double H, double mass, double e0, double de)` (`intrad.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IRFmrt(
    fp: std::ffi::c_int,
    i: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    e0: f64,
    de: f64,
) -> f64 {
    port::IRFmrt(fp, i, mask, L, W, H, mass, e0, de)
}

/// `IRFmrt` as `rust_anything_IRFmrt`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IRFmrt(
    fp: std::ffi::c_int,
    i: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    e0: f64,
    de: f64,
) -> f64 {
    port::IRFmrt(fp, i, mask, L, W, H, mass, e0, de)
}
