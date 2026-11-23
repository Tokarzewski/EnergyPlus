//! C ABI for `irexch.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double IRExchange(int which, int i, int mask, double L, double W, double H, double mass, double rot, double e0, double de, int carroll, int win, double t0, double dt, double twin, int iter)` (`irexch.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IRExchange(
    which: std::ffi::c_int,
    i: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
    e0: f64,
    de: f64,
    carroll: std::ffi::c_int,
    win: std::ffi::c_int,
    t0: f64,
    dt: f64,
    twin: f64,
    iter: std::ffi::c_int,
) -> f64 {
    port::IRExchange(
        which, i, mask, L, W, H, mass, rot, e0, de, carroll, win, t0, dt, twin, iter,
    )
}

/// `IRExchange` as `rust_anything_IRExchange`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IRExchange(
    which: std::ffi::c_int,
    i: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
    e0: f64,
    de: f64,
    carroll: std::ffi::c_int,
    win: std::ffi::c_int,
    t0: f64,
    dt: f64,
    twin: f64,
    iter: std::ffi::c_int,
) -> f64 {
    port::IRExchange(
        which, i, mask, L, W, H, mass, rot, e0, de, carroll, win, t0, dt, twin, iter,
    )
}
