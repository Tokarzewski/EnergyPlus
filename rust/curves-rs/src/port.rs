//! The Rust port of `curves.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Each function is EnergyPlus `Curve::value` (`CurveManager.cc`) for one curve type, called
//! with that type's own number of inputs: clamp each input to its limits, evaluate the formula
//! exactly as the C++ writes it (same operation order, no FMA), then apply the output limits.
//! A NaN `outmin` or `outmax` means that limit is absent. C has no error channel, so nothing
//! here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the C signatures pass every coefficient and limit as its own double"
)]

/// `std::clamp` as libstdc++ 13 defines it, `std::min(std::max(v, lo), hi)`: a NaN `v` passes
/// through, NaN limits are ignored and, when `lo > hi`, the result is `hi`.
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    let t = if v < lo { lo } else { v };
    if hi < t { hi } else { t }
}

/// The output limits: ObjexxFCL `max(Val, min)` then `min(Val, max)`, which are
/// `a < b ? b : a` and `a < b ? a : b`, so a NaN value becomes a present limit.
fn limit(val: f64, outmin: f64, outmax: f64) -> f64 {
    let v = if !outmin.is_nan() && val < outmin {
        outmin
    } else {
        val
    };
    if outmax.is_nan() || v < outmax {
        v
    } else {
        outmax
    }
}

/// No output limits: `FanPressureRise` returns before EnergyPlus applies them.
fn unlimited(val: f64, _outmin: f64, _outmax: f64) -> f64 {
    val
}

/// ObjexxFCL `sign(1.0, y)`: `y >= 0 ? 1 : -1`, so -0.0 gives 1 and NaN gives -1.
fn sign1(y: f64) -> f64 {
    if y >= 0.0 { 1.0 } else { -1.0 }
}

/// `std::erf`, from the `libm` crate (a port of the same fdlibm `s_erf.c` that glibc's is based
/// on). Stable Rust has no `f64::erf`. It matches glibc on all but one recorded case, where the
/// two round differently in the last bit (see `known-diffs.jsonl`).
fn erf(x: f64) -> f64 {
    libm::erf(x)
}

