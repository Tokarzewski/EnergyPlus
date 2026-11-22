//! Rust port of simulation.SimulateYear (simulation.cpp) with the EnergyPlus kernels it calls
//! (ep_kernels.cpp: General::Iterate, the psychrometric functions and the exterior convection
//! correlations). Every kernel keeps the C++ operations in the same order and calls the same glibc
//! libm (exp, ln, pow, cos), so every value, and the checksum, is bit-identical.
//!
//! What is faster, and why it stays exact:
//! - The checksum is only an accumulator: nothing in the simulation reads it. So the year is run in
//!   phases: (A) one sequential pass over the zone states, which do depend on the previous step;
//!   (B) the 60 exterior surfaces in parallel, because each surface's temperature history depends
//!   only on its own past and on the zone temperatures from pass A; (C) every wet-bulb solve in
//!   parallel, because their results feed only the checksum; (D) one sequential replay that adds
//!   every term in exactly the order the C++ adds them.
//! - The boiling point for the last pressure is cached per thread (EnergyPlus keeps the same
//!   last_Patm/last_tBoil cache); PsyTsatFnPb is pure, so a hit returns the value a fresh call would.
#![allow(non_snake_case)]

use std::cell::Cell;

const KELVIN: f64 = 273.15;
const TRIPLE_POINT_OF_WATER_TEMP_KELVIN: f64 = 273.16;
const ICONV_TOL: f64 = 0.0001;
const ROUGHNESS_MULTIPLIER: [f64; 6] = [2.17, 1.67, 1.52, 1.13, 1.11, 1.0];

const N_ZONES: usize = 5;
const TIMESTEPS_PER_HOUR: usize = 6;
const HVAC_ITERS: usize = 3;
const SURFS_PER_ZONE: usize = 12;
const N_SURFS: usize = N_ZONES * SURFS_PER_ZONE;
/// wet-bulb requests per timestep: outdoor air, then zone air and coil for each iteration and zone
const REQS_PER_STEP: usize = 1 + HVAC_ITERS * N_ZONES * 2;
/// The literal simulation.cpp uses for pi (not std::f64::consts::PI, which differs in the last
/// digits and so would change the windward test's bits).
#[allow(clippy::approx_constant)] // must equal the C++ literal, not PI
const CPP_PI: f64 = 3.14159265358979;

thread_local! {
    static LAST_PATM: Cell<f64> = const { Cell::new(f64::NAN) };
    static LAST_TBOIL: Cell<f64> = const { Cell::new(0.0) };
}

/// std::max(a, b) as the C++ standard defines it: (a < b) ? b : a
#[inline(always)]
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// std::min(a, b): (b < a) ? b : a
#[inline(always)]
fn cpp_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}

/// The state General::Iterate carries between calls (X1, Y1, ResultX, Cnvg).
#[derive(Default)]
struct IterState {
    x1: f64,
    y1: f64,
    result_x: f64,
    converged: bool,
}

/// General::Iterate
#[inline(always)]
fn iterate(st: &mut IterState, tol: f64, x0: f64, y0: f64, iter: i32) {
    const SMALL: f64 = 1.0e-9;
    const PERTURB: f64 = 0.1;
    if iter != 1 && ((x0 - st.x1).abs() < tol || y0 == 0.0) {
        st.result_x = x0;
        st.converged = true;
        return;
    }
    st.converged = false;
    if iter == 1 {
        st.result_x = if x0.abs() > SMALL {
            x0 * (1.0 + PERTURB)
        } else {
            PERTURB
        };
    } else {
        let mut dy = y0 - st.y1;
        if dy.abs() < SMALL {
            dy = SMALL;
        }
        st.result_x = (y0 * st.x1 - st.y1 * x0) / dy;
    }
    st.x1 = x0;
    st.y1 = y0;
}

