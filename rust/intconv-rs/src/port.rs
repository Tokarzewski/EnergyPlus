//! The Rust port of `intconv.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `Convect::InitIntConvCoeff` for one heat transfer surface in a zone with no air system: the
//! inside convection coefficient by the surface's (or its zone's) model (a user value, ASHRAE
//! simple, TARP for opaque surfaces and ISO 15099 for windows, or ASTM C1340), each held above the
//! lower limit, then the EMS override and, for exterior windows, the frame adjustment ratio. The
//! correlations are `ConvectionCoefficients.cc`'s, each keeping the C++ order of operations. The
//! oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

const KELVIN: f64 = 273.15;
const ONE_THIRD: f64 = 1.0 / 3.0;
/// `Constant::DegToRad`: EnergyPlus's `Pi` literal rounds to the same double as `PI`.
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
/// The C++ converts one ASTM C1340 tilt with this rounded pi instead of `DegToRad`.
#[expect(
    clippy::approx_constant,
    reason = "CalcASTMC1340ConvCoeff writes 3.14159 here; bit-exact results need the same value"
)]
const ASTM_ROUNDED_PI: f64 = 3.14159;

/// `HcInt` values the port handles.
const VALUE: i32 = 0;
const ASHRAE_SIMPLE: i32 = 3;
const ASHRAE_TARP: i32 = 4;
const ASTM_C1340: i32 = 8;

fn calc_ashrae_simple_int_conv_coeff(tsurf: f64, tamb: f64, cos_tilt: f64) -> f64 {
    if cos_tilt.abs() < 0.3827 {
        return 3.076;
    }
    let delta_temp_cos_tilt = (tamb - tsurf) * cos_tilt;
    if cos_tilt.abs() >= 0.9239 {
        if delta_temp_cos_tilt > 0.0 {
            return 4.040;
        }
        if delta_temp_cos_tilt < 0.0 {
            return 0.948;
        }
        return 3.076;
    }
    if delta_temp_cos_tilt > 0.0 {
        return 3.870;
    }
    if delta_temp_cos_tilt < 0.0 {
        return 2.281;
    }
    3.076
}

fn calc_ashrae_tarp_natural(tsurf: f64, tamb: f64, cos_tilt: f64) -> f64 {
    let delta_temp = tsurf - tamb;
    let cube_root = delta_temp.abs().powf(1.0 / 3.0);
    if delta_temp == 0.0 || cos_tilt == 0.0 {
        return 1.31 * cube_root;
    }
    if (delta_temp < 0.0 && cos_tilt < 0.0) || (delta_temp > 0.0 && cos_tilt > 0.0) {
        return 9.482 * cube_root / (7.238 - cos_tilt.abs());
    }
    1.810 * cube_root / (1.382 + cos_tilt.abs())
}

/// ObjexxFCL `root_4`.
fn root_4(x: f64) -> f64 {
    x.sqrt().sqrt()
}

