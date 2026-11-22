//! The Rust port of `tarcoggas.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! From EnergyPlus TARCOG: `GASSES90` (ISO 15099 and EN 673 gas mixture properties) and
//! `GassesLow` (low-pressure conduction) from `TARCOGGasses90.cc`, and `EquationsSolver` with
//! `ludcmp` and `lubksb` from `TARCOGCommon.cc`, on the gas table and systems the oracle builds.
//! Arithmetic is in the C++'s order; matrices are 1-based row-major `Array2D`s.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]
#![expect(
    clippy::needless_range_loop,
    reason = "the loops index several arrays as the C++ does"
)]

use std::f64::consts::PI;

const UNIVERSAL_GAS_CONST: f64 = 8314.462175; // `Constant::UniversalGasConst`

/// The oracle's gas table (air, argon, krypton, xenon): a, b, c of conductivity, viscosity and
/// specific heat, and the molecular weight.
const GAS: [[f64; 10]; 4] = [
    [
        2.873e-3, 7.760e-5, 0.0, 3.723e-6, 4.940e-8, 0.0, 1002.7370, 1.2324e-2, 0.0, 28.97,
    ],
    [
        2.285e-3, 5.149e-5, 0.0, 3.379e-6, 6.451e-8, 0.0, 521.9285, 0.0, 0.0, 39.948,
    ],
    [
        9.443e-4, 2.826e-5, 0.0, 2.213e-6, 7.777e-8, 0.0, 248.0907, 0.0, 0.0, 83.8,
    ],
    [
        4.538e-4, 1.723e-5, 0.0, 1.069e-6, 7.414e-8, 0.0, 158.3397, 0.0, 0.0, 131.3,
    ],
];

/// One gas's property fits: conductivity, viscosity and specific heat coefficients, and weight.
#[derive(Clone, Copy)]
struct Gas {
    con: [f64; 3],
    vis: [f64; 3],
    cp: [f64; 3],
    wght: f64,
}

fn gas(k: usize, quad: f64) -> Gas {
    let c = &GAS[k];
    Gas {
        con: [c[0], c[1], c[2] + quad * 1e-9],
        vis: [c[3], c[4], c[5] + quad * 1e-12],
        cp: [c[6], c[7], c[8] + quad * 1e-6],
        wght: c[9],
    }
}

/// `GASSES90` (`TARCOGGasses90.cc:82`): conductivity, viscosity, density, specific heat and
/// Prandtl number, or `Err(nperr)` for a zero fraction. `standard` 0 is ISO 15099, 1 and 2 EN 673.
fn gasses90(
    standard: i32,
    tmean: f64,
    pres: f64,
    gases: &[Gas],
    frct: &[f64],
) -> Result<[f64; 5], i32> {
    const EN_PRESSURE: f64 = 1.0e5;
    const GAS_LAW: f64 = 8314.51;
    let two_sqrt_2 = 2.0 * 2.0_f64.sqrt();
    let n = gases.len();
    let t2 = tmean * tmean;
    let fit = |c: &[f64; 3]| c[0] + c[1] * tmean + c[2] * t2;
    let iso = standard == 0;
    let en673 = standard == 1 || standard == 2;
    let mut fcon = [0.0; 4];
    let mut fvis = [0.0; 4];
    let mut fcp = [0.0; 4];
    let mut fdens = [0.0; 4];
    let mut kprime = [0.0; 4];
    let mut kdblprm = [0.0; 4];
    fcon[0] = fit(&gases[0].con);
    fvis[0] = fit(&gases[0].vis);
    fcp[0] = fit(&gases[0].cp);
    fdens[0] = if en673 {
        EN_PRESSURE * gases[0].wght / (GAS_LAW * tmean)
    } else {
        pres * gases[0].wght / (UNIVERSAL_GAS_CONST * tmean)
    };
    let (con, visc, dens, cp);
    if frct[0] == 1.0 {
        (con, visc, dens, cp) = (fcon[0], fvis[0], fdens[0], fcp[0]);
    } else {
        let mut molmix = 0.0;
        let mut cpmixm = 0.0;
        if iso {
            molmix = frct[0] * gases[0].wght;
            cpmixm = molmix * fcp[0];
            kprime[0] = 3.75 * UNIVERSAL_GAS_CONST / gases[0].wght * fvis[0];
            kdblprm[0] = fcon[0] - kprime[0];
        }
        for i in 1..n {
            if frct[i] == 0.0 {
                return Err(2011);
            }
            fcon[i] = fit(&gases[i].con);
            fvis[i] = fit(&gases[i].vis);
            fcp[i] = fit(&gases[i].cp);
            if en673 {
                fdens[i] = EN_PRESSURE * gases[i].wght / (GAS_LAW * tmean);
            }
            if iso {
                molmix += frct[i] * gases[i].wght;
                cpmixm += frct[i] * fcp[i] * gases[i].wght;
                kprime[i] = 3.75 * UNIVERSAL_GAS_CONST / gases[i].wght * fvis[i];
                kdblprm[i] = fcon[i] - kprime[i];
            }
        }
        if iso {
            let (mut mumix, mut kpmix, mut kdpmix) = (0.0, 0.0, 0.0);
            for i in 0..n {
                let wi = gases[i].wght;
                let (mut mukpdwn, mut kpdown, mut kdpdown) = (1.0, 1.0, 1.0);
                for j in 0..n {
                    let wj = gases[j].wght;
                    let x_pow = (wj / wi).sqrt().sqrt(); // `root_4`
                    let s = 1.0 + (fvis[i] / fvis[j]).sqrt() * x_pow;
                    let phimup = s * s;
                    let downer = two_sqrt_2 * (1.0 + (wi / wj)).sqrt();
                    let s = 1.0 + (kprime[i] / kprime[j]).sqrt() / x_pow;
                    let psiup = s * s;
                    let psiterm =
                        1.0 + 2.41 * (wi - wj) * (wi - 0.142 * wj) / ((wi + wj) * (wi + wj));
                    if i != j {
                        mukpdwn += phimup / downer * frct[j] / frct[i];
                        kpdown += psiup * psiterm / downer * frct[j] / frct[i];
                        kdpdown += psiup / downer * frct[j] / frct[i];
                    }
                }
                mumix += fvis[i] / mukpdwn;
                kpmix += kprime[i] / kpdown;
                kdpmix += kdblprm[i] / kdpdown;
            }
            con = kpmix + kdpmix;
            visc = mumix;
            dens = pres * molmix / (UNIVERSAL_GAS_CONST * tmean);
            cp = if molmix > 0.0 { cpmixm / molmix } else { 0.0 };
        } else {
            let (mut c, mut v, mut d, mut p) = (0.0, 0.0, 0.0, 0.0);
            for i in 0..n {
                c += fcon[i] * frct[i];
                v += fvis[i] * frct[i];
                d += fdens[i] * frct[i];
                p += fcp[i] * frct[i];
            }
            (con, visc, dens, cp) = (c, v, d, p);
        }
    }
    Ok([con, visc, dens, cp, cp * visc / con])
}

