//! The Rust port of `wccdetail.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `WaterCoils::CoolingCoil` with the detailed analysis: a dry coil when the entering air's dew
//! point is at or below the water temperature, a completely wet one when the wet coil's surface
//! at the air inlet is at or below the dew point, and otherwise `CoilPartWetPartDry`, which
//! iterates the water temperature at the wet/dry boundary (`General::Iterate`) inside an iteration
//! on the wet fraction of the surface (`CoilAreaFracIter`). The dry and wet coils are
//! `watercc-rs`'s (unit 88). Every expression keeps the C++ order, and `max` is ObjexxFCL's. The
//! oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use general_rs::port::SafeDivide;
use psychrometrics_rs::port::{PsyHFnTdbW, PsyTdpFnWPb};
use std::ffi::c_int;
use watercc_rs::port::{
    Coil, CpCache, MIN_AIR_MASS_FLOW, coil_completely_dry, coil_completely_wet, within,
};

/// Iteration limit of both loops in `CoilPartWetPartDry`.
const ITMAX: i32 = 60;

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
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

/// The two earlier points `CoilAreaFracIter` keeps: (fraction, error) previous and last.
struct FracPoints {
    prev: f64,
    err_prev: f64,
    last: f64,
    err_last: f64,
}

/// `CoilAreaFracIter`: the next wet fraction by perturbation, a linear or a quadratic fit.
/// Returns `(NewSurfAreaWetFrac, converged)`.
fn coil_area_frac_iter(cur: f64, err: f64, p: &mut FracPoints, iter_num: i32) -> (f64, bool) {
    const TOLERANCE: f64 = 1.0e-5;
    const PERTURB_SURF_AREA_FRAC: f64 = 0.1;
    const SMALL_NUM: f64 = 1.0e-9;
    if ((cur - p.prev).abs() < TOLERANCE * max(cur.abs(), SMALL_NUM) && iter_num != 1) || err == 0.0
    {
        return (cur, true);
    }
    let mut mode = iter_num;
    let new = loop {
        if mode == 1 {
            break if cur.abs() > SMALL_NUM {
                cur * (1.0 + PERTURB_SURF_AREA_FRAC)
            } else {
                PERTURB_SURF_AREA_FRAC
            };
        } else if mode == 2 {
            let slope = (p.err_prev - err) / (p.prev - cur);
            if slope == 0.0 {
                mode = 1;
                continue;
            }
            break cur - err / slope;
        }
        if cur == p.prev {
            p.prev = p.last;
            p.err_prev = p.err_last;
            mode = 2;
            continue;
        } else if cur == p.last {
            mode = 2;
            continue;
        }
        let q3 = ((p.err_last - err) / (p.last - cur) - (p.err_prev - err) / (p.prev - cur))
            / (p.last - p.prev);
        let q2 = (p.err_prev - err) / (p.prev - cur) - (p.prev + cur) * q3;
        let q1 = err - (q2 + q3 * cur) * cur;
        if q3.abs() < 1.0e-10
            || ((q1 + (q2 + q3 * p.prev) * p.prev - p.err_prev) / p.err_prev).abs() > 1.0e-4
        {
            mode = 2;
            continue;
        }
        let check = q2 * q2 - 4.0 * q1 * q3;
        if check < 0.0 {
            mode = 2;
            continue;
        } else if check > 0.0 {
            let root = (-q2 + check.sqrt()) / q3 / 2.0;
            let other = -root - q2 / q3;
            break if (root - cur).abs() > (other - cur).abs() {
                other
            } else {
                root
            };
        }
        break -q2 / q3 / 2.0;
    };
    if mode < 3 {
        p.last = p.prev;
        p.err_last = p.err_prev;
    } else if p.err_prev * err > 0.0 && p.err_last * err > 0.0 {
        if p.err_last.abs() > p.err_prev.abs() {
            p.last = p.prev;
            p.err_last = p.err_prev;
        }
    } else if p.err_last * err > 0.0 {
        p.last = p.prev;
        p.err_last = p.err_prev;
    }
    p.prev = cur;
    p.err_prev = err;
    (new, false)
}

/// The coil's surface area, its UAs per unit area, and the wet fraction state.
struct Surface {
    area: f64,
    ua_internal_per_area: f64,
    ua_dry_ext_per_area: f64,
    ua_wet_ext_per_area: f64,
    first: bool,
    saved: f64,
}