// ---------------------------------------------------------------- psychrometrics

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
        let mut st = IterState::default();
        for iter in 1..=ITMAX {
            let p_sat = psy_psat_fn_temp(t_sat);
            let error = press - p_sat;
            iterate(&mut st, CONV_TOL, t_sat, error, iter);
            t_sat = st.result_x;
            if st.converged {
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

fn psy_twb_fn_tdb_w_pb(tdb: f64, dw: f64, patm: f64) -> f64 {
    const ITMAX: i32 = 100;
    let w = if dw < 0.0 { 1.0e-5 } else { dw };
    let t_boil = t_boil(patm);
    let mut wbt = tdb;
    let mut st = IterState::default();
    for iter in 1..=ITMAX {
        if wbt >= t_boil - 0.09 {
            wbt = t_boil - 0.1;
        }
        let psat_star = psy_psat_fn_temp(wbt);
        let wstar = 0.62198 * psat_star / (patm - psat_star);
        let new_w = if wbt >= 0.0 {
            ((2501.0 - 2.326 * wbt) * wstar - 1.006 * (tdb - wbt))
                / (2501.0 + 1.86 * tdb - 4.186 * wbt)
        } else {
            ((2830.0 - 0.24 * wbt) * wstar - 1.006 * (tdb - wbt))
                / (2830.0 + 1.86 * tdb - 2.1 * wbt)
        };
        let error = w - new_w;
        iterate(&mut st, ICONV_TOL, wbt, error, iter);
        wbt = st.result_x;
        if st.converged {
            break;
        }
    }
    if wbt > tdb { tdb } else { wbt }
}

fn psy_w_fn_tdp_pb(tdp: f64, pb: f64) -> f64 {
    let pdew = psy_psat_fn_temp(tdp);
    let w = pdew * 0.62198 / (pb - pdew);
    if w < 0.0 {
        let mut delta_t = 0.0;
        let mut pdew1 = pdew;
        while pdew1 >= pb {
            delta_t += 1.0;
            pdew1 = psy_psat_fn_temp(tdp - delta_t);
        }
        return pdew1 * 0.62198 / (pb - pdew1);
    }
    w
}

#[inline(always)]
fn psy_h_fn_tdb_w(tdb: f64, dw: f64) -> f64 {
    1.00484e3 * tdb + cpp_max(dw, 1.0e-5) * (2.50094e6 + 1.85895e3 * tdb)
}

#[inline(always)]
fn psy_tdb_fn_h_w(h: f64, dw: f64) -> f64 {
    let w = cpp_max(dw, 1.0e-5);
    (h - 2.50094e6 * w) / (1.00484e3 + 1.85895e3 * w)
}

#[inline(always)]
fn psy_cp_air_fn_w(dw: f64) -> f64 {
    let w = cpp_max(dw, 1.0e-5);
    1.00484e3 + w * 1.85895e3
}

#[inline(always)]
fn psy_rho_air_fn_pb_tdb_w(pb: f64, tdb: f64, dw: f64) -> f64 {
    pb / (287.0 * (tdb + KELVIN) * (1.0 + 1.6077687 * cpp_max(dw, 1.0e-5)))
}

// ---------------------------------------------------------------- exterior convection

#[inline(always)]
fn cbrt_abs(d: f64) -> f64 {
    d.abs().powf(1.0 / 3.0)
}

#[inline(always)]
fn calc_ashrae_tarp_natural(tsurf: f64, tamb: f64, cos_tilt: f64) -> f64 {
    let delta_temp = tsurf - tamb;
    if delta_temp == 0.0 || cos_tilt == 0.0 {
        return 1.31 * cbrt_abs(delta_temp); // CalcASHRAEVerticalWall
    }
    if (delta_temp < 0.0 && cos_tilt < 0.0) || (delta_temp > 0.0 && cos_tilt > 0.0) {
        return 9.482 * cbrt_abs(delta_temp) / (7.238 - cos_tilt.abs()); // CalcWaltonUnstableHorizontalOrTilt
    }
    1.810 * cbrt_abs(delta_temp) / (1.382 + cos_tilt.abs()) // CalcWaltonStableHorizontalOrTilt
}

#[inline(always)]
fn calc_ashrae_simp_ext_conv_coeff(roughness: usize, v: f64) -> f64 {
    const D: [f64; 6] = [11.58, 12.49, 10.79, 8.23, 10.22, 8.23];
    const E: [f64; 6] = [5.894, 4.065, 4.192, 4.00, 3.100, 3.33];
    const F: [f64; 6] = [0.0, 0.028, 0.0, -0.057, 0.0, -0.036];
    D[roughness] + E[roughness] * v + F[roughness] * (v * v)
}

#[inline(always)]
fn calc_mowitt_natural(delta_temp: f64) -> f64 {
    0.84 * cbrt_abs(delta_temp)
}

#[inline(always)]
fn calc_mowitt_forced_windward(wind: f64) -> f64 {
    3.26 * wind.powf(0.89)
}

#[inline(always)]
fn calc_mowitt_forced_leeward(wind: f64) -> f64 {
    3.55 * wind.powf(0.617)
}

#[inline(always)]
fn calc_doe2_forced(tsurf: f64, tair: f64, cos_tilt: f64, hf_smooth: f64, roughness: usize) -> f64 {
    let hn = calc_ashrae_tarp_natural(tsurf, tair, cos_tilt);
    let hc_smooth = (hn * hn + hf_smooth * hf_smooth).sqrt();
    ROUGHNESS_MULTIPLIER[roughness] * (hc_smooth - hn)
}

#[inline(always)]
fn calc_sparrow_windward(roughness: usize, perimeter: f64, area: f64, wind: f64) -> f64 {
    2.537 * ROUGHNESS_MULTIPLIER[roughness] * (perimeter * wind / area).sqrt()
}

// ---------------------------------------------------------------- the year

#[derive(Clone, Copy)]
struct Surface {
    cos_tilt: f64,
    azimuth: f64,
    area: f64,
    perimeter: f64,
    roughness: usize,
    model: usize,
}

fn surfaces() -> Vec<Surface> {
    let mut v = Vec::with_capacity(N_SURFS);
    for z in 0..N_ZONES {
        for s in 0..SURFS_PER_ZONE {
            let cos_tilt = if s == 0 {
                1.0
            } else if s == 1 {
                -1.0
            } else {
                0.0
            };
            v.push(Surface {
                cos_tilt,
                azimuth: 90.0 * (s % 4) as f64,
                area: 10.0 + 3.0 * s as f64,
                perimeter: 14.0 + s as f64,
                roughness: s % 6,
                model: (z + s) % 4,
            });
        }
    }
    v
}

/// Outdoor conditions of one zone timestep.
#[derive(Clone, Copy)]
struct Step {
    tout: f64,
    wind: f64,
    wdir: f64,
}

fn threads() -> usize {
    std::thread::available_parallelism().map_or(1, |t| t.get())
}

/// Splits 0..n into at most `parts` contiguous ranges and runs `f` on each in its own thread,
/// returning the results in range order.
fn par_ranges<T: Send>(
    n: usize,
    parts: usize,
    f: impl Fn(std::ops::Range<usize>) -> T + Sync,
) -> Vec<T> {
    let parts = parts.clamp(1, n.max(1));
    let chunk = n.div_ceil(parts).max(1);
    if parts == 1 {
        return vec![f(0..n)];
    }
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..n)
            .step_by(chunk)
            .map(|start| {
                let f = &f;
                scope.spawn(move || f(start..(start + chunk).min(n)))
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("worker thread panicked"))
            .collect()
    })
}

