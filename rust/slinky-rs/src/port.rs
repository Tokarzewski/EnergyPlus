//! The Rust port of `slinky.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The g-function of the EnergyPlus slinky ground heat exchanger (`GLHESlinky` in
//! `GroundHeatExchangers/Slinky.cc`; Xiong et al. 2015): ring-to-ring distances, the near- and
//! mid-field responses, the Simpson integrals over both rings, and the field sum for one time.
//! Every formula keeps the C++ operation order with no FMA, and the Simpson sums add up in the
//! order libstdc++'s `std::reduce` uses. Outside the unit's domain (see `Slinky::new`) every
//! function returns NaN, as the oracle does. C has no error channel: nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the C signatures pass the whole geometry as plain numbers"
)]

use std::f64::consts::PI;

/// ObjexxFCL `pow_2`: `x * x`.
fn pow_2(x: f64) -> f64 {
    x * x
}

/// `std::erfc`. Stable Rust has none; `libm` ports the same fdlibm algorithm as glibc.
fn erfc(x: f64) -> f64 {
    libm::erfc(x)
}

/// `GLHEBase::isEven`.
fn is_even(v: i32) -> bool {
    v % 2 == 0
}

/// `std::reduce(v.begin(), v.end())` as libstdc++ 13 sums doubles: four at a time as
/// `init + ((a + b) + (c + d))`, then the rest one by one. Fed one value at a time.
struct Reduce {
    sum: f64,
    buf: [f64; 4],
    len: usize,
}

impl Reduce {
    fn new() -> Reduce {
        Reduce {
            sum: 0.0,
            buf: [0.0; 4],
            len: 0,
        }
    }

    fn push(&mut self, v: f64) {
        self.buf[self.len] = v;
        self.len += 1;
        if self.len == 4 {
            let b = self.buf;
            self.sum += (b[0] + b[1]) + (b[2] + b[3]);
            self.len = 0;
        }
    }

    fn finish(self) -> f64 {
        self.buf[..self.len].iter().fold(self.sum, |s, &v| s + v)
    }
}

/// Simpson's 1/3 rule over `0..2 pi` with `count` points, as `integral` and `doubleIntegral`
/// write it: weights 1, 4, 2, 4, ..., 4, 1 applied to each value, then `(h / 3) * sum`.
fn simpson(count: i32, mut f: impl FnMut(f64) -> f64) -> f64 {
    let h = (2.0 * PI - 0.0) / (f64::from(count) - 1.0); // `(count - 1)` without overflow
    let mut sum = Reduce::new();
    for j in 0..count {
        let v = f(0.0 + f64::from(j) * h);
        let interior = j >= 1 && j < count - 1;
        sum.push(if !interior {
            v
        } else if !is_even(j) {
            4.0 * v
        } else {
            2.0 * v
        });
    }
    (h / 3.0) * sum.finish()
}

/// One heat exchanger's geometry, with its rings placed as `calcGFunctions` places them:
/// ring `(m, n)` (trench `m`, coil `n`, both from 1) is centred at `X0(n) = pitch (n - 1)`,
/// `Y0(m) = (m - 1) spacing`, `Z0 = depth`.
struct Slinky {
    vertical: bool,
    d: f64,
    pitch: f64,
    depth: f64,
    spacing: f64,
    nt: i32,
    nc: i32,
    alpha: f64,
    r_out: f64,
}

impl Slinky {
    fn x0(&self, n: i32) -> f64 {
        self.pitch * f64::from(n - 1)
    }

    fn y0(&self, m: i32) -> f64 {
        f64::from(m - 1) * self.spacing
    }

    /// Whether `(m, n)` is a ring of this field.
    fn has(&self, m: i32, n: i32) -> bool {
        (1..=self.nt).contains(&m) && (1..=self.nc).contains(&n)
    }

    /// `distToCenter`: centre to centre.
    fn dist_to_center(&self, m: i32, n: i32, m1: i32, n1: i32) -> f64 {
        (pow_2(self.x0(n) - self.x0(n1)) + pow_2(self.y0(m) - self.y0(m1))).sqrt()
    }