/// The ISO 15099 window inside convection coefficient (`CalcISO15099WindowIntConvCoeff`).
fn calc_iso15099_window_int_conv_coeff(s: &Surface, low_limit: f64) -> f64 {
    const G: f64 = 9.81;
    let surf_temp_kelvin = s.tsurf + KELVIN;
    let air_temp_kelvin = s.mat + KELVIN;
    let delta_temp = s.tsurf - s.mat;
    if !(200.0..=400.0).contains(&air_temp_kelvin) || !(180.0..=450.0).contains(&surf_temp_kelvin) {
        return low_limit;
    }
    let tmean_film_kelvin = air_temp_kelvin + 0.25 * delta_temp;
    let tmean_film = tmean_film_kelvin - 273.15;
    let w = s.humrat.max(1.0e-5);
    let rho = s.outbaro / (287.0 * (tmean_film + KELVIN) * (1.0 + 1.6077687 * w));
    let lambda = 2.873E-3 + 7.76E-5 * tmean_film_kelvin;
    let mu = 3.723E-6 + 4.94E-8 * tmean_film_kelvin;
    let cp = 1.00484e3 + w * 1.85895e3;
    let mut tilt_deg = s.tilt;
    if delta_temp > 0.0 {
        tilt_deg = 180.0 - tilt_deg;
    }
    let height = s.height;
    let ra_h = (rho * rho * (height * height * height) * G * cp * delta_temp.abs())
        / (tmean_film_kelvin * mu * lambda);
    let sine_tilt = s.sintilt;
    let nu_int = if (0.0..15.0).contains(&tilt_deg) {
        0.13 * ra_h.powf(ONE_THIRD)
    } else if (15.0..=90.0).contains(&tilt_deg) {
        let ra_cv = 2.5E+5 * ((0.72 * tilt_deg).exp() / sine_tilt).powf(0.2);
        if ra_h <= ra_cv {
            0.56 * root_4(ra_h * sine_tilt)
        } else {
            0.13 * (ra_h.powf(ONE_THIRD) - ra_cv.powf(ONE_THIRD)) + 0.56 * root_4(ra_cv * sine_tilt)
        }
    } else if 90.0 < tilt_deg && tilt_deg <= 179.0 {
        if ra_h * sine_tilt < 1.0E+5 {
            0.56 * root_4(1.0E+5)
        } else if ra_h * sine_tilt >= 1.0E+11 {
            0.56 * root_4(1.0E+11)
        } else {
            0.56 * root_4(ra_h * sine_tilt)
        }
    } else if 179.0 < tilt_deg && tilt_deg <= 180.0 {
        if ra_h > 1.0E+11 {
            0.58 * 1.0E+11_f64.powf(0.2)
        } else {
            0.58 * ra_h.powf(0.2)
        }
    } else {
        // The C++ asserts here and, without assertions, leaves the Nusselt number at 0.
        0.0
    };
    nu_int * lambda / height
}

/// `Grc`, the critical Grashof number for an upward heat flow on a tilted surface.
fn critical_grashof(tilt: f64, flat: bool, steep: bool) -> f64 {
    if flat {
        1_000_000.0
    } else if steep {
        5_000_000_000.0
    } else {
        10.0_f64.powf(tilt / (1.1870 + (0.0870 * tilt)))
    }
}

