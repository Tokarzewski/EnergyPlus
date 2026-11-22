//! The Rust port of `perezsky.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! From EnergyPlus `SolarShading.cc`: `AnisoSkyViewFactors` (the Perez anisotropic sky model,
//! Solar Energy 44, 271-289, 1990) on the one exterior surface the oracle sets up, and the
//! `CLIPLINE` line clipper (Liang, Barsky and Slater). The arithmetic is in the C++'s order; `max`
//! is ObjexxFCL's (`a < b ? b : a`).

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0; // `Constant::DegToRad`

/// Upper limits of the sky clearness (epsilon) bins.
const EPSILON_LIMIT: [f64; 7] = [1.065, 1.23, 1.5, 1.95, 2.8, 4.5, 6.2];
// Circumsolar (F1) and horizon/zenith (F2) brightening coefficients by clearness bin.
const F11R: [f64; 8] = [
    -0.0083117, 0.1299457, 0.3296958, 0.5682053, 0.8730280, 1.1326077, 1.0601591, 0.6777470,
];
const F12R: [f64; 8] = [
    0.5877285, 0.6825954, 0.4868735, 0.1874525, -0.3920403, -1.2367284, -1.5999137, -0.3272588,
];
const F13R: [f64; 8] = [
    -0.0620636, -0.1513752, -0.2210958, -0.2951290, -0.3616149, -0.4118494, -0.3589221, -0.2504286,
];
const F21R: [f64; 8] = [
    -0.0596012, -0.0189325, 0.0554140, 0.1088631, 0.2255647, 0.2877813, 0.2642124, 0.1561313,
];
const F22R: [f64; 8] = [
    0.0721249, 0.0659650, -0.0639588, -0.1519229, -0.4620442, -0.8230357, -1.1272340, -1.3765031,
];
const F23R: [f64; 8] = [
    -0.0220216, -0.0288748, -0.0260542, -0.0139754, 0.0012448, 0.0558651, 0.1310694, 0.2506212,
];

/// ObjexxFCL `max( double, double )`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The sky model's results for one surface, as `AnisoSkyViewFactors` stores them.
struct Sky {
    aniso: f64,
    iso: f64,
    circumsolar: f64,
    horizon_zenith: f64,
    cur_iso_ratio: f64,
}

/// `AnisoSkyViewFactors` (`SolarShading.cc:2628`) on one surface; `None` where the C++ stops with
/// a fatal error (a beam incidence cosine more than 0.0001 outside [-1, 1]).
fn aniso_sky(
    detailed: bool,
    sun: [f64; 3],
    elev: f64,
    beam: f64,
    dif: f64,
    normal: [f64; 3],
    vf_sky: f64,
    tilt: f64,
    iso_ratio: f64,
    sunlit: f64,
    horiz_ratio: f64,
) -> Option<Sky> {
    const COSINE_TOLERANCE: f64 = 0.0001;
    let cos_zenith = sun[2];
    let zenith = cos_zenith.acos();
    let zenith_deg = zenith / DEG_TO_RAD;

    let air_mass_h = 1.0 - 0.1 * elev / 1000.0;
    let air_mass = if zenith_deg <= 75.0 {
        air_mass_h / cos_zenith
    } else {
        air_mass_h / (cos_zenith + 0.15 * (93.9 - zenith_deg).powf(-1.253))
    };
    let kappa_z3 = 1.041 * (zenith * zenith * zenith); // `pow_3`
    let epsilon = ((beam + dif) / dif + kappa_z3) / (1.0 + kappa_z3);
    let delta = dif * air_mass / 1353.0;
    let bin = EPSILON_LIMIT.iter().position(|&l| epsilon < l).unwrap_or(7);
    let f1 = max(0.0, F11R[bin] + F12R[bin] * delta + F13R[bin] * zenith);
    let f2 = F21R[bin] + F22R[bin] * delta + F23R[bin] * zenith;

    let mut cos_inc = sun[0] * normal[0] + sun[1] * normal[1] + sun[2] * normal[2];
    if cos_inc > 1.0 {
        if cos_inc > 1.0 + COSINE_TOLERANCE {
            return None; // ShowFatalError
        }
        cos_inc = 1.0;
    } else if cos_inc < -1.0 {
        if cos_inc < -1.0 - COSINE_TOLERANCE {
            return None; // ShowFatalError
        }
        cos_inc = -1.0;
    }

    let iso = vf_sky * (1.0 - f1);
    let mut circumsolar_fac = max(0.0, cos_inc) / max(0.0871557, cos_zenith);
    if circumsolar_fac > 0.0 && cos_zenith < 0.0871557 && tilt < 2.0 {
        circumsolar_fac = 1.0;
    }
    let circumsolar = f1 * circumsolar_fac;
    let horizon_zenith = f2 * (tilt * DEG_TO_RAD).sin(); // `SinTilt`
    // The shim's detailed flag sets both `DetailedSkyDiffuseAlgorithm` and
    // `ShadingTransmittanceVaries`; that branch reads the same ratios from the hourly arrays.
    let aniso = iso * iso_ratio + circumsolar * sunlit + horizon_zenith * horiz_ratio;
    Some(Sky {
        aniso: max(0.0, aniso),
        iso,
        circumsolar,
        horizon_zenith,
        cur_iso_ratio: if detailed { iso_ratio } else { 0.0 },
    })
}

