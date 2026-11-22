//! C ABI for `vectors.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double VAreaPolygon(int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)` (`vectors.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VAreaPolygon(
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VAreaPolygon(
        n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `VAreaPolygon` as `rust_anything_VAreaPolygon`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VAreaPolygon(
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VAreaPolygon(
        n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `double VNewellArea(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)` (`vectors.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VNewellArea(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VNewellArea(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `VNewellArea` as `rust_anything_VNewellArea`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VNewellArea(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VNewellArea(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `double VNewellNormal(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)` (`vectors.c:25`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VNewellNormal(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VNewellNormal(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `VNewellNormal` as `rust_anything_VNewellNormal`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VNewellNormal(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VNewellNormal(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `double VAzimuthTilt(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)` (`vectors.c:30`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VAzimuthTilt(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VAzimuthTilt(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `VAzimuthTilt` as `rust_anything_VAzimuthTilt`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VAzimuthTilt(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VAzimuthTilt(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `double VCoPlanar(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)` (`vectors.c:35`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VCoPlanar(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VCoPlanar(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `VCoPlanar` as `rust_anything_VCoPlanar`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VCoPlanar(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> f64 {
    port::VCoPlanar(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `int VPointsInPlane(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)` (`vectors.c:40`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VPointsInPlane(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> std::ffi::c_int {
    port::VPointsInPlane(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `VPointsInPlane` as `rust_anything_VPointsInPlane`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VPointsInPlane(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    x3: f64,
    y3: f64,
    z3: f64,
    x4: f64,
    y4: f64,
    z4: f64,
    x5: f64,
    y5: f64,
    z5: f64,
) -> std::ffi::c_int {
    port::VPointsInPlane(
        which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5,
    )
}

/// `double VRound(int which, double x, double y, double z, double roundto)` (`vectors.c:45`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VRound(which: std::ffi::c_int, x: f64, y: f64, z: f64, roundto: f64) -> f64 {
    port::VRound(which, x, y, z, roundto)
}

/// `VRound` as `rust_anything_VRound`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VRound(
    which: std::ffi::c_int,
    x: f64,
    y: f64,
    z: f64,
    roundto: f64,
) -> f64 {
    port::VRound(which, x, y, z, roundto)
}

/// `double VNormalize(int which, double x, double y, double z)` (`vectors.c:50`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VNormalize(which: std::ffi::c_int, x: f64, y: f64, z: f64) -> f64 {
    port::VNormalize(which, x, y, z)
}

/// `VNormalize` as `rust_anything_VNormalize`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VNormalize(which: std::ffi::c_int, x: f64, y: f64, z: f64) -> f64 {
    port::VNormalize(which, x, y, z)
}
