//! The Rust port of `clipalgo.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The polygon overlap of EnergyPlus `SolarShading.cc` under all three `ShadowCalculation`
//! clipping algorithms: `HTRANS1` and `HTRANS0` (homogeneous coordinates of a figure's vertices and
//! sides, and its area); `CLIPPOLY` (Sutherland-Hodgman on 0.01 mm integer coordinates) with its
//! hand-off of an axis-aligned rectangle to `CLIPRECT` and `CLIPLINE` (Slater-Barsky); the legacy
//! convex Weiler-Atherton path, `INCLOS`, `INTCPT` and `ORDER`; and `DeterminePolygonOverlap`
//! (the overlap figure, its status, signed area and transmittance), in the C++'s operation order.
//! The oracle's guards (see its shim) return NaN here too.

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
/// Length of the scratch arrays (1-based). Each clipping pass at most doubles the vertex count, so
/// 4 vertices clipped by 4 sides leave at most 64; the other paths stay well below that. Fixed
/// arrays on the stack where the C++ grows heap arrays: no vertex count reaches the end.
const CAP: usize = 66;

const NO_OVERLAP: i32 = 1;
const FIRST_SURF_WITHIN_SECOND: i32 = 2;
const SECOND_SURF_WITHIN_FIRST: i32 = 3;
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

/// Writes `v[i]` (where the C++ writes past or grows its scratch arrays, `CAP` covers it).
fn put(v: &mut [f64; CAP], i: usize, val: f64) {
    v[i] = val;
}

/// The scratch arrays `XTEMP`, `YTEMP`, `ATEMP`, `BTEMP`, `CTEMP` (1-based: the C++'s linear
/// `XTEMP[j]` is `x[j + 1]` here).
struct Temp {
    x: [f64; CAP],
    y: [f64; CAP],
    a: [f64; CAP],
    b: [f64; CAP],
    c: [f64; CAP],
}

impl Temp {
    fn new() -> Self {
        Temp {
            x: [0.0; CAP],
            y: [0.0; CAP],
            a: [0.0; CAP],
            b: [0.0; CAP],
            c: [0.0; CAP],
        }
    }