/// The sky model's `which` result (see the oracle), NaN for a fatal error or an unknown `which`.
pub fn PerezSky(
    which: i32,
    detailed: i32,
    sx: f64,
    sy: f64,
    sz: f64,
    elev: f64,
    beam: f64,
    dif: f64,
    nx: f64,
    ny: f64,
    nz: f64,
    vfsky: f64,
    tilt: f64,
    iso: f64,
    sunlit: f64,
    horiz: f64,
) -> f64 {
    let Some(s) = aniso_sky(
        detailed != 0,
        [sx, sy, sz],
        elev,
        beam,
        dif,
        [nx, ny, nz],
        vfsky,
        tilt,
        iso,
        sunlit,
        horiz,
    ) else {
        return f64::NAN;
    };
    match which {
        0 => s.aniso,
        1 => s.iso,
        2 => s.circumsolar,
        3 => s.horizon_zenith,
        4 => s.cur_iso_ratio,
        _ => f64::NAN,
    }
}

/// `CLIPLINE` (`SolarShading.cc:4012`): the segment clipped to the window, or `None` when none of
/// it is visible (the C++ then leaves the end points as they are, possibly swapped).
fn clip_line(mut p: [f64; 4], max_x: f64, min_x: f64, max_y: f64, min_y: f64) -> ([f64; 4], bool) {
    let rev = p[0] > p[1];
    if rev {
        p.swap(0, 1);
        p.swap(2, 3);
    }
    let [x0, x1, y0, y1] = p;
    if x0 > max_x || x1 < min_x {
        return (p, false);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let q1 = x0 - min_x;
    let q2 = max_x - x0;
    let q3 = y0 - min_y;
    let q4 = max_y - y0;
    let mut u1 = 0.0;
    let mut u2 = 1.0;
    if (dx == 0.0 && (q1 < 0.0 || q2 < 0.0)) || (dy == 0.0 && (q3 < 0.0 || q4 < 0.0)) {
        return (p, false);
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
        let (lo, hi) = if dy > 0.0 { (r3, r4) } else { (r4, r3) };
        if lo > u1 {
            u1 = lo;
        }
        if hi < u2 {
            u2 = hi;
        }
    }
    if u1 > u2 {
        return (p, false);
    }
    let (xn0, yn0) = (x0 + dx * u1, y0 + dy * u1);
    let (xn1, yn1) = (x0 + dx * u2, y0 + dy * u2);
    let q = if rev {
        [xn1, xn0, yn1, yn0]
    } else {
        [xn0, xn1, yn0, yn1]
    };
    (q, true)
}

/// The clipped x0 (0), x1 (1), y0 (2), y1 (3), or visibility (4) as the oracle returns them.
pub fn ClipLine(
    which: i32,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    maxX: f64,
    minX: f64,
    maxY: f64,
    minY: f64,
) -> f64 {
    let (p, visible) = clip_line([x0, x1, y0, y1], maxX, minX, maxY, minY);
    match which {
        0..=3 => p[which as usize],
        4 => f64::from(u8::from(visible)),
        _ => f64::NAN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_segment_across_the_window_is_cut_at_its_edges() {
        let (p, visible) = clip_line([-1.0, 3.0, 1.0, 1.0], 2.0, 0.0, 2.0, 0.0);
        assert!(visible);
        assert_eq!(p, [0.0, 2.0, 1.0, 1.0]);
    }

    #[test]
    fn overcast_sky_on_a_roof_is_mostly_isotropic() {
        let s = aniso_sky(
            false,
            [0.0, 0.0, 1.0],
            0.0,
            0.0,
            100.0,
            [0.0, 0.0, 1.0],
            1.0,
            0.0,
            1.0,
            1.0,
            1.0,
        );
        let s = s.unwrap_or_else(|| unreachable!());
        assert!((s.aniso - 1.0).abs() < 0.2);
    }
}