    /// The mean of the distances from the point at `theta` on ring `(m, n)` to the inner and
    /// outer pipe wall at `eta` on ring `(m1, n1)`: `(x, z)` and the two walls' `(x, z)`, with
    /// the rings `dy` apart across the trenches.
    fn mean_wall_distance(
        p: (f64, f64),
        wall_in: (f64, f64),
        wall_out: (f64, f64),
        dy: f64,
    ) -> f64 {
        0.5 * (pow_2(p.0 - wall_in.0) + pow_2(dy) + pow_2(p.1 - wall_in.1)).sqrt()
            + 0.5 * (pow_2(p.0 - wall_out.0) + pow_2(dy) + pow_2(p.1 - wall_out.1)).sqrt()
    }

    /// `distance`, or `distanceToFictRing` (the image ring above the ground surface) when
    /// `image` is set; the image is only used for vertical coils.
    fn distance(&self, m: i32, n: i32, m1: i32, n1: i32, eta: f64, theta: f64, image: bool) -> f64 {
        let (cos_theta, sin_theta, cos_eta, sin_eta) =
            (theta.cos(), theta.sin(), eta.cos(), eta.sin());
        let (r, r_in, r_out) = (
            self.d / 2.0,
            self.d / 2.0 - self.r_out,
            self.d / 2.0 + self.r_out,
        );
        let x = self.x0(n) + cos_theta * r;
        let (x_in, x_out) = (self.x0(n1) + cos_eta * r_in, self.x0(n1) + cos_eta * r_out);
        if !self.vertical && !image {
            let y = self.y0(m) + sin_theta * r;
            let (y_in, y_out) = (self.y0(m1) + sin_eta * r_in, self.y0(m1) + sin_eta * r_out);
            return 0.5 * (pow_2(x - x_in) + pow_2(y - y_in)).sqrt()
                + 0.5 * (pow_2(x - x_out) + pow_2(y - y_out)).sqrt();
        }
        let z = if image {
            self.depth + sin_theta * r + 2.0 * self.depth
        } else {
            self.depth + sin_theta * r
        };
        let (z_in, z_out) = (self.depth + sin_eta * r_in, self.depth + sin_eta * r_out);
        let dy = self.y0(m1) - self.y0(m);
        Self::mean_wall_distance((x, z), (x_in, z_in), (x_out, z_out), dy)
    }

    /// `erfc(d / 2 / sqrt(alpha t)) / d - erfc(d2 / 2 / sqrt(alpha t)) / d2`: a ring source and
    /// its image.
    fn source_and_image(d: f64, d2: f64, sqrt_alpha_t: f64) -> f64 {
        erfc(0.5 * d / sqrt_alpha_t) / d - erfc(0.5 * d2 / sqrt_alpha_t) / d2
    }

    /// The image distance of a horizontal ring `d` away, `sqrt(d^2 + 4 depth^2)`.
    fn image_distance(&self, d: f64) -> f64 {
        (pow_2(d) + 4.0 * pow_2(self.depth)).sqrt()
    }

    /// `nearFieldResponseFunction`.
    fn near_field(&self, m: i32, n: i32, m1: i32, n1: i32, eta: f64, theta: f64, t: f64) -> f64 {
        let d1 = self.distance(m, n, m1, n1, eta, theta, false);
        let sqrt_alpha_t = (self.alpha * t).sqrt();
        let d2 = if self.vertical {
            self.distance(m, n, m1, n1, eta, theta, true)
        } else {
            self.image_distance(d1)
        };
        Self::source_and_image(d1, d2, sqrt_alpha_t)
    }

    /// `midFieldResponseFunction`: the rings as point sources at their centres.
    fn mid_field(&self, m: i32, n: i32, m1: i32, n1: i32, t: f64) -> f64 {
        let sqrt_alpha_t = (self.alpha * t).sqrt();
        let d = self.dist_to_center(m, n, m1, n1);
        4.0 * (PI * PI) * Self::source_and_image(d, self.image_distance(d), sqrt_alpha_t)
    }

