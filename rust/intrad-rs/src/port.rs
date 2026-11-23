//! The Rust port of `intrad.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Interior radiation exchange from EnergyPlus `HeatBalanceIntRadExchange.cc`:
//! `CalcApproximateViewFactors`, `FixViewFactors` (reciprocity and completeness),
//! `CalcScriptF` with `CalcMatrixInverse` (Hottel's grey interchange), `CalcFMRT` and `CalcFp`
//! (the Carroll MRT method), on the box zone the oracle builds. Matrices are ObjexxFCL `Array2D`s:
//! row-major and 1-based, read and written in the C++'s order. Nothing here panics: every index is
//! inside its matrix by construction.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

pub const STEFAN_BOLTZMANN: f64 = 5.6697e-8; // `Constant::StefanBoltzmann`

/// The most surfaces a box zone has.
pub const MAX: usize = 7;

/// An N by N `Array2D<Real64>`: row-major, 1-based, N at most `MAX`, kept on the stack.
#[derive(Clone, Copy)]
pub struct Mat {
    pub n: usize,
    pub v: [f64; MAX * MAX],
}

impl Mat {
    pub fn new(n: usize, x: f64) -> Self {
        Mat {
            n,
            v: [x; MAX * MAX],
        }
    }

    pub fn at(&self, r: usize, c: usize) -> f64 {
        self.v[(r - 1) * self.n + (c - 1)]
    }

    pub fn set(&mut self, r: usize, c: usize, x: f64) {
        self.v[(r - 1) * self.n + (c - 1)] = x;
    }

    /// ObjexxFCL `sum`: in storage order.
    fn sum(&self) -> f64 {
        self.v[..self.n * self.n].iter().fold(0.0, |s, &x| s + x)
    }

    /// `dst = 0.5 (src + src^T)`, element pairs in the C++'s order, also in place.
    fn reciprocity_from(&mut self, src: &Mat) {
        for i in 1..=self.n {
            for j in i..=self.n {
                let avg = 0.5 * (src.at(i, j) + src.at(j, i));
                self.set(i, j, avg);
                self.set(j, i, avg);
            }
        }
    }

