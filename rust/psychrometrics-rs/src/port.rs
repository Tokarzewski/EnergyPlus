//! The Rust port of `psychrometrics.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Each function forwards, in the original, to EnergyPlus's `Psychrometrics.hh` / `.cc` through
//! one `EnergyPlusData`. The ports below keep the C++ operation order exactly (bit-exact), and use
//! `ObjexxFCL`'s `max`/`min` (`a < b ? b : a` / `a < b ? a : b`), not `f64::max`/`f64::min`, which
//! differ on NaN.
//!
//! State: the original keeps caches and "last value" saves in `EnergyPlusData` and in function
//! statics. Every recorded case runs in a freshly forked process, so each call sees that state as
//! initialised, and no stateful lookup happens more than once per call except the saturation
//! pressure cache, whose result is a pure function of its (truncated) key. The ports therefore
//! reproduce the fresh-state answers: a cache slot still holding its initial key returns its
//! initial value (`0.0`), and a "last value" save still holding `-99999` / `-100` returns that.
//!
//! `CSplineint` and its `tsat_fn_pb_*` tables are not ported: they run only when
//! `useInterpolationPsychTsatFnPb` is set, which only the input file can do and the shim never does.

#![forbid(unsafe_code)]
// The names and parameters keep the C++ spelling (`PsyTsatFnPb`, `TDB`, `PB`).
#![allow(non_snake_case)]

/// `Constant::Kelvin`.
const KELVIN: f64 = 273.15;
/// `Constant::TriplePointOfWaterTempKelvin`.
const TRIPLE_POINT_OF_WATER_TEMP_KELVIN: f64 = 273.16;
/// `PsychrometricsData::iconvTol`.
const ICONV_TOL: f64 = 0.0001;
/// The initial `last_Patm`, `last_tBoil`, `Press_Save` and `tSat_Save` of `PsychrometricsData`.
const UNSET_SAVE: f64 = -99999.0;
/// The initial `dwSave` / `cpaSave` statics of `PsyCpAirFnW` and `PsyCpAirFnW_fast`.
const UNSET_CP_SAVE: f64 = -100.0;

/// ObjexxFCL `max(double, double)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// ObjexxFCL `min(double, double)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// libstdc++ `std::clamp(v, lo, hi)`: `std::min(std::max(v, lo), hi)`.
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    let m = if v < lo { lo } else { v };
    if hi < m { hi } else { m }
}

/// The validity test `(RHValue < 0.0) || (RHValue > 1.0)`, false for NaN.
// A range `contains` would read the same but hides that NaN counts as in range here.
#[allow(clippy::manual_range_contains)]
fn outside_unit(v: f64) -> bool {
    v < 0.0 || v > 1.0
}

/// The bit pattern of `x` as `Int64`, shifted right arithmetically (the C++ cache tags).
fn tag_i64(x: f64, shift: u32) -> i64 {
    (x.to_bits() as i64) >> shift
}

/// A cache tag shifted back into place and read as a double.
fn untag_i64(tag: i64, shift: u32) -> f64 {
    f64::from_bits((tag << shift) as u64)
}

/// `F6`: a fifth-order polynomial in Horner form.
fn f6(x: f64, a: &[f64; 6]) -> f64 {
    a[0] + x * (a[1] + x * (a[2] + x * (a[3] + x * (a[4] + x * a[5]))))
}

/// `F7`: a sixth-order polynomial in Horner form, divided by 1e10.
fn f7(x: f64, a: &[f64; 7]) -> f64 {
    (a[0] + x * (a[1] + x * (a[2] + x * (a[3] + x * (a[4] + x * (a[5] + x * a[6])))))) / 1.0E10
}

