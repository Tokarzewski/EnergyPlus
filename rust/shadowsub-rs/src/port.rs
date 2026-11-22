//! The Rust port of `shadowsub.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! A receiving surface with a window or door, shaded by one or two shadowing surfaces, as
//! EnergyPlus `SolarShading.cc` computes it: `SHADOW` (the shadow projections, the receiving
//! figure), `SHDGSS` with `MULTOL` (the general shadows, as in unit 31), `SHDSBS` (the
//! subsurface's figure, its overlaps with those shadows, its sunlit area taken off the receiving
//! surface's) and `SHDRVL` (the shading of the subsurface by its reveal, for rectangles and
//! triangles), with both clamps. `DeterminePolygonOverlap` works on a figure table, and
//! `HTRANS1` into it leaves each figure's transmittance as the C++ does, which the triangular
//! reveal's overlap reads. Clipping as in units 28 and 29. The oracle's guards (see its shim)
//! return NaN here too.

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

/// `Constant::SmallDistance`: the geometric tolerance, in metres.
const SMALL_DISTANCE: f64 = 1.0e-4;

/// A point or direction in world coordinates.
type V3 = [f64; 3];

/// Vertex arrays of `MaxVerticesPerSurface + 1` entries, 1-based (`XVert`, `XVrt`, `XVC`, `XVS`).
type Verts = [f64; 6];

/// `CTRANS`: the vertices `vs` in the plane of the base surface with local axes `l` about `o`,
/// written 1-based into `x`, `y`, `z`.
fn ctrans(vs: &[V3], o: V3, l: &[V3; 3], x: &mut Verts, y: &mut Verts, z: &mut Verts) {
    for (n, v) in vs.iter().enumerate() {
        let mut xdif = v[0] - o[0];
        let mut ydif = v[1] - o[1];
        let mut zdif = v[2] - o[2];
        if xdif.abs() <= 1.0e-15 {
            xdif = 0.0;
        }
        if ydif.abs() <= 1.0e-15 {
            ydif = 0.0;
        }
        if zdif.abs() <= 1.0e-15 {
            zdif = 0.0;
        }
        x[n + 1] = l[0][0] * xdif + l[0][1] * ydif + l[0][2] * zdif;
        y[n + 1] = l[1][0] * xdif + l[1][1] * ydif + l[1][2] * zdif;
        z[n + 1] = l[2][0] * xdif + l[2][1] * ydif + l[2][2] * zdif;
    }
}

/// `CLIP`: the part of the `nvt`-vertex casting surface on or above the receiving plane (z >= 0).
/// Returns `NVS` (also `NumVertInShadowOrClippedSurface`) and the clipped vertices, 1-based.
fn clip(
    nvt: usize,
    xvt: &mut Verts,
    yvt: &mut Verts,
    zvt: &mut Verts,
) -> (usize, Verts, Verts, Verts) {
    let (mut xvc, mut yvc, mut zvc) = ([0.0; 6], [0.0; 6], [0.0; 6]);
    let mut nabove = 0;
    let mut non = 0;
    for &z in &zvt[1..=nvt] {
        if z > 0.0 {
            nabove += 1;
        } else if z == 0.0 {
            non += 1;
        }
    }
    let nvs;
    if nabove + non == nvt {
        nvs = nvt;
        xvc[1..=nvt].copy_from_slice(&xvt[1..=nvt]);
        yvc[1..=nvt].copy_from_slice(&yvt[1..=nvt]);
        zvc[1..=nvt].copy_from_slice(&zvt[1..=nvt]);
    } else if nabove == 0 {
        nvs = 0;
    } else {
        nvs = nabove + 2;
        let (mut xin, mut yin, mut xout, mut yout) = (0.0, 0.0, 0.0, 0.0);
        let mut next = 0;
        let mut zp = zvt[1];
        xvt[nvt + 1] = xvt[1];
        yvt[nvt + 1] = yvt[1];
        zvt[nvt + 1] = zp;
        for n in 1..=nvt {
            let p = n + 1;
            let zn = zp;
            zp = zvt[p];
            if zn >= 0.0 && zp < 0.0 {
                let fac = 1.0 / (zp - zn);
                xin = (zp * xvt[n] - zn * xvt[p]) * fac;
                yin = (zp * yvt[n] - zn * yvt[p]) * fac;
            } else if zn <= 0.0 && zp > 0.0 {
                next = n + 1;
                let fac = 1.0 / (zp - zn);
                xout = (zp * xvt[n] - zn * xvt[p]) * fac;
                yout = (zp * yvt[n] - zn * yvt[p]) * fac;
            }
        }
        xvc[1] = xout;
        yvc[1] = yout;
        zvc[1] = 0.0;
        xvc[nvs] = xin;
        yvc[nvs] = yin;
        zvc[nvs] = 0.0;
        for n in 1..=nabove {
            if next > nvt {
                next = 1;
            }
            xvc[n + 1] = xvt[next];
            yvc[n + 1] = yvt[next];
            zvc[n + 1] = zvt[next];
            next += 1;
        }
    }
    (nvs, xvc, yvc, zvc)
}

