//! C ABI for `vert.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double VertDistance(double xi, double yi, double zi, double xj, double yj, double zj, int which)` (`vert.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertDistance(
    xi: f64,
    yi: f64,
    zi: f64,
    xj: f64,
    yj: f64,
    zj: f64,
    which: std::ffi::c_int,
) -> f64 {
    port::VertDistance(xi, yi, zi, xj, yj, zj, which)
}

/// `VertDistance` as `rust_anything_VertDistance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertDistance(
    xi: f64,
    yi: f64,
    zi: f64,
    xj: f64,
    yj: f64,
    zj: f64,
    which: std::ffi::c_int,
) -> f64 {
    port::VertDistance(xi, yi, zi, xj, yj, zj, which)
}

/// `double VertResponse(double d0, double d1, double diffusivity, double currTime)` (`vert.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertResponse(d0: f64, d1: f64, diffusivity: f64, currTime: f64) -> f64 {
    port::VertResponse(d0, d1, diffusivity, currTime)
}

/// `VertResponse` as `rust_anything_VertResponse`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertResponse(
    d0: f64,
    d1: f64,
    diffusivity: f64,
    currTime: f64,
) -> f64 {
    port::VertResponse(d0, d1, diffusivity, currTime)
}

/// `double VertIntegral(double xi, double yi, double zi, double xj, double yj, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime)` (`vert.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertIntegral(
    xi: f64,
    yi: f64,
    zi: f64,
    xj: f64,
    yj: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    currTime: f64,
) -> f64 {
    port::VertIntegral(
        xi,
        yi,
        zi,
        xj,
        yj,
        bhLength,
        bhTopDepth,
        bhDiameter,
        diffusivity,
        currTime,
    )
}

/// `VertIntegral` as `rust_anything_VertIntegral`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertIntegral(
    xi: f64,
    yi: f64,
    zi: f64,
    xj: f64,
    yj: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    currTime: f64,
) -> f64 {
    port::VertIntegral(
        xi,
        yi,
        zi,
        xj,
        yj,
        bhLength,
        bhTopDepth,
        bhDiameter,
        diffusivity,
        currTime,
    )
}

/// `double VertDoubleIntegral(double xi, double yi, double xj, double yj, int same, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime)` (`vert.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertDoubleIntegral(
    xi: f64,
    yi: f64,
    xj: f64,
    yj: f64,
    same: std::ffi::c_int,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    currTime: f64,
) -> f64 {
    port::VertDoubleIntegral(
        xi,
        yi,
        xj,
        yj,
        same,
        bhLength,
        bhTopDepth,
        bhDiameter,
        diffusivity,
        currTime,
    )
}

/// `VertDoubleIntegral` as `rust_anything_VertDoubleIntegral`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertDoubleIntegral(
    xi: f64,
    yi: f64,
    xj: f64,
    yj: f64,
    same: std::ffi::c_int,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    currTime: f64,
) -> f64 {
    port::VertDoubleIntegral(
        xi,
        yi,
        xj,
        yj,
        same,
        bhLength,
        bhTopDepth,
        bhDiameter,
        diffusivity,
        currTime,
    )
}

/// `int VertNumLntts(double bhLength, double diffusivity, double maxSimYears)` (`vert.c:37`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertNumLntts(
    bhLength: f64,
    diffusivity: f64,
    maxSimYears: f64,
) -> std::ffi::c_int {
    port::VertNumLntts(bhLength, diffusivity, maxSimYears)
}

/// `VertNumLntts` as `rust_anything_VertNumLntts`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertNumLntts(
    bhLength: f64,
    diffusivity: f64,
    maxSimYears: f64,
) -> std::ffi::c_int {
    port::VertNumLntts(bhLength, diffusivity, maxSimYears)
}

/// `double VertLntts(double bhLength, double diffusivity, double maxSimYears, int k)` (`vert.c:42`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertLntts(
    bhLength: f64,
    diffusivity: f64,
    maxSimYears: f64,
    k: std::ffi::c_int,
) -> f64 {
    port::VertLntts(bhLength, diffusivity, maxSimYears, k)
}

/// `VertLntts` as `rust_anything_VertLntts`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertLntts(
    bhLength: f64,
    diffusivity: f64,
    maxSimYears: f64,
    k: std::ffi::c_int,
) -> f64 {
    port::VertLntts(bhLength, diffusivity, maxSimYears, k)
}

/// `double VertGFunction(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double maxSimYears, int k)` (`vert.c:47`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertGFunction(
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    maxSimYears: f64,
    k: std::ffi::c_int,
) -> f64 {
    port::VertGFunction(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        diffusivity,
        maxSimYears,
        k,
    )
}

/// `VertGFunction` as `rust_anything_VertGFunction`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertGFunction(
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    maxSimYears: f64,
    k: std::ffi::c_int,
) -> f64 {
    port::VertGFunction(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        diffusivity,
        maxSimYears,
        k,
    )
}

/// `double VertFrictionFactor(double reynoldsNum)` (`vert.c:52`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertFrictionFactor(reynoldsNum: f64) -> f64 {
    port::VertFrictionFactor(reynoldsNum)
}

/// `VertFrictionFactor` as `rust_anything_VertFrictionFactor`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertFrictionFactor(reynoldsNum: f64) -> f64 {
    port::VertFrictionFactor(reynoldsNum)
}

/// `double VertPipeConductionResistance(double outDia, double innerDia, double k)` (`vert.c:57`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertPipeConductionResistance(outDia: f64, innerDia: f64, k: f64) -> f64 {
    port::VertPipeConductionResistance(outDia, innerDia, k)
}

/// `VertPipeConductionResistance` as `rust_anything_VertPipeConductionResistance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertPipeConductionResistance(
    outDia: f64,
    innerDia: f64,
    k: f64,
) -> f64 {
    port::VertPipeConductionResistance(outDia, innerDia, k)
}

/// `double VertTimeSS(double bhLength, double diffusivity)` (`vert.c:62`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VertTimeSS(bhLength: f64, diffusivity: f64) -> f64 {
    port::VertTimeSS(bhLength, diffusivity)
}

/// `VertTimeSS` as `rust_anything_VertTimeSS`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VertTimeSS(bhLength: f64, diffusivity: f64) -> f64 {
    port::VertTimeSS(bhLength, diffusivity)
}