/// `General::Iterate`: one secant step. Returns `(ResultX, converged)` and updates `x1`/`y1`.
fn iterate(tol: f64, x0: f64, y0: f64, x1: &mut f64, y1: &mut f64, iter: i32) -> (f64, bool) {
    const SMALL: f64 = 1.0e-9;
    const PERTURB: f64 = 0.1;
    if iter != 1 && ((x0 - *x1).abs() < tol || y0 == 0.0) {
        return (x0, true);
    }
    let result = if iter == 1 {
        if x0.abs() > SMALL {
            x0 * (1.0 + PERTURB)
        } else {
            PERTURB
        }
    } else {
        let mut dy = y0 - *y1;
        if dy.abs() < SMALL {
            dy = SMALL;
        }
        (y0 * *x1 - *y1 * x0) / dy
    };
    *x1 = x0;
    *y1 = y0;
    (result, false)
}

/// Port of `double PsyRhoAirFnPbTdbW(double pb, double tdb, double dw)` (`psychrometrics.c:44`).
///
/// A negative density is a fatal error in the original (the process aborts); this returns it.
pub fn PsyRhoAirFnPbTdbW(pb: f64, tdb: f64, dw: f64) -> f64 {
    pb / (287.0 * (tdb + KELVIN) * (1.0 + 1.6077687 * max(dw, 1.0e-5)))
}

/// Port of `double PsyRhoAirFnPbTdbW_fast(double pb, double tdb, double dw)` (`psychrometrics.c:49`).
pub fn PsyRhoAirFnPbTdbW_fast(pb: f64, tdb: f64, dw: f64) -> f64 {
    pb / (287.0 * (tdb + KELVIN) * (1.0 + 1.6077687 * dw))
}

/// Port of `double PsyHfgAirFnWTdb(double w, double T)` (`psychrometrics.c:54`).
pub fn PsyHfgAirFnWTdb(_w: f64, T: f64) -> f64 {
    let temperature = max(T, 0.0);
    (2500940.0 + 1858.95 * temperature) - (4180.0 * temperature)
}

/// Port of `double PsyHgAirFnWTdb(double w, double T)` (`psychrometrics.c:59`).
pub fn PsyHgAirFnWTdb(_w: f64, T: f64) -> f64 {
    2500940.0 + 1858.95 * T
}

/// Port of `double PsyHFnTdbW(double TDB, double dW)` (`psychrometrics.c:64`).
pub fn PsyHFnTdbW(TDB: f64, dW: f64) -> f64 {
    1.00484e3 * TDB + max(dW, 1.0e-5) * (2.50094e6 + 1.85895e3 * TDB)
}

/// Port of `double PsyHFnTdbW_fast(double TDB, double dW)` (`psychrometrics.c:69`).
pub fn PsyHFnTdbW_fast(TDB: f64, dW: f64) -> f64 {
    1.00484e3 * TDB + dW * (2.50094e6 + 1.85895e3 * TDB)
}

/// Port of `double PsyCpAirFnW(double dw)` (`psychrometrics.c:74`).
///
/// The original returns its saved result when `dw` equals the saved input, which starts at
/// `-100` with a saved result of `-100`.
pub fn PsyCpAirFnW(dw: f64) -> f64 {
    if dw == UNSET_CP_SAVE {
        return UNSET_CP_SAVE;
    }
    let w = max(dw, 1.0e-5);
    1.00484e3 + w * 1.85895e3
}

/// Port of `double PsyCpAirFnW_fast(double dw)` (`psychrometrics.c:79`).
pub fn PsyCpAirFnW_fast(dw: f64) -> f64 {
    if dw == UNSET_CP_SAVE {
        return UNSET_CP_SAVE;
    }
    1.00484e3 + dw * 1.85895e3
}

/// Port of `double PsyTdbFnHW(double H, double dW)` (`psychrometrics.c:84`).
pub fn PsyTdbFnHW(H: f64, dW: f64) -> f64 {
    let w = max(dW, 1.0e-5);
    (H - 2.50094e6 * w) / (1.00484e3 + 1.85895e3 * w)
}

