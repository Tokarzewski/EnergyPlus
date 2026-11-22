//! The Rust port of `polyclip.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The polygon overlap at the heart of EnergyPlus `SolarShading.cc`: `HTRANS1` and `HTRANS0`
//! (homogeneous coordinates of a figure's vertices and sides, and its area), `CLIPPOLY`
//! (Sutherland-Hodgman clipping on 0.01 mm integer coordinates, with the near-duplicate removal
//! on the last edge) and `DeterminePolygonOverlap` (the overlap figure, its status, signed area and
//! transmittance), in the C++'s operation order. The oracle's guards (see its shim) return NaN
//! here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

/// Metres to homogeneous-coordinate units (0.01 mm).
const HCMULT: f64 = 100000.0;
const SQ_HCMULT_FAC: f64 = 0.5 / (HCMULT * HCMULT);
/// `MaxHCV`: an overlap of this many vertices or more is `TooManyVertices`.
const MAX_HCV: usize = 15;
/// `2 * (MaxVerticesPerSurface + 1)` for surfaces of up to 4 vertices: the guard on the
/// intersections added when a side leaves the clipping half-plane.
const TEMP_GUARD: usize = 10;

const NO_OVERLAP: i32 = 1;
const FIRST_SURF_WITHIN_SECOND: i32 = 2;
const PARTIAL_OVERLAP: i32 = 4;
const TOO_MANY_VERTICES: i32 = 5;

/// One homogeneous-coordinate figure: a row of `HCX`, `HCY`, `HCA`, `HCB`, `HCC` (0-based here,
/// with room for the closing vertex) plus its `HCNV`, `HCAREA` and `HCT`.
#[derive(Clone, Copy, Default)]
struct Figure {
    x: [i64; MAX_HCV + 1],
    y: [i64; MAX_HCV + 1],
    a: [i64; MAX_HCV + 1],
    b: [i64; MAX_HCV + 1],
    c: [i64; MAX_HCV + 1],
    nv: usize,
    area: f64,
    t: f64,
}

/// ObjexxFCL `nint64`: `int64_t(x + sign(x) * 0.5)`, with `sign(x)` = `x >= 0 ? 1 : -1`.
fn nint64(x: f64) -> i64 {
    let s = if x >= 0.0 { 1.0 } else { -1.0 };
    (x + s * 0.5) as i64
}

/// `General::SafeDivide`.
fn safe_divide(a: f64, b: f64) -> f64 {
    const SMALL: f64 = 1.0e-10;
    if b.abs() >= SMALL {
        return a / b;
    }
    a / if b >= 0.0 { SMALL } else { -SMALL }
}

/// `HTRANS0`: sides and area of a figure whose vertices are already set.
fn htrans0(f: &mut Figure, nv: usize) {
    f.nv = nv;
    f.x[nv] = f.x[0];
    f.y[nv] = f.y[0];
    let mut sum = 0.0;
    for n in 0..nv {
        let (xl, yl, xm, ym) = (f.x[n], f.y[n], f.x[n + 1], f.y[n + 1]);
        f.a[n] = yl.wrapping_sub(ym);
        f.b[n] = xm.wrapping_sub(xl);
        f.c[n] = ym.wrapping_mul(xl).wrapping_sub(xm.wrapping_mul(yl));
        sum += f.c[n] as f64;
    }
    f.area = sum * SQ_HCMULT_FAC;
}

/// `HTRANS1`: a figure from vertices in metres.
fn htrans1(xs: &[f64], ys: &[f64]) -> Figure {
    let mut f = Figure::default();
    for (n, (&x, &y)) in xs.iter().zip(ys).enumerate() {
        f.x[n] = nint64(x * HCMULT);
        f.y[n] = nint64(y * HCMULT);
    }
    htrans0(&mut f, xs.len());
    f
}

/// Writes `v[i]`, growing `v` as `Array1D::redimension` grows the scratch arrays.
fn put(v: &mut Vec<f64>, i: usize, val: f64) {
    if i >= v.len() {
        v.resize(i + 1, 0.0);
    }
    v[i] = val;
}

/// The scratch arrays `XTEMP`, `YTEMP`, `ATEMP`, `BTEMP`, `CTEMP` (1-based).
struct Temp {
    x: Vec<f64>,
    y: Vec<f64>,
    a: Vec<f64>,
    b: Vec<f64>,
    c: Vec<f64>,
}

impl Temp {
    /// The intersection of the side through vertex `s` with the clipping side, written at `at`.
    fn intersect(&mut self, at: usize, s: usize, hca: f64, hcb: f64, hcc: f64) {
        let (a_s, b_s, c_s) = (self.a[s], self.b[s], self.c[s]);
        let w = hcb * a_s - hca * b_s;
        let (x, y) = if w != 0.0 {
            let w_inv = 1.0 / w;
            (
                nint64((hcc * b_s - hcb * c_s) * w_inv) as f64,
                nint64((hca * c_s - hcc * a_s) * w_inv) as f64,
            )
        } else {
            (
                safe_divide(hcc * b_s - hcb * c_s, w),
                safe_divide(hca * c_s - hcc * a_s, w),
            )
        };
        put(&mut self.x, at, x);
        put(&mut self.y, at, y);
    }

