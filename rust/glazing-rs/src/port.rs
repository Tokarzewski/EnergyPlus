//! The Rust port of `glazing.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Glazing optics and small solvers from EnergyPlus `WindowManager.cc`: `TransAndReflAtPhi`
//! (Fresnel angular dependence, its clear and bronze fallbacks and the simple glazing curves),
//! `W5LsqFit`, `DiffuseAverage`, `ViewFac` (blind slats), `InvertMatrix` with `LUdecomposition`
//! and `LUsolution`, and `Interpolate`, in the C++'s operation order. Matrices keep the C++'s
//! `[column][row]`-style indexing as written. The interpolation helpers come from `leaf-rs`.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]
#![expect(
    clippy::needless_range_loop,
    reason = "the loops index several arrays as the C++ does"
)]

use leaf_rs::port::{InterpolateBetweenFourValues as four, InterpolateBetweenTwoValues as two};

const NUM_PHIS: usize = 10;
const MAX_POLY_COEF: usize = 6;
const MAX_ARRAY_SIZE: usize = 10; // `2 * maxGlassLayers`
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
/// `Window::cosPhis`, as the header spells them.
#[expect(
    clippy::excessive_precision,
    reason = "the header's literals, kept as written"
)]
const COS_PHIS: [f64; NUM_PHIS] = [
    1.0,
    0.98480775301220802,
    0.93969262078590842,
    0.86602540378443871,
    0.76604444311897812,
    0.64278760968653936,
    0.50000000000000011,
    0.34202014332566882,
    0.17364817766693041,
    0.0,
];

/// `std::clamp`: `v < lo ? lo : hi < v ? hi : v`.
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    if v < lo {
        lo
    } else if hi < v {
        hi
    } else {
        v
    }
}

