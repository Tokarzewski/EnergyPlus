//! The Rust port of `wingas.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Window gap gas routines from EnergyPlus `WindowManager.cc`: `WindowGasConductance`,
//! `WindowGasPropertiesAtTemp` and `NusseltNumber`, on the gap the oracle sets up from
//! `Material::gases`. Arithmetic is in the C++'s order, including the places where the two gas
//! routines group the same formula differently.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

const PRES: f64 = 1.0e5; // gap gas pressure (Pa)
const GAS_LAW: f64 = 8314.51; // molar gas constant (J/kMol-K)
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

/// `Material::gases` 1 to 4 (air, argon, krypton, xenon): conductivity, viscosity and specific
/// heat coefficients, and the molecular weight.
type GasCoeffs = ([f64; 3], [f64; 3], [f64; 3], f64);

const GASES: [GasCoeffs; 4] = [
    (
        [2.873e-3, 7.760e-5, 0.0],
        [3.723e-6, 4.940e-8, 0.0],
        [1002.737, 1.2324e-2, 0.0],
        28.97,
    ),
    (
        [2.285e-3, 5.149e-5, 0.0],
        [3.379e-6, 6.451e-8, 0.0],
        [521.929, 0.0, 0.0],
        39.948,
    ),
    (
        [9.443e-4, 2.826e-5, 0.0],
        [2.213e-6, 7.777e-8, 0.0],
        [248.091, 0.0, 0.0],
        83.8,
    ),
    (
        [4.538e-4, 1.723e-5, 0.0],
        [1.069e-6, 7.414e-8, 0.0],
        [158.340, 0.0, 0.0],
        131.3,
    ),
];

#[derive(Clone, Copy)]
struct Gas {
    con: [f64; 3],
    vis: [f64; 3],
    cp: [f64; 3],
    wght: f64,
}

/// A gap's gases and their fractions.
type Mixture = (Vec<Gas>, Vec<f64>);

/// The gap's gases and fractions as the oracle sets them up, or `None` for a bad count or gas.
fn mixture(nmix: i32, g: [i32; 4], f: [f64; 4], quad: f64) -> Option<Mixture> {
    let n = usize::try_from(nmix).ok().filter(|n| (1..=4).contains(n))?;
    let mut gases = Vec::with_capacity(n);
    for &k in &g[..n] {
        let (con, vis, cp, wght) =
            GASES[usize::try_from(k).ok().filter(|k| (1..=4).contains(k))? - 1];
        gases.push(Gas {
            con: [con[0], con[1], con[2] + quad * 1e-9],
            vis: [vis[0], vis[1], vis[2] + quad * 1e-12],
            cp: [cp[0], cp[1], cp[2] + quad * 1e-6],
            wght,
        });
    }
    Some((gases, f[..n].to_vec()))
}

fn fit(c: &[f64; 3], t: f64, t2: f64) -> f64 {
    c[0] + c[1] * t + c[2] * t2
}

/// `root_4`, then `pow_2` of `1 + a b` as the mixing terms write it.
fn sq1(a: f64, b: f64) -> f64 {
    let s = 1.0 + a * b;
    s * s
}

