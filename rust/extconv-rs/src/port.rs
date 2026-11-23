//! The Rust port of `extconv.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `Convect::InitExtConvCoeff` for a surface facing the outside air (not a Kiva foundation): the
//! outside convection coefficient by the surface's (or its zone's) model (a user value, ASHRAE
//! simple, TARP/ASHRAE TARP/BLAST, MoWiTT or DOE-2), with the EMS override and the frame
//! adjustment ratio, then the linearized radiation coefficients to the sky, the ground, the air
//! and any surrounding surfaces. The correlations are `ConvectionCoefficients.cc`'s, each keeping
//! the C++ order of operations. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

const KELVIN: f64 = 273.15;
const STEFAN_BOLTZMANN: f64 = 5.6697E-8;
/// `Convect::RoughnessMultiplier`, very rough to very smooth.
const ROUGHNESS_MULTIPLIER: [f64; 6] = [2.17, 1.67, 1.52, 1.13, 1.11, 1.0];

/// `HcExt` values the port handles.
const VALUE: i32 = 0;
const ASHRAE_SIMPLE: i32 = 3;
const ASHRAE_TARP: i32 = 4;
const TARP: i32 = 5;
const MOWITT: i32 = 6;
const DOE2: i32 = 7;
const BLAST: i32 = 8;

/// ObjexxFCL `pow_4`.
fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

fn calc_ashrae_vertical_wall(delta_temp: f64) -> f64 {
    1.31 * delta_temp.abs().powf(1.0 / 3.0)
}

fn calc_walton_unstable_horizontal_or_tilt(delta_temp: f64, cosine_tilt: f64) -> f64 {
    9.482 * delta_temp.abs().powf(1.0 / 3.0) / (7.238 - cosine_tilt.abs())
}

fn calc_walton_stable_horizontal_or_tilt(delta_temp: f64, cosine_tilt: f64) -> f64 {
    1.810 * delta_temp.abs().powf(1.0 / 3.0) / (1.382 + cosine_tilt.abs())
}

fn calc_ashrae_tarp_natural(tsurf: f64, tamb: f64, cos_tilt: f64) -> f64 {
    let delta_temp = tsurf - tamb;
    if delta_temp == 0.0 || cos_tilt == 0.0 {
        return calc_ashrae_vertical_wall(delta_temp);
    }
    if (delta_temp < 0.0 && cos_tilt < 0.0) || (delta_temp > 0.0 && cos_tilt > 0.0) {
        return calc_walton_unstable_horizontal_or_tilt(delta_temp, cos_tilt);
    }
    calc_walton_stable_horizontal_or_tilt(delta_temp, cos_tilt)
}

fn calc_ashrae_simp_ext_conv_coeff(rough: usize, wind: f64) -> f64 {
    const D: [f64; 6] = [11.58, 12.49, 10.79, 8.23, 10.22, 8.23];
    const E: [f64; 6] = [5.894, 4.065, 4.192, 4.00, 3.100, 3.33];
    const F: [f64; 6] = [0.0, 0.028, 0.0, -0.057, 0.0, -0.036];
    D[rough] + E[rough] * wind + F[rough] * (wind * wind)
}

fn windward(cos_tilt: f64, azimuth: f64, wind_direction: f64) -> bool {
    if cos_tilt.abs() >= 0.98 {
        return true;
    }
    let mut diff = (wind_direction - azimuth).abs();
    if (diff - 180.0) > 0.001 {
        diff -= 360.0;
    }
    (diff.abs() - 90.0) <= 0.001
}

fn calc_sparrow_windward(rough: usize, perimeter: f64, area: f64, wind: f64) -> f64 {
    2.537 * ROUGHNESS_MULTIPLIER[rough] * (perimeter * wind / area).sqrt()
}

fn calc_mowitt_natural(delta_temp: f64) -> f64 {
    0.84 * delta_temp.abs().powf(1.0 / 3.0)
}

fn calc_mowitt_forced_windward(wind: f64) -> f64 {
    3.26 * wind.powf(0.89)
}

fn calc_mowitt_forced_leeward(wind: f64) -> f64 {
    3.55 * wind.powf(0.617)
}

/// `CalcMoWITTWindward` or `CalcMoWITTLeeward`, by which forced part is given.
fn calc_mowitt(delta_temp: f64, hf: f64) -> f64 {
    let hn = calc_mowitt_natural(delta_temp);
    (hn * hn + hf * hf).sqrt()
}

fn calc_doe2_forced(
    surface_temp: f64,
    air_temp: f64,
    cos_tilt: f64,
    hf_smooth: f64,
    rough: usize,
) -> f64 {
    let hn = calc_ashrae_tarp_natural(surface_temp, air_temp, cos_tilt);
    let hc_smooth = (hn * hn + hf_smooth * hf_smooth).sqrt();
    ROUGHNESS_MULTIPLIER[rough] * (hc_smooth - hn)
}