pub fn SimulateYear(
    tdb: Vec<f64>,
    tdp: Vec<f64>,
    pb: Vec<f64>,
    wspd: Vec<f64>,
    wdir: Vec<f64>,
) -> Result<f64, String> {
    Ok(simulate_year(&tdb, &tdp, &pb, &wspd, &wdir))
}

pub fn simulate_year(tdb: &[f64], tdp: &[f64], pb: &[f64], wspd: &[f64], wdir: &[f64]) -> f64 {
    let n = tdb.len();
    if n == 0 || tdp.len() < n || pb.len() < n || wspd.len() < n || wdir.len() < n {
        return 0.0;
    }
    let t0 = std::time::Instant::now();
    let surfs = surfaces();
    let n_steps = n * TIMESTEPS_PER_HOUR;

    // (A) sequential: outdoor state and zone air/coil states. Records the checksum terms of this
    // part in order, the wet-bulb requests in order, and each zone's temperature after every
    // iteration (what the surface balance reads).
    let mut steps = Vec::with_capacity(n_steps);
    let mut zone_terms = Vec::with_capacity(n_steps * (1 + HVAC_ITERS * N_ZONES));
    let mut zone_t_hist = Vec::with_capacity(n_steps * HVAC_ITERS * N_ZONES);
    let mut reqs: Vec<[f64; 4]> = Vec::with_capacity(n_steps * REQS_PER_STEP);
    let mut zone_t = [22.0f64; N_ZONES];
    let mut zone_w = [0.008f64; N_ZONES];
    for h in 0..n {
        let c = h;
        let x = (h + 1) % n;
        for ts in 0..TIMESTEPS_PER_HOUR {
            let frac = (ts as f64 + 1.0) / TIMESTEPS_PER_HOUR as f64;
            let tout = tdb[c] + (tdb[x] - tdb[c]) * frac;
            let tdp_now = cpp_min(tout, tdp[c] + (tdp[x] - tdp[c]) * frac);
            let pb_now = pb[c] + (pb[x] - pb[c]) * frac;
            let wind = wspd[c] + (wspd[x] - wspd[c]) * frac;
            steps.push(Step {
                tout,
                wind,
                wdir: wdir[c],
            });

            let w_out = psy_w_fn_tdp_pb(tdp_now, pb_now);
            reqs.push([tout, w_out, pb_now, 1e-3]);
            zone_terms.push(psy_h_fn_tdb_w(tout, w_out) * 1e-9);

            for it in 0..HVAC_ITERS {
                for z in 0..N_ZONES {
                    let tz = zone_t[z] + 0.05 * (tout - zone_t[z]) / (1 + it) as f64;
                    let wz = zone_w[z] + 0.02 * (w_out - zone_w[z]);
                    reqs.push([tz, wz, pb_now, 1e-3]);
                    let hz = psy_h_fn_tdb_w(tz, wz);
                    let rho = psy_rho_air_fn_pb_tdb_w(pb_now, tz, wz);
                    let cp = psy_cp_air_fn_w(wz);
                    let h_coil =
                        hz - 0.1 * (hz - psy_h_fn_tdb_w(if tz > 24.0 { 13.0 } else { 30.0 }, wz));
                    let t_coil = psy_tdb_fn_h_w(h_coil, wz);
                    reqs.push([t_coil, wz, pb_now, 1e-4]);
                    zone_t[z] = tz + (21.0 + (if tz > 24.0 { 3.0 } else { 0.0 }) - tz) * 0.3;
                    zone_w[z] = wz;
                    zone_terms.push(rho * cp * 1e-6);
                    zone_t_hist.push(zone_t[z]);
                }
            }
        }
    }

    let t_a = std::time::Instant::now();
    let n_threads = threads();

    // (B) parallel over surfaces: each surface's convection coefficients through the year, as
    // checksum terms in (step, iteration) order.
    let per_surface: Vec<Vec<Vec<f64>>> = par_ranges(N_SURFS, n_threads, |range| {
        range
            .map(|i| {
                let sf = surfs[i];
                let z = i / SURFS_PER_ZONE;
                let mut surf_t = 20.0;
                let mut terms = Vec::with_capacity(n_steps * HVAC_ITERS);
                for (k, st) in steps.iter().enumerate() {
                    let windward = ((st.wdir - sf.azimuth) * CPP_PI / 180.0).cos() > 0.0;
                    for it in 0..HVAC_ITERS {
                        let tz = zone_t_hist[(k * HVAC_ITERS + it) * N_ZONES + z];
                        let ts_ = surf_t;
                        let tout = st.tout;
                        let wind = st.wind;
                        let hc = match sf.model {
                            0 => calc_doe2_forced(
                                ts_,
                                tout,
                                sf.cos_tilt,
                                if windward {
                                    calc_mowitt_forced_windward(wind)
                                } else {
                                    calc_mowitt_forced_leeward(wind)
                                },
                                sf.roughness,
                            ),
                            1 => {
                                let hn = calc_mowitt_natural(ts_ - tout);
                                let hf = if windward {
                                    calc_mowitt_forced_windward(wind)
                                } else {
                                    calc_mowitt_forced_leeward(wind)
                                };
                                (hn * hn + hf * hf).sqrt()
                            }
                            2 => {
                                calc_ashrae_tarp_natural(ts_, tout, sf.cos_tilt)
                                    + calc_sparrow_windward(
                                        sf.roughness,
                                        sf.perimeter,
                                        sf.area,
                                        wind,
                                    )
                            }
                            _ => calc_ashrae_simp_ext_conv_coeff(sf.roughness, wind),
                        };
                        let u = 1.0 / (0.4 + 1.0 / (hc + 1e-9));
                        surf_t = tout + (tz - tout) * u * 0.4;
                        terms.push(hc * 1e-5);
                    }
                }
                terms
            })
            .collect()
    });
    let t_b = std::time::Instant::now();
    let surf_terms: Vec<&Vec<f64>> = per_surface.iter().flatten().collect();

    // (C) parallel over wet-bulb requests: twb * weight, in request order.
    let wb_terms: Vec<f64> = par_ranges(reqs.len(), n_threads, |range| {
        reqs[range]
            .iter()
            .map(|r| psy_twb_fn_tdb_w_pb(r[0], r[1], r[2]) * r[3])
            .collect::<Vec<f64>>()
    })
    .concat();

    let t_c = std::time::Instant::now();
    // (D) sequential replay of the checksum in the C++ order; wet bulbs are added at each day's
    // flush and once at the end.
    let mut checksum = 0.0f64;
    let mut zi = 0;
    let mut flushed = 0;
    for h in 0..n {
        for ts in 0..TIMESTEPS_PER_HOUR {
            let k = h * TIMESTEPS_PER_HOUR + ts;
            checksum += zone_terms[zi];
            zi += 1;
            for it in 0..HVAC_ITERS {
                for z in 0..N_ZONES {
                    checksum += zone_terms[zi];
                    zi += 1;
                    for s in 0..SURFS_PER_ZONE {
                        checksum += surf_terms[z * SURFS_PER_ZONE + s][k * HVAC_ITERS + it];
                    }
                }
            }
        }
        if h % 24 == 23 {
            let end = (h + 1) * TIMESTEPS_PER_HOUR * REQS_PER_STEP;
            for t in &wb_terms[flushed..end] {
                checksum += t;
            }
            flushed = end;
        }
    }
    for t in &wb_terms[flushed..] {
        checksum += t;
    }
    if std::env::var_os("SIM_TIMING").is_some() {
        eprintln!("pass A {:?} ", t_a - t0);
        eprintln!(
            "A..B {:?} B..C {:?} D {:?}",
            t_b - t_a,
            t_c - t_b,
            t_c.elapsed()
        );
    }
    checksum += psy_tsat_fn_pb(pb[0]) * 1e-3 + psy_psat_fn_temp(20.0) * 1e-6;
    checksum
}