    /// Writes vertex `i` (1-based).
    fn set(&mut self, i: usize, x: f64, y: f64) {
        put(&mut self.x, i, x);
        put(&mut self.y, i, y);
    }

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

/// `CLIPPOLY`: clips `f1` by `f2` with Sutherland-Hodgman, or with `CLIPRECT` when `slater` is
/// set and `f2` is an axis-aligned rectangle. Returns the overlap's vertex count, the status it
/// sets (if any) and the clipped vertices.
fn clippoly(f1: &Figure, f2: &Figure, slater: bool) -> (usize, Option<i32>, Temp) {
    let (nv1, nv2) = (f1.nv, f2.nv);
    let mut t = Temp::new();
    let mut x1 = [0.0; CAP];
    let mut y1 = [0.0; CAP];
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
    if slater && nv2 == 4 && is_rect(f2) {
        let (nv3, status) = cliprect(f2, nv1, &mut t);
        return (nv3, status, t);
    }
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

/// `CLIPPOLY`'s test for an axis-aligned rectangle clipping polygon of 4 vertices.
fn is_rect(f: &Figure) -> bool {
    let (x, y) = (&f.x, &f.y);
    ((x[0] == x[1] && y[0] != y[1]) && (y[2] == y[1] && y[3] == y[0]) && x[2] == x[3])
        || ((y[0] == y[1] && x[0] != x[1]) && (x[2] == x[1] && x[3] == x[0]) && y[2] == y[3])
}

/// `neq` in `SolarShading.cc`: more than 2 units apart.
fn neq(a: f64, b: f64) -> bool {
    (a - b).abs() > 2.0
}

/// `d_eq` in `SolarShading.cc`: less than 2 units apart.
fn d_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 2.0
}

/// The clip window of `CLIPLINE`.
#[derive(Clone, Copy)]
struct Window {
    max_x: f64,
    min_x: f64,
    max_y: f64,
    min_y: f64,
}

/// `CLIPLINE`: Liang-Barsky clipping of the segment (x0, y0)-(x1, y1) to the window, in place.
/// Returns `visible`. A rejected segment may come back with its ends swapped, as in the C++.
fn clipline(x0: &mut f64, x1: &mut f64, y0: &mut f64, y1: &mut f64, w: Window) -> bool {
    let mut rev = false;
    if *x0 > *x1 {
        std::mem::swap(x0, x1);
        std::mem::swap(y0, y1);
        rev = true;
    }
    if *x0 > w.max_x || *x1 < w.min_x {
        return false;
    }
    let dx = *x1 - *x0;
    let dy = *y1 - *y0;
    let q1 = *x0 - w.min_x;
    let q2 = w.max_x - *x0;
    let q3 = *y0 - w.min_y;
    let q4 = w.max_y - *y0;
    let mut u1 = 0.0;
    let mut u2 = 1.0;
    if (dx == 0.0 && (q1 < 0.0 || q2 < 0.0)) || (dy == 0.0 && (q3 < 0.0 || q4 < 0.0)) {
        return false;
    }
    if dx != 0.0 {
        let r1 = q1 / -dx;
        if r1 > u1 {
            u1 = r1;
        }
        let r2 = q2 / dx;
        if r2 < u2 {
            u2 = r2;
        }
    }
    if dy != 0.0 {
        let r3 = q3 / -dy;
        let r4 = q4 / dy;
        if dy > 0.0 {
            if r3 > u1 {
                u1 = r3;
            }
            if r4 < u2 {
                u2 = r4;
            }
        } else {
            if r4 > u1 {
                u1 = r4;
            }
            if r3 < u2 {
                u2 = r3;
            }
        }
    }
    if u1 > u2 {
        return false;
    }
    let xn0 = *x0 + dx * u1;
    let yn0 = *y0 + dy * u1;
    let xn1 = *x0 + dx * u2;
    let yn1 = *y0 + dy * u2;
    if rev {
        *x0 = xn1;
        *y0 = yn1;
        *x1 = xn0;
        *y1 = yn0;
    } else {
        *x0 = xn0;
        *y0 = yn0;
        *x1 = xn1;
        *y1 = yn1;
    }
    true
}

/// `CLIPRECT`'s test that a corner is inside the subject polygon: not to the left of any of its
/// `nv1` sides (`strict`: not on one either, as the empty-overlap case asks). The inputs are
/// finite, so `h <= 0.0` is the C++'s `!(h > 0.0)`.
fn corner_inside(t: &Temp, nv1: usize, cx: f64, cy: f64, strict: bool) -> bool {
    (1..=nv1).all(|j| {
        let h = t.a[j] * cx + cy * t.b[j] + t.c[j];
        if strict { h < 0.0 } else { h <= 0.0 }
    })
}

/// `CLIPRECT`: clips the subject polygon in `t` (its `nv1` vertices and sides) by the rectangle
/// `f2`. Returns the overlap's vertex count and the status it sets (if any); the vertices are
/// left in `t`. The homogeneous sides it recomputes at the end are not read again by
/// `DeterminePolygonOverlap`, so they are not ported.
fn cliprect(f2: &Figure, nv1: usize, t: &mut Temp) -> (usize, Option<i32>) {
    let mut intflag = false;
    let (max_x, min_x) = if f2.x[0] > f2.x[2] {
        (f2.x[0] as f64, f2.x[2] as f64)
    } else {
        (f2.x[2] as f64, f2.x[0] as f64)
    };
    let (max_y, min_y) = if f2.y[0] > f2.y[2] {
        (f2.y[0] as f64, f2.y[2] as f64)
    } else {
        (f2.y[2] as f64, f2.y[0] as f64)
    };
    let win = Window {
        max_x,
        min_x,
        max_y,
        min_y,
    };
    // `arrx`, `arry` and `arrc`: each side adds at most 2 points.
    let mut arrx = [0.0; 20];
    let mut arry = [0.0; 20];
    let mut n = 0usize;
    for j in 0..nv1 {
        let (mut x_1, mut y_1) = (t.x[j + 1], t.y[j + 1]);
        let (mut x_2, mut y_2) = (t.x[(j + 1) % nv1 + 1], t.y[(j + 1) % nv1 + 1]);
        let (x1, x2, y1, y2) = (x_1, x_2, y_1, y_2);
        if !clipline(&mut x_1, &mut x_2, &mut y_1, &mut y_2, win) {
            continue;
        }
        if (x_1 != x1 || y_1 != y1) || (x_2 != x2 || y_2 != y2) {
            intflag = true;
        }
        let off_first = |x: f64, y: f64, ax: &[f64], ay: &[f64]| neq(ax[0], x) || neq(ay[0], y);
        if n == 0
            || ((neq(arrx[n - 1], x_1) || neq(arry[n - 1], y_1))
                && off_first(x_1, y_1, &arrx, &arry))
        {
            arrx[n] = x_1;
            arry[n] = y_1;
            n += 1;
            if (neq(x_1, x_2) || neq(y_1, y_2)) && off_first(x_2, y_2, &arrx, &arry) {
                arrx[n] = x_2;
                arry[n] = y_2;
                n += 1;
            }
        } else if (neq(arrx[n - 1], x_2) || neq(arry[n - 1], y_2))
            && off_first(x_2, y_2, &arrx, &arry)
        {
            arrx[n] = x_2;
            arry[n] = y_2;
            n += 1;
        }
    }
    let arrc = n;
    let mut nv3 = arrc;
    let corner_xs = [min_x, min_x, max_x, max_x];
    let corner_ys = [min_y, max_y, max_y, min_y];
    if nv3 > 1 {
        let mut last_edge: i32 = -1;
        let mut incr = 0usize;
        let edges = [min_x, max_y, max_x, min_y];
        let (mut last_x, mut last_y) = (0.0, 0.0);
        for i in 0..=arrc {
            let k = i % arrc;
            let (cur_x, cur_y) = (arrx[k], arry[k]);
            let mut edge_count = 0;
            let mut edge_index: i32 = -1;
            for (m, &e) in edges.iter().enumerate() {
                let on = if m % 2 == 0 {
                    d_eq(cur_x, e)
                } else {
                    d_eq(cur_y, e)
                };
                if on {
                    edge_count += 1;
                    edge_index = m as i32;
                }
            }
            if edge_count == 0 {
                if i != arrc {
                    incr += 1;
                    t.set(incr, cur_x, cur_y);
                }
                continue;
            }
            if edge_count > 1 {
                edge_index = if d_eq(cur_x, min_x) {
                    if d_eq(cur_y, min_y) { 3 } else { 0 }
                } else if d_eq(cur_y, max_y) {
                    1
                } else {
                    2
                };
            }
            if (last_edge > -1 && edge_index > -1) && last_edge != edge_index {
                let (e, l) = (edge_index, last_edge);
                let jump = if (e == 0 && l == 3) || (e - l == 1) {
                    1
                } else if e % 2 == l % 2 {
                    2
                } else if (e == 3 && l == 0) || (l - e == 1) {
                    3
                } else {
                    0
                };
                if jump > 0 {
                    let mut added = 0;
                    let mut i1 = ((l + 1) % 4) as usize;
                    for _ in 0..jump {
                        let (cx, cy) = (corner_xs[i1], corner_ys[i1]);
                        i1 = (i1 + 1) % 4;
                        if cx == last_x && cy == last_y {
                            continue;
                        }
                        if corner_inside(t, nv1, cx, cy, false)
                            && (incr == 0
                                || ((neq(cx, t.x[incr]) || neq(cy, t.y[incr]))
                                    && (neq(cx, t.x[1]) || neq(cy, t.y[1]))))
                        {
                            incr += 1;
                            t.set(incr, cx, cy);
                            added += 1;
                        }
                    }
                    if jump > 2 && (added == jump && edge_count == 1) {
                        if i != arrc {
                            incr += 1;
                            t.set(incr, cur_x, cur_y);
                        }
                        break;
                    }
                }
            }
            if i != arrc {
                incr += 1;
                t.set(incr, cur_x, cur_y);
            }
            last_edge = edge_index;
            last_x = cur_x;
            last_y = cur_y;
        }
        nv3 = incr;
    } else {
        if nv3 == 1 {
            t.set(1, arrx[0], arry[0]);
        }
        if nv3 == 0 && corner_inside(t, nv1, corner_xs[0], corner_ys[0], true) {
            for i1 in 0..4 {
                t.set(i1 + 1, corner_xs[i1], corner_ys[i1]);
            }
            nv3 = 4;
            intflag = true;
        }
    }
    let status = if nv3 < 3 {
        Some(NO_OVERLAP)
    } else if !intflag {
        Some(FIRST_SURF_WITHIN_SECOND)
    } else {
        None
    };
    (nv3, status)
}

/// `HCX(N1, N) * HCA(N2, M) + HCY(N1, N) * HCB(N2, M) + HCC(N2, M)` in `Int64`: where vertex `n`
/// of `f` lies against side `m` of `g`.
fn side_value(f: &Figure, n: usize, g: &Figure, m: usize) -> i64 {
    f.x[n]
        .wrapping_mul(g.a[m])
        .wrapping_add(f.y[n].wrapping_mul(g.b[m]))
        .wrapping_add(g.c[m])
}

/// `INCLOS`: appends to `t` the vertices of `fa` on or to the right of every side of `fb` that are
/// not already there, and returns how many of `fa`'s vertices are enclosed (`NIN`).
fn inclos(fa: &Figure, fb: &Figure, nv3: &mut usize, t: &mut Temp) -> usize {
    let mut nin = 0;
    'vertices: for n in 0..fa.nv {
        for m in 0..fb.nv {
            let hfunct = side_value(fa, n, fb, m) as f64;
            if hfunct > 0.0 {
                continue 'vertices;
            }
        }
        nin += 1;
        let (x, y) = (fa.x[n] as f64, fa.y[n] as f64);
        if (1..=*nv3).any(|k| t.x[k] == x && t.y[k] == y) {
            continue;
        }
        *nv3 += 1;
        t.set(*nv3, x, y);
    }
    nin
}