/// Port of `double PsyRhovFnTdbRhLBnd0C(double Tdb, double RH)` (`psychrometrics.c:89`).
pub fn PsyRhovFnTdbRhLBnd0C(Tdb: f64, RH: f64) -> f64 {
    RH / (461.52 * (Tdb + KELVIN)) * (23.7093 - 4111.0 / ((Tdb + KELVIN) - 35.45)).exp()
}

/// Port of `double PsyRhovFnTdbWPb(double Tdb, double dW, double PB)` (`psychrometrics.c:94`).
pub fn PsyRhovFnTdbWPb(Tdb: f64, dW: f64, PB: f64) -> f64 {
    let w = max(dW, 1.0e-5);
    w * PB / (461.52 * (Tdb + KELVIN) * (w + 0.62198))
}

/// Port of `double PsyRhovFnTdbWPb_fast(double Tdb, double dW, double PB)` (`psychrometrics.c:99`).
pub fn PsyRhovFnTdbWPb_fast(Tdb: f64, dW: f64, PB: f64) -> f64 {
    dW * PB / (461.52 * (Tdb + KELVIN) * (dW + 0.62198))
}

/// Port of `double PsyRhFnTdbRhovLBnd0C(double Tdb, double Rhovapor)` (`psychrometrics.c:104`).
pub fn PsyRhFnTdbRhovLBnd0C(Tdb: f64, Rhovapor: f64) -> f64 {
    let rh = if Rhovapor > 0.0 {
        Rhovapor * 461.52 * (Tdb + KELVIN) * (-23.7093 + 4111.0 / ((Tdb + KELVIN) - 35.45)).exp()
    } else {
        0.0
    };
    if outside_unit(rh) {
        return clamp(rh, 0.01, 1.0);
    }
    rh
}

/// Port of `double PsyTwbFnTdbWPb(double TDB, double dW, double Patm)` (`psychrometrics.c:109`).
///
/// The cached variant: each input keeps its top 32 bits, and `_raw` runs on those truncated
/// values. A cache slot starts with all-zero keys and a result of `0.0`.
pub fn PsyTwbFnTdbWPb(TDB: f64, dW: f64, Patm: f64) -> f64 {
    const GRID_SHIFT: u32 = 64 - 12 - 20;
    let tdb_tag = TDB.to_bits() >> GRID_SHIFT;
    let w_tag = dW.to_bits() >> GRID_SHIFT;
    let pb_tag = Patm.to_bits() >> GRID_SHIFT;
    if tdb_tag == 0 && w_tag == 0 && pb_tag == 0 {
        return 0.0;
    }
    PsyTwbFnTdbWPb_raw(
        f64::from_bits(tdb_tag << GRID_SHIFT),
        f64::from_bits(w_tag << GRID_SHIFT),
        f64::from_bits(pb_tag << GRID_SHIFT),
    )
}

/// Port of `double PsyTwbFnTdbWPb_raw(double TDB, double dW, double Patm)` (`psychrometrics.c:114`).
pub fn PsyTwbFnTdbWPb_raw(TDB: f64, dW: f64, Patm: f64) -> f64 {
    const ITMAX: i32 = 100;
    let mut w = dW;
    if w < 0.0 {
        w = 1.0e-5;
    }

    // Initial temperature guess at atmospheric pressure (`last_Patm` starts unset).
    let t_boil = if Patm != UNSET_SAVE {
        PsyTsatFnPb(Patm)
    } else {
        UNSET_SAVE
    };

    let mut wbt = TDB;
    let mut x1 = 0.0;
    let mut y1 = 0.0;
    for iter in 1..=ITMAX {
        if wbt >= (t_boil - 0.09) {
            wbt = t_boil - 0.1;
        }
        let p_sat_star = PsyPsatFnTemp(wbt);
        let w_star = 0.62198 * p_sat_star / (Patm - p_sat_star);
        let new_w = if wbt >= 0.0 {
            ((2501.0 - 2.326 * wbt) * w_star - 1.006 * (TDB - wbt))
                / (2501.0 + 1.86 * TDB - 4.186 * wbt)
        } else {
            ((2830.0 - 0.24 * wbt) * w_star - 1.006 * (TDB - wbt))
                / (2830.0 + 1.86 * TDB - 2.1 * wbt)
        };
        let error = w - new_w;
        let (result, converged) = iterate(ICONV_TOL, wbt, error, &mut x1, &mut y1, iter);
        wbt = result;
        if converged {
            break;
        }
    }

    let mut twb = wbt;
    if twb > TDB {
        twb = TDB;
    }
    twb
}