/// The linearized radiation coefficient `sigma * factor * (Ts^4 - T^4) / (Ts - T)`, with the
/// factor's terms multiplied in the C++ order first.
fn rad_coeff(factor: f64, tsurf: f64, t: f64) -> f64 {
    factor * (pow_4(tsurf) - pow_4(t)) / (tsurf - t)
}

/// One surface as `InitExtConvCoeff` reads it.
struct Surface {
    model: i32,
    hmov: f64,
    rough: usize,
    abs_ext: f64,
    temp_ext: f64,
    out_db: f64,
    sky_temp_k: f64,
    /// Surrounding surfaces temperature and view factor, when there is a list.
    srd: Option<(f64, f64)>,
    gnd_temp: Option<f64>,
    wind_speed: f64,
    wind_dir: f64,
    gross_area: f64,
    height: f64,
    cos_tilt: f64,
    azimuth: f64,
    hvalue: f64,
    ems: Option<f64>,
    adj_ratio: f64,
    vf_sky: f64,
    vf_gnd: f64,
    air_sky_split: f64,
}

/// `[HExt, HSky, HGround, HAir, HSrdSurf]`.
fn init_ext_conv_coeff(s: &Surface) -> [f64; 5] {
    let tair = s.out_db + KELVIN;
    let mut tsurf = s.temp_ext + KELVIN;
    let tsky = s.sky_temp_k;
    let mut tground = tair;
    let mut hsrd = 0.0;
    if let Some((srd_temp, vf_srd)) = s.srd {
        let srd_tk = srd_temp + KELVIN;
        if tsurf != srd_tk {
            hsrd = STEFAN_BOLTZMANN * s.abs_ext * vf_srd * (pow_4(tsurf) - pow_4(srd_tk))
                / (tsurf - srd_tk);
        }
    }
    if let Some(g) = s.gnd_temp {
        tground = g + KELVIN;
    }
    let wind = s.wind_speed;
    let mut hext = match s.model {
        VALUE => s.hvalue,
        ASHRAE_SIMPLE => calc_ashrae_simp_ext_conv_coeff(s.rough, wind),
        ASHRAE_TARP | TARP | BLAST => {
            let hf = if s.gross_area != 0.0 && s.height != 0.0 {
                let perimeter = 2.0 * (s.gross_area / s.height + s.height);
                let windward_hf = calc_sparrow_windward(s.rough, perimeter, s.gross_area, wind);
                if windward(s.cos_tilt, s.azimuth, s.wind_dir) {
                    windward_hf
                } else {
                    0.5 * windward_hf
                }
            } else {
                0.0
            };
            if s.hmov > 0.0 {
                tsurf = (s.hmov * tsurf + hf * tair) / (s.hmov + hf);
            }
            calc_ashrae_tarp_natural(tsurf, tair, s.cos_tilt) + hf
        }
        MOWITT => {
            let hf = if windward(s.cos_tilt, s.azimuth, s.wind_dir) {
                calc_mowitt_forced_windward(wind)
            } else {
                calc_mowitt_forced_leeward(wind)
            };
            calc_mowitt(tair - tsurf, hf)
        }
        DOE2 => {
            let hf_smooth = if windward(s.cos_tilt, s.azimuth, s.wind_dir) {
                calc_mowitt_forced_windward(wind)
            } else {
                calc_mowitt_forced_leeward(wind)
            };
            let hf = calc_doe2_forced(tsurf, tair, s.cos_tilt, hf_smooth, s.rough);
            if s.hmov > 0.0 {
                tsurf = (s.hmov * tsurf + hf * tair) / (s.hmov + hf);
            }
            calc_ashrae_tarp_natural(tsurf, tair, s.cos_tilt) + hf
        }
        // The exported function refuses other models before calling this.
        _ => f64::NAN,
    };
    if let Some(v) = s.ems {
        hext = v;
    }
    hext *= s.adj_ratio;
    let simple = s.model == ASHRAE_SIMPLE;
    let hsky = if tsurf == tsky || simple {
        0.0
    } else {
        rad_coeff(
            STEFAN_BOLTZMANN * s.abs_ext * s.vf_sky * s.air_sky_split,
            tsurf,
            tsky,
        )
    };
    let (hground, hair) = if tsurf == tair || simple {
        (0.0, 0.0)
    } else {
        (
            rad_coeff(STEFAN_BOLTZMANN * s.abs_ext * s.vf_gnd, tsurf, tground),
            rad_coeff(
                STEFAN_BOLTZMANN * s.abs_ext * s.vf_sky * (1.0 - s.air_sky_split),
                tsurf,
                tair,
            ),
        )
    };
    [hext, hsky, hground, hair, hsrd]
}