/// `CalcASTMC1340ConvCoeff` for air speed `vair` (or the wind, for a surface facing outside).
fn calc_astm_c1340_conv_coeff(s: &Surface, vair: f64) -> f64 {
    const G: f64 = 9.807;
    let (tsurf, tair, tilt) = (s.tsurf, s.mat, s.tilt);
    let l = if tilt == 0.0 || tilt == 180.0 {
        s.area.sqrt()
    } else {
        s.height
    };
    let v = if s.exterior { s.outwind } else { vair };
    let pr = 0.7880 - (2.631 * 1e-4 * (tair + 273.15));
    let beta_si = 1.0 / (tair + 273.15);
    let rho_si = (22.0493 / (tair + 273.15)) * 16.0;
    let cp_si = 0.068559 * (3.4763 + (1.066 * 1e-4 * (tair + 273.15))) * 4186.8;
    let dv = (241.9 * 1e-7) * (145.8 * (tair + 273.15) * (tair + 273.15).powf(0.5))
        / ((tair + 273.15) + 110.4);
    let visc = dv * (0.45359237 / (0.3048 * 3600.0)) / rho_si;
    let k_si_n = 0.6325 * 1e-5 * (tair + 273.15).powf(0.5) * 241.77;
    let k_si_d = 1.0 + (245.4 * 10.0_f64.powf(-12.0 / (tair + 273.15))) / (tair + 273.15);
    let k_si = 1.730735 * (k_si_n / k_si_d);
    let delta_temp = tsurf - tair;
    let ra = (G * beta_si * rho_si * cp_si * delta_temp * (l * l * l)).abs() / (visc * k_si);
    let re = (v * l) / visc;
    let tilted = |sin_tilt: f64, grc: f64| {
        if (ra / pr) <= grc {
            0.56 * (ra * sin_tilt).powf(0.25)
        } else {
            0.14 * (ra.powf(ONE_THIRD) - (grc * pr).powf(ONE_THIRD))
                + 0.56 * (grc * pr * (tilt * DEG_TO_RAD).sin()).powf(0.25)
        }
    };
    let horizontal_up = || {
        if ra < 8_000_000.0 {
            0.54 * ra.powf(0.25)
        } else {
            0.15 * ra.powf(1.0 / 3.0)
        }
    };
    let nun = if tilt == 0.0 {
        if delta_temp > 0.0 {
            0.58 * ra.powf(0.2)
        } else {
            horizontal_up()
        }
    } else if tilt > 0.0 && tilt < 90.0 {
        if delta_temp > 0.0 {
            if tilt < 2.0 {
                0.58 * ra.powf(0.2)
            } else {
                0.56 * (ra * (tilt * DEG_TO_RAD).sin()).powf(0.25)
            }
        } else {
            tilted(
                (tilt * ASTM_ROUNDED_PI / 180.0).sin(),
                critical_grashof(tilt, tilt < 15.0, tilt > 75.0),
            )
        }
    } else if tilt == 180.0 {
        if delta_temp <= 0.0 {
            0.58 * ra.powf(0.2)
        } else {
            horizontal_up()
        }
    } else if tilt > 90.0 && tilt < 180.0 {
        if delta_temp <= 0.0 {
            if tilt > 178.0 {
                0.58 * ra.powf(0.2)
            } else {
                0.56 * (ra * (tilt * DEG_TO_RAD).sin()).powf(0.25)
            }
        } else {
            tilted(
                (tilt * DEG_TO_RAD).sin(),
                critical_grashof(tilt, tilt > 165.0, tilt > 105.0),
            )
        }
    } else if ra < 1_000_000_000.0 {
        0.59 * ra.powf(0.25)
    } else {
        0.10 * ra.powf(1.0 / 3.0)
    };
    let nuf = if re < 500_000.0 {
        0.664 * pr.powf(1.0 / 3.0) * re.powf(0.5)
    } else {
        pr.powf(1.0 / 3.0) * ((0.037 * re.powf(0.8)) - 850.0)
    };
    let hf = nuf * k_si / l;
    let hn = nun * k_si / l;
    (hf.powf(3.0) + hn.powf(3.0)).powf(1.0 / 3.0)
}

/// One surface and its zone as `InitIntConvCoeff` reads them.
struct Surface {
    model: i32,
    tsurf: f64,
    mat: f64,
    humrat: f64,
    cos_tilt: f64,
    tilt: f64,
    sintilt: f64,
    height: f64,
    area: f64,
    window: bool,
    exterior: bool,
    outwind: f64,
    outbaro: f64,
    volume: f64,
    hvalue: f64,
    ems: Option<f64>,
    adj_ratio: f64,
}