/// Port of `double PsyVFnTdbWPb(double TDB, double dW, double PB)` (`psychrometrics.c:119`).
pub fn PsyVFnTdbWPb(TDB: f64, dW: f64, PB: f64) -> f64 {
    let w = max(dW, 1.0e-5);
    let v = 1.59473e2 * (1.0 + 1.6078 * w) * (1.8 * TDB + 492.0) / PB;
    if v < 0.0 {
        return 0.83;
    }
    v
}

/// Port of `double PsyWFnTdbH(double TDB, double H)` (`psychrometrics.c:124`).
pub fn PsyWFnTdbH(TDB: f64, H: f64) -> f64 {
    let w = (H - 1.00484e3 * TDB) / (2.50094e6 + 1.85895e3 * TDB);
    if w < 0.0 {
        return 1.0e-5;
    }
    w
}

/// Port of `double PsyPsatFnTemp(double T)` (`psychrometrics.c:129`).
///
/// The cached variant: `T` keeps its top 36 bits and `_raw` runs on the truncated value. A cache
/// slot starts with the key `-1000` and a result of `0.0`.
pub fn PsyPsatFnTemp(T: f64) -> f64 {
    const GRID_SHIFT: u32 = 64 - 12 - 24;
    let tdb_tag = tag_i64(T, GRID_SHIFT);
    if tdb_tag == -1000 {
        return 0.0;
    }
    PsyPsatFnTemp_raw(untag_i64(tdb_tag, GRID_SHIFT))
}

/// Port of `double PsyPsatFnTemp_raw(double T)` (`psychrometrics.c:134`).
pub fn PsyPsatFnTemp_raw(T: f64) -> f64 {
    let tkel = T + KELVIN;
    if tkel < 173.15 {
        0.001405102123874164
    } else if tkel < TRIPLE_POINT_OF_WATER_TEMP_KELVIN {
        const C1: f64 = -5674.5359;
        const C2: f64 = 6.3925247;
        const C3: f64 = -0.9677843e-2;
        const C4: f64 = 0.62215701e-6;
        const C5: f64 = 0.20747825e-8;
        const C6: f64 = -0.9484024e-12;
        const C7: f64 = 4.1635019;
        (C1 / tkel + C2 + tkel * (C3 + tkel * (C4 + tkel * (C5 + C6 * tkel))) + C7 * tkel.ln())
            .exp()
    } else if tkel <= 473.15 {
        const C8: f64 = -5800.2206;
        const C9: f64 = 1.3914993;
        const C10: f64 = -0.048640239;
        const C11: f64 = 0.41764768e-4;
        const C12: f64 = -0.14452093e-7;
        const C13: f64 = 6.5459673;
        (C8 / tkel + C9 + tkel * (C10 + tkel * (C11 + tkel * C12)) + C13 * tkel.ln()).exp()
    } else {
        1555073.745636215
    }
}