    /// `integral`: over the point on ring `(m, n)`, `J0` points.
    fn integral(&self, m: i32, n: i32, m1: i32, n1: i32, t: f64, eta: f64, j0: i32) -> f64 {
        simpson(j0, |theta| self.near_field(m, n, m1, n1, eta, theta, t))
    }

    /// `doubleIntegral`: over the pipe wall of ring `(m1, n1)` too, `I0` points.
    fn double_integral(&self, m: i32, n: i32, m1: i32, n1: i32, t: f64, i0: i32, j0: i32) -> f64 {
        simpson(i0, |eta| self.integral(m, n, m1, n1, t, eta, j0))
    }

    /// The g-function at time step `k` of `calcGFunctions` (`GFNC[k]`). Each step is
    /// independent there (the stored values are cleared for each), so only step `k` is computed.
    fn gfunction(&self, k: i32) -> f64 {
        // `std::pow(10, tLg)`: kept a `pow` call (LLVM may turn `pow(10, x)` into `exp10`).
        let t = std::hint::black_box(10.0_f64).powf(-2.0 + 0.25 * f64::from(k)) * 3600.0;
        let (num_lc, num_rc) = (
            (f64::from(self.nc) / 2.0).ceil() as i32,
            (f64::from(self.nt) / 2.0).ceil() as i32,
        );
        let fraction = if self.nt > 1 { 0.25 } else { 0.5 };
        let (odd_t, odd_c) = (!is_even(self.nt), !is_even(self.nc));
        // `valStored(mm1, nn1)`: the response at a trench and coil offset, -1 until computed.
        let width = self.nc as usize + 1;
        let mut stored = vec![-1.0; (self.nt as usize + 1) * width];
        let mut g_func = 0.0;
        for m1 in 1..=num_rc {
            for n1 in 1..=num_lc {
                for m in 1..=self.nt {
                    for n in 1..=self.nc {
                        let dis_ring = self.dist_to_center(m, n, m1, n1);
                        let key = (m - m1).unsigned_abs() as usize * width
                            + (n - n1).unsigned_abs() as usize;
                        let near = dis_ring <= 2.5 + self.d;
                        if !near && dis_ring > (10.0 + self.d) {
                            g_func += 0.0; // far field
                            continue;
                        }
                        if stored[key] < 0.0 {
                            let j0 = if m1 == m && n1 == n { 1089 } else { 561 };
                            stored[key] = if near {
                                self.double_integral(m, n, m1, n1, t, 33, j0)
                            } else {
                                self.mid_field(m, n, m1, n1, t)
                            };
                        }
                        // By symmetry, ring (m1, n1) counts 0.25, 0.5 or 1 times.
                        let val = stored[key];
                        let last_t = odd_t && m1 == num_rc && self.nt > 1;
                        g_func += if last_t && odd_c && n1 == num_lc {
                            0.25 * val
                        } else if last_t || (odd_c && n1 == num_lc) {
                            0.5 * val
                        } else {
                            val
                        };
                    }
                }
            }
        }
        (g_func * (self.d / 2.0)) / (4.0 * PI * fraction * f64::from(self.nt) * f64::from(self.nc))
    }
}

/// The number of g-functions `calcGFunctions` makes for `years`, as it computes it; 0 where
/// that is not a count it can allocate (the oracle's guard).
fn num_gfunctions(years: f64) -> i32 {
    let tlg_max = (years * (356.0 * 24.0 * 60.0 * 60.0) / 3600.0).log10();
    let pairs = (tlg_max - -2.0) / 0.25 + 1.0;
    let months_fit = years * 12.0 < 2.0e9; // `QnMonthlyAgg.allocate(int(years * 12))`
    if (1.0..1.0e6).contains(&pairs) && months_fit {
        pairs as i32
    } else {
        0
    }
}

