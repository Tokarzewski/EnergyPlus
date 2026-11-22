//! The Rust port of `solarpos.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Sun position and sky helpers from EnergyPlus `WeatherManager.cc`: `CalcSkyEmissivity`,
//! `ASHRAETauModel`, `CalculateDailySolarCoeffs`, `CalculateSunDirectionCosines`,
//! `DetermineSunUpDown`, `WaterMainsTempFromCorrelation` and `DayltgLuminousEfficacy`, in the
//! C++'s operation order. `min` and `max` are ObjexxFCL's (`a < b ? a : b`, `a < b ? b : a`);
//! the saturation pressure comes from `psychrometrics-rs` (with its cache) and the air mass from
//! `weather-rs`.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

use psychrometrics_rs::port::PsyPsatFnTemp;
use std::f64::consts::PI;
use weather_rs::port::AirMass;

const DEG_TO_RAD: f64 = PI / 180.0; // `Constant::DegToRad`
const KELVIN: f64 = 273.15; // `Constant::Kelvin`
const SUN_IS_UP: f64 = 0.00001; // `DataEnvironment::SunIsUpValue`

fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The site's latitude sine and cosine, as `CheckLocationValidity` computes them.
fn sin_cos_lat(lat: f64) -> (f64, f64) {
    ((DEG_TO_RAD * lat).sin(), (DEG_TO_RAD * lat).cos())
}

/// `CalcSkyEmissivity` (`WeatherManager.cc:3188`): model 0 Clark and Allen, 1 Brunt, 2 Idso,
/// 3 Berdahl and Martin.
pub fn SkyEmissivity(model: i32, osky: f64, drybulb: f64, dewpoint: f64, relhum: f64) -> f64 {
    let e_sky = match model {
        1 => 0.618 + 0.056 * (relhum * PsyPsatFnTemp(drybulb) * 0.01).powf(0.5),
        2 => {
            0.685
                + 0.000032
                    * (relhum * PsyPsatFnTemp(drybulb) * 0.01)
                    * (1699.0 / (drybulb + KELVIN)).exp()
        }
        3 => {
            let t = min(drybulb, dewpoint) / 100.0;
            0.758 + 0.521 * t + 0.625 * (t * t)
        }
        0 => 0.787 + 0.764 * ((min(drybulb, dewpoint) + KELVIN) / KELVIN).ln(),
        _ => return f64::NAN,
    };
    e_sky * (1.0 + 0.0224 * osky - 0.0035 * (osky * osky) + 0.00028 * (osky * osky * osky))
}

/// `ASHRAETauModel` (`WeatherManager.cc:4061`): beam normal, diffuse and global horizontal.
fn tau_model(tau2017: bool, etr: f64, cos_zen: f64, tau_b: f64, tau_d: f64) -> [f64; 3] {
    if cos_zen < SUN_IS_UP || tau_b <= 0.0 || tau_d <= 0.0 {
        return [0.0; 3];
    }
    let (ab, ad) = if tau2017 {
        (
            1.454 - 0.406 * tau_b - 0.268 * tau_d + 0.021 * tau_b * tau_d,
            0.507 + 0.205 * tau_b - 0.080 * tau_d - 0.190 * tau_b * tau_d,
        )
    } else {
        (
            1.219 - 0.043 * tau_b - 0.151 * tau_d - 0.204 * tau_b * tau_d,
            0.202 + 0.852 * tau_b - 0.007 * tau_d - 0.357 * tau_b * tau_d,
        )
    };
    let m = AirMass(cos_zen);
    let dir_n = etr * (-tau_b * m.powf(ab)).exp();
    let dif_h = etr * (-tau_d * m.powf(ad)).exp();
    [dir_n, dif_h, dir_n * cos_zen + dif_h]
}

