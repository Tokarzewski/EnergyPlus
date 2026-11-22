//! Rust port of ep_kernels.PsyTwbFnTdbWPbBatch: the wet bulb of every state, from EnergyPlus
//! Psychrometrics.cc PsyTwbFnTdbWPb with General::Iterate, PsyPsatFnTemp and PsyTsatFnPb.
//! Same operations in the same order as the C++, so results are bit-identical.
//! The only change: the boiling point for the last pressure is cached (EnergyPlus itself keeps
//! the same last_Patm/last_tBoil cache in state.dataPsychrometrics); PsyTsatFnPb is pure, so the
//! cached value is the value a fresh call would return.
#![allow(non_snake_case)]

use std::cell::Cell;

const KELVIN: f64 = 273.15;
const TRIPLE_POINT_OF_WATER_TEMP_KELVIN: f64 = 273.16;
const ICONV_TOL: f64 = 0.0001;

thread_local! {
    static LAST_PATM: Cell<f64> = const { Cell::new(f64::NAN) };
    static LAST_TBOIL: Cell<f64> = const { Cell::new(0.0) };
}

#[inline(always)]
fn iterate(result_x: &mut f64, tol: f64, x0: f64, y0: f64, x1: &mut f64, y1: &mut f64, iter: i32, cnvg: &mut i32) {
    const SMALL: f64 = 1.0e-9;
    const PERTURB: f64 = 0.1;
    if iter != 1 && ((x0 - *x1).abs() < tol || y0 == 0.0) {
        *result_x = x0;
        *cnvg = 1;
        return;
    }
    *cnvg = 0;
    if iter == 1 {
        *result_x = if x0.abs() > SMALL { x0 * (1.0 + PERTURB) } else { PERTURB };
    } else {
        let mut dy = y0 - *y1;
        if dy.abs() < SMALL {
            dy = SMALL;
        }
        *result_x = (y0 * *x1 - *y1 * x0) / dy;
    }
    *x1 = x0;
    *y1 = y0;
}

#[inline(always)]
fn psy_psat_fn_temp(t: f64) -> f64 {
    let tkel = t + KELVIN;
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
        (C1 / tkel + C2 + tkel * (C3 + tkel * (C4 + tkel * (C5 + C6 * tkel))) + C7 * tkel.ln()).exp()
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

fn psy_tsat_fn_pb(press: f64) -> f64 {
    const ITMAX: i32 = 50;
    const CONV_TOL: f64 = 0.0001;
    let mut t_sat = 100.0;
    if press >= 1555000.0 {
        t_sat = 200.0;
    } else if press <= 0.0017 {
        t_sat = -100.0;
    } else if press > 611.000 && press < 611.25 {
        t_sat = 0.0;
    } else {
        let (mut x1, mut y1, mut result_x, mut icvg) = (0.0, 0.0, 0.0, 0);
        for iter in 1..=ITMAX {
            let p_sat = psy_psat_fn_temp(t_sat);
            let error = press - p_sat;
            iterate(&mut result_x, CONV_TOL, t_sat, error, &mut x1, &mut y1, iter, &mut icvg);
            t_sat = result_x;
            if icvg == 1 {
                break;
            }
        }
    }
    t_sat
}

#[inline(always)]
fn t_boil(patm: f64) -> f64 {
    // to_bits comparison so a NaN pressure is never treated as a cache hit
    if LAST_PATM.get().to_bits() == patm.to_bits() {
        return LAST_TBOIL.get();
    }
    let t = psy_tsat_fn_pb(patm);
    LAST_PATM.set(patm);
    LAST_TBOIL.set(t);
    t
}

/// Below this many states the batch runs on the calling thread; above it, it is split across cores.
const PARALLEL_MIN: usize = 2048;

pub fn PsyTwbFnTdbWPbBatch(tdb: Vec<f64>, w: Vec<f64>, pb: Vec<f64>) -> Result<Vec<f64>, String> {
    // C++ indexes w[i] and pb[i] for every i < tdb.size(); shorter inputs are undefined behaviour there.
    if w.len() < tdb.len() || pb.len() < tdb.len() {
        return Err("w and pb must be at least as long as tdb".into());
    }
    let n = tdb.len();
    let mut out = vec![0.0; n];
    let threads = std::thread::available_parallelism().map_or(1, |t| t.get());
    if n < PARALLEL_MIN || threads == 1 {
        for i in 0..n {
            out[i] = psy_twb_fn_tdb_w_pb(tdb[i], w[i], pb[i]);
        }
        return Ok(out);
    }
    let chunk = n.div_ceil(threads);
    std::thread::scope(|s| {
        for (k, slot) in out.chunks_mut(chunk).enumerate() {
            let start = k * chunk;
            let (tdb, w, pb) = (&tdb[start..], &w[start..], &pb[start..]);
            s.spawn(move || {
                for (j, o) in slot.iter_mut().enumerate() {
                    *o = psy_twb_fn_tdb_w_pb(tdb[j], w[j], pb[j]);
                }
            });
        }
    });
    Ok(out)
}

pub fn psy_twb_fn_tdb_w_pb(tdb: f64, dw: f64, patm: f64) -> f64 {
    const ITMAX: i32 = 100;
    let w = if dw < 0.0 { 1.0e-5 } else { dw };
    let t_boil = t_boil(patm);
    let mut wbt = tdb;
    let (mut x1, mut y1, mut result_x, mut icvg) = (0.0, 0.0, 0.0, 0);
    for iter in 1..=ITMAX {
        if wbt >= t_boil - 0.09 {
            wbt = t_boil - 0.1;
        }
        let psat_star = psy_psat_fn_temp(wbt);
        let wstar = 0.62198 * psat_star / (patm - psat_star);
        let new_w = if wbt >= 0.0 {
            ((2501.0 - 2.326 * wbt) * wstar - 1.006 * (tdb - wbt)) / (2501.0 + 1.86 * tdb - 4.186 * wbt)
        } else {
            ((2830.0 - 0.24 * wbt) * wstar - 1.006 * (tdb - wbt)) / (2830.0 + 1.86 * tdb - 2.1 * wbt)
        };
        let error = w - new_w;
        iterate(&mut result_x, ICONV_TOL, wbt, error, &mut x1, &mut y1, iter, &mut icvg);
        wbt = result_x;
        if icvg == 1 {
            break;
        }
    }
    if wbt > tdb { tdb } else { wbt }
}