/// Defines one export per method: the geometry, then the method's own parameters. A field
/// with no trench or coil gives NaN; `$ok` adds the method's own domain.
macro_rules! slinky {
    ($($(#[$doc:meta])* $name:ident($s:ident; $($p:ident: $t:ty),*) if $ok:expr => $body:expr;)*) => {$(
        $(#[$doc])*
        pub fn $name(verticalConfig: i32, coilDiameter: f64, coilPitch: f64, coilDepth: f64,
            trenchSpacing: f64, numTrenches: i32, numCoils: i32, diffusivity: f64, outRadius: f64,
            $($p: $t),*) -> f64 {
            let $s = Slinky {
                vertical: verticalConfig != 0, d: coilDiameter, pitch: coilPitch, depth: coilDepth,
                spacing: trenchSpacing, nt: numTrenches, nc: numCoils, alpha: diffusivity, r_out: outRadius,
            };
            if $s.nt < 1 || $s.nc < 1 || !$ok { f64::NAN } else { $body }
        }
    )*};
}

slinky! {
    /// Port of `GLHESlinky::distToCenter` (`Slinky.cc:541`).
    SlinkyDistToCenter(s; m: i32, n: i32, m1: i32, n1: i32)
        if s.has(m, n) && s.has(m1, n1) => s.dist_to_center(m, n, m1, n1);
    /// Port of `GLHESlinky::distance` (`Slinky.cc:471`).
    SlinkyDistance(s; m: i32, n: i32, m1: i32, n1: i32, eta: f64, theta: f64)
        if s.has(m, n) && s.has(m1, n1) => s.distance(m, n, m1, n1, eta, theta, false);
    /// Port of `GLHESlinky::distanceToFictRing` (`Slinky.cc:509`).
    SlinkyDistanceToFictRing(s; m: i32, n: i32, m1: i32, n1: i32, eta: f64, theta: f64)
        if s.has(m, n) && s.has(m1, n1) => s.distance(m, n, m1, n1, eta, theta, true);
    /// Port of `GLHESlinky::nearFieldResponseFunction` (`Slinky.cc:417`).
    SlinkyNearFieldResponse(s; m: i32, n: i32, m1: i32, n1: i32, eta: f64, theta: f64, t: f64)
        if s.has(m, n) && s.has(m1, n1) => s.near_field(m, n, m1, n1, eta, theta, t);
    /// Port of `GLHESlinky::midFieldResponseFunction` (`Slinky.cc:448`).
    SlinkyMidFieldResponse(s; m: i32, n: i32, m1: i32, n1: i32, t: f64)
        if s.has(m, n) && s.has(m1, n1) => s.mid_field(m, n, m1, n1, t);
    /// Port of `GLHESlinky::integral` (`Slinky.cc:555`).
    SlinkyIntegral(s; m: i32, n: i32, m1: i32, n1: i32, t: f64, eta: f64, J0: i32)
        if s.has(m, n) && s.has(m1, n1) => s.integral(m, n, m1, n1, t, eta, J0);
    /// Port of `GLHESlinky::doubleIntegral` (`Slinky.cc:219`).
    SlinkyDoubleIntegral(s; m: i32, n: i32, m1: i32, n1: i32, t: f64, I0: i32, J0: i32)
        if s.has(m, n) && s.has(m1, n1) && I0 >= 1 => s.double_integral(m, n, m1, n1, t, I0, J0);
    /// `GFNC[k]` after `GLHESlinky::calcGFunctions` (`Slinky.cc:257`) for a simulation of
    /// `maxSimYears`.
    SlinkyGFunction(s; maxSimYears: f64, k: i32)
        if (0..num_gfunctions(maxSimYears)).contains(&k) => s.gfunction(k);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce_order_and_domain() {
        let mut r = Reduce::new();
        for v in [1e16, 1.0, -1e16, 1.0, 1.0] {
            r.push(v);
        }
        // ((1e16 + 1) + (-1e16 + 1)) loses both ones; the fifth is added after.
        assert_eq!(r.finish(), 1.0);
        assert_eq!(num_gfunctions(10.0), 28);
        assert_eq!(num_gfunctions(0.0), 0);
        assert!(
            SlinkyDistToCenter(0, 1.0, 0.2, 1.5, 2.0, 3, 10, 4.4e-7, 0.01, 0, 1, 1, 1).is_nan()
        );
        assert_eq!(
            SlinkyDistToCenter(0, 1.0, 0.2, 1.5, 2.0, 3, 10, 4.4e-7, 0.01, 1, 1, 2, 1),
            2.0
        );
    }
}