/// Defines one port per curve type. Every parameter is an `f64`, in the C order: coefficients,
/// then the limits of each input, then the output limits, then the inputs. Each input is
/// clamped to its limits, then the formula is passed through `limit` (or `unlimited`).
macro_rules! curves {
    ($($(#[$doc:meta])* $name:ident($($c:ident),*; $($v:ident: $lo:ident..$hi:ident),*)
        $out:ident $f:block)*) => {$(
        $(#[$doc])*
        pub fn $name($($c: f64,)* $($lo: f64, $hi: f64,)* outmin: f64, outmax: f64, $($v: f64),*)
            -> f64 {
            $(let $v = clamp($v, $lo, $hi);)*
            $out($f, outmin, outmax)
        }
    )*};
}

curves! {
    /// Port of `CurveLinear` (`curves.c:27`).
    CurveLinear(c0, c1; V1: x1min..x1max) limit { c0 + V1 * c1 }

    /// Port of `CurveQuadratic` (`curves.c:32`).
    CurveQuadratic(c0, c1, c2; V1: x1min..x1max) limit { c0 + V1 * (c1 + V1 * c2) }

    /// Port of `CurveCubic` (`curves.c:37`).
    CurveCubic(c0, c1, c2, c3; V1: x1min..x1max) limit { c0 + V1 * (c1 + V1 * (c2 + V1 * c3)) }

    /// Port of `CurveQuartic` (`curves.c:42`).
    CurveQuartic(c0, c1, c2, c3, c4; V1: x1min..x1max) limit {
        c0 + V1 * (c1 + V1 * (c2 + V1 * (c3 + V1 * c4)))
    }

    /// Port of `CurveExponent` (`curves.c:47`).
    CurveExponent(c0, c1, c2; V1: x1min..x1max) limit { c0 + c1 * V1.powf(c2) }

    /// Port of `CurveExponentialSkewNormal` (`curves.c:52`). Not bit-exact: see [`erf`].
    CurveExponentialSkewNormal(c0, c1, c2, c3; V1: x1min..x1max) limit {
        let z1 = (V1 - c0) / c1;
        let z2 = (c3 * V1 * (c2 * V1).exp() - c0) / c1;
        let z3 = -c0 / c1;
        let sqrt_2_inv = 1.0 / 2.0_f64.sqrt();
        let numer = (-0.5 * (z1 * z1)).exp() * (1.0 + sign1(z2) * erf(z2.abs() * sqrt_2_inv));
        let denom = (-0.5 * (z3 * z3)).exp() * (1.0 + sign1(z3) * erf(z3.abs() * sqrt_2_inv));
        numer / denom
    }

    /// Port of `CurveSigmoid` (`curves.c:57`).
    CurveSigmoid(c0, c1, c2, c3, c4; V1: x1min..x1max) limit {
        let curve_value_exp = ((c2 - V1) / c3).exp();
        c0 + c1 / (1.0 + curve_value_exp).powf(c4)
    }

    /// Port of `CurveRectangularHyperbola1` (`curves.c:62`).
    CurveRectangularHyperbola1(c0, c1, c2; V1: x1min..x1max) limit {
        (c0 * V1) / (c1 + V1) + c2
    }

    /// Port of `CurveRectangularHyperbola2` (`curves.c:67`).
    CurveRectangularHyperbola2(c0, c1, c2; V1: x1min..x1max) limit {
        (c0 * V1) / (c1 + V1) + (c2 * V1)
    }

    /// Port of `CurveExponentialDecay` (`curves.c:72`).
    CurveExponentialDecay(c0, c1, c2; V1: x1min..x1max) limit { c0 + c1 * (c2 * V1).exp() }

    /// Port of `CurveDoubleExponentialDecay` (`curves.c:77`).
    CurveDoubleExponentialDecay(c0, c1, c2, c3, c4; V1: x1min..x1max) limit {
        c0 + c1 * (c2 * V1).exp() + c3 * (c4 * V1).exp()
    }

    /// Port of `CurveBiQuadratic` (`curves.c:82`).
    CurveBiQuadratic(c0, c1, c2, c3, c4, c5; V1: x1min..x1max, V2: x2min..x2max) limit {
        c0 + V1 * (c1 + V1 * c2) + V2 * (c3 + V2 * c4) + V1 * V2 * c5
    }

    /// Port of `CurveQuadraticLinear` (`curves.c:87`).
    CurveQuadraticLinear(c0, c1, c2, c3, c4, c5; V1: x1min..x1max, V2: x2min..x2max) limit {
        (c0 + V1 * (c1 + V1 * c2)) + (c3 + V1 * (c4 + V1 * c5)) * V2
    }

    /// Port of `CurveCubicLinear` (`curves.c:92`).
    CurveCubicLinear(c0, c1, c2, c3, c4, c5; V1: x1min..x1max, V2: x2min..x2max) limit {
        (c0 + V1 * (c1 + V1 * (c2 + V1 * c3))) + (c4 + V1 * c5) * V2
    }

    /// Port of `CurveBiCubic` (`curves.c:97`).
    CurveBiCubic(c0, c1, c2, c3, c4, c5, c6, c7, c8, c9; V1: x1min..x1max, V2: x2min..x2max)
    limit {
        c0 + V1 * c1 + V1 * V1 * c2 + V2 * c3 + V2 * V2 * c4 + V1 * V2 * c5 + V1 * V1 * V1 * c6
            + V2 * V2 * V2 * c7 + V1 * V1 * V2 * c8 + V1 * V2 * V2 * c9
    }

    /// Port of `CurveFanPressureRise` (`curves.c:102`). As in EnergyPlus, this curve type
    /// returns before the output limits are applied, so `outmin` and `outmax` have no effect.
    CurveFanPressureRise(c0, c1, c2, c3; V1: x1min..x1max, V2: x2min..x2max) unlimited {
        V1 * (c0 * V1 + c1 + c2 * V2.sqrt()) + c3 * V2
    }

    /// Port of `CurveChillerPartLoadWithLift` (`curves.c:107`).
    CurveChillerPartLoadWithLift(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11;
        V1: x1min..x1max, V2: x2min..x2max, V3: x3min..x3max
    ) limit {
        c0 + c1 * V1 + c2 * V1 * V1 + c3 * V2 + c4 * V2 * V2 + c5 * V1 * V2 + c6 * V1 * V1 * V1
            + c7 * V2 * V2 * V2 + c8 * V1 * V1 * V2 + c9 * V1 * V2 * V2
            + c10 * V1 * V1 * V2 * V2 + c11 * V3 * V2 * V2 * V2
    }

    /// Port of `CurveTriQuadratic` (`curves.c:112`).
    CurveTriQuadratic(
        c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17, c18,
        c19, c20, c21, c22, c23, c24, c25, c26;
        V1: x1min..x1max, V2: x2min..x2max, V3: x3min..x3max
    ) limit {
        let (V1s, V2s, V3s) = (V1 * V1, V2 * V2, V3 * V3);
        c0 + c1 * V1s + c2 * V1 + c3 * V2s + c4 * V2 + c5 * V3s + c6 * V3 + c7 * V1s * V2s
            + c8 * V1 * V2 + c9 * V1 * V2s + c10 * V1s * V2 + c11 * V1s * V3s + c12 * V1 * V3
            + c13 * V1 * V3s + c14 * V1s * V3 + c15 * V2s * V3s + c16 * V2 * V3
            + c17 * V2 * V3s + c18 * V2s * V3 + c19 * V1s * V2s * V3s + c20 * V1s * V2s * V3
            + c21 * V1s * V2 * V3s + c22 * V1 * V2s * V3s + c23 * V1s * V2 * V3
            + c24 * V1 * V2s * V3 + c25 * V1 * V2 * V3s + c26 * V1 * V2 * V3
    }

    /// Port of `CurveQuadLinear` (`curves.c:117`).
    CurveQuadLinear(
        c0, c1, c2, c3, c4;
        V1: x1min..x1max, V2: x2min..x2max, V3: x3min..x3max, V4: x4min..x4max
    ) limit {
        c0 + V1 * c1 + V2 * c2 + V3 * c3 + V4 * c4
    }

    /// Port of `CurveQuintLinear` (`curves.c:122`).
    CurveQuintLinear(
        c0, c1, c2, c3, c4, c5;
        V1: x1min..x1max, V2: x2min..x2max, V3: x3min..x3max, V4: x4min..x4max,
        V5: x5min..x5max
    ) limit {
        c0 + V1 * c1 + V2 * c2 + V3 * c3 + V4 * c4 + V5 * c5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAN: f64 = f64::NAN;

    #[test]
    fn clamp_follows_libstdcxx() {
        assert_eq!(clamp(5.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp(-5.0, 0.0, 1.0), 0.0);
        assert!(clamp(NAN, 0.0, 1.0).is_nan());
        assert_eq!(clamp(5.0, NAN, NAN), 5.0);
        // lo > hi: the result is hi, whatever the value.
        assert_eq!(clamp(-10.0, 3.0, -2.5), -2.5);
        assert_eq!(clamp(10.0, 3.0, -2.5), -2.5);
    }

    #[test]
    fn output_limits_follow_objexxfcl() {
        assert_eq!(limit(5.0, NAN, NAN), 5.0);
        assert_eq!(limit(5.0, 0.0, 1.0), 1.0);
        assert_eq!(limit(-5.0, 0.0, NAN), 0.0);
        // A NaN value becomes the maximum when one is present.
        assert_eq!(limit(NAN, -0.0, -1.0), -1.0);
        assert!(limit(NAN, 0.0, NAN).is_nan());
    }

    #[test]
    fn one_value_per_family() {
        // Recorded from EnergyPlus: V1 = -inf is clamped with lo > hi to hi = -2.5.
        let inf = f64::INFINITY;
        assert_eq!(
            CurveLinear(100.0, -2.5, 2147483648.0, -2.5, -2.5, 2147483648.0, -inf),
            106.25
        );
        assert_eq!(
            CurveQuadratic(1.0, 2.0, 3.0, 0.0, 10.0, NAN, NAN, 2.0),
            17.0
        );
        assert_eq!(CurveExponent(1.0, 2.0, 0.5, 0.0, 10.0, NAN, NAN, 4.0), 5.0);
        let v = CurveBiQuadratic(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 9.0, 0.0, 9.0, NAN, NAN, 1.0, 2.0,
        );
        assert_eq!(v, 11.0);
        let (lo, hi) = (0.0, 9.0);
        let v = CurveQuadLinear(
            1.0, 1.0, 1.0, 1.0, 1.0, lo, hi, lo, hi, lo, hi, lo, hi, NAN, NAN, 1.0, 2.0, 3.0, 4.0,
        );
        assert_eq!(v, 11.0);
        // FanPressureRise ignores its output limits.
        assert_eq!(
            CurveFanPressureRise(1.0, 0.0, 0.0, 0.0, lo, hi, lo, hi, 0.0, 1.0, 3.0, 4.0),
            9.0
        );
        assert_eq!((erf(0.0), erf(7.0), erf(inf)), (0.0, 1.0, 1.0));
    }
}