/// `CoilPartWetPartDry`: outlet water and air temperatures, outlet humidity ratio, total and
/// sensible rates, the wet fraction, and the saved wet fraction afterwards.
fn coil_part_wet_part_dry(
    c: &Coil,
    s: &Surface,
    inlet_water_temp: f64,
    inlet_air_temp: f64,
    air_dew_point_temp: f64,
) -> [f64; 7] {
    const SMALL_TEMP_DIFF: f64 = 1.0e-9;
    let mut outlet_water_temp = inlet_air_temp;
    let (mut dry_heat, mut wet_total, mut wet_sensible) = (0.0, 0.0, 0.0);
    let (mut outlet_air_temp, mut outlet_air_hum_rat) = (0.0, 0.0);
    let mut frac = if s.first {
        let interface_water_temp = air_dew_point_temp;
        if (outlet_water_temp - inlet_water_temp).abs() > SMALL_TEMP_DIFF {
            (interface_water_temp - inlet_water_temp) / (outlet_water_temp - inlet_water_temp)
        } else {
            0.0
        }
    } else {
        s.saved
    };
    let mut points = FracPoints {
        prev: frac,
        err_prev: 0.0,
        last: frac,
        err_last: 0.0,
    };
    let (mut x1t, mut y1t) = (0.0, 0.0);
    for iter in 1..=ITMAX {
        let surf_area_wet = frac * s.area;
        let surf_area_dry = s.area - surf_area_wet;
        let dry_coil_ua =
            surf_area_dry / (1.0 / s.ua_internal_per_area + 1.0 / s.ua_dry_ext_per_area);
        let wet_part_ua_external = s.ua_wet_ext_per_area * surf_area_wet;
        let wet_part_ua_internal = s.ua_internal_per_area * surf_area_wet;
        let mut interface_water_temp =
            inlet_water_temp + frac * (outlet_water_temp - inlet_water_temp);
        let mut interface_surf_temp = 0.0;
        for it_t in 1..=ITMAX {
            let dry = coil_completely_dry(c, interface_water_temp, inlet_air_temp, dry_coil_ua);
            outlet_water_temp = dry[0];
            dry_heat = dry[3];
            let wet = coil_completely_wet(
                c,
                inlet_water_temp,
                dry[1],
                dry[2],
                [wet_part_ua_internal, wet_part_ua_external],
            );
            [
                outlet_air_temp,
                outlet_air_hum_rat,
                wet_total,
                wet_sensible,
                interface_surf_temp,
            ] = [wet[1], wet[2], wet[3], wet[4], wet[6]];
            let error_t = wet[0] - interface_water_temp;
            let (result, converged) = iterate(
                0.001,
                interface_water_temp,
                error_t,
                &mut x1t,
                &mut y1t,
                it_t,
            );
            interface_water_temp = result;
            if converged {
                break;
            }
        }
        if frac <= 0.0 && interface_surf_temp >= air_dew_point_temp {
            let dry_coil_ua = s.area / (1.0 / s.ua_internal_per_area + 1.0 / s.ua_dry_ext_per_area);
            let dry = coil_completely_dry(c, inlet_water_temp, inlet_air_temp, dry_coil_ua);
            return [dry[0], dry[1], dry[2], dry[3], dry[3], 0.0, s.saved];
        }
        let error = interface_surf_temp - air_dew_point_temp;
        let (new, converged) = coil_area_frac_iter(frac, error, &mut points, iter);
        frac = new;
        if converged {
            break;
        }
        if frac > 1.0 {
            frac = 1.0;
        }
        if frac <= 0.0 {
            frac = 0.0098;
        }
    }
    [
        outlet_water_temp,
        outlet_air_temp,
        outlet_air_hum_rat,
        dry_heat + wet_total,
        dry_heat + wet_sensible,
        frac,
        frac,
    ]
}

