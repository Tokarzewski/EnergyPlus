//! The Rust port of `blindoptics.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Venetian blind optics from EnergyPlus: `BlindOpticsDiffuse` and `BlindOpticsBeam`
//! (`WindowManager.cc`) and `MaterialBlind::BeamBeamTrans` (`Material.cc`), the slat-gap
//! radiosity model on the blind the oracle sets up. The view factors, the matrix inverse and the
//! diffuse average come from `glazing-rs`. Arithmetic is in the C++'s order.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]
#![expect(
    clippy::needless_range_loop,
    reason = "the loops index several arrays as the C++ does"
)]

use glazing_rs::port::{
    MAX_ARRAY_SIZE, Matrix, NUM_PHIS, diffuse_average, invert_matrix, view_fac,
};
use std::f64::consts::{FRAC_PI_2, PI};

/// The blind's slat geometry (`MaterialBlind`'s width, separation and thickness).
#[derive(Clone, Copy)]
struct Slats {
    width: f64,
    sep: f64,
    thick: f64,
}

fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The slat edge correction factor the three routines share, for slat normal elevation `phib`
/// and source elevation `phis`.
fn edge(b: Slats, phib: f64, phis: f64) -> f64 {
    let gamma = phib - phis;
    if gamma.sin().abs() > 0.01 {
        let mut f1 = 0.0;
        if (phib > 0.0 && phib <= FRAC_PI_2 && phis <= phib)
            || (phib > FRAC_PI_2 && phib <= PI && phis > -(PI - phib))
        {
            f1 = b.thick * gamma.sin().abs() / ((b.sep + b.thick / phib.sin().abs()) * phis.cos());
        }
        min(1.0, f1.abs())
    } else {
        0.0
    }
}

/// `MaterialBlind::BeamBeamTrans` (`Material.cc:3614`).
fn beam_beam_trans(b: Slats, prof: f64, slat: f64) -> f64 {
    let cos_prof = prof.cos();
    let gamma = slat - prof;
    let wbar = if cos_prof != 0.0 {
        b.width * gamma.cos() / cos_prof
    } else {
        b.sep
    };
    let mut t = max(0.0, 1.0 - (wbar / b.sep).abs());
    if t > 0.0 {
        t *= 1.0 - edge(b, slat, prof);
    }
    t
}

/// The slat-gap exchange matrix `X` (`X J = Q`) for front, back and transmittance factors.
fn exchange(f: &[[f64; 6]; 6], rf: f64, rb: f64, t: f64) -> Matrix {
    let mut x = [[0.0; MAX_ARRAY_SIZE]; MAX_ARRAY_SIZE];
    for k in [2, 4] {
        for m in 2..6 {
            x[m - 2][k - 2] = -rb * f[k][m] - t * f[k + 1][m];
            x[m - 2][k - 1] = -t * f[k][m] - rf * f[k + 1][m];
        }
    }
    for k in 0..4 {
        x[k][k] += 1.0;
    }
    x
}

/// The irradiance `G` on the six sections from a unit source on section `src` (0 or 1), with the
/// sources `Q` built from `rb`, `t` and `rf` as `BlindOpticsDiffuse` builds them.
fn irradiance(f: &[[f64; 6]; 6], xinv: &Matrix, src: usize, rf: f64, rb: f64, t: f64) -> [f64; 6] {
    let mut q = [0.0; 6];
    q[2] = rb * f[2][src] + t * f[3][src];
    q[3] = t * f[2][src] + rf * f[3][src];
    q[4] = rb * f[4][src] + t * f[5][src];
    q[5] = t * f[4][src] + rf * f[5][src];
    let mut j = [0.0; 6];
    j[src] = 1.0;
    for k in 2..6 {
        j[k] = 0.0;
        for m in 2..6 {
            j[k] += xinv[m - 2][k - 2] * q[m];
        }
    }
    let mut g = [0.0; 6];
    for k in 0..6 {
        for m in 0..6 {
            g[k] += j[m] * f[k][m];
        }
    }
    g
}

/// `BlindOpticsDiffuse` (`WindowManager.cc:7581`): the blind properties `p` (8 to 14 set), or
/// `None` where `InvertMatrix` would stop with a fatal error.
fn blind_diffuse(b: Slats, isolvis: i32, c: &[f64; 15], b_el: f64) -> Option<[f64; 16]> {
    let mut p = [0.0; 16];
    let f = view_fac(c[1], c[2], b_el, FRAC_PI_2);
    let xinv = invert_matrix(&mut exchange(&f, c[10], c[11], c[9]), 4)?;
    let delphis = FRAC_PI_2 / 10.0;
    let mut f_edge_a = [0.0; 2];
    for up_down in 0..2 {
        let mut src = [0.0; NUM_PHIS];
        for i in 0..NUM_PHIS {
            let phis = if up_down == 1 {
                (i as f64 + 0.5) * delphis
            } else {
                -(i as f64 + 0.5) * delphis
            };
            src[i] = edge(b, b_el, phis);
        }
        f_edge_a[up_down] = diffuse_average(&src);
    }
    let f_edge = 0.5 * (f_edge_a[0] + f_edge_a[1]);
    let g = irradiance(&f, &xinv, 0, c[10], c[11], c[9]);
    p[8] = g[1] * (1.0 - f_edge);
    p[9] = g[0] * (1.0 - f_edge) + f_edge * c[10];
    let g = irradiance(&f, &xinv, 1, c[10], c[11], c[9]);
    p[10] = g[0] * (1.0 - f_edge);
    p[11] = g[1] * (1.0 - f_edge) + f_edge * c[10];
    if isolvis == 1 {
        let ri = 1.0 - c[12] - c[13];
        let rib = 1.0 - c[12] - c[14];
        let xinv = invert_matrix(&mut exchange(&f, ri, rib, c[12]), 4)?;
        let g = irradiance(&f, &xinv, 0, ri, rib, c[12]);
        p[12] = g[1] * (1.0 - f_edge);
        let refl_front = g[0] * (1.0 - f_edge) + f_edge * ri;
        p[13] = max(0.0001, 1.0 - p[12] - refl_front);
        let g = irradiance(&f, &xinv, 1, ri, rib, c[12]);
        let refl_back = g[1] * (1.0 - f_edge) + f_edge * ri;
        p[14] = max(0.0001, 1.0 - p[12] - refl_back);
    }
    Some(p)
}