const TOO_MANY_FIGURES: i32 = 6;
/// `MaxHCS` in the oracle: two casters, the subsurface and its reveal need at most 14 figures.
const MAX_HCS: usize = 16;
/// `SHDGSS`'s `int const sunIsUp = DataEnvironment::SunIsUpValue;`: the 0.00001 truncates to 0
/// in the `int`, so a caster casts nothing once the sun is on it at all.
const SUN_IS_UP: i32 = 0;

/// `SHDRVL`'s `RevealStatus` values.
const REVEAL_NONE: i32 = 0;
const ENTIRE_WINDOW_SHADED_BY_REVEAL: i32 = 1;
const WINDOW_SHADED_ONLY_BY_REVEAL: i32 = 2;

/// The figure table `HCX(NS, ..)` and the rest, 1-based (`2 * MaxHCS` figures).
type Figures = [Figure; 2 * MAX_HCS + 1];

/// ObjexxFCL `max(a, b)`: `a < b ? b : a`.
fn fmax(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// ObjexxFCL `min(a, b)`: `a < b ? a : b`.
fn fmin(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// `HTRANS1` into a figure of the table: its vertices from metres, then `HTRANS0`. Like the C++,
/// it leaves the figure's transmittance as it was.
fn htrans1_into(f: &mut Figure, xs: &[f64], ys: &[f64]) {
    for (n, (&x, &y)) in xs.iter().zip(ys).enumerate() {
        f.x[n] = nint64(x * HCMULT);
        f.y[n] = nint64(y * HCMULT);
    }
    htrans0(f, xs.len());
}

/// `DeterminePolygonOverlap(NS1, NS2, NS3)` on the figure table: the overlap of figure `ns2` on
/// figure `ns1`, written to figure `ns3` (which may be `ns2`, as `SHDGSS` calls it: then the sign
/// test reads the overlap's own area). Returns the overlap status.
fn dpo(figs: &mut Figures, ns1: usize, ns2: usize, ns3: usize, slater: bool) -> i32 {
    if ns3 > MAX_HCS {
        return TOO_MANY_FIGURES;
    }
    let mut status = PARTIAL_OVERLAP;
    let (f1, f2) = (figs[ns1], figs[ns2]);
    let (nv3, set, t) = clippoly(&f1, &f2, slater);
    if let Some(s) = set {
        status = s;
    }
    if nv3 >= MAX_HCV {
        return TOO_MANY_VERTICES;
    }
    let f3 = &mut figs[ns3];
    for n in 0..nv3 {
        f3.x[n] = nint64(t.x[n + 1]);
        f3.y[n] = nint64(t.y[n + 1]);
    }
    htrans0(f3, nv3);
    if figs[ns3].area.abs() * HCMULT < figs[ns1].area.abs() {
        return NO_OVERLAP;
    }
    if figs[ns1].area * figs[ns2].area > 0.0 {
        figs[ns3].area = -figs[ns3].area;
    }
    let (t1, t2) = (figs[ns1].t, figs[ns2].t);
    figs[ns3].t = if t2 == 1.0 || t1 == 1.0 {
        t1 * t2
    } else {
        (t1 + t2) - t1 * t2
    };
    status
}

/// `TooManyVertices` or `TooManyFigures`.
fn too_many(status: i32) -> bool {
    status == TOO_MANY_VERTICES || status == TOO_MANY_FIGURES
}

/// The window or door on the receiving surface: its vertices in the receiving plane (`ShadeV`),
/// reveal depth, `NetAreaShadowCalc` and `Area`.
struct SubSurf<'a> {
    xv: &'a [f64],
    yv: &'a [f64],
    reveal: f64,
    net_area: f64,
    area: f64,
}

/// The shading state `SHADOW`, `SHDGSS`, `MULTOL`, `SHDSBS` and `SHDRVL` move along.
struct Shading {
    figs: Figures,
    lochca: usize,
    status: i32,
    fgsshc: usize,
    ngsshc: i64,
    fsbshc: usize,
    nsbshc: i64,
    frvlhc: usize,
    nrvlhc: i64,
    slater: bool,
    xproj: f64,
    yproj: f64,
}

impl Shading {
    /// `MULTOL(NNN, LOC0, NRFIGS)`: overlaps figure `nnn` on figures `loc0 + 1 ..= loc0 + nrfigs`,
    /// keeping each overlap found.
    fn multol(&mut self, nnn: usize, loc0: usize, nrfigs: usize) {
        for i in 1..=nrfigs {
            let ns3 = self.lochca + 1;
            self.status = dpo(&mut self.figs, loc0 + i, nnn, ns3, self.slater);
            if self.status == NO_OVERLAP {
                continue;
            }
            if too_many(self.status) {
                break;
            }
            self.lochca = ns3;
        }
    }

    /// Sums the areas of `n` figures from `first`, each but the first weighted by its opacity.
    fn area_sum(&self, first: usize, n: i64) -> f64 {
        let mut a = self.figs[first].area;
        for j in 2..=n.max(1) as usize {
            let f = &self.figs[first - 1 + j];
            a += f.area * (1.0 - f.t);
        }
        a
    }

    /// `SHDSBS` for the one subsurface: its figure, its overlaps with the general shadows, its
    /// sunlit area (taken off `hts`, the receiving surface's) and `SHDRVL`, then the clamp.
    /// Returns the subsurface's sunlit area, `SurfSunlitFracWithoutReveal` and the reveal status.
    fn shdsbs(&mut self, hts: &mut f64, w: &SubSurf<'_>) -> (f64, f64, i32) {
        self.fsbshc = self.lochca + 1;
        let mut sa: f64;
        // Left at 0 where SHDSBS does not write it.
        let mut frac = 0.0;
        let mut reveal = REVEAL_NONE;
        if !too_many(self.status) && *hts > 0.0 {
            let nvs = w.xv.len();
            let (mut xvs, mut yvs) = ([0.0; 4], [0.0; 4]);
            for n in 0..nvs {
                xvs[n] = w.xv[nvs - 1 - n];
                yvs[n] = w.yv[nvs - 1 - n];
            }
            self.lochca = self.fsbshc;
            let f = &mut self.figs[self.lochca];
            htrans1_into(f, &xvs[..nvs], &yvs[..nvs]);
            f.area = -f.area;
            f.t = 1.0;
            self.nsbshc = (self.lochca - self.fsbshc + 1) as i64;
            if self.ngsshc > 0 {
                self.multol(self.lochca, self.fgsshc - 1, self.ngsshc as usize);
                if !too_many(self.status) {
                    self.nsbshc = (self.lochca - self.fsbshc + 1) as i64;
                }
            }
        }
        if too_many(self.status) || *hts <= 0.0 {
            sa = 0.0;
        } else if self.ngsshc <= 0 || self.nsbshc == 1 {
            sa = self.figs[self.fsbshc].area;
            *hts -= sa;
            frac = sa / w.net_area;
            reveal = self.shdrvl(w, &mut sa);
            if too_many(self.status) {
                sa = 0.0;
            }
        } else {
            sa = self.area_sum(self.fsbshc, self.nsbshc);
            if sa > 0.0 {
                *hts -= sa;
                frac = sa / w.area;
                reveal = self.shdrvl(w, &mut sa);
                if too_many(self.status) {
                    sa = 0.0;
                }
            } else {
                sa = 0.0;
            }
        }
        // An opaque construction (TransDiff 0): no interior overlaps.
        let sa = fmin(fmax(0.0, sa), w.net_area);
        (sa, frac, reveal)
    }

    /// `SHDRVL`: the shading of the subsurface by its reveal, which may set its sunlit area `sa`.
    /// Returns the reveal status.
    fn shdrvl(&mut self, w: &SubSurf<'_>, sa: &mut f64) -> i32 {
        let mut status = REVEAL_NONE;
        let mut set = false;
        let r = w.reveal;
        if r <= 0.0 {
            set = true;
        }
        if !set {
            self.frvlhc = self.lochca + 1;
            self.lochca += 1;
            let nvs = w.xv.len();
            let (xp, yp) = (self.xproj, self.yproj);
            let (mut xvt, mut yvt) = ([0.0; 4], [0.0; 4]);
            if nvs == 4 {
                xvt[0] = w.xv[0] + r * fmax(xp, 0.0);
                xvt[1] = w.xv[1] + r * fmax(xp, 0.0);
                xvt[2] = w.xv[2] + r * fmin(xp, 0.0);
                xvt[3] = w.xv[3] + r * fmin(xp, 0.0);
                yvt[0] = w.yv[0] + r * fmin(yp, 0.0);
                yvt[1] = w.yv[1] + r * fmax(yp, 0.0);
                yvt[2] = w.yv[2] + r * fmax(yp, 0.0);
                yvt[3] = w.yv[3] + r * fmin(yp, 0.0);
                if xvt[1] >= xvt[2] || yvt[1] >= yvt[0] {
                    status = ENTIRE_WINDOW_SHADED_BY_REVEAL;
                    set = true;
                } else {
                    let xs = [xvt[3], xvt[2], xvt[1], xvt[0]];
                    let ys = [yvt[3], yvt[2], yvt[1], yvt[0]];
                    let f = &mut self.figs[self.frvlhc];
                    htrans1_into(f, &xs, &ys);
                    f.area = -f.area;
                    f.t = 1.0;
                    if f.area <= 0.0 {
                        status = ENTIRE_WINDOW_SHADED_BY_REVEAL;
                        set = true;
                    }
                }
            } else if nvs == 3 {
                for n in 0..3 {
                    xvt[n] = w.xv[n] + r * xp;
                    yvt[n] = w.yv[n] + r * yp;
                }
                let ns1 = self.lochca + 1;
                self.lochca = ns1;
                htrans1_into(
                    &mut self.figs[ns1],
                    &[xvt[2], xvt[1], xvt[0]],
                    &[yvt[2], yvt[1], yvt[0]],
                );
                let ns2 = self.lochca + 1;
                self.lochca = ns2;
                htrans1_into(
                    &mut self.figs[ns2],
                    &[w.xv[2], w.xv[1], w.xv[0]],
                    &[w.yv[2], w.yv[1], w.yv[0]],
                );
                self.figs[self.frvlhc].t = 1.0;
                self.status = dpo(&mut self.figs, ns1, ns2, self.frvlhc, self.slater);
                if self.status == NO_OVERLAP {
                    status = ENTIRE_WINDOW_SHADED_BY_REVEAL;
                    set = true;
                }
            }
        }
        if !set {
            if self.nsbshc <= 1 {
                status = WINDOW_SHADED_ONLY_BY_REVEAL;
                set = true;
            } else {
                self.lochca = self.frvlhc;
                self.multol(self.lochca, self.fsbshc, (self.nsbshc - 1) as usize);
                if too_many(self.status) {
                    status = REVEAL_NONE;
                    set = true;
                } else {
                    self.nrvlhc = (self.lochca - self.frvlhc + 1) as i64;
                    if self.nrvlhc <= 1 {
                        status = WINDOW_SHADED_ONLY_BY_REVEAL;
                        set = true;
                    }
                }
            }
        }
        if !set {
            *sa = self.area_sum(self.frvlhc, self.nrvlhc);
        }
        if status == ENTIRE_WINDOW_SHADED_BY_REVEAL || *sa < 0.0 {
            *sa = 0.0;
        } else if status == WINDOW_SHADED_ONLY_BY_REVEAL {
            *sa = self.figs[self.frvlhc].area;
        }
        status
    }
}

/// One shadowing surface: its world vertices, the cosine of the sun on it and its schedule minimum.
struct Caster<'a> {
    verts: &'a [V3],
    cos: f64,
    sched: f64,
}

