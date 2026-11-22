//! C ABI for `shadowcast.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double ShadowCast(int which, int algo, int nvr, double rx1, double ry1, double rz1, double rx2, double ry2, double rz2, double rx3, double ry3, double rz3, double rx4, double ry4, double rz4, int nvc, double cx1, double cy1, double cz1, double cx2, double cy2, double cz2, double cx3, double cy3, double cz3, double cx4, double cy4, double cz4, double ox, double oy, double oz, double lxx, double lxy, double lxz, double lyx, double lyy, double lyz, double lzx, double lzy, double lzz, double sx, double sy, double sz, double sched, double area)` (`shadowcast.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ShadowCast(
    which: std::ffi::c_int,
    algo: std::ffi::c_int,
    nvr: std::ffi::c_int,
    rx1: f64,
    ry1: f64,
    rz1: f64,
    rx2: f64,
    ry2: f64,
    rz2: f64,
    rx3: f64,
    ry3: f64,
    rz3: f64,
    rx4: f64,
    ry4: f64,
    rz4: f64,
    nvc: std::ffi::c_int,
    cx1: f64,
    cy1: f64,
    cz1: f64,
    cx2: f64,
    cy2: f64,
    cz2: f64,
    cx3: f64,
    cy3: f64,
    cz3: f64,
    cx4: f64,
    cy4: f64,
    cz4: f64,
    ox: f64,
    oy: f64,
    oz: f64,
    lxx: f64,
    lxy: f64,
    lxz: f64,
    lyx: f64,
    lyy: f64,
    lyz: f64,
    lzx: f64,
    lzy: f64,
    lzz: f64,
    sx: f64,
    sy: f64,
    sz: f64,
    sched: f64,
    area: f64,
) -> f64 {
    port::ShadowCast(
        which, algo, nvr, rx1, ry1, rz1, rx2, ry2, rz2, rx3, ry3, rz3, rx4, ry4, rz4, nvc, cx1,
        cy1, cz1, cx2, cy2, cz2, cx3, cy3, cz3, cx4, cy4, cz4, ox, oy, oz, lxx, lxy, lxz, lyx, lyy,
        lyz, lzx, lzy, lzz, sx, sy, sz, sched, area,
    )
}

/// `ShadowCast` as `rust_anything_ShadowCast`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ShadowCast(
    which: std::ffi::c_int,
    algo: std::ffi::c_int,
    nvr: std::ffi::c_int,
    rx1: f64,
    ry1: f64,
    rz1: f64,
    rx2: f64,
    ry2: f64,
    rz2: f64,
    rx3: f64,
    ry3: f64,
    rz3: f64,
    rx4: f64,
    ry4: f64,
    rz4: f64,
    nvc: std::ffi::c_int,
    cx1: f64,
    cy1: f64,
    cz1: f64,
    cx2: f64,
    cy2: f64,
    cz2: f64,
    cx3: f64,
    cy3: f64,
    cz3: f64,
    cx4: f64,
    cy4: f64,
    cz4: f64,
    ox: f64,
    oy: f64,
    oz: f64,
    lxx: f64,
    lxy: f64,
    lxz: f64,
    lyx: f64,
    lyy: f64,
    lyz: f64,
    lzx: f64,
    lzy: f64,
    lzz: f64,
    sx: f64,
    sy: f64,
    sz: f64,
    sched: f64,
    area: f64,
) -> f64 {
    port::ShadowCast(
        which, algo, nvr, rx1, ry1, rz1, rx2, ry2, rz2, rx3, ry3, rz3, rx4, ry4, rz4, nvc, cx1,
        cy1, cz1, cx2, cy2, cz2, cx3, cy3, cz3, cx4, cy4, cz4, ox, oy, oz, lxx, lxy, lxz, lyx, lyy,
        lyz, lzx, lzy, lzz, sx, sy, sz, sched, area,
    )
}
