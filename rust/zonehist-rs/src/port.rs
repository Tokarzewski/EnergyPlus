//! The Rust port of `zonehist.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The zone air history updates of `ZoneTempPredictorCorrector` for a fully mixed zone:
//! `pushZoneTimestepHistory` (shift the zone time step temperature and humidity ratio histories,
//! take the new averages, the relative humidity, and the Exact and Euler method terms),
//! `pushSystemTimestepHistory` (the same for the down-stepped system time step histories),
//! `revertZoneTimestepHistory` (shift them back), and both forms of
//! `DownInterpolate4HistoryValues` (history values on a shorter time step). The relative humidity
//! comes from `psychrometrics-rs`. Every expression keeps the C++ order. The oracle's guards (see
//! its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::PsyRhFnTdbWPb;

/// `DataHeatBalance::SolutionAlgo::ThirdOrder`.
const THIRD_ORDER: i32 = 0;

/// The zone's air history terms (`ZoneSpaceHeatBalanceData`).
#[derive(Clone, Copy)]
struct History {
    xmat: [f64; 4],
    w_prev_zone_ts: [f64; 4],
    dsxmat: [f64; 4],
    ds_w_prev_zone_ts: [f64; 4],
    ztav: f64,
    zt: f64,
    mat: f64,
    air_hum_rat: f64,
    air_hum_rat_avg: f64,
    air_hum_rat_temp: f64,
    air_rel_hum: f64,
    xmpt: f64,
    w_time_minus_p: f64,
    tm2: f64,
    tmx: f64,
    wm2: f64,
    wmx: f64,
}

impl History {
    /// `pushZoneTimestepHistory`.
    fn push_zone_timestep_history(&mut self, algo: i32, out_baro_press: f64) {
        for i in (1..=3).rev() {
            self.xmat[i] = self.xmat[i - 1];
            self.w_prev_zone_ts[i] = self.w_prev_zone_ts[i - 1];
        }
        self.xmat[0] = self.ztav;
        self.xmpt = self.zt;
        self.w_prev_zone_ts[0] = self.air_hum_rat_avg;
        self.air_hum_rat = self.air_hum_rat_temp;
        self.w_time_minus_p = self.air_hum_rat_temp;
        self.air_rel_hum = 100.0 * PsyRhFnTdbWPb(self.zt, self.air_hum_rat, out_baro_press);
        if algo != THIRD_ORDER {
            self.tm2 = self.tmx;
            self.tmx = self.ztav;
            self.wm2 = self.wmx;
            self.wmx = self.air_hum_rat_avg;
        }
    }

    /// `pushSystemTimestepHistory`.
    fn push_system_timestep_history(&mut self, algo: i32) {
        for i in (1..=3).rev() {
            self.dsxmat[i] = self.dsxmat[i - 1];
            self.ds_w_prev_zone_ts[i] = self.ds_w_prev_zone_ts[i - 1];
        }
        self.dsxmat[0] = self.mat;
        self.ds_w_prev_zone_ts[0] = self.air_hum_rat;
        if algo != THIRD_ORDER {
            self.tm2 = self.tmx;
            self.tmx = self.mat;
            self.wm2 = self.wmx;
            self.wmx = self.air_hum_rat_temp;
        }
    }

    /// `revertZoneTimestepHistory`.
    fn revert_zone_timestep_history(&mut self) {
        for i in 0..=2 {
            self.xmat[i] = self.xmat[i + 1];
            self.w_prev_zone_ts[i] = self.w_prev_zone_ts[i + 1];
        }
    }
}

/// The five-value `DownInterpolate4HistoryValues`: `newVal0` to `newVal4`.
fn down_interpolate_4_history_values(
    old_time_step: f64,
    new_time_step: f64,
    old_val0: f64,
    old_val1: f64,
    old_val2: f64,
) -> [f64; 5] {
    let ds_ratio = old_time_step / new_time_step;
    if (ds_ratio - 2.0).abs() < 0.01 {
        [
            old_val0,
            (old_val0 + old_val1) / 2.0,
            old_val1,
            (old_val1 + old_val2) / 2.0,
            old_val2,
        ]
    } else if (ds_ratio - 3.0).abs() < 0.01 {
        let delta10 = (old_val1 - old_val0) / 3.0;
        let new_val1 = old_val0 + delta10;
        [
            old_val0,
            new_val1,
            new_val1 + delta10,
            old_val1,
            old_val1 + (old_val2 - old_val1) / 3.0,
        ]
    } else {
        let delta10 = (old_val1 - old_val0) / ds_ratio;
        let new_val1 = old_val0 + delta10;
        let new_val2 = new_val1 + delta10;
        let new_val3 = new_val2 + delta10;
        [old_val0, new_val1, new_val2, new_val3, new_val3 + delta10]
    }
}

/// The array `DownInterpolate4HistoryValues`: the new values, and its return value `oldVals[0]`.
fn down_interpolate_4_history_values_array(
    old_time_step: f64,
    new_time_step: f64,
    old_vals: &[f64; 4],
) -> ([f64; 4], f64) {
    let ds_ratio = old_time_step / new_time_step;
    let new_vals = if (ds_ratio - 2.0).abs() < 0.01 {
        [
            old_vals[0],
            (old_vals[0] + old_vals[1]) / 2.0,
            old_vals[1],
            (old_vals[1] + old_vals[2]) / 2.0,
        ]
    } else if (ds_ratio - 3.0).abs() < 0.01 {
        let delta10 = (old_vals[1] - old_vals[0]) / 3.0;
        let new_val1 = old_vals[0] + delta10;
        [old_vals[0], new_val1, new_val1 + delta10, old_vals[1]]
    } else {
        let delta10 = (old_vals[1] - old_vals[0]) / ds_ratio;
        let new_val1 = old_vals[0] + delta10;
        let new_val2 = new_val1 + delta10;
        [old_vals[0], new_val1, new_val2, new_val2 + delta10]
    };
    (new_vals, old_vals[0])
}