/// What `SHADOW` leaves for the receiving surface and its subsurface.
struct Shadow {
    sunlit: f64,
    sub_sunlit: f64,
    frac: f64,
    reveal: i32,
    sh: Shading,
}

/// `SHADOW` for receiving surface `recv` (local axes `l` about `o`, net area `area`) with the
/// general shadowing surfaces `casts` (heat transfer surfaces) and the subsurface `win`, sun
/// direction `s`: the shadow projections, the receiving figure, `SHDGSS` with `MULTOL`, `SHDSBS`
/// with `SHDRVL`, and the clamp of the receiving surface's sunlit area.
fn shadow(
    recv: &[V3],
    casts: &[Caster<'_>],
    win: &SubSurf<'_>,
    o: V3,
    l: &[V3; 3],
    s: V3,
    area: f64,
    slater: bool,
) -> Shadow {
    let xs = l[0][0] * s[0] + l[0][1] * s[1] + l[0][2] * s[2];
    let ys = l[1][0] * s[0] + l[1][1] * s[1] + l[1][2] * s[2];
    let zs = l[2][0] * s[0] + l[2][1] * s[1] + l[2][2] * s[2];
    let (mut xproj, mut yproj) = (0.0, 0.0);
    if zs.abs() > SMALL_DISTANCE {
        xproj = xs / zs;
        yproj = ys / zs;
        if xproj.abs() < 1.0e-8 {
            xproj = 0.0;
        }
        if yproj.abs() < 1.0e-8 {
            yproj = 0.0;
        }
    }
    let (mut xv, mut yv, mut zv) = ([0.0; 6], [0.0; 6], [0.0; 6]);
    ctrans(recv, o, l, &mut xv, &mut yv, &mut zv);
    let nvt = recv.len();
    let (mut xr, mut yr) = ([0.0; 4], [0.0; 4]);
    for n in 0..nvt {
        xr[n] = xv[nvt - n];
        yr[n] = yv[nvt - n];
    }
    let mut sh = Shading {
        figs: [Figure::default(); 2 * MAX_HCS + 1],
        lochca: 1,
        status: NO_OVERLAP,
        fgsshc: 0,
        ngsshc: 0,
        fsbshc: 0,
        nsbshc: 0,
        frvlhc: 0,
        nrvlhc: 0,
        slater,
        xproj,
        yproj,
    };
    htrans1_into(&mut sh.figs[1], &xr[..nvt], &yr[..nvt]);
    sh.figs[1].area = -sh.figs[1].area;
    sh.figs[1].t = 1.0;

    // SHDGSS.
    let fgsshc = sh.lochca + 1;
    sh.fgsshc = fgsshc;
    let mut exit_status = -1;
    for c in casts {
        if c.cos > f64::from(SUN_IS_UP) {
            continue;
        }
        let (mut xv, mut yv, mut zv) = ([0.0; 6], [0.0; 6], [0.0; 6]);
        ctrans(c.verts, o, l, &mut xv, &mut yv, &mut zv);
        let (nvs, xvc, yvc, zvc) = clip(c.verts.len(), &mut xv, &mut yv, &mut zv);
        let num_vert = nvs;
        if num_vert <= 2 {
            continue;
        }
        let (mut xvs, mut yvs) = ([0.0; 6], [0.0; 6]);
        for n in 1..=num_vert {
            xvs[n] = xvc[n] - xproj * zvc[n];
            yvs[n] = yvc[n] - yproj * zvc[n];
        }
        let ns3 = sh.lochca + 1;
        let f1 = sh.figs[1];
        let f2 = &mut sh.figs[ns3];
        htrans1_into(f2, &xvs[1..=nvs], &yvs[1..=nvs]);
        // Snap each shadow vertex to the first receiving vertex within 6 units.
        for n in 0..num_vert {
            let (hx, hy) = (f2.x[n], f2.y[n]);
            for m in 0..f1.nv {
                let del_x = f1.x[m].wrapping_sub(hx).wrapping_abs() as f64;
                if del_x > 6.0 {
                    continue;
                }
                let del_y = f1.y[m].wrapping_sub(hy).wrapping_abs() as f64;
                if del_y > 6.0 {
                    continue;
                }
                if del_x > 0.0 {
                    f2.x[n] = f1.x[m];
                }
                if del_y > 0.0 {
                    f2.y[n] = f1.y[m];
                }
                break;
            }
        }
        htrans0(f2, num_vert);
        f2.t = c.sched;
        sh.status = dpo(&mut sh.figs, 1, ns3, ns3, slater);
        if sh.status == FIRST_SURF_WITHIN_SECOND && c.sched > 0.0 {
            sh.status = PARTIAL_OVERLAP;
        }
        let main = sh.status;
        exit_status = main;
        if main == SECOND_SURF_WITHIN_FIRST || main == PARTIAL_OVERLAP {
            sh.lochca = ns3;
            let ngsshc = sh.lochca - fgsshc + 1;
            if ngsshc > 1 {
                sh.multol(sh.lochca, fgsshc - 1, ngsshc - 1);
            }
        } else if main != NO_OVERLAP {
            // FirstSurfWithinSecond, TooManyVertices, TooManyFigures (or anything else) ends it.
            break;
        }
        exit_status = -1;
    }
    let area1 = sh.figs[1].area;
    let mut sunlit = if exit_status == FIRST_SURF_WITHIN_SECOND {
        sh.lochca = fgsshc;
        0.0
    } else if too_many(exit_status) {
        0.25 * area1
    } else if sh.lochca < fgsshc {
        // NGSSHC = LOCHCA - FGSSHC + 1 <= 0: no shadow kept.
        area1
    } else {
        let mut a = area1;
        for f in &sh.figs[fgsshc..=sh.lochca] {
            a += f.area * (1.0 - f.t);
        }
        if a <= 0.0 {
            sh.lochca = fgsshc;
            0.0
        } else {
            a
        }
    };
    sh.ngsshc = sh.lochca as i64 - fgsshc as i64 + 1;

    let (sub_sunlit, frac, reveal) = sh.shdsbs(&mut sunlit, win);
    // ObjexxFCL max(0, a) then min(a, area).
    let sunlit = fmin(fmax(0.0, sunlit), area);
    Shadow {
        sunlit,
        sub_sunlit,
        frac,
        reveal,
        sh,
    }
}

/// The sunlit areas of receiving surface 1 and its subsurface under the shadows of `ncast`
/// casters and the subsurface's reveal (see the oracle's shim for the arguments and `which`).
pub fn ShadowSub(
    which: i32,
    algo: i32,
    nvr: i32,
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
    ncast: i32,
    nv1: i32,
    c1x1: f64,
    c1y1: f64,
    c1z1: f64,
    c1x2: f64,
    c1y2: f64,
    c1z2: f64,
    c1x3: f64,
    c1y3: f64,
    c1z3: f64,
    c1x4: f64,
    c1y4: f64,
    c1z4: f64,
    q1: f64,
    t1: f64,
    nv2: i32,
    c2x1: f64,
    c2y1: f64,
    c2z1: f64,
    c2x2: f64,
    c2y2: f64,
    c2z2: f64,
    c2x3: f64,
    c2y3: f64,
    c2z3: f64,
    c2x4: f64,
    c2y4: f64,
    c2z4: f64,
    q2: f64,
    t2: f64,
    nvw: i32,
    wx1: f64,
    wy1: f64,
    wx2: f64,
    wy2: f64,
    wx3: f64,
    wy3: f64,
    wx4: f64,
    wy4: f64,
    rev: f64,
    warea: f64,
    wgross: f64,
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
    area: f64,
) -> f64 {
    let r = [rx1, ry1, rz1, rx2, ry2, rz2, rx3, ry3, rz3, rx4, ry4, rz4];
    let c = [
        [
            c1x1, c1y1, c1z1, c1x2, c1y2, c1z2, c1x3, c1y3, c1z3, c1x4, c1y4, c1z4,
        ],
        [
            c2x1, c2y1, c2z1, c2x2, c2y2, c2z2, c2x3, c2y3, c2z3, c2x4, c2y4, c2z4,
        ],
    ];
    let nv = [nv1, nv2];
    let q = [q1, q2];
    let t = [t1, t2];
    let w = [wx1, wy1, wx2, wy2, wx3, wy3, wx4, wy4];
    let l = [lxx, lxy, lxz, lyx, lyy, lyz, lzx, lzy, lzz, sx, sy, sz];
    if !(0..=1).contains(&algo)
        || !(3..=4).contains(&nvr)
        || !(1..=2).contains(&ncast)
        || !(3..=4).contains(&nvw)
    {
        return f64::NAN;
    }
    // Written as the shim's `!(abs(v) <= lim)` so NaN is refused too.
    let within = |v: f64, lim: f64| v.abs() <= lim;
    if !(0..12).all(|i| within(r[i], 30.0) && within(l[i], 1.0)) {
        return f64::NAN;
    }
    let ncast = ncast as usize;
    for k in 0..ncast {
        if !(3..=4).contains(&nv[k])
            || !within(q[k], 1.0)
            || !(0.0..=1.0).contains(&t[k])
            || !c[k].iter().all(|&v| within(v, 30.0))
        {
            return f64::NAN;
        }
    }
    if !(within(ox, 30.0) && within(oy, 30.0) && within(oz, 30.0)) || !(0.0..=1.0e6).contains(&area)
    {
        return f64::NAN;
    }
    if !w.iter().all(|&v| within(v, 30.0))
        || !(0.0..=2.0).contains(&rev)
        || !(warea > 0.0 && warea <= 1.0e6)
        || !(0.0..=1.0e6).contains(&wgross)
    {
        return f64::NAN;
    }
    let xs = lxx * sx + lxy * sy + lxz * sz;
    let ys = lyx * sx + lyy * sy + lyz * sz;
    let zs = lzx * sx + lzy * sy + lzz * sz;
    if zs.abs() > SMALL_DISTANCE && !((xs / zs).abs() <= 4.0 && (ys / zs).abs() <= 4.0) {
        return f64::NAN;
    }
    let pts = |v: &[f64; 12]| -> [V3; 4] {
        std::array::from_fn(|i| [v[3 * i], v[3 * i + 1], v[3 * i + 2]])
    };
    let recv = pts(&r);
    let cv = [pts(&c[0]), pts(&c[1])];
    let casts: [Caster<'_>; 2] = std::array::from_fn(|k| Caster {
        verts: &cv[k][..nv[k].clamp(0, 4) as usize],
        cos: q[k],
        sched: t[k],
    });
    let nw = nvw as usize;
    let (wxs, wys) = ([wx1, wx2, wx3, wx4], [wy1, wy2, wy3, wy4]);
    let win = SubSurf {
        xv: &wxs[..nw],
        yv: &wys[..nw],
        reveal: rev,
        net_area: warea,
        area: wgross,
    };
    let axes = [[lxx, lxy, lxz], [lyx, lyy, lyz], [lzx, lzy, lzz]];
    let res = shadow(
        &recv[..nvr as usize],
        &casts[..ncast],
        &win,
        [ox, oy, oz],
        &axes,
        [sx, sy, sz],
        area,
        algo == 1,
    );
    let sh = &res.sh;
    let figs = &sh.figs;
    match which {
        0 => res.sunlit,
        1 => res.sub_sunlit,
        2 => res.frac,
        3 => f64::from(res.reveal),
        4 => f64::from(sh.status),
        5 => sh.lochca as f64,
        6 => sh.ngsshc as f64,
        7 => sh.fgsshc as f64,
        8 => sh.fsbshc as f64,
        9 => sh.nsbshc as f64,
        10 => sh.frvlhc as f64,
        11 => sh.nrvlhc as f64,
        12 => figs[1].area,
        13..=51 => {
            let f = &figs[2 + (which - 13) as usize / 3];
            match (which - 13) % 3 {
                0 => f.nv as f64,
                1 => f.area,
                _ => f.t,
            }
        }
        52..=441 => {
            let f = &figs[2 + (which - 52) as usize / 30];
            let j = ((which - 52) % 30) as usize;
            if j < 15 && j < f.nv {
                f.x[j] as f64
            } else if j >= 15 && j - 15 < f.nv {
                f.y[j - 15] as f64
            } else {
                f64::NAN
            }
        }
        _ => f64::NAN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const H: f64 = std::f64::consts::FRAC_1_SQRT_2;

    /// A 4 m by 3 m south wall at the origin under a sun 45 degrees up from the south, shaded by
    /// a 1 m overhang along its top edge, with a 2 m by 1 m window at (1, `y0`) and reveal `rev`.
    fn wall(which: i32, y0: f64, rev: f64) -> f64 {
        let (x0, x1, y1) = (1.0, 3.0, y0 + 1.0);
        ShadowSub(
            which, 0, 4, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0, 0.0, 3.0, 1, 4, 0.0,
            -1.0, 3.0, 0.0, 0.0, 3.0, 4.0, 0.0, 3.0, 4.0, -1.0, 3.0, -1.0, 0.0, 3, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 4, x0, y1, x0, y0, x1, y0, x1,
            y1, rev, 2.0, 2.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0,
            -H, H, 10.0,
        )
    }

    #[test]
    fn window_below_the_shadow_is_sunlit() {
        // The overhang shades the top metre (y from 2 to 3); a window from 0.5 to 1.5 is clear.
        let w = wall(1, 0.5, 0.0);
        assert!((w - 2.0).abs() < 1e-6, "{w}");
        // The wall keeps its 10 m2 net area less the 4 m2 shadow: 6 m2 sunlit.
        let s = wall(0, 0.5, 0.0);
        assert!((s - 6.0).abs() < 1e-6, "{s}");
    }

    #[test]
    fn window_half_in_the_shadow() {
        // A window from 1.5 to 2.5 has its top half in the shadow.
        let w = wall(1, 1.5, 0.0);
        assert!((w - 1.0).abs() < 1e-6, "{w}");
    }

    #[test]
    fn deep_reveal_shades_the_window() {
        // A 2 m reveal under a 45 degree sun shades all of a 1 m tall window.
        assert_eq!(wall(1, 0.5, 2.0), 0.0);
        assert_eq!(wall(3, 0.5, 2.0), f64::from(ENTIRE_WINDOW_SHADED_BY_REVEAL));
    }

    #[test]
    fn guards_return_nan() {
        assert!(wall(0, 0.5, 3.0).is_nan());
        assert!(wall(0, 40.0, 0.0).is_nan());
        assert!(wall(999, 0.5, 0.0).is_nan());
    }

    #[test]
    fn objexx_max_min() {
        assert_eq!(fmax(0.0, -0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(fmin(1.0, 2.0), 1.0);
        assert_eq!(nint64(-2.5), -3);
        assert_eq!(safe_divide(1.0, 0.0), 1.0e10);
    }
}