/// `std::max` and ObjexxFCL `max`: `a < b ? b : a`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The simple glazing model's angular transmittance and reflectance multipliers.
fn simple_glazing(cs: f64, shgc: f64, u: f64) -> (f64, f64) {
    #[expect(
        clippy::approx_constant,
        reason = "3.14 is curve F's coefficient, not pi"
    )]
    const T: [[f64; 4]; 10] = [
        [3.36, -3.85, 1.49, 0.01],
        [2.83, -2.42, 0.04, 0.55],
        [2.45, -1.58, -0.64, 0.77],
        [2.85, -2.58, 0.40, 0.35],
        [1.51, 2.49, -5.87, 2.88],
        [1.21, 3.14, -6.37, 3.03],
        [1.09, 3.54, -6.84, 3.23],
        [0.98, 3.83, -7.13, 3.33],
        [0.79, 3.93, -6.86, 3.15],
        [0.08, 6.02, -8.84, 3.74],
    ];
    const R: [[f64; 4]; 10] = [
        [-0.70, 2.57, -3.20, 1.33],
        [-1.87, 6.50, -7.86, 3.23],
        [-2.52, 8.40, -9.86, 3.99],
        [-1.85, 6.40, -7.64, 3.11],
        [-1.57, 5.60, -6.82, 2.80],
        [-3.15, 10.98, -13.14, 5.32],
        [-3.25, 11.32, -13.54, 5.49],
        [-3.39, 11.70, -13.94, 5.64],
        [-4.06, 13.55, -15.74, 6.27],
        [-4.35, 14.27, -16.32, 6.39],
    ];
    let (c2, c3) = (cs * cs, cs * cs * cs);
    let c4 = {
        let t = cs * cs;
        t * t
    };
    let poly = |c0: f64, k: &[f64; 4]| c0 + k[0] * cs + k[1] * c2 + k[2] * c3 + k[3] * c4;
    let mut t = [0.0; 10];
    let mut r = [0.0; 10];
    for i in 0..10 {
        t[i] = poly(0.00, &T[i]);
        r[i] = poly(1.00, &R[i]) - t[i];
    }
    let [a, _b, _c, d, e, f, g, h, i, j] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map(|k| (t[k], r[k]));
    let fghi = (
        (t[5] + t[6] + t[7] + t[8]) / 4.0,
        (r[5] + r[6] + r[7] + r[8]) / 4.0,
    );
    let fh = ((t[5] + t[7]) / 2.0, (r[5] + r[7]) / 2.0);
    let bdcd = (
        (t[1] + t[3] + t[2] + t[3]) / 4.0,
        (r[1] + r[3] + r[2] + r[3]) / 4.0,
    );
    let _ = (f, g, h, i);
    // Two- and four-point interpolation of a (transmittance, reflectance) pair.
    let i2 = |x: f64, x0: f64, x1: f64, p: (f64, f64), q: (f64, f64)| {
        (two(x, x0, x1, p.0, q.0), two(x, x0, x1, p.1, q.1))
    };
    let i4 = |x1: f64,
              x2: f64,
              y1: f64,
              y2: f64,
              p: (f64, f64),
              q: (f64, f64),
              v: (f64, f64),
              w: (f64, f64)| {
        (
            four(u, shgc, x1, x2, y1, y2, p.0, q.0, v.0, w.0),
            four(u, shgc, x1, x2, y1, y2, p.1, q.1, v.1, w.1),
        )
    };
    let zero = (0.0, 0.0);
    if u < 1.4195 {
        if shgc > 0.45 {
            e
        } else if (0.35..=0.45).contains(&shgc) {
            i2(shgc, 0.35, 0.45, j, e)
        } else if shgc < 0.35 {
            j
        } else {
            zero
        }
    } else if (1.4195..=1.7034).contains(&u) {
        let (u0, u1) = (1.4195, 1.7034);
        if shgc > 0.55 {
            e
        } else if 0.5 < shgc && shgc <= 0.55 {
            i4(u0, u1, 0.50, 0.55, e, e, fghi, e)
        } else if 0.45 < shgc && shgc <= 0.5 {
            i2(u, u0, u1, e, fghi)
        } else if 0.35 < shgc && shgc <= 0.45 {
            i4(u0, u1, 0.35, 0.45, j, e, fghi, fghi)
        } else if 0.3 < shgc && shgc <= 0.35 {
            i2(u, u0, u1, j, fghi)
        } else if 0.25 < shgc && shgc <= 0.3 {
            i4(u0, u1, 0.25, 0.3, j, j, fh, fghi)
        } else if shgc <= 0.25 {
            i2(u, u0, u1, j, fh)
        } else {
            zero
        }
    } else if 1.7034 < u && u < 3.4068 {
        if shgc > 0.55 {
            e
        } else if (0.5..=0.55).contains(&shgc) {
            i2(shgc, 0.5, 0.55, fghi, e)
        } else if 0.3 < shgc && shgc < 0.5 {
            fghi
        } else if (0.25..=0.3).contains(&shgc) {
            i2(shgc, 0.25, 0.30, fh, fghi)
        } else if shgc < 0.25 {
            fh
        } else {
            zero
        }
    } else if (3.4068..=4.5424).contains(&u) {
        let (u0, u1) = (3.4068, 4.5424);
        if shgc > 0.65 {
            i2(u, u0, u1, e, a)
        } else if 0.6 < shgc && shgc <= 0.65 {
            i4(u0, u1, 0.6, 0.65, e, e, bdcd, a)
        } else if 0.55 < shgc && shgc <= 0.6 {
            i2(u, u0, u1, e, bdcd)
        } else if 0.5 < shgc && shgc <= 0.55 {
            i4(u0, u1, 0.5, 0.55, fghi, e, bdcd, bdcd)
        } else if 0.45 < shgc && shgc <= 0.5 {
            i2(u, u0, u1, fghi, bdcd)
        } else if 0.3 < shgc && shgc <= 0.45 {
            i4(u0, u1, 0.3, 0.45, fghi, fghi, d, bdcd)
        } else if 0.25 < shgc && shgc <= 0.3 {
            i4(u0, u1, 0.25, 0.3, fh, fghi, d, d)
        } else if shgc <= 0.25 {
            i2(u, u0, u1, fh, d)
        } else {
            zero
        }
    } else if shgc > 0.65 {
        a // U above 4.5424 (the oracle keeps NaN away from these branches)
    } else if (0.6..=0.65).contains(&shgc) {
        i2(shgc, 0.6, 0.65, bdcd, a)
    } else if 0.45 < shgc && shgc < 0.6 {
        bdcd
    } else if (0.3..=0.45).contains(&shgc) {
        i2(shgc, 0.3, 0.45, d, bdcd)
    } else {
        d
    }
}