/// `INTCPT`: appends to `t` the intercepts of the sides of `f1` with the sides of `f2`, dropping
/// near-duplicates.
fn intcpt(f1: &Figure, f2: &Figure, nv3: &mut usize, t: &mut Temp) {
    for n in 0..f1.nv {
        for m in 0..f2.nv {
            let i1 = side_value(f2, m, f1, n);
            let i2 = side_value(f2, m + 1, f1, n);
            if (i1 >= 0 && i2 >= 0) || (i1 <= 0 && i2 <= 0) {
                continue;
            }
            let i1 = side_value(f1, n, f2, m);
            let i2 = side_value(f1, n + 1, f2, m);
            if (i1 >= 0 && i2 >= 0) || (i1 <= 0 && i2 <= 0) {
                continue;
            }
            let kk = *nv3;
            *nv3 += 1;
            let (a1, b1, c1) = (f1.a[n], f1.b[n], f1.c[n]);
            let (a2, b2, c2) = (f2.a[m], f2.b[m], f2.c[m]);
            let w = b2.wrapping_mul(a1).wrapping_sub(a2.wrapping_mul(b1)) as f64;
            let xu = c2.wrapping_mul(b1).wrapping_sub(b2.wrapping_mul(c1)) as f64 / w;
            let yu = a2.wrapping_mul(c1).wrapping_sub(c2.wrapping_mul(a1)) as f64 / w;
            t.set(*nv3, nint64(xu) as f64, nint64(yu) as f64);
            if kk != 0 {
                let (x, y) = (t.x[*nv3], t.y[*nv3]);
                for k in 1..=kk {
                    if (x - t.x[k]).abs() > 2.0 {
                        continue;
                    }
                    if (y - t.y[k]).abs() > 2.0 {
                        continue;
                    }
                    *nv3 = kk;
                    break;
                }
            }
        }
    }
}