pub fn Gasses90(
    which: i32,
    standard: i32,
    tmean: f64,
    pres: f64,
    nmix: i32,
    g1: i32,
    g2: i32,
    g3: i32,
    g4: i32,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    quad: f64,
) -> f64 {
    let Some(n) = usize::try_from(nmix).ok().filter(|n| (1..=4).contains(n)) else {
        return f64::NAN;
    };
    if !(0..=2).contains(&standard) {
        return f64::NAN;
    }
    let mut gases = Vec::with_capacity(n);
    for &g in &[g1, g2, g3, g4][..n] {
        let Some(k) = usize::try_from(g).ok().filter(|g| (1..=4).contains(g)) else {
            return f64::NAN;
        };
        gases.push(gas(k - 1, quad));
    }
    let r = match gasses90(standard, tmean, pres, &gases, &[f1, f2, f3, f4][..n]) {
        Ok(p) => [p[0], p[1], p[2], p[3], p[4], 0.0],
        Err(nperr) => [-1.0, -1.0, -1.0, -1.0, -1.0, f64::from(nperr)],
    };
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `GassesLow` (`TARCOGGasses90.cc:257`): `Err(40)` for gamma 1.
fn gasses_low(tmean: f64, mwght: f64, pressure: f64, gama: f64) -> Result<f64, i32> {
    const ALPHA1: f64 = 0.5;
    const ALPHA2: f64 = 0.5;
    let alpha = ALPHA1 * ALPHA2 / (ALPHA2 + ALPHA1 * (1.0 - ALPHA2));
    if gama == 1.0 {
        return Err(40);
    }
    let b = alpha * (gama + 1.0) / (gama - 1.0)
        * (UNIVERSAL_GAS_CONST / (8.0 * PI * mwght * tmean)).sqrt();
    Ok(b * pressure)
}

pub fn GassesLow(which: i32, tmean: f64, mwght: f64, pressure: f64, gama: f64) -> f64 {
    let (con, nperr) = match gasses_low(tmean, mwght, pressure, gama) {
        Ok(c) => (c, 0),
        Err(e) => (-1.0, e),
    };
    match which {
        0 => con,
        1 => f64::from(nperr),
        _ => f64::NAN,
    }
}

/// An n by n `Array2D<Real64>`, 1-based and row-major.
struct Mat {
    n: usize,
    v: Vec<f64>,
}

impl Mat {
    fn at(&self, r: usize, c: usize) -> f64 {
        self.v[(r - 1) * self.n + (c - 1)]
    }

    fn set(&mut self, r: usize, c: usize, x: f64) {
        self.v[(r - 1) * self.n + (c - 1)] = x;
    }
}

/// Why `ludcmp` stops.
enum LuStop {
    /// The C++'s nperr 13 (a zero row): `b` is left as it was.
    Singular,
    /// A pivot index left at 0 (NaN entries): the C++ then reads outside its arrays.
    Undefined,
}

/// `ludcmp` (`TARCOGCommon.cc:298`) in place: the row permutation, 1-based.
fn ludcmp(a: &mut Mat) -> Result<Vec<usize>, LuStop> {
    const TINY: f64 = 1.0e-20;
    let n = a.n;
    let mut vv = vec![0.0; n + 1];
    let mut indx = vec![0; n + 1];
    let mut imax = 0;
    for i in 1..=n {
        let mut aamax = 0.0;
        for j in 1..=n {
            if a.at(j, i).abs() > aamax {
                aamax = a.at(j, i).abs();
            }
        }
        if aamax == 0.0 {
            return Err(LuStop::Singular);
        }
        vv[i] = 1.0 / aamax;
    }
    for j in 1..=n {
        for i in 1..j {
            let mut sum = a.at(j, i);
            for k in 1..i {
                sum -= a.at(k, i) * a.at(j, k);
            }
            a.set(j, i, sum);
        }
        let mut aamax = 0.0;
        for i in j..=n {
            let mut sum = a.at(j, i);
            for k in 1..j {
                sum -= a.at(k, i) * a.at(j, k);
            }
            a.set(j, i, sum);
            let dum = vv[i] * sum.abs();
            if dum >= aamax {
                imax = i;
                aamax = dum;
            }
        }
        if imax == 0 {
            return Err(LuStop::Undefined);
        }
        if j != imax {
            for k in 1..=n {
                let dum = a.at(k, imax);
                a.set(k, imax, a.at(k, j));
                a.set(k, j, dum);
            }
            vv[imax] = vv[j];
        }
        indx[j] = imax;
        if a.at(j, j) == 0.0 {
            a.set(j, j, TINY);
        }
        if j != n {
            let dum = 1.0 / a.at(j, j);
            for i in j + 1..=n {
                a.set(j, i, a.at(j, i) * dum);
            }
        }
    }
    Ok(indx)
}

/// `lubksb` (`TARCOGCommon.cc:371`); `b` is 1-based (index 0 unused).
fn lubksb(a: &Mat, indx: &[usize], b: &mut [f64]) {
    let n = a.n;
    let mut ii = 0;
    for i in 1..=n {
        let ll = indx[i];
        let mut sum = b[ll];
        b[ll] = b[i];
        if ii != 0 {
            for j in ii..i {
                sum -= a.at(j, i) * b[j];
            }
        } else if sum != 0.0 {
            ii = i;
        }
        b[i] = sum;
    }
    for i in (1..=n).rev() {
        let mut sum = b[i];
        for j in i + 1..=n {
            sum -= a.at(j, i) * b[j];
        }
        b[i] = sum / a.at(i, i);
    }
}

pub fn TarcogSolve(which: i32, n: i32, d: f64, k: f64, o: f64, p: f64) -> f64 {
    let (Ok(n), Ok(w)) = (usize::try_from(n), usize::try_from(which)) else {
        return f64::NAN;
    };
    if !(1..=20).contains(&n) || w > n {
        return f64::NAN;
    }
    let mut a = Mat {
        n,
        v: vec![0.0; n * n],
    };
    let mut b = vec![0.0; n + 1];
    for r in 1..=n {
        for c in 1..=n {
            a.set(
                r,
                c,
                if r == c {
                    d + k * r as f64
                } else {
                    o * (1.7 * r as f64 + 0.9 * c as f64 + p).cos()
                },
            );
        }
        b[r] = (0.5 * r as f64 + p).sin();
    }
    let nperr = match ludcmp(&mut a) {
        Ok(indx) => {
            lubksb(&a, &indx, &mut b);
            0
        }
        Err(LuStop::Singular) => 13,
        Err(LuStop::Undefined) => return f64::NAN,
    };
    if w == n { f64::from(nperr) } else { b[w + 1] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argon_conducts_less_than_air() {
        let air = gasses90(0, 283.15, 101325.0, &[gas(0, 0.0)], &[1.0])
            .unwrap_or_else(|_| unreachable!());
        let argon = gasses90(0, 283.15, 101325.0, &[gas(1, 0.0)], &[1.0])
            .unwrap_or_else(|_| unreachable!());
        assert!(argon[0] < air[0] && (air[0] - 0.0248).abs() < 0.001);
    }

    #[test]
    fn a_zero_fraction_is_error_2011() {
        assert!(matches!(
            gasses90(
                0,
                283.15,
                101325.0,
                &[gas(0, 0.0), gas(1, 0.0)],
                &[0.5, 0.0]
            ),
            Err(2011)
        ));
    }
}