/// `TransAndReflAtPhi` (`WindowManager.cc:4879`): transmittance, front and back reflectance at
/// incidence cosine `cs`.
fn trans_and_refl(
    cs: f64,
    tf0: f64,
    rf0: f64,
    rb0: f64,
    simple: bool,
    shgc: f64,
    u: f64,
) -> [f64; 3] {
    let sq = |x: f64| x * x;
    if simple {
        let (mut t, mut r) = simple_glazing(cs, shgc, u);
        if cs == 1.0 {
            (t, r) = (1.0, 0.0);
        }
        let tfp = clamp(tf0 * t, 0.0, 1.0);
        let rfp = clamp(rf0 * (1.0 - r) + r, 0.0, max(0.9999 - tfp, 0.0));
        return [tfp, rfp, rfp];
    }
    if tf0 <= 0.0 {
        return [0.0, rf0, rb0];
    }
    let betaf = sq(tf0) - sq(rf0) + 2.0 * rf0 + 1.0;
    let betab = sq(tf0) - sq(rb0) + 2.0 * rb0 + 1.0;
    let r0f = (betaf - (sq(betaf) - 4.0 * (2.0 - rf0) * rf0).sqrt()) / (2.0 * (2.0 - rf0));
    let r0b = (betab - (sq(betab) - 4.0 * (2.0 - rb0) * rb0).sqrt()) / (2.0 * (2.0 - rb0));
    let testval = if (r0f - r0b).abs() != 0.0 {
        (r0f - r0b).abs() / (r0f + r0b)
    } else {
        0.0
    };
    if testval < 0.001 {
        let ln_or_zero = |num: f64, den: f64| if num != 0.0 { (num / den).ln() } else { 0.0 };
        let abf = ln_or_zero(r0f * tf0, rf0 - r0f);
        let abb = ln_or_zero(r0b * tf0, rb0 - r0b);
        let ngf = (1.0 + r0f.sqrt()) / (1.0 - r0f.sqrt());
        let ngb = (1.0 + r0b.sqrt()) / (1.0 - r0b.sqrt());
        let cgf = (1.0 - (1.0 - cs * cs) / sq(ngf)).sqrt();
        let cgb = (1.0 - (1.0 - cs * cs) / sq(ngb)).sqrt();
        let refl = |num: f64, den: f64| if num != 0.0 { sq(num / den) } else { 0.0 };
        let rpf1 = refl(ngf * cs - cgf, ngf * cs + cgf);
        let rpf2 = refl(ngf * cgf - cs, ngf * cgf + cs);
        let rpb1 = refl(ngb * cs - cgb, ngb * cs + cgb);
        let rpb2 = refl(ngb * cgb - cs, ngb * cgb + cs);
        let (tpf1, tpf2) = (1.0 - rpf1, 1.0 - rpf2);
        let e1 = if cgf != 0.0 { (-abf / cgf).exp() } else { 0.0 };
        let e2 = if cgf != 0.0 {
            (-2.0 * abf / cgf).exp()
        } else {
            0.0
        };
        let tfp1 = if tpf1 != 0.0 {
            sq(tpf1) * e1 / (1.0 - sq(rpf1) * e2)
        } else {
            0.0
        };
        let rfp1 = rpf1 * (1.0 + tfp1 * e1);
        let tfp2 = if tpf2 != 0.0 {
            sq(tpf2) * e1 / (1.0 - sq(rpf2) * e2)
        } else {
            0.0
        };
        let rfp2 = rpf2 * (1.0 + tfp2 * e1);
        let eb = if -abb != 0.0 { (-abb / cgb).exp() } else { 0.0 };
        let rbp1 = rpb1 * (1.0 + tfp1 * eb);
        let rbp2 = rpb2 * (1.0 + tfp2 * eb);
        return [
            0.5 * (tfp1 + tfp2),
            0.5 * (rfp1 + rfp2),
            0.5 * (rbp1 + rbp2),
        ];
    }
    // Unequal front and back reflectance: the clear (tf0 > 0.645) or bronze glass curves.
    let (t, r) = if cs > 0.999 {
        (1.0, 0.0)
    } else if cs < 0.001 {
        (0.0, 1.0)
    } else if tf0 > 0.645 {
        let t = -0.0015 + (3.355 + (-3.840 + (1.460 + 0.0288 * cs) * cs) * cs) * cs;
        (
            t,
            0.999 + (-0.563 + (2.043 + (-2.532 + 1.054 * cs) * cs) * cs) * cs - t,
        )
    } else {
        let t = -0.002 + (2.813 + (-2.341 + (-0.05725 + 0.599 * cs) * cs) * cs) * cs;
        (
            t,
            0.997 + (-1.868 + (6.513 + (-7.862 + 3.225 * cs) * cs) * cs) * cs - t,
        )
    };
    [tf0 * t, rf0 * (1.0 - r) + r, rb0 * (1.0 - r) + r]
}