    /// Near-duplicate removal on the last edge: the vertex just written at `nvtemp` is dropped
    /// (the count goes back to `kk`) when it lies within 2 units of an earlier one.
    fn dedup(&self, nvtemp: usize, kk: usize) -> usize {
        let (x, y) = (self.x[nvtemp], self.y[nvtemp]);
        for k in 1..=kk {
            if (x - self.x[k]).abs() > 2.0 {
                continue;
            }
            if (y - self.y[k]).abs() > 2.0 {
                continue;
            }
            return kk;
        }
        nvtemp
    }
}

/// `CLIPPOLY` with Sutherland-Hodgman: clips `f1` by `f2`. Returns the overlap's vertex count, the
/// status it sets (if any) and the clipped vertices.
fn clippoly(f1: &Figure, f2: &Figure) -> (usize, Option<i32>, Temp) {
    let (nv1, nv2) = (f1.nv, f2.nv);
    let len = TEMP_GUARD + 1;
    let mut t = Temp {
        x: vec![0.0; len],
        y: vec![0.0; len],
        a: vec![0.0; len],
        b: vec![0.0; len],
        c: vec![0.0; len],
    };
    let mut x1: Vec<f64> = vec![0.0; len];
    let mut y1: Vec<f64> = vec![0.0; len];
    for j in 0..nv1 {
        t.x[j + 1] = f1.x[j] as f64;
        t.y[j + 1] = f1.y[j] as f64;
        t.a[j + 1] = f1.a[j] as f64;
        t.b[j + 1] = f1.b[j] as f64;
        t.c[j + 1] = f1.c[j] as f64;
    }
    let mut nvout = nv1;
    let mut intflag = false;
    let mut nvtemp = 0usize;
    for e in 1..=nv2 {
        for p in 1..=nvout {
            put(&mut x1, p, t.x[p]);
            put(&mut y1, p, t.y[p]);
        }
        let mut s = nvout;
        let (hca, hcb, hcc) = (f2.a[e - 1] as f64, f2.b[e - 1] as f64, f2.c[e - 1] as f64);
        let mut xs = x1[s];
        let mut ys = y1[s];
        for p in 1..=nvout {
            let (xp, yp) = (x1[p], y1[p]);
            let h = xp * hca + yp * hcb + hcc;
            if h <= 0.0 {
                let h = xs * hca + ys * hcb + hcc;
                if h > 0.0 {
                    let kk = nvtemp;
                    nvtemp += 1;
                    t.intersect(nvtemp, s, hca, hcb, hcc);
                    intflag = true;
                    if e == nv2 && kk != 0 {
                        nvtemp = t.dedup(nvtemp, kk);
                    }
                }
                let kk = nvtemp;
                nvtemp += 1;
                put(&mut t.x, nvtemp, xp);
                put(&mut t.y, nvtemp, yp);
                if e == nv2 && kk != 0 {
                    nvtemp = t.dedup(nvtemp, kk);
                }
            } else {
                let h = xs * hca + ys * hcb + hcc;
                if h <= 0.0 && nvtemp < TEMP_GUARD {
                    let kk = nvtemp;
                    nvtemp += 1;
                    t.intersect(nvtemp, s, hca, hcb, hcc);
                    intflag = true;
                    if e == nv2 && kk != 0 {
                        nvtemp = t.dedup(nvtemp, kk);
                    }
                }
            }
            s = p;
            xs = xp;
            ys = yp;
        }
        nvout = nvtemp;
        if nvout == 0 {
            break;
        }
        nvtemp = 0;
        if e != nv2 && nvout > 2 {
            let (x_1, y_1) = (t.x[1], t.y[1]);
            let (mut xp, mut yp, mut xp1, mut yp1) = (x_1, y_1, x_1, y_1);
            for p in 1..nvout {
                xp1 = t.x[p + 1];
                yp1 = t.y[p + 1];
                put(&mut t.a, p, yp - yp1);
                put(&mut t.b, p, xp1 - xp);
                put(&mut t.c, p, xp * yp1 - yp * xp1);
                xp = xp1;
                yp = yp1;
            }
            put(&mut t.a, nvout, yp1 - y_1);
            put(&mut t.b, nvout, x_1 - xp1);
            put(&mut t.c, nvout, xp1 * y_1 - yp1 * x_1);
        }
    }
    let status = if nvout < 3 {
        Some(NO_OVERLAP)
    } else if !intflag {
        Some(FIRST_SURF_WITHIN_SECOND)
    } else {
        None
    };
    (nvout, status, t)
}