/// `WindowGasConductance` (`WindowManager.cc:4367`): conductance, Prandtl and Grashof numbers.
fn gas_conductance(tleft: f64, tright: f64, width: f64, g: &[Gas], frct: &[f64]) -> [f64; 3] {
    let two_sqrt_2 = 2.0 * 2.0_f64.sqrt();
    let n = g.len();
    let tmean = 0.5 * (tleft + tright);
    let t2 = tmean * tmean;
    let mut fcon = [0.0; 4];
    let mut fvis = [0.0; 4];
    let mut fcp = [0.0; 4];
    fcon[0] = fit(&g[0].con, tmean, t2);
    fvis[0] = fit(&g[0].vis, tmean, t2);
    fcp[0] = fit(&g[0].cp, tmean, t2);
    let fdens0 = PRES * g[0].wght / (GAS_LAW * tmean);
    let (con, visc, cp, dens);
    if n == 1 {
        (con, visc, cp, dens) = (fcon[0], fvis[0], fcp[0], fdens0);
    } else {
        let mut kprime = [0.0; 4];
        let mut kdblprm = [0.0; 4];
        let mut molmix = frct[0] * g[0].wght;
        let mut cpmixm = molmix * fcp[0];
        kprime[0] = 3.75 * (GAS_LAW / g[0].wght) * fvis[0];
        kdblprm[0] = fcon[0] - kprime[0];
        for i in 1..n {
            fcon[i] = fit(&g[i].con, tmean, t2);
            fvis[i] = fit(&g[i].vis, tmean, t2);
            fcp[i] = fit(&g[i].cp, tmean, t2);
            molmix += frct[i] * g[i].wght;
            cpmixm += frct[i] * fcp[i] * g[i].wght;
            kprime[i] = 3.75 * GAS_LAW / g[i].wght * fvis[i];
            kdblprm[i] = fcon[i] - kprime[i];
        }
        let (mut mumix, mut kpmix, mut kdpmix) = (0.0, 0.0, 0.0);
        for i in 0..n {
            let (mut mukpdwn, mut kpdown, mut kdpdown) = (1.0, 1.0, 1.0);
            let wi = g[i].wght;
            for j in 0..n {
                let wj = g[j].wght;
                let phimup = sq1((fvis[i] / fvis[j]).sqrt(), (wj / wi).sqrt().sqrt());
                let downer = two_sqrt_2 * (1.0 + (wi / wj)).sqrt();
                let psiup = sq1((kprime[i] / kprime[j]).sqrt(), (wi / wj).sqrt().sqrt());
                let psiterm = 1.0 + 2.41 * (wi - wj) * (wi - 0.142 * wj) / ((wi + wj) * (wi + wj));
                if i != j {
                    mukpdwn += phimup / downer * frct[j] / frct[i];
                    kpdown += psiup * (psiterm / downer) * (frct[j] / frct[i]);
                    kdpdown += (psiup / downer) * (frct[j] / frct[i]); // `phikup` is `psiup`
                }
            }
            mumix += fvis[i] / mukpdwn;
            kpmix += kprime[i] / kpdown;
            kdpmix += kdblprm[i] / kdpdown;
        }
        dens = PRES * molmix / (GAS_LAW * tmean);
        con = kpmix + kdpmix;
        visc = mumix;
        cp = cpmixm / molmix;
    }
    let pr = cp * visc / con;
    let w3 = width * width * width;
    let gr = 9.807 * w3 * (tleft - tright).abs() * (dens * dens) / (tmean * (visc * visc));
    [con, pr, gr]
}