/// `BlindOpticsBeam` (`WindowManager.cc:7888`): beam properties of the front (`p[0]`, `p[4]`,
/// `p[5]`) and back (`p[2]`, `p[6]`, `p[7]`), or `None` for the fatal error.
fn blind_beam(b: Slats, c: &[f64; 15], b_el: f64, s_el: f64) -> Option<[f64; 16]> {
    let mut p = [0.0; 16];
    let phis = s_el;
    let mut phib = b_el;
    for i in [0, 2] {
        if i == 2 {
            phib = PI - phib;
        }
        let f_edge = edge(b, phib, phis);
        p[i] = beam_beam_trans(b, phis, phib);
        p[1 + i] = 0.0;
        let f = view_fac(c[1], c[2], phib, phis);
        let xinv = invert_matrix(&mut exchange(&f, c[10], c[11], c[9]), 4)?;
        let mut q = [0.0; 6];
        if (phis - phib).abs() <= FRAC_PI_2 {
            q[2] = c[3] + c[6];
            q[3] = c[4] + c[7];
        } else {
            q[2] = c[5] + c[8];
            q[3] = c[3] + c[6];
        }
        q[2] *= 1.0 - p[i];
        q[3] *= 1.0 - p[i];
        let mut j = [0.0; 6];
        for k in 2..6 {
            for m in 2..4 {
                j[k] += xinv[m - 2][k - 2] * q[m];
            }
        }
        let mut g = [0.0; 6];
        for k in 0..6 {
            for m in 2..6 {
                g[k] += j[m] * f[k][m];
            }
        }
        p[4 + i] = g[1] * (1.0 - f_edge);
        p[5 + i] = g[0] * (1.0 - f_edge) + f_edge * c[7];
    }
    Some(p)
}

fn pick(p: Option<[f64; 16]>, which: i32) -> f64 {
    let Some(p) = p else { return f64::NAN };
    usize::try_from(which)
        .ok()
        .and_then(|w| p.get(w).copied())
        .unwrap_or(f64::NAN)
}

pub fn BlindDiffuse(
    which: i32,
    isolvis: i32,
    width: f64,
    sep: f64,
    thick: f64,
    b_el: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
    c12: f64,
    c13: f64,
    c14: f64,
) -> f64 {
    if !(0..=15).contains(&which) {
        return f64::NAN;
    }
    let c = [
        0.0, width, sep, 0.0, 0.0, 0.0, c6, c7, c8, c9, c10, c11, c12, c13, c14,
    ];
    pick(
        blind_diffuse(Slats { width, sep, thick }, isolvis, &c, b_el),
        which,
    )
}

pub fn BlindBeam(
    which: i32,
    width: f64,
    sep: f64,
    thick: f64,
    b_el: f64,
    s_el: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
) -> f64 {
    if !(0..=15).contains(&which) {
        return f64::NAN;
    }
    let c = [
        0.0, width, sep, c3, c4, c5, c6, c7, c8, c9, c10, c11, 0.0, 0.0, 0.0,
    ];
    pick(
        blind_beam(Slats { width, sep, thick }, &c, b_el, s_el),
        which,
    )
}

pub fn BlindBeamBeam(width: f64, sep: f64, thick: f64, profAng: f64, slatAng: f64) -> f64 {
    beam_beam_trans(Slats { width, sep, thick }, profAng, slatAng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_slats_pass_a_level_beam() {
        let b = Slats {
            width: 0.025,
            sep: 0.01875,
            thick: 0.0,
        };
        assert!((beam_beam_trans(b, 0.0, FRAC_PI_2) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn white_slats_reflect_more_than_dark_ones() {
        let b = Slats {
            width: 0.025,
            sep: 0.01875,
            thick: 0.001,
        };
        let c = |r: f64| {
            [
                0.0, 0.025, 0.01875, 0.0, 0.0, 0.0, 0.0, r, r, 0.0, r, r, 0.0, 0.9, 0.9,
            ]
        };
        let white = blind_diffuse(b, 1, &c(0.8), 0.5).unwrap_or_else(|| unreachable!());
        let dark = blind_diffuse(b, 1, &c(0.1), 0.5).unwrap_or_else(|| unreachable!());
        assert!(white[9] > dark[9]);
    }
}