pub fn GlassPhi(
    which: i32,
    cs: f64,
    tf0: f64,
    rf0: f64,
    rb0: f64,
    simple: i32,
    shgc: f64,
    u: f64,
) -> f64 {
    if simple != 0 && (shgc.is_nan() || u.is_nan()) {
        return f64::NAN;
    }
    let r = trans_and_refl(cs, tf0, rf0, rb0, simple != 0, shgc, u);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `W5LsqFit` (`WindowManager.cc:5431`): a fifth-order polynomial least squares fit.
fn lsq_fit(ivars: &[f64; NUM_PHIS], dvars: &[f64; NUM_PHIS]) -> [f64; MAX_POLY_COEF] {
    let mut a = [[0.0; MAX_POLY_COEF]; MAX_POLY_COEF];
    let mut b = [0.0; MAX_POLY_COEF];
    let mut d = [[0.0; NUM_PHIS]; MAX_POLY_COEF];
    d[0] = *ivars;
    for i in 1..MAX_POLY_COEF {
        for m in 0..NUM_PHIS {
            d[i][m] = d[i - 1][m] * ivars[m];
        }
    }
    for i in 0..MAX_POLY_COEF {
        b[i] = (0..NUM_PHIS).fold(0.0, |s, m| s + dvars[m] * d[i][m]);
        for j in 0..MAX_POLY_COEF {
            let s2 = (0..NUM_PHIS).fold(0.0, |s, m| s + d[i][m] * d[j][m]);
            a[j][i] = s2;
            a[i][j] = s2;
        }
    }
    for k in 0..MAX_POLY_COEF - 1 {
        for i in k + 1..MAX_POLY_COEF {
            let acon = a[k][i] / a[k][k];
            b[i] -= b[k] * acon;
            for j in k..MAX_POLY_COEF {
                a[j][i] -= a[j][k] * acon;
            }
        }
    }
    let mut c = [0.0; MAX_POLY_COEF];
    c[MAX_POLY_COEF - 1] = b[MAX_POLY_COEF - 1] / a[MAX_POLY_COEF - 1][MAX_POLY_COEF - 1];
    for l in (0..MAX_POLY_COEF - 1).rev() {
        let s = (l + 1..MAX_POLY_COEF).fold(0.0, |s, j| s + a[j][l] * c[j]);
        c[l] = (b[l] - s) / a[l][l];
    }
    c
}

/// `DiffuseAverage` (`WindowManager.cc:5512`).
fn diffuse_average(props: &[f64; NUM_PHIS]) -> f64 {
    const D_PHI_R: f64 = 10.0 * DEG_TO_RAD;
    let mut avg = 0.0;
    for i in 0..NUM_PHIS - 1 {
        avg += 0.5
            * D_PHI_R
            * (props[i] * (2.0 * i as f64 * D_PHI_R).sin()
                + props[i + 1] * (2.0 * (i + 1) as f64 * D_PHI_R).sin());
    }
    if avg < 0.0 { 0.0 } else { avg }
}

pub fn GlassFit(which: i32, tf0: f64, rf0: f64, rb0: f64) -> f64 {
    let mut t = [0.0; NUM_PHIS];
    let mut r = [0.0; NUM_PHIS];
    for i in 0..NUM_PHIS {
        let p = trans_and_refl(COS_PHIS[i], tf0, rf0, rb0, false, 0.0, 0.0);
        (t[i], r[i]) = (p[0], p[1]);
    }
    match which {
        0..=5 => lsq_fit(&COS_PHIS, &t)[which as usize],
        6 => diffuse_average(&t),
        7 => diffuse_average(&r),
        _ => f64::NAN,
    }
}

/// `ViewFac` (`WindowManager.cc:8084`): view factors between the six sections of a slat gap.
fn view_fac(s: f64, h: f64, phib: f64, phis: f64) -> [[f64; 6]; 6] {
    let h2 = h * h;
    let ht = 2.0 * h;
    let mut co = phis.cos();
    if co.abs() < 0.001 {
        co = 0.0;
    }
    let w = if co != 0.0 {
        s * (phib - phis).cos() / co
    } else {
        ht
    };
    let mut l3 = s * h / w.abs();
    if l3 > s {
        l3 = s;
    }
    let l5 = s - l3;
    let a = ht * phib.cos();
    let d = |l: f64, sign: f64| max(0.0, l * l + h2 + sign * a * l).sqrt();
    // `+ a s` and `- a s`: `sign * a * l` is exact for sign 1 or -1, so `x + (-a s)` is `x - a s`.
    let (d1, d2, d3, d4, d5, d6) = (
        d(s, 1.0),
        d(s, -1.0),
        d(l3, 1.0),
        d(l3, -1.0),
        d(l5, -1.0),
        d(l5, 1.0),
    );
    let mut f = [[0.0; 6]; 6];
    f[1][0] = (d1 + d2 - 2.0 * s) / ht;
    f[2][0] = (h + l3 - d3) / ht;
    f[3][0] = (h + l3 - d4) / ht;
    f[4][0] = (l5 + d3 - d1) / ht;
    f[5][0] = (l5 + d4 - d2) / ht;
    f[2][1] = (l3 + d5 - d2) / ht;
    f[3][1] = (l3 + d6 - d1) / ht;
    f[4][1] = (h + l5 - d5) / ht;
    f[5][1] = (h + l5 - d6) / ht;
    f[3][2] = (d3 + d4 - ht) / (2.0 * l3);
    f[5][2] = (d2 + h - d4 - d5) / (2.0 * l3);
    f[4][3] = (d1 + h - d3 - d6) / (2.0 * l3);
    if l5 > 0.0 {
        f[5][4] = (d5 + d6 - ht) / (2.0 * l5);
    }
    let len = [h, h, l3, l3, l5, l5];
    for i in 1..6 {
        for j in 0..i {
            f[j][i] = if len[i] > 0.0 {
                f[i][j] * len[j] / len[i]
            } else {
                0.0
            };
        }
    }
    f
}

pub fn SlatViewFac(i: i32, j: i32, s: f64, h: f64, phib: f64, phis: f64) -> f64 {
    match (usize::try_from(i), usize::try_from(j)) {
        (Ok(i @ 0..=5), Ok(j @ 0..=5)) => view_fac(s, h, phib, phis)[i][j],
        _ => f64::NAN,
    }
}

type Matrix = [[f64; MAX_ARRAY_SIZE]; MAX_ARRAY_SIZE];

/// `LUdecomposition` (`WindowManager.cc:4236`) in place; `None` for a singular matrix (the C++'s
/// fatal error).
fn lu_decomposition(a: &mut Matrix, n: usize) -> Option<[usize; MAX_ARRAY_SIZE]> {
    let mut vv = [0.0; MAX_ARRAY_SIZE];
    let mut indx = [0; MAX_ARRAY_SIZE];
    let mut imax = 0;
    for i in 0..n {
        let mut aamax = 0.0;
        for j in 0..n {
            if a[j][i].abs() > aamax {
                aamax = a[j][i].abs();
            }
        }
        if aamax == 0.0 {
            return None;
        }
        vv[i] = 1.0 / aamax;
    }
    for j in 0..n {
        for i in 0..j {
            let mut sum = a[j][i];
            for k in 0..i {
                sum -= a[k][i] * a[j][k];
            }
            a[j][i] = sum;
        }
        let mut aamax = 0.0;
        for i in j..n {
            let mut sum = a[j][i];
            for k in 0..j {
                sum -= a[k][i] * a[j][k];
            }
            a[j][i] = sum;
            let dum = vv[i] * sum.abs();
            if dum >= aamax {
                imax = i;
                aamax = dum;
            }
        }
        if j != imax {
            for row in a.iter_mut().take(n) {
                row.swap(imax, j);
            }
            vv[imax] = vv[j];
        }
        indx[j] = imax;
        if a[j][j] == 0.0 {
            a[j][j] = f64::EPSILON; // `Constant::rTinyValue`
        }
        if j != n - 1 {
            let dum = 1.0 / a[j][j];
            for i in j + 1..n {
                a[j][i] *= dum;
            }
        }
    }
    Some(indx)
}

/// `LUsolution` (`WindowManager.cc:4319`).
fn lu_solution(
    a: &Matrix,
    n: usize,
    indx: &[usize; MAX_ARRAY_SIZE],
    b: &mut [f64; MAX_ARRAY_SIZE],
) {
    let mut ii = None;
    for i in 0..n {
        let ll = indx[i];
        let mut sum = b[ll];
        b[ll] = b[i];
        if let Some(ii) = ii {
            for j in ii..i {
                sum -= a[j][i] * b[j];
            }
        } else if sum != 0.0 {
            ii = Some(i);
        }
        b[i] = sum;
    }
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in i + 1..n {
            sum -= a[j][i] * b[j];
        }
        b[i] = sum / a[i][i];
    }
}

