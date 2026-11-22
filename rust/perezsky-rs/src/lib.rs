//! C ABI for `perezsky.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double PerezSky(int which, int detailed, double sx, double sy, double sz, double elev, double beam, double dif, double nx, double ny, double nz, double vfsky, double tilt, double iso, double sunlit, double horiz)` (`perezsky.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PerezSky(
    which: std::ffi::c_int,
    detailed: std::ffi::c_int,
    sx: f64,
    sy: f64,
    sz: f64,
    elev: f64,
    beam: f64,
    dif: f64,
    nx: f64,
    ny: f64,
    nz: f64,
    vfsky: f64,
    tilt: f64,
    iso: f64,
    sunlit: f64,
    horiz: f64,
) -> f64 {
    port::PerezSky(
        which, detailed, sx, sy, sz, elev, beam, dif, nx, ny, nz, vfsky, tilt, iso, sunlit, horiz,
    )
}

/// `PerezSky` as `rust_anything_PerezSky`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PerezSky(
    which: std::ffi::c_int,
    detailed: std::ffi::c_int,
    sx: f64,
    sy: f64,
    sz: f64,
    elev: f64,
    beam: f64,
    dif: f64,
    nx: f64,
    ny: f64,
    nz: f64,
    vfsky: f64,
    tilt: f64,
    iso: f64,
    sunlit: f64,
    horiz: f64,
) -> f64 {
    port::PerezSky(
        which, detailed, sx, sy, sz, elev, beam, dif, nx, ny, nz, vfsky, tilt, iso, sunlit, horiz,
    )
}

/// `double ClipLine(int which, double x0, double x1, double y0, double y1, double maxX, double minX, double maxY, double minY)` (`perezsky.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ClipLine(
    which: std::ffi::c_int,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    maxX: f64,
    minX: f64,
    maxY: f64,
    minY: f64,
) -> f64 {
    port::ClipLine(which, x0, x1, y0, y1, maxX, minX, maxY, minY)
}

/// `ClipLine` as `rust_anything_ClipLine`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ClipLine(
    which: std::ffi::c_int,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    maxX: f64,
    minX: f64,
    maxY: f64,
    minY: f64,
) -> f64 {
    port::ClipLine(which, x0, x1, y0, y1, maxX, minX, maxY, minY)
}