/// `ORDER`: puts the `nv3` vertices in `t` into `f3` in clockwise order, by decreasing slope from
/// the left-most vertex (a bubble sort, as in the C++).
fn order(t: &mut Temp, nv3: usize, f3: &mut Figure) {
    let mut slope = [0.0; CAP];
    let mut xmin = t.x[1];
    let mut yxmin = t.y[1];
    for n in 2..=nv3 {
        if t.x[n] >= xmin {
            continue;
        }
        xmin = t.x[n];
        yxmin = t.y[n];
    }
    let mut p = 1;
    let mut m = 0;
    for n in 1..=nv3 {
        let dx = t.x[n] - xmin;
        let dy = t.y[n] - yxmin;
        if dx.abs() > 0.5 {
            m += 1;
            put(&mut slope, m, dy / dx);
            let (x, y) = (t.x[n], t.y[n]);
            t.set(m, x, y);
        } else if dy > 0.5 {
            p = 2;
            f3.x[1] = nint64(t.x[n]);
            f3.y[1] = nint64(t.y[n]);
        } else if dy < -0.5 {
            f3.x[nv3 - 1] = nint64(t.x[n]);
            f3.y[nv3 - 1] = nint64(t.y[n]);
        } else {
            f3.x[0] = nint64(xmin);
            f3.y[0] = nint64(yxmin);
        }
    }
    if m != 1 {
        for i in 2..=m {
            for j in 1..i {
                if slope[i] <= slope[j] {
                    continue;
                }
                t.x.swap(i, j);
                t.y.swap(i, j);
                slope.swap(i, j);
            }
        }
    }
    for n in 1..=m {
        f3.x[n + p - 1] = nint64(t.x[n]);
        f3.y[n + p - 1] = nint64(t.y[n]);
    }
}

