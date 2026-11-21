//! C ABI for `curves.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double CurveLinear(double c0, double c1, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveLinear(
    c0: f64,
    c1: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveLinear(c0, c1, x1min, x1max, outmin, outmax, V1)
}

/// `CurveLinear` as `rust_anything_CurveLinear`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveLinear(
    c0: f64,
    c1: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveLinear(c0, c1, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveQuadratic(double c0, double c1, double c2, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveQuadratic(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveQuadratic(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `CurveQuadratic` as `rust_anything_CurveQuadratic`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveQuadratic(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveQuadratic(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveCubic(double c0, double c1, double c2, double c3, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:37`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveCubic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveCubic(c0, c1, c2, c3, x1min, x1max, outmin, outmax, V1)
}

/// `CurveCubic` as `rust_anything_CurveCubic`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveCubic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveCubic(c0, c1, c2, c3, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveQuartic(double c0, double c1, double c2, double c3, double c4, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:42`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveQuartic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveQuartic(c0, c1, c2, c3, c4, x1min, x1max, outmin, outmax, V1)
}

/// `CurveQuartic` as `rust_anything_CurveQuartic`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveQuartic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveQuartic(c0, c1, c2, c3, c4, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveExponent(double c0, double c1, double c2, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:47`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveExponent(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveExponent(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `CurveExponent` as `rust_anything_CurveExponent`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveExponent(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveExponent(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveExponentialSkewNormal(double c0, double c1, double c2, double c3, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:52`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveExponentialSkewNormal(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveExponentialSkewNormal(c0, c1, c2, c3, x1min, x1max, outmin, outmax, V1)
}

/// `CurveExponentialSkewNormal` as `rust_anything_CurveExponentialSkewNormal`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveExponentialSkewNormal(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveExponentialSkewNormal(c0, c1, c2, c3, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveSigmoid(double c0, double c1, double c2, double c3, double c4, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:57`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveSigmoid(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveSigmoid(c0, c1, c2, c3, c4, x1min, x1max, outmin, outmax, V1)
}

/// `CurveSigmoid` as `rust_anything_CurveSigmoid`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveSigmoid(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveSigmoid(c0, c1, c2, c3, c4, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveRectangularHyperbola1(double c0, double c1, double c2, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:62`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveRectangularHyperbola1(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveRectangularHyperbola1(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `CurveRectangularHyperbola1` as `rust_anything_CurveRectangularHyperbola1`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveRectangularHyperbola1(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveRectangularHyperbola1(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveRectangularHyperbola2(double c0, double c1, double c2, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:67`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveRectangularHyperbola2(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveRectangularHyperbola2(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `CurveRectangularHyperbola2` as `rust_anything_CurveRectangularHyperbola2`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveRectangularHyperbola2(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveRectangularHyperbola2(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveExponentialDecay(double c0, double c1, double c2, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:72`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveExponentialDecay(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveExponentialDecay(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `CurveExponentialDecay` as `rust_anything_CurveExponentialDecay`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveExponentialDecay(
    c0: f64,
    c1: f64,
    c2: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveExponentialDecay(c0, c1, c2, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveDoubleExponentialDecay(double c0, double c1, double c2, double c3, double c4, double x1min, double x1max, double outmin, double outmax, double V1)` (`curves.c:77`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveDoubleExponentialDecay(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveDoubleExponentialDecay(c0, c1, c2, c3, c4, x1min, x1max, outmin, outmax, V1)
}

/// `CurveDoubleExponentialDecay` as `rust_anything_CurveDoubleExponentialDecay`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveDoubleExponentialDecay(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
) -> f64 {
    port::CurveDoubleExponentialDecay(c0, c1, c2, c3, c4, x1min, x1max, outmin, outmax, V1)
}

/// `double CurveBiQuadratic(double c0, double c1, double c2, double c3, double c4, double c5, double x1min, double x1max, double x2min, double x2max, double outmin, double outmax, double V1, double V2)` (`curves.c:82`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveBiQuadratic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveBiQuadratic(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `CurveBiQuadratic` as `rust_anything_CurveBiQuadratic`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveBiQuadratic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveBiQuadratic(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `double CurveQuadraticLinear(double c0, double c1, double c2, double c3, double c4, double c5, double x1min, double x1max, double x2min, double x2max, double outmin, double outmax, double V1, double V2)` (`curves.c:87`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveQuadraticLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveQuadraticLinear(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `CurveQuadraticLinear` as `rust_anything_CurveQuadraticLinear`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveQuadraticLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveQuadraticLinear(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `double CurveCubicLinear(double c0, double c1, double c2, double c3, double c4, double c5, double x1min, double x1max, double x2min, double x2max, double outmin, double outmax, double V1, double V2)` (`curves.c:92`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveCubicLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveCubicLinear(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `CurveCubicLinear` as `rust_anything_CurveCubicLinear`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveCubicLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveCubicLinear(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `double CurveBiCubic(double c0, double c1, double c2, double c3, double c4, double c5, double c6, double c7, double c8, double c9, double x1min, double x1max, double x2min, double x2max, double outmin, double outmax, double V1, double V2)` (`curves.c:97`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveBiCubic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveBiCubic(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `CurveBiCubic` as `rust_anything_CurveBiCubic`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveBiCubic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveBiCubic(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `double CurveFanPressureRise(double c0, double c1, double c2, double c3, double x1min, double x1max, double x2min, double x2max, double outmin, double outmax, double V1, double V2)` (`curves.c:102`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveFanPressureRise(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveFanPressureRise(
        c0, c1, c2, c3, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `CurveFanPressureRise` as `rust_anything_CurveFanPressureRise`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveFanPressureRise(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
) -> f64 {
    port::CurveFanPressureRise(
        c0, c1, c2, c3, x1min, x1max, x2min, x2max, outmin, outmax, V1, V2,
    )
}

/// `double CurveChillerPartLoadWithLift(double c0, double c1, double c2, double c3, double c4, double c5, double c6, double c7, double c8, double c9, double c10, double c11, double x1min, double x1max, double x2min, double x2max, double x3min, double x3max, double outmin, double outmax, double V1, double V2, double V3)` (`curves.c:107`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveChillerPartLoadWithLift(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
) -> f64 {
    port::CurveChillerPartLoadWithLift(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, x1min, x1max, x2min, x2max, x3min, x3max,
        outmin, outmax, V1, V2, V3,
    )
}

/// `CurveChillerPartLoadWithLift` as `rust_anything_CurveChillerPartLoadWithLift`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveChillerPartLoadWithLift(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
) -> f64 {
    port::CurveChillerPartLoadWithLift(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, x1min, x1max, x2min, x2max, x3min, x3max,
        outmin, outmax, V1, V2, V3,
    )
}

/// `double CurveTriQuadratic(double c0, double c1, double c2, double c3, double c4, double c5, double c6, double c7, double c8, double c9, double c10, double c11, double c12, double c13, double c14, double c15, double c16, double c17, double c18, double c19, double c20, double c21, double c22, double c23, double c24, double c25, double c26, double x1min, double x1max, double x2min, double x2max, double x3min, double x3max, double outmin, double outmax, double V1, double V2, double V3)` (`curves.c:112`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveTriQuadratic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
    c12: f64,
    c13: f64,
    c14: f64,
    c15: f64,
    c16: f64,
    c17: f64,
    c18: f64,
    c19: f64,
    c20: f64,
    c21: f64,
    c22: f64,
    c23: f64,
    c24: f64,
    c25: f64,
    c26: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
) -> f64 {
    port::CurveTriQuadratic(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19,
        c20, c21, c22, c23, c24, c25, c26, x1min, x1max, x2min, x2max, x3min, x3max, outmin,
        outmax, V1, V2, V3,
    )
}

/// `CurveTriQuadratic` as `rust_anything_CurveTriQuadratic`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveTriQuadratic(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
    c12: f64,
    c13: f64,
    c14: f64,
    c15: f64,
    c16: f64,
    c17: f64,
    c18: f64,
    c19: f64,
    c20: f64,
    c21: f64,
    c22: f64,
    c23: f64,
    c24: f64,
    c25: f64,
    c26: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
) -> f64 {
    port::CurveTriQuadratic(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18, c19,
        c20, c21, c22, c23, c24, c25, c26, x1min, x1max, x2min, x2max, x3min, x3max, outmin,
        outmax, V1, V2, V3,
    )
}

/// `double CurveQuadLinear(double c0, double c1, double c2, double c3, double c4, double x1min, double x1max, double x2min, double x2max, double x3min, double x3max, double x4min, double x4max, double outmin, double outmax, double V1, double V2, double V3, double V4)` (`curves.c:117`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveQuadLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    x4min: f64,
    x4max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
    V4: f64,
) -> f64 {
    port::CurveQuadLinear(
        c0, c1, c2, c3, c4, x1min, x1max, x2min, x2max, x3min, x3max, x4min, x4max, outmin, outmax,
        V1, V2, V3, V4,
    )
}

/// `CurveQuadLinear` as `rust_anything_CurveQuadLinear`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveQuadLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    x4min: f64,
    x4max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
    V4: f64,
) -> f64 {
    port::CurveQuadLinear(
        c0, c1, c2, c3, c4, x1min, x1max, x2min, x2max, x3min, x3max, x4min, x4max, outmin, outmax,
        V1, V2, V3, V4,
    )
}

/// `double CurveQuintLinear(double c0, double c1, double c2, double c3, double c4, double c5, double x1min, double x1max, double x2min, double x2max, double x3min, double x3max, double x4min, double x4max, double x5min, double x5max, double outmin, double outmax, double V1, double V2, double V3, double V4, double V5)` (`curves.c:122`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CurveQuintLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    x4min: f64,
    x4max: f64,
    x5min: f64,
    x5max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
    V4: f64,
    V5: f64,
) -> f64 {
    port::CurveQuintLinear(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, x3min, x3max, x4min, x4max, x5min,
        x5max, outmin, outmax, V1, V2, V3, V4, V5,
    )
}

/// `CurveQuintLinear` as `rust_anything_CurveQuintLinear`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CurveQuintLinear(
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    x1min: f64,
    x1max: f64,
    x2min: f64,
    x2max: f64,
    x3min: f64,
    x3max: f64,
    x4min: f64,
    x4max: f64,
    x5min: f64,
    x5max: f64,
    outmin: f64,
    outmax: f64,
    V1: f64,
    V2: f64,
    V3: f64,
    V4: f64,
    V5: f64,
) -> f64 {
    port::CurveQuintLinear(
        c0, c1, c2, c3, c4, c5, x1min, x1max, x2min, x2max, x3min, x3max, x4min, x4max, x5min,
        x5max, outmin, outmax, V1, V2, V3, V4, V5,
    )
}