pub fn TauModel(which: i32, tau2017: i32, etr: f64, cosZen: f64, taub: f64, taud: f64) -> f64 {
    let r = tau_model(tau2017 != 0, etr, cosZen, taub, taud);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// The nine-term Fourier series `CalculateDailySolarCoeffs` writes out for each coefficient.
fn series(k: &[f64; 9], s: f64, c: f64) -> f64 {
    k[0] + k[1] * s
        + k[2] * c
        + k[3] * (s * c * 2.0)
        + k[4] * (c * c - s * s)
        + k[5] * (s * (c * c - s * s) + c * (s * c * 2.0))
        + k[6] * (c * (c * c - s * s) - s * (s * c * 2.0))
        + k[7] * (2.0 * (s * c * 2.0) * (c * c - s * s))
        + k[8] * ((c * c - s * s) * (c * c - s * s) - (s * c * 2.0) * (s * c * 2.0))
}

/// `CalculateDailySolarCoeffs` (`WeatherManager.cc:4128`): A, B, C, the annual variation of the
/// solar constant, the equation of time, and the sine and cosine of the declination.
fn daily_solar(doy: i32, lat: f64) -> [f64; 7] {
    const SINE_DECL: [f64; 9] = [
        0.00561800,
        0.0657911,
        -0.392779,
        0.00064440,
        -0.00618495,
        -0.00010101,
        -0.00007951,
        -0.00011691,
        0.00002096,
    ];
    const EQ_OF_TIME: [f64; 9] = [
        0.00021971,
        -0.122649,
        0.00762856,
        -0.156308,
        -0.0530028,
        -0.00388702,
        -0.00123978,
        -0.00270502,
        -0.00167992,
    ];
    const A: [f64; 9] = [
        1161.6685, 1.1554, 77.3575, -0.5359, -3.7622, 0.9875, -3.3924, -1.7445, 1.1198,
    ];
    const B: [f64; 9] = [
        0.171631,
        -0.00400448,
        -0.0344923,
        0.00000209,
        0.00325428,
        -0.00085429,
        0.00229562,
        0.0009034,
        -0.0011867,
    ];
    const C: [f64; 9] = [
        0.0905151,
        -0.00322522,
        -0.0407966,
        0.000104164,
        0.00745899,
        -0.00086461,
        0.0013111,
        0.000808275,
        -0.00170515,
    ];
    let mut x = PI * 2.0 / 366.0 * f64::from(doy);
    let (mut s, mut c) = (x.sin(), x.cos());
    let sin_decl = series(&SINE_DECL, s, c);
    let cos_decl = (1.0 - sin_decl * sin_decl).sqrt();
    let eq_of_time = series(&EQ_OF_TIME, s, c);
    let ann_var = 1.000047 + 0.000352615 * s + 0.0334454 * c;
    let a = series(&A, s, c);
    if lat < 0.0 {
        x -= PI;
        (s, c) = (x.sin(), x.cos());
    }
    [
        a,
        series(&B, s, c),
        series(&C, s, c),
        ann_var,
        eq_of_time,
        sin_decl,
        cos_decl,
    ]
}

pub fn DailySolar(which: i32, doy: i32, lat: f64) -> f64 {
    let r = daily_solar(doy, lat);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `CalculateSunDirectionCosines` (`WeatherManager.cc:4236`).
fn sun_cosines(
    time: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> [f64; 3] {
    let (sin_lat, cos_lat) = sin_cos_lat(lat);
    let h = (15.0 * (12.0 - (time + eqt)) + (tzm - lon)) * DEG_TO_RAD;
    let cos_h = h.cos();
    let z = sind * sin_lat + cosd * cos_lat * cos_h;
    if z >= SUN_IS_UP {
        [cosd * h.sin(), sind * cos_lat - cosd * sin_lat * cos_h, z]
    } else {
        [0.0, 0.0, z]
    }
}

pub fn SunCosines(
    which: i32,
    time: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> f64 {
    let r = sun_cosines(time, eqt, sind, cosd, lat, lon, tzm);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `DetermineSunUpDown` (`WeatherManager.cc:4273`): the sun's direction cosines, the hour angle,
/// the altitude and azimuth in degrees, and whether the sun is up (1 or 0).
fn sun_up_down(
    ts_in_hour: i32,
    cur_time: f64,
    ts1off: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> [f64; 7] {
    let (sin_lat, cos_lat) = sin_cos_lat(lat);
    let hr_angle = if ts_in_hour != 1 {
        15.0 * (12.0 - (cur_time + eqt)) + (tzm - lon)
    } else {
        15.0 * (12.0 - ((cur_time + ts1off) + eqt)) + (tzm - lon)
    };
    let h = hr_angle * DEG_TO_RAD;
    let cos_zenith = sin_lat * sind + cos_lat * cosd * h.cos();
    let zenith = cos_zenith.acos();
    let altitude = (cos_lat * cosd * h.cos() + sin_lat * sind).asin();
    let cos_az = -(sin_lat * cos_zenith - sind) / (cos_lat * zenith.sin());
    let cos_az = min(1.0, max(cos_az, -1.0));
    let mut azimuth = cos_az.acos() / DEG_TO_RAD;
    if hr_angle < 0.0 {
        azimuth = 360.0 - azimuth;
    }
    let (x, y, up) = if cos_zenith < SUN_IS_UP {
        (0.0, 0.0, 0.0)
    } else {
        (
            cosd * h.sin(),
            sind * cos_lat - cosd * sin_lat * h.cos(),
            1.0,
        )
    };
    [
        x,
        y,
        cos_zenith,
        hr_angle,
        altitude / DEG_TO_RAD,
        azimuth,
        up,
    ]
}

pub fn SunUpDown(
    which: i32,
    tsInHour: i32,
    curTime: f64,
    ts1off: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> f64 {
    let r = sun_up_down(tsInHour, curTime, ts1off, eqt, sind, cosd, lat, lon, tzm);
    usize::try_from(which)
        .ok()
        .and_then(|w| r.get(w).copied())
        .unwrap_or(f64::NAN)
}

/// `WaterMainsTempFromCorrelation` (`WeatherManager.cc:7168`) on day `doy` at latitude `lat`.
pub fn WaterMains(doy: i32, lat: f64, annualAvg: f64, maxDiff: f64, mult: f64, offset: f64) -> f64 {
    let t_avg = annualAvg * (9.0 / 5.0) + 32.0;
    let t_diff = maxDiff * (9.0 / 5.0);
    let ratio = 0.4 + 0.01 * (t_avg - 44.0);
    let lag = 35.0 - 1.0 * (t_avg - 44.0);
    let latitude_sign = if lat >= 0.0 { 1.0 } else { -1.0 };
    let mut t = t_avg
        + 6.0
        + ratio
            * (t_diff / 2.0)
            * latitude_sign
            * ((0.986 * (f64::from(doy) - 15.0 - lag) - 90.0) * DEG_TO_RAD).sin();
    if t < 32.0 {
        t = 32.0;
    }
    t = (t - 32.0) * (5.0 / 9.0);
    t * mult + offset
}

/// `DayltgLuminousEfficacy` (`WeatherManager.cc:7339`): diffuse and beam luminous efficacy, sky
/// clearness and sky brightness, for month 1 to 12.
fn lum_eff(
    month: i32,
    cos_zen: f64,
    dif: f64,
    beam: f64,
    elev: f64,
    dewpoint: f64,
) -> Option<[f64; 4]> {
    const A_DIF: [f64; 8] = [
        97.24, 107.22, 104.97, 102.39, 100.71, 106.42, 141.88, 152.23,
    ];
    const B_DIF: [f64; 8] = [-0.46, 1.15, 2.96, 5.59, 5.94, 3.83, 1.90, 0.35];
    const C_DIF: [f64; 8] = [12.00, 0.59, -5.53, -13.95, -22.75, -36.15, -53.24, -45.27];
    const D_DIF: [f64; 8] = [-8.91, -3.95, -8.77, -13.90, -23.74, -28.83, -14.03, -7.98];
    const A_DIR: [f64; 8] = [57.20, 98.99, 109.83, 110.34, 106.36, 107.19, 105.75, 101.18];
    const B_DIR: [f64; 8] = [-4.55, -3.46, -4.90, -5.84, -3.97, -1.25, 0.77, 1.58];
    const C_DIR: [f64; 8] = [-2.98, -1.21, -1.71, -1.99, -1.75, -1.51, -1.26, -1.10];
    const D_DIR: [f64; 8] = [117.12, 12.38, -8.81, -4.56, -6.16, -26.73, -34.44, -8.29];
    const EXTRA_DIR_NORM_ILL: [f64; 12] = [
        131153.0, 130613.0, 128992.0, 126816.0, 124731.0, 123240.0, 122652.0, 123120.0, 124576.0,
        126658.0, 128814.0, 130471.0,
    ];
    const CLEARNESS_LIMIT: [f64; 7] = [1.065, 1.23, 1.50, 1.95, 2.80, 4.50, 6.20];
    let extra = *EXTRA_DIR_NORM_ILL.get(usize::try_from(month).ok()?.checked_sub(1)?)?;
    let zenith = cos_zen.acos();
    let altitude = PI / 2.0 - zenith; // `Constant::PiOvr2`
    let zeta = 1.041 * (zenith * zenith * zenith);
    let clearness = ((dif + beam) / (dif + 0.0001) + zeta) / (1.0 + zeta);
    let rel_air_mass = (1.0 - 0.1 * elev / 1000.0)
        / (altitude.sin() + 0.15 / (altitude / DEG_TO_RAD + 3.885).powf(1.253));
    let brightness = (dif * 93.73) * rel_air_mass / extra;
    let i = CLEARNESS_LIMIT
        .iter()
        .position(|&l| clearness <= l)
        .unwrap_or(7);
    let moisture = (0.07 * dewpoint - 0.075).exp();
    let (diff, dir) = if brightness <= 0.0 {
        (0.0, 0.0)
    } else {
        (
            A_DIF[i] + B_DIF[i] * moisture + C_DIF[i] * cos_zen + D_DIF[i] * brightness.ln(),
            max(
                0.0,
                A_DIR[i]
                    + B_DIR[i] * moisture
                    + C_DIR[i] * (5.73 * zenith - 5.0).exp()
                    + D_DIR[i] * brightness,
            ),
        )
    };
    Some([diff, dir, clearness, brightness])
}

pub fn LumEff(
    which: i32,
    month: i32,
    cosZen: f64,
    dif: f64,
    beam: f64,
    elev: f64,
    dewpoint: f64,
) -> f64 {
    let Some(r) = lum_eff(month, cosZen, dif, beam, elev, dewpoint) else {
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
    fn declination_peaks_near_the_solstices() {
        let june = daily_solar(172, 40.0)[5];
        let december = daily_solar(355, 40.0)[5];
        assert!(june > 0.39 && december < -0.39);
    }

    #[test]
    fn the_sun_is_overhead_at_noon_on_the_equator_at_an_equinox() {
        let r = sun_up_down(1, 12.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert!((r[2] - 1.0).abs() < 1e-12 && r[6] == 1.0);
    }
}