/// The clipping algorithm of `ShadowCalculation`.
#[derive(Clone, Copy, PartialEq)]
enum Algo {
    SutherlandHodgman,
    SlaterBarsky,
    ConvexWeilerAtherton,
}

/// `DeterminePolygonOverlap(1, 2, 3)`: the overlap of `f2` on `f1`, and its status.
fn overlap(f1: &Figure, f2: &Figure, algo: Algo) -> (Figure, i32) {
    let mut f3 = Figure::default();
    let mut status = PARTIAL_OVERLAP;
    let (nv3, mut t) = if algo == Algo::ConvexWeilerAtherton {
        let mut t = Temp::new();
        let mut nv3 = 0;
        if inclos(f1, f2, &mut nv3, &mut t) >= f1.nv {
            status = FIRST_SURF_WITHIN_SECOND;
        } else if inclos(f2, f1, &mut nv3, &mut t) >= f2.nv {
            status = SECOND_SURF_WITHIN_FIRST;
        } else {
            intcpt(f1, f2, &mut nv3, &mut t);
            if nv3 < 3 {
                return (f3, NO_OVERLAP);
            }
        }
        (nv3, t)
    } else {
        let (nv3, set, t) = clippoly(f1, f2, algo == Algo::SlaterBarsky);
        if let Some(s) = set {
            status = s;
        }
        (nv3, t)
    };
    if nv3 < MAX_HCV {
        if algo == Algo::ConvexWeilerAtherton {
            order(&mut t, nv3, &mut f3);
        } else {
            for n in 0..nv3 {
                f3.x[n] = nint64(t.x[n + 1]);
                f3.y[n] = nint64(t.y[n + 1]);
            }
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

/// The overlap of polygon 2 on polygon 1 under clipping algorithm `algo` (see the oracle's shim
/// for `algo` and `which`).
pub fn ClipOverlap(
    which: i32,
    algo: i32,
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
    let algo = match algo {
        0 => Algo::SutherlandHodgman,
        1 => Algo::SlaterBarsky,
        2 => Algo::ConvexWeilerAtherton,
        _ => return f64::NAN,
    };
    if !(3..=4).contains(&nv1) || !(3..=4).contains(&nv2) {
        return f64::NAN;
    }
    // Written as the shim's `!(abs(v) <= lim)` so NaN is refused too.
    let lim = if algo == Algo::ConvexWeilerAtherton {
        10.0
    } else {
        1000.0
    };
    let ok = |v: f64| v.abs() <= lim;
    if !a.iter().chain(&b).all(|&v| ok(v)) {
        return f64::NAN;
    }
    let figure = |c: &[f64; 8], nv: i32, t: f64| {
        let n = nv as usize;
        let xs = [c[0], c[2], c[4], c[6]];
        let ys = [c[1], c[3], c[5], c[7]];
        let mut f = htrans1(&xs[..n], &ys[..n]);
        f.t = t;
        f
    };
    let f1 = figure(&a, nv1, t1);
    let f2 = figure(&b, nv2, t2);
    let (f3, status) = overlap(&f1, &f2, algo);
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
        sqa(w, 0, x, y, s, ox, oy, os)
    }

    fn sqa(w: i32, algo: i32, x: f64, y: f64, s: f64, ox: f64, oy: f64, os: f64) -> f64 {
        ClipOverlap(
            w,
            algo,
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
    fn all_algorithms_agree_on_half_overlapping_squares() {
        for algo in 0..3 {
            assert_eq!(
                sqa(1, algo, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0),
                4.0,
                "algo {algo}"
            );
            assert_eq!(
                sqa(2, algo, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0).abs(),
                0.5,
                "algo {algo}"
            );
        }
    }

    #[test]
    fn rectangle_clip_takes_slater_barsky() {
        let f2 = htrans1(&[0.0, 0.0, 1.0, 1.0], &[0.0, 1.0, 1.0, 0.0]);
        assert!(is_rect(&f2));
        let tri = htrans1(&[0.0, 0.5, 1.0], &[0.0, 1.0, 0.0]);
        assert!(!is_rect(&tri));
        let w = Window {
            max_x: 10.0,
            min_x: 0.0,
            max_y: 10.0,
            min_y: 0.0,
        };
        // A segment crossing the whole window is cut at both sides; one beside it is rejected.
        let (mut x0, mut x1, mut y0, mut y1) = (-5.0, 15.0, 5.0, 5.0);
        assert!(clipline(&mut x0, &mut x1, &mut y0, &mut y1, w));
        assert_eq!((x0, x1, y0, y1), (0.0, 10.0, 5.0, 5.0));
        let (mut x0, mut x1, mut y0, mut y1) = (-5.0, -1.0, 5.0, 5.0);
        assert!(!clipline(&mut x0, &mut x1, &mut y0, &mut y1, w));
    }

    #[test]
    fn weiler_atherton_status_and_guards() {
        assert_eq!(
            sqa(0, 2, 5.0, 5.0, 1.0, 0.0, 0.0, 1.0),
            f64::from(NO_OVERLAP)
        );
        assert!(sqa(0, 2, 20.0, 0.0, 1.0, 0.0, 0.0, 1.0).is_nan());
        assert!(sqa(0, 3, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0).is_nan());
        assert!(sqa(0, -1, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0).is_nan());
    }

    #[test]
    fn rounding_matches_nint64() {
        assert_eq!(nint64(2.5), 3);
        assert_eq!(nint64(-2.5), -3);
        assert_eq!(nint64(-0.4), 0);
        assert_eq!(safe_divide(1.0, 0.0), 1.0e10);
    }
}