/// `InvertMatrix` (`WindowManager.cc:8181`).
fn invert_matrix(a: &mut Matrix, n: usize) -> Option<Matrix> {
    let indx = lu_decomposition(a, n)?;
    let mut y = [[0.0; MAX_ARRAY_SIZE]; MAX_ARRAY_SIZE];
    for j in 0..n {
        let mut tmp = [0.0; MAX_ARRAY_SIZE];
        tmp[j] = 1.0;
        lu_solution(a, n, &indx, &mut tmp);
        y[j][..n].copy_from_slice(&tmp[..n]);
    }
    Some(y)
}

pub fn MatInverse(which: i32, n: i32, d: f64, k: f64, o: f64, p: f64) -> f64 {
    let (Ok(n), Ok(w)) = (usize::try_from(n), usize::try_from(which)) else {
        return f64::NAN;
    };
    if !(1..=MAX_ARRAY_SIZE).contains(&n) || w >= n * n {
        return f64::NAN;
    }
    let mut a = [[0.0; MAX_ARRAY_SIZE]; MAX_ARRAY_SIZE];
    for r in 0..n {
        for c in 0..n {
            a[r][c] = if r == c {
                d + k * r as f64
            } else {
                o * (1.7 * r as f64 + 0.9 * c as f64 + p).cos()
            };
        }
    }
    invert_matrix(&mut a, n).map_or(f64::NAN, |y| y[w / n][w % n])
}

