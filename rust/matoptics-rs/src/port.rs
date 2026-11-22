//! The Rust port of `matoptics.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Optics helpers from EnergyPlus `Material.cc`: `CalcScreenTransmittance` (the screen beam and
//! scattered transmittance model), `GetRelativePhiTheta`, `NormalizePhiTheta`,
//! `GetPhiThetaIndices`, `GetProfIndices`, `GetSlatIndicesInterpFac` and
//! `MaterialGlass::SetupSimpleWindowGlazingSystem`, in the C++'s operation order. The oracle's
//! guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use std::f64::consts::{FRAC_PI_2, PI};

const DEG_TO_RAD: f64 = PI / 180.0;
const RAD_TO_DEG: f64 = 180.0 / PI;
/// `Material::maxIPhi` and `maxITheta`: `int(Pi * RadToDeg / minDegResolution) + 1`.
const MAX_I_PHI: i32 = (PI * RAD_TO_DEG / 5.0) as i32 + 1;
const MAX_PROF_ANGS: i32 = 37;
const D_PROF_ANG: f64 = PI / (MAX_PROF_ANGS - 1) as f64;
const MAX_SLAT_ANGS: i32 = 181;
const D_SLAT_ANG: f64 = PI / (MAX_SLAT_ANGS - 1) as f64;

/// ObjexxFCL `max`: `a < b ? b : a`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// Whether `x` converts to an int (the oracle's guard on the index helpers).
fn fits(x: f64) -> bool {
    x.abs() < 2.0e9
}

/// `CalcScreenTransmittance` (`Material.cc:3391`): the twelve `ScreenBmTransAbsRef` results in
/// the oracle's order.
fn screen_trans(
    model: i32,
    gamma: f64,
    refl_cyl: f64,
    refl_cyl_vis: f64,
    mut phi: f64,
    mut theta: f64,
) -> [f64; 12] {
    const SMALL: f64 = 1.0e-9;
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let tan_phi = sin_phi / cos_phi;
    let cos_theta = theta.cos();
    let sun_in_front = phi < FRAC_PI_2 && theta < FRAC_PI_2;
    if phi > FRAC_PI_2 {
        phi = PI - phi;
    }
    if theta > FRAC_PI_2 {
        theta = PI - theta;
    }
    let beta = FRAC_PI_2 - theta;
    let trans_y = if beta > SMALL && (phi - FRAC_PI_2).abs() > SMALL {
        let a = (tan_phi / cos_theta).atan();
        let cot = 1.0 / beta.tan();
        max(
            0.0,
            1.0 - gamma * (a.cos() + a.sin() * tan_phi * (1.0 + cot * cot).sqrt()),
        )
    } else {
        0.0
    };
    let cos_mu = (cos_phi * cos_phi * (cos_theta * cos_theta) + sin_phi * sin_phi).sqrt();
    let trans_x = if cos_mu <= SMALL {
        1.0 - gamma
    } else {
        let epsilon = (cos_phi * cos_theta / cos_mu).acos();
        let eta = FRAC_PI_2 - epsilon;
        if epsilon.cos() != 0.0 && eta != 0.0 {
            let tan_mu = cos_mu.acos().tan();
            let mu_prime = (tan_mu / epsilon.cos()).atan();
            let cot = 1.0 / eta.tan();
            max(
                0.0,
                1.0 - gamma * (mu_prime.cos() + mu_prime.sin() * tan_mu * (1.0 + cot * cot).sqrt()),
            )
        } else {
            0.0
        }
    };
    let t_direct = max(0.0, trans_x * trans_y);
    let (mut ts, mut ts_vis) = (0.0, 0.0);
    if !((FRAC_PI_2 - theta) < SMALL || (FRAC_PI_2 - phi) < SMALL) {
        let delta_max = 89.7 - (10.0 * gamma / 0.16);
        let (td, pd) = (theta / DEG_TO_RAD, phi / DEG_TO_RAD);
        let delta = (td * td + pd * pd).sqrt();
        let ts_max = |r: f64| {
            0.0229 * gamma + 0.2971 * r - 0.03624 * (gamma * gamma) + 0.04763 * (r * r)
                - 0.44416 * gamma * r
        };
        let d = delta - delta_max;
        let exp_int = -(d * d) / 600.0;
        let exp_ext = -d.abs().powf(2.5) / 600.0;
        let scatter = |r: f64, exponent: f64| {
            let ratio = 1.0 / (0.2 * (1.0 - gamma) * r);
            0.2 * (1.0 - gamma) * r * ts_max(r) * (1.0 + (ratio - 1.0) * exponent.exp())
        };
        if delta > delta_max {
            ts = scatter(refl_cyl, exp_ext);
            ts_vis = scatter(refl_cyl_vis, exp_ext);
            let fall = max(0.0, (delta - delta_max) / (90.0 - delta_max));
            ts -= (0.2 * (1.0 - gamma) * refl_cyl * ts_max(refl_cyl)) * fall;
            ts_vis -= (0.2 * (1.0 - gamma) * refl_cyl_vis * ts_max(refl_cyl_vis)) * fall;
        } else {
            ts = scatter(refl_cyl, exp_int);
            ts_vis = scatter(refl_cyl_vis, exp_int);
        }
    }
    // (BmTrans, BmTransVis, BmTransBack) from the beam reflectance model.
    let beam = match model {
        1 => (t_direct + ts, t_direct + ts_vis, t_direct + ts),
        _ => (t_direct, t_direct, t_direct),
    };
    if model == 2 {
        (ts, ts_vis) = (max(0.0, ts), max(0.0, ts_vis));
    } else {
        (ts, ts_vis) = (0.0, 0.0);
    }
    let ref_sol = max(0.0, refl_cyl * (1.0 - t_direct) - ts);
    let ref_vis = max(0.0, refl_cyl_vis * (1.0 - t_direct) - ts_vis);
    let abs_sol = max(0.0, (1.0 - t_direct) * (1.0 - refl_cyl));
    if sun_in_front {
        [
            beam.0, 0.0, beam.1, ts, 0.0, ts_vis, ref_sol, ref_vis, 0.0, 0.0, abs_sol, 0.0,
        ]
    } else {
        [
            0.0, beam.2, 0.0, 0.0, ts, 0.0, 0.0, 0.0, ref_sol, ref_vis, 0.0, abs_sol,
        ]
    }
}