/// One value after a zone air history update (see the oracle's shim for the arguments, `op` and
/// `which`).
pub fn ZoneHistory(
    which: i32,
    op: i32,
    algo: i32,
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    w0: f64,
    w1: f64,
    w2: f64,
    w3: f64,
    dx0: f64,
    dx1: f64,
    dx2: f64,
    dx3: f64,
    dw0: f64,
    dw1: f64,
    dw2: f64,
    dw3: f64,
    ztav: f64,
    zt: f64,
    mat: f64,
    airhumrat: f64,
    airhumratavg: f64,
    airhumrattemp: f64,
    airrelhum: f64,
    xmpt: f64,
    wtimeminusp: f64,
    tm2: f64,
    tmx: f64,
    wm2: f64,
    wmx: f64,
    outbaro: f64,
    oldts: f64,
    newts: f64,
) -> f64 {
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    if which > 29 || !(0..=4).contains(&op) || !(0..=2).contains(&algo) {
        return f64::NAN;
    }
    let mut h = History {
        xmat: [x0, x1, x2, x3],
        w_prev_zone_ts: [w0, w1, w2, w3],
        dsxmat: [dx0, dx1, dx2, dx3],
        ds_w_prev_zone_ts: [dw0, dw1, dw2, dw3],
        ztav,
        zt,
        mat,
        air_hum_rat: airhumrat,
        air_hum_rat_avg: airhumratavg,
        air_hum_rat_temp: airhumrattemp,
        air_rel_hum: airrelhum,
        xmpt,
        w_time_minus_p: wtimeminusp,
        tm2,
        tmx,
        wm2,
        wmx,
    };
    let mut new_vals = [0.0; 5];
    let mut ret = 0.0;
    match op {
        0 => h.push_zone_timestep_history(algo, outbaro),
        1 => h.push_system_timestep_history(algo),
        2 => h.revert_zone_timestep_history(),
        3 => new_vals = down_interpolate_4_history_values(oldts, newts, x0, x1, x2),
        _ => {
            let (v, r) = down_interpolate_4_history_values_array(oldts, newts, &[x0, x1, x2, x3]);
            new_vals[..4].copy_from_slice(&v);
            ret = r;
        }
    }
    match which {
        0..=3 => h.xmat[which],
        4..=7 => h.w_prev_zone_ts[which - 4],
        8..=11 => h.dsxmat[which - 8],
        12..=15 => h.ds_w_prev_zone_ts[which - 12],
        16 => h.xmpt,
        17 => h.air_hum_rat,
        18 => h.w_time_minus_p,
        19 => h.air_rel_hum,
        20 => h.tm2,
        21 => h.tmx,
        22 => h.wm2,
        23 => h.wmx,
        24..=28 => new_vals[which - 24],
        _ => ret,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Zone 1's histories 20, 19, 18, 17 C, under algorithm `algo`, after `op`, value `which`.
    fn run(which: i32, op: i32, algo: i32, oldts: f64, newts: f64) -> f64 {
        ZoneHistory(
            which, op, algo, 20.0, 19.0, 18.0, 17.0, 0.01, 0.009, 0.008, 0.007, 20.0, 19.5, 19.0,
            18.5, 0.01, 0.0095, 0.009, 0.0085, 21.0, 21.5, 21.2, 0.011, 0.0105, 0.012, 50.0, 20.5,
            0.01, 18.0, 19.0, 0.008, 0.009, 101325.0, oldts, newts,
        )
    }

    #[test]
    fn push_and_revert_shift_the_histories() {
        assert_eq!(run(0, 0, 0, 900.0, 450.0), 21.0);
        assert_eq!(run(1, 0, 0, 900.0, 450.0), 20.0);
        assert_eq!(run(17, 0, 0, 900.0, 450.0), 0.012);
        // Third order keeps the Exact and Euler terms; the others shift them.
        assert_eq!(run(20, 0, 0, 900.0, 450.0), 18.0);
        assert_eq!(run(20, 0, 1, 900.0, 450.0), 19.0);
        assert_eq!(run(8, 1, 0, 900.0, 450.0), 21.2);
        assert_eq!(run(0, 2, 0, 900.0, 450.0), 19.0);
        assert_eq!(run(3, 2, 0, 900.0, 450.0), 17.0);
    }

    #[test]
    fn interpolation_by_step_ratio() {
        assert_eq!(run(25, 3, 0, 900.0, 450.0), 19.5);
        assert_eq!(run(27, 3, 0, 900.0, 300.0), 19.0);
        assert_eq!(run(28, 3, 0, 900.0, 225.0), 19.0);
        assert_eq!(run(27, 4, 0, 900.0, 450.0), 18.5);
        assert_eq!(run(29, 4, 0, 900.0, 450.0), 20.0);
        assert!(run(30, 0, 0, 900.0, 450.0).is_nan());
    }
}
