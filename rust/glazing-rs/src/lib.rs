//! C ABI for `glazing.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double GlassPhi(int which, double cs, double tf0, double rf0, double rb0, int simple, double shgc, double u)` (`glazing.c:12`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GlassPhi(
    which: std::ffi::c_int,
    cs: f64,
    tf0: f64,
    rf0: f64,
    rb0: f64,
    simple: std::ffi::c_int,
    shgc: f64,
    u: f64,
) -> f64 {
    port::GlassPhi(which, cs, tf0, rf0, rb0, simple, shgc, u)
}

/// `GlassPhi` as `rust_anything_GlassPhi`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GlassPhi(
    which: std::ffi::c_int,
    cs: f64,
    tf0: f64,
    rf0: f64,
    rb0: f64,
    simple: std::ffi::c_int,
    shgc: f64,
    u: f64,
) -> f64 {
    port::GlassPhi(which, cs, tf0, rf0, rb0, simple, shgc, u)
}

/// `double GlassFit(int which, double tf0, double rf0, double rb0)` (`glazing.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GlassFit(which: std::ffi::c_int, tf0: f64, rf0: f64, rb0: f64) -> f64 {
    port::GlassFit(which, tf0, rf0, rb0)
}

/// `GlassFit` as `rust_anything_GlassFit`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GlassFit(
    which: std::ffi::c_int,
    tf0: f64,
    rf0: f64,
    rb0: f64,
) -> f64 {
    port::GlassFit(which, tf0, rf0, rb0)
}

/// `double SlatViewFac(int i, int j, double s, double h, double phib, double phis)` (`glazing.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlatViewFac(
    i: std::ffi::c_int,
    j: std::ffi::c_int,
    s: f64,
    h: f64,
    phib: f64,
    phis: f64,
) -> f64 {
    port::SlatViewFac(i, j, s, h, phib, phis)
}

/// `SlatViewFac` as `rust_anything_SlatViewFac`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlatViewFac(
    i: std::ffi::c_int,
    j: std::ffi::c_int,
    s: f64,
    h: f64,
    phib: f64,
    phis: f64,
) -> f64 {
    port::SlatViewFac(i, j, s, h, phib, phis)
}

/// `double MatInverse(int which, int n, double d, double k, double o, double p)` (`glazing.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn MatInverse(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    d: f64,
    k: f64,
    o: f64,
    p: f64,
) -> f64 {
    port::MatInverse(which, n, d, k, o, p)
}

/// `MatInverse` as `rust_anything_MatInverse`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_MatInverse(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    d: f64,
    k: f64,
    o: f64,
    p: f64,
) -> f64 {
    port::MatInverse(which, n, d, k, o, p)
}

/// `double Interp(double xin, int npts, double x0, double x1, double x2, double x3, double x4, double y0, double y1, double y2, double y3, double y4)` (`glazing.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Interp(
    xin: f64,
    npts: std::ffi::c_int,
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    x4: f64,
    y0: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    y4: f64,
) -> f64 {
    port::Interp(xin, npts, x0, x1, x2, x3, x4, y0, y1, y2, y3, y4)
}

/// `Interp` as `rust_anything_Interp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Interp(
    xin: f64,
    npts: std::ffi::c_int,
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    x4: f64,
    y0: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    y4: f64,
) -> f64 {
    port::Interp(xin, npts, x0, x1, x2, x3, x4, y0, y1, y2, y3, y4)
}