/// Port of `double PsyTsatFnHPb(double H, double PB)` (`psychrometrics.c:139`).
///
/// The cached variant: keys are the inputs' top 40 bits, but `_raw` runs on the inputs as given.
/// A cache slot starts with both keys `0` and a result of `0.0`.
pub fn PsyTsatFnHPb(H: f64, PB: f64) -> f64 {
    const GRID_SHIFT: u32 = 64 - 12 - 28;
    if tag_i64(H, GRID_SHIFT) == 0 && tag_i64(PB, GRID_SHIFT) == 0 {
        return 0.0;
    }
    PsyTsatFnHPb_raw(H, PB)
}

/// Port of `double PsyTsatFnHPb_raw(double H, double PB)` (`psychrometrics.c:144`).
pub fn PsyTsatFnHPb_raw(H: f64, PB: f64) -> f64 {
    const CASE_RANGE: [f64; 10] = [
        -4.24e4, -2.2138e4, -6.7012e2, 2.7297e4, 7.5222e4, 1.8379e5, 4.7577e5, 1.5445e6, 3.8353e6,
        4.5866e7,
    ];
    let mut hh = H + 1.78637e4;
    let hloc = if H >= 0.0 {
        max(0.00001, H)
    } else {
        min(-0.00001, H)
    };

    let mut beg = 0usize;
    let mut end = 9usize;
    while beg + 1 < end {
        let mid = (beg + end) >> 1;
        if hh > CASE_RANGE[mid] {
            beg = mid;
        } else {
            end = mid;
        }
    }

    let mut t = match beg + 1 {
        1 => {
            if hh < -4.24e4 {
                hh = -4.24e4;
            }
            f6(
                hh,
                &[
                    -19.44,
                    8.53675e-4,
                    -5.12637e-9,
                    -9.85546e-14,
                    -1.00102e-18,
                    -4.2705e-24,
                ],
            )
        }
        2 => f6(
            hh,
            &[
                -1.94224e1,
                8.5892e-4,
                -4.50709e-9,
                -6.19492e-14,
                8.71734e-20,
                8.73051e-24,
            ],
        ),
        3 => f6(
            hh,
            &[
                -1.94224e1,
                8.59061e-4,
                -4.4875e-9,
                -5.76696e-14,
                7.72217e-19,
                3.97894e-24,
            ],
        ),
        4 => f6(
            hh,
            &[
                -2.01147e1,
                9.04936e-4,
                -6.83305e-9,
                2.3261e-14,
                7.27237e-20,
                -6.31939e-25,
            ],
        ),
        5 => f6(
            hh,
            &[
                -1.82124e1,
                8.31683e-4,
                -6.16461e-9,
                3.06411e-14,
                -8.60964e-20,
                1.03003e-25,
            ],
        ),
        6 => f6(
            hh,
            &[
                -1.29419,
                3.88538e-4,
                -1.30237e-9,
                2.78254e-15,
                -3.27225e-21,
                1.60969e-27,
            ],
        ),
        7 => f6(
            hh,
            &[
                2.39214e1,
                1.27519e-4,
                -1.52089e-10,
                1.1043e-16,
                -4.33919e-23,
                7.05296e-30,
            ],
        ),
        8 => f6(
            hh,
            &[
                4.88446e1,
                3.85534e-5,
                -1.78805e-11,
                4.87224e-18,
                -7.15283e-25,
                4.36246e-32,
            ],
        ),
        _ => {
            if hh > 4.5866e7 {
                hh = 4.5866e7;
            }
            f7(
                hh,
                &[
                    7.60565e11,
                    5.80534e4,
                    -7.36433e-3,
                    5.11531e-10,
                    -1.93619e-17,
                    3.70511e-25,
                    -2.77313e-33,
                ],
            )
        }
    };

    if (PB - 1.0133e5).abs() / 1.0133e5 > 0.01 {
        let mut iter_count = 0;
        let mut t1 = t;
        let h1 = PsyHFnTdbW(t1, PsyWFnTdbTwbPb(t1, t1, PB));
        let mut y1 = h1 - hloc;
        if (y1 / hloc).abs() <= 0.1e-4 {
            t = t1;
        } else {
            let mut t2 = t1 * 0.9;
            while iter_count <= 30 {
                iter_count += 1;
                let h2 = PsyHFnTdbW(t2, PsyWFnTdbTwbPb(t2, t2, PB));
                let y2 = h2 - hloc;
                if (y2 / hloc).abs() <= 0.1e-4 || y2 == y1 {
                    t = t2;
                    break;
                }
                let tn = t2 - y2 / (y2 - y1) * (t2 - t1);
                t1 = t2;
                t2 = tn;
                y1 = y2;
            }
        }
    }
    t
}