pub fn GapCon(
    which: i32,
    tleft: f64,
    tright: f64,
    width: f64,
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
    let Some((g, f)) = mixture(nmix, [g1, g2, g3, g4], [f1, f2, f3, f4], quad) else {
        return f64::NAN;
    };
    let r = gas_conductance(tleft, tright, width, &g, &f);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `WindowGasPropertiesAtTemp` (`WindowManager.cc:4537`): density and viscosity.
fn gas_properties(tmean: f64, g: &[Gas], frct: &[f64]) -> [f64; 2] {
    let two_sqrt_2 = 2.0 * 2.0_f64.sqrt();
    let n = g.len();
    let t2 = tmean * tmean;
    let mut fvis = [0.0; 4];
    fvis[0] = fit(&g[0].vis, tmean, t2);
    if n == 1 {
        return [PRES * g[0].wght / (GAS_LAW * tmean), fvis[0]];
    }
    let mut molmix = frct[0] * g[0].wght;
    for i in 1..n {
        fvis[i] = fit(&g[i].vis, tmean, t2);
        molmix += frct[i] * g[i].wght;
    }
    let mut mumix = 0.0;
    for i in 0..n {
        let mut mukpdwn = 1.0;
        for j in 0..n {
            let phimup = sq1(
                (fvis[i] / fvis[j]).sqrt(),
                (g[j].wght / g[i].wght).sqrt().sqrt(),
            );
            let downer = two_sqrt_2 * (1.0 + (g[i].wght / g[j].wght)).sqrt();
            if i != j {
                mukpdwn += phimup / downer * frct[j] / frct[i];
            }
        }
        mumix += fvis[i] / mukpdwn;
    }
    [PRES * molmix / (GAS_LAW * tmean), mumix]
}

pub fn GapProps(
    which: i32,
    tmean: f64,
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
    let Some((g, f)) = mixture(nmix, [g1, g2, g3, g4], [f1, f2, f3, f4], quad) else {
        return f64::NAN;
    };
    let r = gas_properties(tmean, &g, &f);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// ObjexxFCL `max`: `a < b ? b : a`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `NusseltNumber` (`WindowManager.cc:4772`): the gap Nusselt number, -1 where the C++ leaves it
/// unset (a NaN tilt with the window heated from below).
pub fn GapNusselt(tso: f64, tsi: f64, width: f64, height: f64, tilt: f64, gr: f64, pr: f64) -> f64 {
    let asp = if height > 0.0 {
        height / width
    } else {
        1.524 / width
    };
    let tiltr = tilt * DEG_TO_RAD;
    let ra = gr * pr;
    let mut gnu901 = 0.0;
    if ra <= 1.0e4 {
        gnu901 = 1.0 + 1.7596678e-10 * ra.powf(2.2984755);
    }
    if ra > 1.0e4 && ra <= 5.0e4 {
        gnu901 = 0.028154 * ra.powf(0.4134);
    }
    if ra > 5.0e4 {
        gnu901 = 0.0673838 * ra.powf(1.0 / 3.0);
    }
    let gnu902 = 0.242 * (ra / asp).powf(0.272);
    let gnu90 = max(gnu901, gnu902);
    if tso > tsi {
        return 1.0 + (gnu90 - 1.0) * tiltr.sin();
    }
    let mut gnu = -1.0;
    if tilt >= 60.0 {
        let g = if ra >= 0.001 {
            0.5 * (1.0 + (ra / 3160.0).powf(20.6)).powf(-0.1)
        } else {
            0.5
        };
        let x = 0.0936 * ra.powf(0.314) / (1.0 + g);
        let t = x * x * x; // `pow_7`
        let gnu601 = (1.0 + t * t * x).powf(0.142857);
        let gnu602 = (0.104 + 0.175 / asp) * ra.powf(0.283);
        let gnu60 = max(gnu601, gnu602);
        gnu = ((90.0 - tilt) * gnu60 + (tilt - 60.0) * gnu90) / 30.0;
    }
    if tilt < 60.0 {
        let cra = ra * tiltr.cos();
        let a = 1.0 - 1708.0 / cra;
        let b = (cra / 5830.0).powf(0.33333) - 1.0;
        let gnua = (a.abs() + a) / 2.0;
        let gnub = (b.abs() + b) / 2.0;
        let ang = 1708.0 * (1.8 * tiltr).sin().powf(1.6);
        gnu = 1.0 + 1.44 * gnua * (1.0 - ang / cra) + gnub;
    }
    gnu
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argon_fill_conducts_less_than_air() {
        let air = GapCon(
            0, 283.15, 293.15, 0.0127, 1, 1, 1, 1, 1, 1.0, 0.0, 0.0, 0.0, 0.0,
        );
        let argon = GapCon(
            0, 283.15, 293.15, 0.0127, 2, 2, 1, 1, 1, 0.9, 0.1, 0.0, 0.0, 0.0,
        );
        assert!(argon < air && (air - 0.0248).abs() < 0.001);
    }

    #[test]
    fn a_still_gap_has_a_nusselt_number_of_about_one() {
        assert!((GapNusselt(283.15, 293.15, 0.0127, 1.5, 90.0, 10.0, 0.71) - 1.0).abs() < 0.01);
    }
}
