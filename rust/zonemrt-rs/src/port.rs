//! The Rust port of `zonemrt.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatBalanceSurfaceManager::CalculateZoneMRT` for a zone of two spaces, each its own radiant
//! enclosure: the area-emissivity weights of the surfaces, the zone's mean radiant temperature
//! (or its air temperature when the weights sum to 0.01 or less) and each enclosure's (or its
//! space's volume-weighted air temperature), then the spaces' MRTs. Every sum runs in the C++
//! order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

const MAX_SURF: usize = 6;

/// One enclosure's running sums and MRT, as `EnclRadInfo` holds them.
#[derive(Default, Clone, Copy)]
struct Enclosure {
    sum_ae: f64,
    sum_aet: f64,
    mrt: f64,
}

/// One space: its air temperature and volume.
struct Space {
    mat: f64,
    volume: f64,
}

/// Everything `CalculateZoneMRT` sets for the zone.
struct Results {
    zone_mrt: f64,
    encl: [Enclosure; 2],
    zone_ae_sum: f64,
}

/// `CalculateZoneMRT` for surfaces of area `a`, absorptance `e` and face temperature `t`, the
/// first `n1` in space (and enclosure) 1 and the rest in space 2.
fn calculate_zone_mrt(
    n1: usize,
    a: &[f64],
    e: &[f64],
    t: &[f64],
    zone_mat: f64,
    spaces: &[Space; 2],
) -> Results {
    let encl_of = |k: usize| usize::from(k >= n1);
    let surface_ae: Vec<f64> = a.iter().zip(e).map(|(a, e)| a * e).collect();
    let mut zone_ae_sum = 0.0;
    let mut encl = [Enclosure::default(); 2];
    for (k, ae) in surface_ae.iter().enumerate() {
        zone_ae_sum += ae;
        encl[encl_of(k)].sum_ae += ae;
    }
    let zone_mrt = if zone_ae_sum > 0.01 {
        let mut zone_sum_aet = 0.0;
        for (k, (ae, t)) in surface_ae.iter().zip(t).enumerate() {
            let surf_aet = ae * t;
            zone_sum_aet += surf_aet;
            encl[encl_of(k)].sum_aet += surf_aet;
        }
        zone_sum_aet / zone_ae_sum
    } else {
        zone_mat
    };
    for (r, (x, space)) in encl.iter_mut().zip(spaces).enumerate() {
        if x.sum_ae > 0.01 {
            x.sum_aet = 0.0;
            for (k, (ae, t)) in surface_ae.iter().zip(t).enumerate() {
                if encl_of(k) == r {
                    x.sum_aet += ae * t;
                }
            }
            x.mrt = x.sum_aet / x.sum_ae;
        } else {
            // The volume-weighted air temperature of the enclosure's one space.
            let sum_vol = 0.0 + space.volume;
            let sum_mat_vol = 0.0 + space.mat * space.volume;
            let sum_mat = 0.0 + space.mat;
            x.mrt = if sum_vol > 0.01 {
                sum_mat_vol / sum_vol
            } else {
                sum_mat / 1.0
            };
        }
    }
    Results {
        zone_mrt,
        encl,
        zone_ae_sum,
    }
}

/// One result of `CalculateZoneMRT` (see the oracle's shim for the arguments and `which`).
pub fn ZoneMRT(
    which: i32,
    n1: i32,
    n2: i32,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    e1: f64,
    e2: f64,
    e3: f64,
    e4: f64,
    e5: f64,
    e6: f64,
    t1: f64,
    t2: f64,
    t3: f64,
    t4: f64,
    t5: f64,
    t6: f64,
    mat: f64,
    mat1: f64,
    mat2: f64,
    vol1: f64,
    vol2: f64,
) -> f64 {
    let (Ok(which), Ok(n1), Ok(n2)) = (
        usize::try_from(which),
        usize::try_from(n1),
        usize::try_from(n2),
    ) else {
        return f64::NAN;
    };
    if which > 10 || n1 + n2 > MAX_SURF {
        return f64::NAN;
    }
    let n = n1 + n2;
    let a = [a1, a2, a3, a4, a5, a6];
    let e = [e1, e2, e3, e4, e5, e6];
    let t = [t1, t2, t3, t4, t5, t6];
    let spaces = [
        Space {
            mat: mat1,
            volume: vol1,
        },
        Space {
            mat: mat2,
            volume: vol2,
        },
    ];
    let r = calculate_zone_mrt(n1, &a[..n], &e[..n], &t[..n], mat, &spaces);
    match which {
        0 | 1 => r.zone_mrt,
        2 | 4 => r.encl[0].mrt,
        3 | 5 => r.encl[1].mrt,
        6 => r.encl[0].sum_ae,
        7 => r.encl[0].sum_aet,
        8 => r.encl[1].sum_ae,
        9 => r.encl[1].sum_aet,
        _ => r.zone_ae_sum,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zone(which: i32, n1: i32, n2: i32, a: [f64; 6], e: f64, t: [f64; 6]) -> f64 {
        ZoneMRT(
            which, n1, n2, a[0], a[1], a[2], a[3], a[4], a[5], e, e, e, e, e, e, t[0], t[1], t[2],
            t[3], t[4], t[5], 21.0, 20.0, 22.0, 100.0, 50.0,
        )
    }

    #[test]
    fn mrt_is_area_emissivity_weighted() {
        // Enclosure 1: (1 * 10 + 3 * 30) / 4 = 25; enclosure 2: 20; zone: (10 + 90 + 40) / 6.
        let a = [1.0, 3.0, 2.0, 0.0, 0.0, 0.0];
        let t = [10.0, 30.0, 20.0, 0.0, 0.0, 0.0];
        assert!((zone(2, 2, 1, a, 0.9, t) - 25.0).abs() < 1e-12);
        assert!((zone(3, 2, 1, a, 0.9, t) - 20.0).abs() < 1e-12);
        assert!((zone(0, 2, 1, a, 0.9, t) - 140.0 / 6.0).abs() < 1e-12);
    }

    #[test]
    fn no_surfaces_fall_back_to_air_temperatures() {
        let z = [0.0; 6];
        assert_eq!(zone(0, 0, 0, z, 0.9, z), 21.0);
        assert_eq!(zone(4, 0, 0, z, 0.9, z), 20.0);
        assert_eq!(zone(5, 0, 0, z, 0.9, z), 22.0);
    }

    #[test]
    fn guards_return_nan() {
        let z = [0.0; 6];
        assert!(zone(11, 0, 0, z, 0.9, z).is_nan());
        assert!(zone(0, 4, 3, z, 0.9, z).is_nan());
        assert!(zone(0, -1, 0, z, 0.9, z).is_nan());
    }
}