/// Port of `double WaterCoolDetail(int which, int cycling, int design, int hex, int first, ...)`
/// (`wccdetail.c:8`): `CoolingCoil` with the detailed analysis, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn WaterCoolDetail(
    which: c_int,
    cycling: c_int,
    design: c_int,
    hex: c_int,
    first: c_int,
    avail: f64,
    uaint: f64,
    uaext: f64,
    area: f64,
    saved: f64,
    desvol: f64,
    mdota: f64,
    tain: f64,
    win: f64,
    mdotw: f64,
    mdotwmax: f64,
    twin: f64,
    hwin: f64,
    plr: f64,
    pb: f64,
) -> f64 {
    let flag = |v: c_int| v == 0 || v == 1;
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    if which > 8
        || !flag(cycling)
        || !flag(design)
        || !flag(first)
        || (hex != 1 && hex != 2)
        || !within(avail, 0.0, 1.0)
        || !within(saved, 0.0, 1.0)
        || !within(uaint, 1.0, 1.0e7)
        || !within(uaext, 1.0, 1.0e7)
        || !within(area, 0.1, 1.0e4)
        || !within(desvol, 0.0, 100.0)
        || !within(mdota, 0.0, 50.0)
        || !within(tain, -40.0, 60.0)
        || !within(win, 0.0, 0.04)
        || !within(mdotwmax, 0.0, 100.0)
        || !within(mdotw, 0.0, mdotwmax)
        || !within(twin, 1.0, 99.0)
        || !within(hwin, -1.0e6, 1.0e6)
        || !within(plr, 0.0, 1.0)
        || !within(pb, 50000.0, 120000.0)
    {
        return f64::NAN;
    }
    // As InitWaterCoil sets them.
    let uatot = 1.0 / (1.0 / uaext + 1.0 / uaint);
    let ua_wet_ext_per_area = uaext / area;
    let s = Surface {
        area,
        ua_internal_per_area: uaint / area,
        ua_dry_ext_per_area: ua_wet_ext_per_area,
        ua_wet_ext_per_area,
        first: first == 1,
        saved,
    };
    let c = Coil {
        hex,
        mdot_air: mdota,
        w_in: win,
        mdot_water: mdotw,
        mdot_water_max: mdotwmax,
        cycling: cycling == 1,
        plr,
        pb,
        cp: CpCache::default(),
    };
    let air_mass_flow = if c.cycling && plr > 0.0 {
        mdota / plr
    } else {
        mdota
    };
    // The shim zeroes the rates, which a coil that does not run leaves as they were.
    let mut r = [tain, win, twin, 0.0, 0.0, 0.0, hwin, saved];
    if (avail > 0.0
        && mdotw > 0.0
        && air_mass_flow >= MIN_AIR_MASS_FLOW
        && desvol > 0.0
        && mdotwmax > 0.0)
        || design == 1
    {
        let air_dew_point_temp = PsyTdpFnWPb(win, pb);
        // [outlet water, outlet air, humidity ratio, total, sensible, wet fraction, saved]
        let out = if air_dew_point_temp <= twin {
            let dry = coil_completely_dry(&c, twin, tain, uatot);
            [dry[0], dry[1], dry[2], dry[3], dry[3], 0.0, saved]
        } else {
            let wet = coil_completely_wet(&c, twin, tain, win, [uaint, uaext]);
            if air_dew_point_temp < wet[6] {
                coil_part_wet_part_dry(&c, &s, twin, tain, air_dew_point_temp)
            } else {
                [wet[0], wet[1], wet[2], wet[3], wet[4], wet[5], saved]
            }
        };
        let (mut tot, mut sen) = (out[3], out[4]);
        if c.cycling {
            tot *= plr;
            sen *= plr;
        }
        r = [
            out[1],
            out[2],
            out[0],
            tot,
            sen,
            out[5],
            hwin + SafeDivide(tot, mdotw),
            out[6],
        ];
    }
    let [
        t_air_out,
        w_out,
        t_water_out,
        tot,
        sen,
        frac,
        h_water_out,
        saved_out,
    ] = r;
    [
        t_air_out,
        w_out,
        t_water_out,
        tot,
        sen,
        frac,
        h_water_out,
        PsyHFnTdbW(t_air_out, w_out),
        saved_out,
    ][which]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coil(which: c_int, first: c_int, win: f64) -> f64 {
        WaterCoolDetail(
            which, 0, 0, 2, first, 1.0, 12000.0, 9000.0, 100.0, 0.5, 2.0, 2.4, 26.0, win, 2.0, 3.0,
            7.0, 29330.0, 1.0, 101325.0,
        )
    }

    #[test]
    fn humid_air_wets_part_of_the_coil() {
        let frac = coil(5, 1, 0.0095);
        assert!(frac > 0.0 && frac <= 1.0);
        assert_eq!(coil(8, 1, 0.0095), frac);
        assert!(coil(3, 1, 0.0095) > coil(4, 1, 0.0095));
        assert!(coil(9, 1, 0.0095).is_nan());
    }

    #[test]
    fn dry_air_keeps_the_saved_fraction() {
        assert_eq!(coil(5, 0, 0.004), 0.0);
        assert_eq!(coil(8, 0, 0.004), 0.5);
    }

    #[test]
    fn area_fraction_iteration_converges_on_a_line() {
        let mut p = FracPoints {
            prev: 0.5,
            err_prev: 0.0,
            last: 0.5,
            err_last: 0.0,
        };
        let (mut x, mut done) = (0.5, false);
        for it in 1..=20 {
            (x, done) = coil_area_frac_iter(x, 2.0 * (x - 0.3), &mut p, it);
            if done {
                break;
            }
        }
        assert!(done && (x - 0.3).abs() < 1.0e-6);
    }
}