    fn reciprocity(&mut self) {
        let src = *self;
        // In place the C++ reads (i, j) and (j, i) before writing either, and later pairs never
        // read an earlier pair's cells, so averaging from a copy gives the same values.
        self.reciprocity_from(&src);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Class {
    Floor,
    Roof,
    Wall,
    IntMass,
}

/// The oracle's box zone: the surfaces in `mask`, with areas, azimuths, tilts and emissivities.
pub struct Zone {
    pub n: usize,
    pub class: [Class; MAX],
    pub area: [f64; MAX],
    pub azimuth: [f64; MAX],
    pub tilt: [f64; MAX],
    pub emiss: [f64; MAX],
}

pub fn zone(
    mask: i32,
    l: f64,
    w: f64,
    h: f64,
    mass: f64,
    rot: f64,
    e0: f64,
    de: f64,
) -> Option<Zone> {
    if !(1..=127).contains(&mask) {
        return None;
    }
    let a = [l * w, l * w, l * h, w * h, l * h, w * h, mass];
    let az = [rot, rot, rot, rot + 90.0, rot + 180.0, rot + 270.0, rot];
    let tl = [180.0, 0.0, 90.0, 90.0, 90.0, 90.0, 90.0];
    let cls = [
        Class::Floor,
        Class::Roof,
        Class::Wall,
        Class::Wall,
        Class::Wall,
        Class::Wall,
        Class::IntMass,
    ];
    let mut z = Zone {
        n: 0,
        class: [Class::Wall; MAX],
        area: [0.0; MAX],
        azimuth: [0.0; MAX],
        tilt: [0.0; MAX],
        emiss: [0.0; MAX],
    };
    for k in (0..MAX).filter(|k| (mask >> k) & 1 == 1) {
        let i = z.n;
        z.emiss[i] = e0 + de * i as f64;
        z.class[i] = cls[k];
        z.area[i] = a[k];
        z.azimuth[i] = az[k];
        z.tilt[i] = tl[k];
        z.n += 1;
    }
    Some(z)
}

/// `CalcApproximateViewFactors` (`HeatBalanceIntRadExchange.cc:1487`).
pub fn approximate_view_factors(z: &Zone) -> Mat {
    let n = z.n;
    let sees = |i: usize, j: usize| {
        let (ci, cj) = (z.class[i - 1], z.class[j - 1]);
        if i == j || (cj == Class::Floor && ci == Class::Floor) {
            return false;
        }
        let daz = (z.azimuth[i - 1] - z.azimuth[j - 1]).abs();
        cj == Class::IntMass
            || ci == Class::IntMass
            || cj == Class::Floor
            || ci == Class::Floor
            || (daz > 10.0 && daz < 360.0 - 10.0)
            || (z.tilt[i - 1] - z.tilt[j - 1]).abs() > 10.0
    };
    let mut f = Mat::new(n, 0.0);
    for i in 1..=n {
        let zone_area = (1..=n)
            .filter(|&j| sees(i, j))
            .fold(0.0, |s, j| s + z.area[j - 1]);
        for j in (1..=n).filter(|&j| sees(i, j)) {
            if zone_area > 0.0 {
                f.set(j, i, z.area[j - 1] / zone_area);
            }
        }
    }
    f
}

/// What `FixViewFactors` reports besides the matrix.
#[derive(Default)]
pub struct FixChecks {
    pub original: f64,
    pub fixed: f64,
    pub final_: f64,
    pub iterations: i32,
    pub row_sum: f64,
}

/// `FixViewFactors` (`HeatBalanceIntRadExchange.cc:1601`); `None` where the C++ stops with a
/// fatal error.
pub fn fix_view_factors(a: &[f64], f: &mut Mat) -> Option<FixChecks> {
    let n = a.len();
    let nf = n as f64;
    let at = |i: usize| a[i - 1];
    let mut ck = FixChecks {
        original: (f.sum() - nf).abs(),
        ..FixChecks::default()
    };
    let mut fixed_af = *f;
    let largest = a[1..].iter().fold(a[0], |r, &x| if r < x { x } else { r }); // `maxval`
    let sum_a = a.iter().fold(0.0, |s, &x| s + x);
    let dominant = largest > 0.99 * (sum_a - largest) && n > 3;
    if let Some(i) = (1..=n).find(|&i| dominant && largest == at(i)) {
        let self_view = 1.2 * largest / sum_a;
        fixed_af.set(i, i, if 0.9 < self_view { 0.9 } else { self_view });
    }
    let mut af = Mat::new(n, 0.0);
    for i in 1..=n {
        for j in 1..=n {
            af.set(j, i, fixed_af.at(j, i) * at(i));
        }
    }
    fixed_af.reciprocity_from(&af);

    let mut fixed_f = Mat::new(n, 0.0);
    if n <= 3 {
        for i in 1..=n {
            for j in 1..=n {
                fixed_f.set(j, i, fixed_af.at(j, i) / at(i));
            }
        }
        ck.row_sum = fixed_f.sum();
        if ck.row_sum > nf + 0.01 {
            let mut max_row = 0.0;
            for i in 1..=n {
                let row = (1..=n).fold(0.0, |s, j| s + fixed_f.at(i, j));
                if i == 1 || row > max_row {
                    max_row = row;
                }
            }
            if max_row < 1.0 {
                return None; // ShowFatalError
            }
            let inv = 1.0 / max_row;
            fixed_f.v[..n * n].iter_mut().for_each(|x| *x *= inv);
            ck.row_sum = fixed_f.sum();
        }
        ck.fixed = (ck.row_sum - nf).abs();
        ck.final_ = ck.fixed;
        *f = fixed_f;
        return Some(ck);
    }

    let mut convrg_old = 10.0;
    let mut convrg_new;
    loop {
        ck.iterations += 1;
        for i in 1..=n {
            let s = (1..=n).fold(0.0, |s, j| s + fixed_af.at(j, i));
            let row_coefficient = if s.abs() > 1.0e-10 { at(i) / s } else { 1.0 };
            for j in 1..=n {
                fixed_af.set(j, i, fixed_af.at(j, i) * row_coefficient);
            }
        }
        fixed_af.reciprocity();
        for i in 1..=n {
            for j in 1..=n {
                let x = fixed_af.at(j, i) / at(i);
                if x.abs() < 1.0e-10 {
                    fixed_f.set(j, i, 0.0);
                    fixed_af.set(j, i, 0.0);
                } else {
                    fixed_f.set(j, i, x);
                }
            }
        }
        convrg_new = (fixed_f.sum() - nf).abs();
        let converged = (convrg_old - convrg_new).abs() < 0.00001 || convrg_new <= 0.001;
        convrg_old = convrg_new;
        if ck.iterations > 400 {
            fixed_af.reciprocity();
            for i in 1..=n {
                for j in 1..=n {
                    fixed_f.set(j, i, fixed_af.at(j, i) / at(i));
                }
            }
            let s = fixed_f.sum();
            ck.fixed = (s - nf).abs();
            ck.final_ = ck.fixed;
            ck.row_sum = s;
            if ck.fixed.abs() < ck.original.abs() {
                *f = fixed_f;
            }
            return Some(ck);
        }
        if converged {
            break;
        }
    }
    ck.fixed = convrg_new;
    if ck.fixed < ck.original {
        *f = fixed_f;
        ck.final_ = ck.fixed;
    } else {
        ck.final_ = ck.original;
        ck.row_sum = fixed_f.sum();
        if (ck.row_sum - nf).abs() < 0.001 {
            *f = fixed_f;
            ck.final_ = ck.fixed;
        }
    }
    Some(ck)
}

/// `CalcMatrixInverse` (`HeatBalanceIntRadExchange.cc:2002`): Gauss elimination with partial
/// pivoting, as the C++ walks `A` and `I`.
fn matrix_inverse(a: &mut Mat) -> Mat {
    let n = a.n;
    let mut inv = Mat::new(n, 0.0);
    for i in 1..=n {
        inv.set(i, i, 1.0);
    }
    for i in 1..=n {
        let mut i_piv = i;
        let mut a_piv = a.at(i, i).abs();
        for k in i + 1..=n {
            let a_ki = a.at(i, k).abs();
            if a_ki > a_piv {
                i_piv = k;
                a_piv = a_ki;
            }
        }
        if i_piv != i {
            for j in 1..=n {
                let t = a.at(j, i);
                a.set(j, i, a.at(j, i_piv));
                a.set(j, i_piv, t);
                let t = inv.at(j, i);
                inv.set(j, i, inv.at(j, i_piv));
                inv.set(j, i_piv, t);
            }
        }
        let aii_inv = 1.0 / a.at(i, i);
        for k in i + 1..=n {
            let multiplier = a.at(i, k) * aii_inv;
            a.set(i, k, multiplier);
            if multiplier != 0.0 {
                for j in i + 1..=n {
                    a.set(j, k, a.at(j, k) - multiplier * a.at(j, i));
                }
                for j in 1..=n {
                    let iij = inv.at(j, i);
                    if iij != 0.0 {
                        inv.set(j, k, inv.at(j, k) - multiplier * iij);
                    }
                }
            }
        }
    }
    for k in (1..=n).rev() {
        let akk_inv = 1.0 / a.at(k, k);
        for j in 1..=n {
            inv.set(j, k, inv.at(j, k) * akk_inv);
        }
        for i in 1..k {
            let aik = a.at(k, i);
            for j in 1..=n {
                inv.set(j, i, inv.at(j, i) - aik * inv.at(j, k));
            }
        }
    }
    inv
}

/// `CalcScriptF` (`HeatBalanceIntRadExchange.cc:1903`): the transposed ScriptF matrix; clips
/// emissivities above 0.99999 as the C++ does.
pub fn script_f(a: &[f64], f: &Mat, emiss: &mut [f64]) -> Mat {
    let n = a.len();
    let mut c = Mat::new(n, 0.0);
    for j in 1..=n {
        for i in 1..=n {
            c.set(j, i, a[i - 1] * f.at(j, i));
        }
    }
    let mut excite = [0.0; MAX + 1];
    for i in 1..=n {
        if emiss[i - 1] > 0.99999 {
            emiss[i - 1] = 0.99999;
        }
        let e = emiss[i - 1];
        let fac = a[i - 1] / (1.0 - e);
        excite[i] = -e * fac;
        c.set(i, i, c.at(i, i) - fac);
    }
    let mut cinv = matrix_inverse(&mut c);
    for (j, &ex) in excite.iter().enumerate().take(n + 1).skip(1) {
        for i in 1..=n {
            cinv.set(j, i, cinv.at(j, i) * ex);
        }
    }
    let mut sf = Mat::new(n, 0.0);
    for i in 1..=n {
        let e = emiss[i - 1];
        let fac = e / (1.0 - e);
        for j in 1..=n {
            let x = if i == j {
                fac * (cinv.at(j, i) - e)
            } else {
                fac * cinv.at(j, i)
            };
            sf.set(i, j, x);
        }
    }
    sf
}

/// `CalcFMRT` (`HeatBalanceIntRadExchange.cc:2106`).
pub fn fmrt(a: &[f64]) -> [f64; MAX] {
    let n = a.len();
    let mut f = [1.0; MAX];
    let mut sum_af = a.iter().fold(0.0, |s, &x| s + x);
    let mut sum_af_new = sum_af;
    for _ in 0..100 {
        let mut f_change = 0.0;
        let mut errors = false;
        sum_af = sum_af_new;
        sum_af_new = 0.0;
        for s in 0..n {
            let last = f[s];
            f[s] = 1.0 / (1.0 - a[s] * f[s] / sum_af);
            if f[s] > 100.0 {
                errors = true;
                break;
            }
            f_change += (f[s] - last).abs();
            sum_af_new += a[s] * f[s];
        }
        if errors || f_change / (n as f64) < 0.0001 {
            break;
        }
    }
    f
}

fn idx(i: i32, n: usize) -> Option<usize> {
    usize::try_from(i).ok().filter(|&i| (1..=n).contains(&i))
}

/// The approximate (fixed 0) or fixed (fixed 1) view factor F(i, j), or with i 0 and fixed 1 the
/// check values (j 1 to 5), as the oracle defines them.
pub fn IRView(
    fixed: i32,
    i: i32,
    j: i32,
    mask: i32,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
) -> f64 {
    let Some(z) = zone(mask, L, W, H, mass, rot, 0.9, 0.0) else {
        return f64::NAN;
    };
    let n = z.n;
    let mut f = approximate_view_factors(&z);
    if fixed != 0 {
        let Some(ck) = fix_view_factors(&z.area[..z.n], &mut f) else {
            return f64::NAN;
        };
        if i == 0 {
            return match j {
                1 => ck.original,
                2 => ck.fixed,
                3 => ck.final_,
                4 => f64::from(ck.iterations),
                5 => ck.row_sum,
                _ => f64::NAN,
            };
        }
    }
    match (idx(i, n), idx(j, n)) {
        (Some(i), Some(j)) => f.at(i, j),
        _ => f64::NAN,
    }
}

pub fn IRScriptF(
    i: i32,
    j: i32,
    mask: i32,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
    e0: f64,
    de: f64,
) -> f64 {
    let Some(mut z) = zone(mask, L, W, H, mass, rot, e0, de) else {
        return f64::NAN;
    };
    let n = z.n;
    let (Some(i), Some(j)) = (idx(i, n), idx(j, n)) else {
        return f64::NAN;
    };
    let mut f = approximate_view_factors(&z);
    if fix_view_factors(&z.area[..z.n], &mut f).is_none() {
        return f64::NAN;
    }
    script_f(&z.area[..z.n], &f, &mut z.emiss[..z.n]).at(i, j)
}

pub fn IRFmrt(
    fp: i32,
    i: i32,
    mask: i32,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    e0: f64,
    de: f64,
) -> f64 {
    let Some(z) = zone(mask, L, W, H, mass, 0.0, e0, de) else {
        return f64::NAN;
    };
    let Some(i) = idx(i, z.n) else {
        return f64::NAN;
    };
    let f = fmrt(&z.area[..z.n]);
    if fp == 0 {
        return f[i - 1];
    }
    let e = z.emiss[i - 1];
    STEFAN_BOLTZMANN * e / (e / f[i - 1] + 1.0 - e) // `CalcFp`
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_view_factors_of_a_cube_are_complete() {
        let z = zone(63, 3.0, 3.0, 3.0, 0.0, 0.0, 0.9, 0.0).unwrap_or_else(|| unreachable!());
        let mut f = approximate_view_factors(&z);
        let ck = fix_view_factors(&z.area[..z.n], &mut f).unwrap_or_else(|| unreachable!());
        assert!(ck.final_ < 0.001);
        for i in 1..=6 {
            assert!(((1..=6).map(|j| f.at(i, j)).sum::<f64>() - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn inverse_of_a_diagonal_matrix() {
        let mut m = Mat::new(2, 0.0);
        m.set(1, 1, 2.0);
        m.set(2, 2, 4.0);
        let inv = matrix_inverse(&mut m);
        assert_eq!((inv.at(1, 1), inv.at(2, 2), inv.at(1, 2)), (0.5, 0.25, 0.0));
    }
}
