//! C ABI for `plantnode.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double InitNodes(int which, double minm, double maxm)` (`plantnode.c:12`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn InitNodes(which: std::ffi::c_int, minm: f64, maxm: f64) -> f64 {
    port::InitNodes(which, minm, maxm)
}

/// `InitNodes` as `rust_anything_InitNodes`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_InitNodes(which: std::ffi::c_int, minm: f64, maxm: f64) -> f64 {
    port::InitNodes(which, minm, maxm)
}

/// `double SafeCopy(int which, int lp, double t, double m, double q, double h, double tmin, double tmax, double mmin, double mminav, double mmax, double mmaxav, double w, double p, double op)` (`plantnode.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SafeCopy(
    which: std::ffi::c_int,
    lp: std::ffi::c_int,
    t: f64,
    m: f64,
    q: f64,
    h: f64,
    tmin: f64,
    tmax: f64,
    mmin: f64,
    mminav: f64,
    mmax: f64,
    mmaxav: f64,
    w: f64,
    p: f64,
    op: f64,
) -> f64 {
    port::SafeCopy(
        which, lp, t, m, q, h, tmin, tmax, mmin, mminav, mmax, mmaxav, w, p, op,
    )
}

/// `SafeCopy` as `rust_anything_SafeCopy`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SafeCopy(
    which: std::ffi::c_int,
    lp: std::ffi::c_int,
    t: f64,
    m: f64,
    q: f64,
    h: f64,
    tmin: f64,
    tmax: f64,
    mmin: f64,
    mminav: f64,
    mmax: f64,
    mmaxav: f64,
    w: f64,
    p: f64,
    op: f64,
) -> f64 {
    port::SafeCopy(
        which, lp, t, m, q, h, tmin, tmax, mmin, mminav, mmax, mmaxav, w, p, op,
    )
}

/// `double BoundNode(double v, double minav, double maxav)` (`plantnode.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BoundNode(v: f64, minav: f64, maxav: f64) -> f64 {
    port::BoundNode(v, minav, maxav)
}

/// `BoundNode` as `rust_anything_BoundNode`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BoundNode(v: f64, minav: f64, maxav: f64) -> f64 {
    port::BoundNode(v, minav, maxav)
}

/// `double Tighten(int which, double oldmin, double oldmax, double newmin, double newmax)` (`plantnode.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Tighten(
    which: std::ffi::c_int,
    oldmin: f64,
    oldmax: f64,
    newmin: f64,
    newmax: f64,
) -> f64 {
    port::Tighten(which, oldmin, oldmax, newmin, newmax)
}

/// `Tighten` as `rust_anything_Tighten`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Tighten(
    which: std::ffi::c_int,
    oldmin: f64,
    oldmax: f64,
    newmin: f64,
    newmax: f64,
) -> f64 {
    port::Tighten(which, oldmin, oldmax, newmin, newmax)
}

/// `double BoundTwo(double v, double lo, double hi)` (`plantnode.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BoundTwo(v: f64, lo: f64, hi: f64) -> f64 {
    port::BoundTwo(v, lo, hi)
}

/// `BoundTwo` as `rust_anything_BoundTwo`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BoundTwo(v: f64, lo: f64, hi: f64) -> f64 {
    port::BoundTwo(v, lo, hi)
}