/// One coefficient from `InitExtConvCoeff` (see the oracle's shim for the arguments and `which`).
pub fn ExtConvCoeff(
    which: i32,
    model: i32,
    setbyzone: i32,
    hmov: f64,
    rough: i32,
    absext: f64,
    tempext: f64,
    outdb: f64,
    skytempk: f64,
    hassrd: i32,
    srdtemp: f64,
    vfsrd: f64,
    usegnd: i32,
    gndtemp: f64,
    extwind: i32,
    window: i32,
    extshade: i32,
    windspeed: f64,
    winddir: f64,
    grossarea: f64,
    height: f64,
    costilt: f64,
    azimuth: f64,
    hvalue: f64,
    emsover: i32,
    emsval: f64,
    adjratio: f64,
    vfsky: f64,
    vfgnd: f64,
    airskysplit: f64,
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    let known = model == VALUE || (ASHRAE_SIMPLE..=BLAST).contains(&model);
    let (Ok(which), Ok(rough)) = (usize::try_from(which), usize::try_from(rough)) else {
        return f64::NAN;
    };
    if which > 4
        || !known
        || ![
            setbyzone, hassrd, usegnd, extwind, window, extshade, emsover,
        ]
        .into_iter()
        .all(flag)
        || rough > 5
        || (setbyzone == 1 && model == VALUE)
    {
        return f64::NAN;
    }
    // No wind exposure, or a window behind an exterior shade, sees no wind.
    let wind_speed = if extwind == 0 || (window == 1 && extshade == 1) {
        0.0
    } else {
        windspeed
    };
    let s = Surface {
        model,
        hmov,
        rough,
        abs_ext: absext,
        temp_ext: tempext,
        out_db: outdb,
        sky_temp_k: skytempk,
        srd: (hassrd == 1).then_some((srdtemp, vfsrd)),
        gnd_temp: (usegnd == 1).then_some(gndtemp),
        wind_speed,
        wind_dir: winddir,
        gross_area: grossarea,
        height,
        cos_tilt: costilt,
        azimuth,
        hvalue,
        ems: (emsover == 1).then_some(emsval),
        adj_ratio: adjratio,
        vf_sky: vfsky,
        vf_gnd: vfgnd,
        air_sky_split: airskysplit,
    };
    init_ext_conv_coeff(&s)[which]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A vertical south wall, medium rough, absorptance 0.9, at `tempext` in 0 C air with 4 m/s
    /// wind from the south, sky at 263.15 K, 20 m2 and 2.5 m high, half view to sky and ground.
    fn wall(which: i32, model: i32, tempext: f64, hmov: f64) -> f64 {
        ExtConvCoeff(
            which, model, 0, hmov, 2, 0.9, tempext, 0.0, 263.15, 0, 0.0, 0.0, 0, 0.0, 1, 0, 0, 4.0,
            180.0, 20.0, 2.5, 0.0, 180.0, 7.0, 0, 0.0, 1.0, 0.5, 0.5, 0.5,
        )
    }

    #[test]
    fn tarp_adds_sparrow_forced_to_natural() {
        // Perimeter 2 * (8 + 2.5) = 21; Hf = 2.537 * 1.52 * sqrt(21 * 4 / 20); Hn = 1.31 * 10^(1/3).
        let hf = 2.537 * 1.52 * (21.0_f64 * 4.0 / 20.0).sqrt();
        let hn = 1.31 * 10.0_f64.powf(1.0 / 3.0);
        assert!((wall(0, TARP, 10.0, 0.0) - (hn + hf)).abs() < 1e-12);
    }

    #[test]
    fn leeward_wind_halves_sparrow() {
        let lee = ExtConvCoeff(
            0, TARP, 0, 0.0, 2, 0.9, 10.0, 0.0, 263.15, 0, 0.0, 0.0, 0, 0.0, 1, 0, 0, 4.0, 0.0,
            20.0, 2.5, 0.0, 180.0, 7.0, 0, 0.0, 1.0, 0.5, 0.5, 0.5,
        );
        let hn = 1.31 * 10.0_f64.powf(1.0 / 3.0);
        assert!((lee - hn - 0.5 * (wall(0, TARP, 10.0, 0.0) - hn)).abs() < 1e-12);
    }

    #[test]
    fn ashrae_simple_folds_in_radiation() {
        // 10.79 + 4.192 * 4, and no separate sky, ground or air radiation.
        assert!((wall(0, ASHRAE_SIMPLE, 10.0, 0.0) - (10.79 + 4.192 * 4.0)).abs() < 1e-12);
        assert_eq!(wall(1, ASHRAE_SIMPLE, 10.0, 0.0), 0.0);
        assert_eq!(wall(3, ASHRAE_SIMPLE, 10.0, 0.0), 0.0);
    }

    #[test]
    fn face_at_air_temperature_has_no_ground_or_air_radiation() {
        assert_eq!(wall(2, TARP, 0.0, 0.0), 0.0);
        assert_eq!(wall(3, TARP, 0.0, 0.0), 0.0);
        assert!(wall(1, TARP, 0.0, 0.0) > 0.0);
    }

    #[test]
    fn user_value_and_guards() {
        assert_eq!(wall(0, VALUE, 10.0, 0.0), 7.0);
        assert!(wall(5, TARP, 10.0, 0.0).is_nan());
        assert!(wall(0, 1, 10.0, 0.0).is_nan());
    }
}