/// Port of `double PsyRhovFnTdbRh(double Tdb, double RH)` (`psychrometrics.c:149`).
pub fn PsyRhovFnTdbRh(Tdb: f64, RH: f64) -> f64 {
    (PsyPsatFnTemp(Tdb) * RH) / (461.52 * (Tdb + KELVIN))
}

/// Port of `double PsyRhFnTdbRhov(double Tdb, double Rhovapor)` (`psychrometrics.c:154`).
pub fn PsyRhFnTdbRhov(Tdb: f64, Rhovapor: f64) -> f64 {
    let rh = if Rhovapor > 0.0 {
        Rhovapor * 461.52 * (Tdb + KELVIN) / PsyPsatFnTemp(Tdb)
    } else {
        0.0
    };
    if outside_unit(rh) {
        return clamp(rh, 0.01, 1.0);
    }
    rh
}

/// Port of `double PsyRhFnTdbWPb(double TDB, double dW, double PB)` (`psychrometrics.c:159`).
pub fn PsyRhFnTdbWPb(TDB: f64, dW: f64, PB: f64) -> f64 {
    let pws = PsyPsatFnTemp(TDB);
    let w = max(dW, 1.0e-5);
    let u = w / (0.62198 * pws / (PB - pws));
    let rh = u / (1.0 - (1.0 - u) * (pws / PB));
    if outside_unit(rh) {
        return clamp(rh, 0.01, 1.0);
    }
    rh
}

/// Port of `double PsyWFnTdpPb(double TDP, double PB)` (`psychrometrics.c:164`).
///
/// Like the original, the search for a dew point below `PB` has no iteration cap: it does not end
/// when no temperature gives a saturation pressure below `PB` (`PB` <= 0.0014 Pa, or `TDP` = inf).
pub fn PsyWFnTdpPb(TDP: f64, PB: f64) -> f64 {
    let pdew = PsyPsatFnTemp(TDP);
    let w = pdew * 0.62198 / (PB - pdew);
    if w < 0.0 {
        let mut delta_t = 0.0;
        let mut pdew1 = pdew;
        while pdew1 >= PB {
            delta_t += 1.0;
            pdew1 = PsyPsatFnTemp(TDP - delta_t);
        }
        return pdew1 * 0.62198 / (PB - pdew1);
    }
    w
}

/// Port of `double PsyWFnTdbRhPb(double TDB, double RH, double PB)` (`psychrometrics.c:169`).
pub fn PsyWFnTdbRhPb(TDB: f64, RH: f64, PB: f64) -> f64 {
    let pdew = RH * PsyPsatFnTemp(TDB);
    let w = pdew * 0.62198 / max(PB - pdew, 1000.0);
    if w < 1.0e-5 {
        return 1.0e-5;
    }
    w
}

/// Port of `double PsyWFnTdbTwbPb(double TDB, double TWBin, double PB)` (`psychrometrics.c:174`).
pub fn PsyWFnTdbTwbPb(TDB: f64, TWBin: f64, PB: f64) -> f64 {
    let mut twb = TWBin;
    if twb > TDB {
        twb = TDB;
    }
    let pwet = PsyPsatFnTemp(twb);
    let wet = 0.62198 * pwet / (PB - pwet);
    let w = ((2501.0 - 2.381 * twb) * wet - (TDB - twb)) / (2501.0 + 1.805 * TDB - 4.186 * twb);
    if w < 0.0 {
        return PsyWFnTdbRhPb(TDB, 0.0001, PB);
    }
    w
}

