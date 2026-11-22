//! C ABI for `slinky.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SlinkyDistToCenter(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, int m, int n, int m1, int n1)` (`slinky.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyDistToCenter(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
) -> f64 {
    port::SlinkyDistToCenter(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
    )
}

/// `SlinkyDistToCenter` as `rust_anything_SlinkyDistToCenter`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyDistToCenter(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
) -> f64 {
    port::SlinkyDistToCenter(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
    )
}

/// `double SlinkyDistance(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, int m, int n, int m1, int n1, double eta, double theta)` (`slinky.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyDistance(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    eta: f64,
    theta: f64,
) -> f64 {
    port::SlinkyDistance(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        eta,
        theta,
    )
}

/// `SlinkyDistance` as `rust_anything_SlinkyDistance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyDistance(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    eta: f64,
    theta: f64,
) -> f64 {
    port::SlinkyDistance(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        eta,
        theta,
    )
}

/// `double SlinkyDistanceToFictRing(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, int m, int n, int m1, int n1, double eta, double theta)` (`slinky.c:25`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyDistanceToFictRing(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    eta: f64,
    theta: f64,
) -> f64 {
    port::SlinkyDistanceToFictRing(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        eta,
        theta,
    )
}

/// `SlinkyDistanceToFictRing` as `rust_anything_SlinkyDistanceToFictRing`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyDistanceToFictRing(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    eta: f64,
    theta: f64,
) -> f64 {
    port::SlinkyDistanceToFictRing(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        eta,
        theta,
    )
}

/// `double SlinkyNearFieldResponse(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, int m, int n, int m1, int n1, double eta, double theta, double t)` (`slinky.c:30`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyNearFieldResponse(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    eta: f64,
    theta: f64,
    t: f64,
) -> f64 {
    port::SlinkyNearFieldResponse(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        eta,
        theta,
        t,
    )
}

/// `SlinkyNearFieldResponse` as `rust_anything_SlinkyNearFieldResponse`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyNearFieldResponse(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    eta: f64,
    theta: f64,
    t: f64,
) -> f64 {
    port::SlinkyNearFieldResponse(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        eta,
        theta,
        t,
    )
}

/// `double SlinkyMidFieldResponse(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, int m, int n, int m1, int n1, double t)` (`slinky.c:35`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyMidFieldResponse(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    t: f64,
) -> f64 {
    port::SlinkyMidFieldResponse(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        t,
    )
}

/// `SlinkyMidFieldResponse` as `rust_anything_SlinkyMidFieldResponse`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyMidFieldResponse(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    t: f64,
) -> f64 {
    port::SlinkyMidFieldResponse(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        t,
    )
}

/// `double SlinkyIntegral(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, int m, int n, int m1, int n1, double t, double eta, int J0)` (`slinky.c:40`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyIntegral(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    t: f64,
    eta: f64,
    J0: std::ffi::c_int,
) -> f64 {
    port::SlinkyIntegral(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        t,
        eta,
        J0,
    )
}

/// `SlinkyIntegral` as `rust_anything_SlinkyIntegral`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyIntegral(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    t: f64,
    eta: f64,
    J0: std::ffi::c_int,
) -> f64 {
    port::SlinkyIntegral(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        t,
        eta,
        J0,
    )
}

/// `double SlinkyDoubleIntegral(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, int m, int n, int m1, int n1, double t, int I0, int J0)` (`slinky.c:45`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyDoubleIntegral(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    t: f64,
    I0: std::ffi::c_int,
    J0: std::ffi::c_int,
) -> f64 {
    port::SlinkyDoubleIntegral(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        t,
        I0,
        J0,
    )
}

/// `SlinkyDoubleIntegral` as `rust_anything_SlinkyDoubleIntegral`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyDoubleIntegral(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    m: std::ffi::c_int,
    n: std::ffi::c_int,
    m1: std::ffi::c_int,
    n1: std::ffi::c_int,
    t: f64,
    I0: std::ffi::c_int,
    J0: std::ffi::c_int,
) -> f64 {
    port::SlinkyDoubleIntegral(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        m,
        n,
        m1,
        n1,
        t,
        I0,
        J0,
    )
}

/// `double SlinkyGFunction(int verticalConfig, double coilDiameter, double coilPitch, double coilDepth, double trenchSpacing, int numTrenches, int numCoils, double diffusivity, double outRadius, double maxSimYears, int k)` (`slinky.c:50`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlinkyGFunction(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    maxSimYears: f64,
    k: std::ffi::c_int,
) -> f64 {
    port::SlinkyGFunction(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        maxSimYears,
        k,
    )
}

/// `SlinkyGFunction` as `rust_anything_SlinkyGFunction`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlinkyGFunction(
    verticalConfig: std::ffi::c_int,
    coilDiameter: f64,
    coilPitch: f64,
    coilDepth: f64,
    trenchSpacing: f64,
    numTrenches: std::ffi::c_int,
    numCoils: std::ffi::c_int,
    diffusivity: f64,
    outRadius: f64,
    maxSimYears: f64,
    k: std::ffi::c_int,
) -> f64 {
    port::SlinkyGFunction(
        verticalConfig,
        coilDiameter,
        coilPitch,
        coilDepth,
        trenchSpacing,
        numTrenches,
        numCoils,
        diffusivity,
        outRadius,
        maxSimYears,
        k,
    )
}