/// `Interpolate` (`WindowManager.cc:2025`).
pub fn Interp(
    xin: f64,
    npts: i32,
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    x4: f64,
    y0: f64,
    y1: f64,
    y2: f64,
    y3: f64,
    y4: f64,
) -> f64 {
    let Some(n) = usize::try_from(npts).ok().filter(|n| (1..=5).contains(n)) else {
        return f64::NAN;
    };
    let (x, y) = ([x0, x1, x2, x3, x4], [y0, y1, y2, y3, y4]);
    for i in 0..n {
        if xin <= x[i] {
            if i == 0 {
                return y[0];
            }
            return y[i - 1] + (y[i] - y[i - 1]) * (xin - x[i - 1]) / (x[i] - x[i - 1]);
        }
    }
    y[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_glass_at_normal_incidence_keeps_its_properties() {
        let [t, r, _] = trans_and_refl(1.0, 0.837, 0.075, 0.075, false, 0.0, 0.0);
        assert!((t - 0.837).abs() < 1e-3 && (r - 0.075).abs() < 1e-3);
    }

    #[test]
    fn inverse_times_matrix_is_the_identity() {
        let mut a = [[0.0; MAX_ARRAY_SIZE]; MAX_ARRAY_SIZE];
        a[0] = [2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        a[1] = [1.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let orig = a;
        let y = invert_matrix(&mut a, 2).unwrap_or_else(|| unreachable!());
        for i in 0..2 {
            for j in 0..2 {
                let s: f64 = (0..2).map(|k| orig[i][k] * y[k][j]).sum();
                assert!((s - if i == j { 1.0 } else { 0.0 }).abs() < 1e-12);
            }
        }
    }
}