/// Port of `double PsyHFnTdbRhPb(double TDB, double RH, double PB)` (`psychrometrics.c:179`).
pub fn PsyHFnTdbRhPb(TDB: f64, RH: f64, PB: f64) -> f64 {
    PsyHFnTdbW(TDB, max(PsyWFnTdbRhPb(TDB, RH, PB), 1.0e-5))
}

/// Port of `double PsyTsatFnPb(double Press)` (`psychrometrics.c:184`).
///
/// The cached variant: the key is the input's top 36 bits, but `_raw` runs on the input as given.
/// A cache slot starts with the key `0` and a result of `0.0`.
pub fn PsyTsatFnPb(Press: f64) -> f64 {
    const GRID_SHIFT: u32 = 64 - 12 - 24;
    if tag_i64(Press, GRID_SHIFT) == 0 {
        return 0.0;
    }
    PsyTsatFnPb_raw(Press)
}

/// Port of `double PsyTsatFnPb_raw(double Press)` (`psychrometrics.c:189`).
///
/// The original returns its saved result when `Press` equals the saved input, which starts at
/// `-99999` with a saved result of `-99999`.
pub fn PsyTsatFnPb_raw(Press: f64) -> f64 {
    const ITMAX: i32 = 50;
    const CONV_TOL: f64 = 0.0001;
    if Press == UNSET_SAVE {
        return UNSET_SAVE;
    }
    let mut t_sat = 100.0;
    if Press >= 1555000.0 {
        t_sat = 200.0;
    } else if Press <= 0.0017 {
        t_sat = -100.0;
    } else if (Press > 611.000) && (Press < 611.25) {
        t_sat = 0.0;
    } else {
        let mut x1 = 0.0;
        let mut y1 = 0.0;
        for iter in 1..=ITMAX {
            let p_sat = PsyPsatFnTemp(t_sat);
            let error = Press - p_sat;
            let (result, converged) = iterate(CONV_TOL, t_sat, error, &mut x1, &mut y1, iter);
            t_sat = result;
            if converged {
                break;
            }
        }
    }
    t_sat
}

/// Port of `double PsyTdpFnWPb(double W, double PB)` (`psychrometrics.c:194`).
pub fn PsyTdpFnWPb(W: f64, PB: f64) -> f64 {
    let w0 = max(W, 1.0e-5);
    let pdew = PB * w0 / (0.62198 + w0);
    PsyTsatFnPb(pdew)
}

/// Port of `double PsyTdpFnTdbTwbPb(double TDB, double TWB, double PB)` (`psychrometrics.c:199`).
pub fn PsyTdpFnTdbTwbPb(TDB: f64, TWB: f64, PB: f64) -> f64 {
    let w = max(PsyWFnTdbTwbPb(TDB, TWB, PB), 1.0e-5);
    let tdp = PsyTdpFnWPb(w, PB);
    if tdp > TWB {
        return TWB;
    }
    tdp
}

/// Port of `double CPCW(double Temperature)` (`psychrometrics.c:204`).
pub fn CPCW(_Temperature: f64) -> f64 {
    4180.0
}

/// Port of `double CPHW(double Temperature)` (`psychrometrics.c:209`).
pub fn CPHW(_Temperature: f64) -> f64 {
    4180.0
}

/// Port of `double RhoH2O(double TB)` (`psychrometrics.c:214`).
pub fn RhoH2O(TB: f64) -> f64 {
    // pow_2(TB) = TB * TB and pow_3(TB) = TB * TB * TB in ObjexxFCL.
    1000.1207 + 8.3215874e-04 * TB - 4.929976e-03 * (TB * TB) + 8.4791863e-06 * (TB * TB * TB)
}