/// `DeterminePolygonOverlap(1, 2, 3)`: the overlap of `f2` on `f1`, and its status.
fn overlap(f1: &Figure, f2: &Figure) -> (Figure, i32) {
    let (nv3, set, t) = clippoly(f1, f2);
    let mut status = set.unwrap_or(PARTIAL_OVERLAP);
    let mut f3 = Figure::default();
    if nv3 < MAX_HCV {
        for n in 0..nv3 {
            f3.x[n] = nint64(t.x[n + 1]);
            f3.y[n] = nint64(t.y[n + 1]);
        }
        htrans0(&mut f3, nv3);
        if f3.area.abs() * HCMULT < f1.area.abs() {
            status = NO_OVERLAP;
        } else {
            if f1.area * f2.area > 0.0 {
                f3.area = -f3.area;
            }
            f3.t = if f2.t == 1.0 || f1.t == 1.0 {
                f1.t * f2.t
            } else {
                (f1.t + f2.t) - f1.t * f2.t
            };
        }
    } else {
        status = TOO_MANY_VERTICES;
    }
    (f3, status)
}

/// The overlap of polygon 2 on polygon 1 (see the oracle's shim for `which`).
pub fn PolyOverlap(
    which: i32,
    nv1: i32,
    ax1: f64,
    ay1: f64,
    ax2: f64,
    ay2: f64,
    ax3: f64,
    ay3: f64,
    ax4: f64,
    ay4: f64,
    nv2: i32,
    bx1: f64,
    by1: f64,
    bx2: f64,
    by2: f64,
    bx3: f64,
    by3: f64,
    bx4: f64,
    by4: f64,
    t1: f64,
    t2: f64,
) -> f64 {
    let a = [ax1, ay1, ax2, ay2, ax3, ay3, ax4, ay4];
    let b = [bx1, by1, bx2, by2, bx3, by3, bx4, by4];
    if !(3..=4).contains(&nv1) || !(3..=4).contains(&nv2) {
        return f64::NAN;
    }
    // Written as the shim's `!(abs(v) <= 1000)` so NaN is refused too.
    let ok = |v: f64| v.abs() <= 1000.0;
    if !a.iter().chain(&b).all(|&v| ok(v)) {
        return f64::NAN;
    }
    let figure = |c: &[f64; 8], nv: i32, t: f64| {
        let n = nv as usize;
        let xs: Vec<f64> = (0..n).map(|i| c[2 * i]).collect();
        let ys: Vec<f64> = (0..n).map(|i| c[2 * i + 1]).collect();
        let mut f = htrans1(&xs, &ys);
        f.t = t;
        f
    };
    let f1 = figure(&a, nv1, t1);
    let f2 = figure(&b, nv2, t2);
    let (f3, status) = overlap(&f1, &f2);
    let nv3 = f3.nv as i32;
    match which {
        0 => f64::from(status),
        1 => f64::from(nv3),
        2 => f3.area,
        3 => f3.t,
        4 => f1.area,
        5 => f2.area,
        6..=20 if which - 6 < nv3 => f3.x[(which - 6) as usize] as f64,
        21..=35 if which - 21 < nv3 => f3.y[(which - 21) as usize] as f64,
        _ => f64::NAN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sq(w: i32, x: f64, y: f64, s: f64, ox: f64, oy: f64, os: f64) -> f64 {
        PolyOverlap(
            w,
            4,
            x,
            y,
            x,
            y + s,
            x + s,
            y + s,
            x + s,
            y,
            4,
            ox,
            oy,
            ox,
            oy + os,
            ox + os,
            oy + os,
            ox + os,
            oy,
            1.0,
            0.5,
        )
    }

    #[test]
    fn half_overlap_of_two_unit_squares() {
        assert_eq!(
            sq(0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0),
            f64::from(PARTIAL_OVERLAP)
        );
        assert_eq!(sq(1, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0), 4.0);
        assert_eq!(sq(2, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0).abs(), 0.5);
        assert_eq!(sq(3, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0), 0.5);
    }

    #[test]
    fn disjoint_squares_do_not_overlap() {
        assert_eq!(sq(0, 0.0, 0.0, 1.0, 5.0, 5.0, 1.0), f64::from(NO_OVERLAP));
        assert_eq!(sq(1, 0.0, 0.0, 1.0, 5.0, 5.0, 1.0), 0.0);
    }

    #[test]
    fn square_inside_a_larger_one() {
        assert_eq!(
            sq(0, 0.25, 0.25, 0.5, 0.0, 0.0, 1.0),
            f64::from(FIRST_SURF_WITHIN_SECOND)
        );
        assert_eq!(sq(2, 0.25, 0.25, 0.5, 0.0, 0.0, 1.0).abs(), 0.25);
    }

    #[test]
    fn guards_return_nan() {
        assert!(sq(0, 2000.0, 0.0, 1.0, 0.0, 0.0, 1.0).is_nan());
        assert!(sq(0, f64::NAN, 0.0, 1.0, 0.0, 0.0, 1.0).is_nan());
        assert!(sq(99, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0).is_nan());
        assert!(sq(10, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0).is_nan());
    }

    #[test]
    fn rounding_matches_nint64() {
        assert_eq!(nint64(2.5), 3);
        assert_eq!(nint64(-2.5), -3);
        assert_eq!(nint64(-0.4), 0);
        assert_eq!(safe_divide(1.0, 0.0), 1.0e10);
    }
}