pub fn ScreenTrans(
    which: i32,
    model: i32,
    gamma: f64,
    refl: f64,
    reflVis: f64,
    phi: f64,
    theta: f64,
) -> f64 {
    if !(0..=2).contains(&model) || !(0.0..=PI).contains(&phi) || !(0.0..=PI).contains(&theta) {
        return f64::NAN;
    }
    let r = screen_trans(model, gamma, refl, reflVis, phi, theta);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `NormalizePhiTheta` (`Material.cc:3588`).
fn normalize(mut phi: f64, mut theta: f64) -> (f64, f64) {
    while phi > 2.0 * PI {
        phi -= 2.0 * PI;
    }
    if phi > PI {
        phi = 2.0 * PI - phi;
    }
    while theta > 2.0 * PI {
        theta -= 2.0 * PI;
    }
    if theta > PI {
        theta = 2.0 * PI - theta;
    }
    (phi, theta)
}

fn pick2(r: (f64, f64), which: i32) -> f64 {
    match which {
        0 => r.0,
        1 => r.1,
        _ => f64::NAN,
    }
}

pub fn RelPhiTheta(which: i32, phiWin: f64, thetaWin: f64, sx: f64, sy: f64, sz: f64) -> f64 {
    if !(phiWin.abs() <= 1000.0 && thetaWin.abs() <= 1000.0) {
        return f64::NAN;
    }
    pick2(
        normalize((sz.acos() - phiWin).abs(), (sx.atan2(sy) - thetaWin).abs()),
        which,
    )
}

pub fn NormPhiTheta(which: i32, phi: f64, theta: f64) -> f64 {
    if !(phi.abs() <= 1000.0 && theta.abs() <= 1000.0) {
        return f64::NAN;
    }
    pick2(normalize(phi, theta), which)
}

/// An index and the next one, held at the last grid point as `GetPhiThetaIndices` does.
fn index_pair(x: f64) -> [i32; 2] {
    let i = x as i32; // in range: the oracle refuses values that would not fit
    [i, if i == MAX_I_PHI - 1 { i } else { i + 1 }]
}

pub fn PhiThetaIdx(which: i32, phi: f64, theta: f64, dPhi: f64, dTheta: f64) -> f64 {
    if !fits(phi / dPhi) || !fits(theta / dTheta) {
        return f64::NAN;
    }
    let [p1, p2] = index_pair(phi / dPhi);
    let [t1, t2] = index_pair(theta / dTheta);
    usize::try_from(which)
        .ok()
        .and_then(|w| [p1, p2, t1, t2].get(w).map(|&i| f64::from(i)))
        .unwrap_or(f64::NAN)
}

pub fn ProfIdx(which: i32, profAng: f64) -> f64 {
    let x = (profAng + FRAC_PI_2) / D_PROF_ANG;
    if !fits(x) {
        return f64::NAN;
    }
    let lo = x as i32 + 1;
    pick2((f64::from(lo), f64::from(MAX_PROF_ANGS.min(lo + 1))), which)
}

pub fn SlatIdx(which: i32, slatAng: f64) -> f64 {
    let x = slatAng / D_SLAT_ANG;
    if !fits(x) {
        return f64::NAN;
    }
    let lo = x as i32;
    let r = [
        f64::from(lo),
        f64::from(MAX_SLAT_ANGS.min(lo + 1)),
        (slatAng - (f64::from(lo) * D_SLAT_ANG)) / D_SLAT_ANG,
    ];
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `MaterialGlass::SetupSimpleWindowGlazingSystem` (`Material.cc:3217`): Trans,
/// ReflectSolBeamBack, ReflectSolBeamFront, TransVis, ReflectVisBeamBack, ReflectVisBeamFront,
/// Thickness, Conductivity and Resistance, or `None` where the C++ stops with a fatal error
/// (a conductivity of 0 or less).
fn simple_glazing(u: f64, shgc: f64, vt_by_user: bool, vt: f64) -> Option<[f64; 9]> {
    let riw = if u < 5.85 {
        1.0 / (0.359073 * u.ln() + 6.949915)
    } else {
        1.0 / (1.788041 * u - 2.886625)
    };
    let row = 1.0 / (0.025342 * u + 29.163853);
    let mut rlw = (1.0 / u) - riw - row;
    if rlw <= 0.0 {
        rlw = max(rlw, 0.001); // with a warning
    }
    let thickness = if (1.0 / rlw) > 7.0 {
        0.002
    } else {
        0.05914 - (0.00714 / rlw)
    };
    let conductivity = thickness / rlw;
    if conductivity.is_nan() || conductivity <= 0.0 {
        return None;
    }
    let hi_side = if shgc < 0.7206 {
        0.939998 * (shgc * shgc) + 0.20332 * shgc
    } else {
        1.30415 * shgc - 0.30515
    };
    let low_side = if shgc <= 0.15 {
        0.41040 * shgc
    } else {
        0.085775 * (shgc * shgc) + 0.963954 * shgc - 0.084958
    };
    let frac = (u - 3.4) / (4.5 - 3.4);
    let mut trans = if u > 4.5 {
        hi_side
    } else if u < 3.4 {
        low_side
    } else {
        frac * (hi_side - low_side) + low_side
    };
    if trans < 0.0 {
        trans = 0.0;
    }
    let d = shgc - trans;
    let cubic = |a: f64, b: f64, c: f64, e: f64| 1.0 / (a * (d * d * d) - b * (d * d) + c * d + e);
    let ris_hi = || cubic(29.436546, 21.943415, 9.945872, 7.426151);
    let ris_low = || cubic(199.8208128, 90.639733, 19.737055, 6.766575);
    let ros_hi = || 1.0 / (2.225824 * d + 20.577080);
    let ros_low = || 1.0 / (5.763355 * d + 20.541528);
    let (ris, ros) = if u > 4.5 {
        (ris_hi(), ros_hi())
    } else if u < 3.4 {
        (ris_low(), ros_low())
    } else {
        let (lo, hi) = (ris_low(), ris_hi());
        let ris = frac * (lo - hi) + lo;
        let (lo, hi) = (ros_low(), ros_hi());
        (ris, frac * (lo - hi) + lo)
    };
    let inflow = (ros + 0.5 * rlw) / (ros + rlw + ris);
    let solar_absorb = (shgc - trans) / inflow;
    let refl_sol = 1.0 - trans - solar_absorb;
    let (trans_vis, refl_vis_back, refl_vis_front) = if vt_by_user {
        let poly = |a: f64, b: f64, c: f64, e: f64| {
            let r = a * (vt * vt * vt) + b * (vt * vt) - c * vt + e;
            if vt + r >= 1.0 { 0.999 - vt } else { r }
        };
        (
            vt,
            poly(-0.7409, 1.6531, 1.2299, 0.4545),
            poly(-0.0622, 0.4277, 0.4169, 0.2399),
        )
    } else {
        (trans, refl_sol, refl_sol)
    };
    Some([
        trans,
        refl_sol,
        refl_sol,
        trans_vis,
        refl_vis_back,
        refl_vis_front,
        thickness,
        conductivity,
        rlw,
    ])
}

pub fn SimpleGlazing(which: i32, u: f64, shgc: f64, vtByUser: i32, vt: f64) -> f64 {
    let Some(r) = simple_glazing(u, shgc, vtByUser != 0, vt) else {
        return f64::NAN;
    };
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_screen_passes_less_beam_at_a_low_sun() {
        let normal = screen_trans(0, 0.2, 0.3, 0.3, 0.0, 0.0)[0];
        let low = screen_trans(0, 0.2, 0.3, 0.3, 1.2, 0.0)[0];
        assert!((normal - 0.8 * 0.8).abs() < 1e-9 && low < normal);
    }

    #[test]
    fn simple_glazing_keeps_its_shgc_balance() {
        let r = simple_glazing(2.0, 0.4, false, 0.0).unwrap_or_else(|| unreachable!());
        assert!(r[0] > 0.0 && r[0] < 0.4 && r[1] > 0.0);
        assert!((MAX_I_PHI, MAX_PROF_ANGS) == (37, 37));
    }
}