/// Port of `double PsyDeltaHSenFnTdb2Tdb1W(double TDB2, double TDB1, double W)` (`psychrometrics.c:219`).
pub fn PsyDeltaHSenFnTdb2Tdb1W(TDB2: f64, TDB1: f64, W: f64) -> f64 {
    (1.00484e3 + max(1.0e-5, W) * 1.85895e3) * (TDB2 - TDB1)
}

/// Port of `double PsyDeltaHSenFnTdb2W2Tdb1W1(double TDB2, double W2, double TDB1, double W1)` (`psychrometrics.c:224`).
pub fn PsyDeltaHSenFnTdb2W2Tdb1W1(TDB2: f64, W2: f64, TDB1: f64, W1: f64) -> f64 {
    let w_min = min(W1, W2);
    PsyDeltaHSenFnTdb2Tdb1W(TDB2, TDB1, w_min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objexx_max_min_follow_the_comparison_on_nan() {
        assert!(max(f64::NAN, 1.0e-5).is_nan());
        assert_eq!(max(1.0e-5, f64::NAN), 1.0e-5);
        assert_eq!(min(f64::NAN, 1.0), 1.0);
        assert!(min(1.0, f64::NAN).is_nan());
    }

    #[test]
    fn psat_cache_truncates_the_low_28_bits() {
        let t: f64 = 21.123456789;
        let truncated = f64::from_bits(t.to_bits() & !((1u64 << 28) - 1));
        assert_ne!(t, truncated);
        assert_eq!(
            PsyPsatFnTemp(t).to_bits(),
            PsyPsatFnTemp_raw(truncated).to_bits()
        );
        // Negative inputs shift arithmetically: the sign bit stays.
        let cold: f64 = -12.3456789;
        let cold_truncated = f64::from_bits(cold.to_bits() & !((1u64 << 28) - 1));
        assert_eq!(
            PsyPsatFnTemp(cold).to_bits(),
            PsyPsatFnTemp_raw(cold_truncated).to_bits()
        );
    }

    #[test]
    fn psat_known_values() {
        // About 2339 Pa at 20 C, 101.4 kPa near 100 C, and the pegged ends.
        assert!((PsyPsatFnTemp_raw(20.0) - 2339.0).abs() < 2.0);
        assert!((PsyPsatFnTemp_raw(100.0) - 101_418.0).abs() < 200.0);
        assert_eq!(PsyPsatFnTemp_raw(-150.0), 0.001405102123874164);
        assert_eq!(PsyPsatFnTemp_raw(250.0), 1555073.745636215);
        assert_eq!(PsyPsatFnTemp_raw(f64::NAN), 1555073.745636215);
    }

    #[test]
    fn tsat_inverts_psat_at_sea_level() {
        let t = PsyTsatFnPb(101_325.0);
        assert!((t - 100.0).abs() < 0.1, "{t}");
        assert_eq!(PsyTsatFnPb_raw(611.1), 0.0);
    }

    #[test]
    fn fresh_cache_slots_return_their_initial_values() {
        // A zero key finds the slot's initial key and result.
        assert_eq!(PsyTsatFnPb(0.0), 0.0);
        assert_eq!(PsyTsatFnPb_raw(0.0), -100.0);
        assert_eq!(PsyTsatFnHPb(0.0, 0.0), 0.0);
        assert_eq!(PsyTwbFnTdbWPb(0.0, 0.0, 0.0), 0.0);
        assert_eq!(PsyTsatFnPb_raw(-99999.0), -99999.0);
        assert_eq!(PsyCpAirFnW(-100.0), -100.0);
    }

    #[test]
    fn wet_bulb_is_below_dry_bulb() {
        let twb = PsyTwbFnTdbWPb(30.0, 0.01, 101_325.0);
        assert!(twb < 30.0 && twb > 15.0, "{twb}");
    }
}