/// `SurfHConvInt` after `InitIntConvCoeff`.
fn init_int_conv_coeff(s: &Surface, low_limit: f64) -> f64 {
    let floor = |h: f64| if h < low_limit { low_limit } else { h };
    let mut h = match s.model {
        VALUE => floor(s.hvalue),
        ASHRAE_SIMPLE => {
            let h = calc_ashrae_simple_int_conv_coeff(s.tsurf, s.mat, s.cos_tilt);
            // ObjexxFCL `max`, then the dispatcher's own limit.
            floor(if h < low_limit { low_limit } else { h })
        }
        ASHRAE_TARP => {
            if s.window {
                let mut h = calc_iso15099_window_int_conv_coeff(s, low_limit);
                match s.ems {
                    Some(v) => h = v,
                    None => h *= s.adj_ratio,
                }
                floor(floor(h))
            } else {
                floor(floor(calc_ashrae_tarp_natural(s.tsurf, s.mat, -s.cos_tilt)))
            }
        }
        ASTM_C1340 => {
            // A zone with no air system has no air change rate.
            let vair = s.volume.powf(1.0 / 3.0) * 0.0 / 3600.0;
            floor(calc_astm_c1340_conv_coeff(s, vair))
        }
        // The exported function refuses other models before calling this.
        _ => f64::NAN,
    };
    if let Some(v) = s.ems {
        h = v;
    }
    if s.window && s.exterior {
        h *= s.adj_ratio;
    }
    h
}

/// `SurfHConvInt` of the surface after `InitIntConvCoeff` (see the oracle's shim for the
/// arguments).
pub fn IntConvCoeff(
    model: i32,
    setbyzone: i32,
    tsurf: f64,
    mat: f64,
    humrat: f64,
    costilt: f64,
    tilt: f64,
    sintilt: f64,
    height: f64,
    area: f64,
    window: i32,
    extbound: i32,
    outwind: f64,
    outbaro: f64,
    volume: f64,
    hvalue: f64,
    emsover: i32,
    emsval: f64,
    adjratio: f64,
    lowlimit: f64,
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    let known = matches!(model, VALUE | ASHRAE_SIMPLE | ASHRAE_TARP | ASTM_C1340);
    if !known
        || ![setbyzone, window, extbound, emsover].into_iter().all(flag)
        || (setbyzone == 1 && model == VALUE)
    {
        return f64::NAN;
    }
    let s = Surface {
        model,
        tsurf,
        mat,
        humrat,
        cos_tilt: costilt,
        tilt,
        sintilt,
        height,
        area,
        window: window == 1,
        exterior: extbound == 0,
        outwind,
        outbaro,
        volume,
        hvalue,
        ems: (emsover == 1).then_some(emsval),
        adj_ratio: adjratio,
    };
    init_int_conv_coeff(&s, lowlimit)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A wall (`window` 0) or window facing outside, at `tsurf` in 20 C air, tilt `tilt`.
    fn surf(model: i32, window: i32, tsurf: f64, tilt: f64) -> f64 {
        let r = tilt.to_radians();
        let (c, s) = if tilt == 90.0 {
            (0.0, 1.0)
        } else {
            (r.cos(), r.sin())
        };
        IntConvCoeff(
            model, 0, tsurf, 20.0, 0.008, c, tilt, s, 2.5, 10.0, window, 0, 3.0, 101325.0, 300.0,
            4.0, 0, 0.0, 1.0, 0.1,
        )
    }

    #[test]
    fn ashrae_simple_vertical_is_constant() {
        assert_eq!(surf(ASHRAE_SIMPLE, 0, 10.0, 90.0), 3.076);
    }

    #[test]
    fn tarp_vertical_wall_uses_the_ashrae_correlation() {
        let h = surf(ASHRAE_TARP, 0, 12.0, 90.0);
        assert!((h - 1.31 * 8.0_f64.powf(1.0 / 3.0)).abs() < 1e-12, "{h}");
    }

    #[test]
    fn face_at_air_temperature_falls_to_the_lower_limit() {
        assert_eq!(surf(ASHRAE_TARP, 0, 20.0, 90.0), 0.1);
    }

    #[test]
    fn iso15099_window_gives_a_plausible_coefficient() {
        let h = surf(ASHRAE_TARP, 1, 5.0, 90.0);
        assert!(h > 1.0 && h < 5.0, "{h}");
    }

    #[test]
    fn user_value_and_guards() {
        assert_eq!(surf(VALUE, 0, 12.0, 90.0), 4.0);
        assert!(surf(1, 0, 12.0, 90.0).is_nan());
    }
}
