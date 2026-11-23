//! The Rust port of `updtemp.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneSpaceHeatBalanceData::updateTemperatures` for one fully mixed zone (no space): rolling the
//! system node back to the zone time step history when the system time step shortens,
//! down-interpolating the air temperature and humidity ratio histories when the system time step
//! count changed, and picking the history the balance equations use. Both variants of
//! `DownInterpolate4HistoryValues`: the four-value one these histories use and the five-value one
//! the contaminant predictor uses. `PsyHFnTdbW` comes from `psychrometrics-rs`. Every expression
//! keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use psychrometrics_rs::port::PsyHFnTdbW;

/// The down step ratio `OldTimeStep / NewTimeStep` when it is within 0.01 of 2 (`Some(2)`) or 3
/// (`Some(3)`), `None` for any other ratio.
fn near_two_or_three(ds_ratio: f64) -> Option<i32> {
    if (ds_ratio - 2.0).abs() < 0.01 {
        Some(2)
    } else if (ds_ratio - 3.0).abs() < 0.01 {
        Some(3)
    } else {
        None
    }
}

/// `Real64 DownInterpolate4HistoryValues(OldTimeStep, NewTimeStep, oldVals, newVals)`: fills
/// `new_vals` and returns `old_vals[0]`.
fn down_interpolate4(
    old_time_step: f64,
    new_time_step: f64,
    old_vals: &[f64; 4],
    new_vals: &mut [f64; 4],
) -> f64 {
    let ds_ratio = old_time_step / new_time_step;
    new_vals[0] = old_vals[0];
    match near_two_or_three(ds_ratio) {
        Some(2) => {
            new_vals[1] = (old_vals[0] + old_vals[1]) / 2.0;
            new_vals[2] = old_vals[1];
            new_vals[3] = (old_vals[1] + old_vals[2]) / 2.0;
        }
        Some(_) => {
            let delta10 = (old_vals[1] - old_vals[0]) / 3.0;
            new_vals[1] = old_vals[0] + delta10;
            new_vals[2] = new_vals[1] + delta10;
            new_vals[3] = old_vals[1];
        }
        None => {
            let delta10 = (old_vals[1] - old_vals[0]) / ds_ratio;
            new_vals[1] = old_vals[0] + delta10;
            new_vals[2] = new_vals[1] + delta10;
            new_vals[3] = new_vals[2] + delta10;
        }
    }
    old_vals[0]
}

/// The five-value `void DownInterpolate4HistoryValues(OldTimeStep, NewTimeStep, oldVal0 to 2,
/// newVal0 to 4)`.
fn down_interpolate5(old_time_step: f64, new_time_step: f64, old: [f64; 3]) -> [f64; 5] {
    let ds_ratio = old_time_step / new_time_step;
    let [v0, v1, v2] = old;
    match near_two_or_three(ds_ratio) {
        Some(2) => [v0, (v0 + v1) / 2.0, v1, (v1 + v2) / 2.0, v2],
        Some(_) => {
            let delta10 = (v1 - v0) / 3.0;
            let n1 = v0 + delta10;
            [v0, n1, n1 + delta10, v1, v1 + (v2 - v1) / 3.0]
        }
        None => {
            let delta10 = (v1 - v0) / ds_ratio;
            let n1 = v0 + delta10;
            let n2 = n1 + delta10;
            let n3 = n2 + delta10;
            [v0, n1, n2, n3, n3 + delta10]
        }
    }
}

/// The zone terms `updateTemperatures` reads and writes.
struct Zone {
    mat: f64,
    air_hum_rat: f64,
    xmat: [f64; 4],
    dsxmat: [f64; 4],
    w_prev_zone_ts: [f64; 4],
    ds_w_prev_zone_ts: [f64; 4],
    ztm: [f64; 4],
    w_prev_zone_ts_temp: [f64; 4],
    /// The system node's Temp, HumRat and Enthalpy, and the thermostat air temperature.
    node: Option<[f64; 4]>,
}

impl Zone {
    /// `updateTemperatures`. `nsys_changed` is `NumOfSysTimeSteps !=
    /// NumOfSysTimeStepsLastZoneTimeStep`.
    fn update_temperatures(
        &mut self,
        shorten: bool,
        use_zone_hist: bool,
        prior: f64,
        tsys: f64,
        nsys_changed: bool,
    ) {
        if shorten {
            if let Some(node) = &mut self.node {
                node[0] = self.xmat[0];
                node[3] = self.xmat[0];
                node[1] = self.w_prev_zone_ts[0];
                node[2] = PsyHFnTdbW(self.xmat[0], self.w_prev_zone_ts[0]);
            }
            if nsys_changed {
                self.mat = down_interpolate4(prior, tsys, &self.xmat, &mut self.dsxmat);
                self.air_hum_rat = down_interpolate4(
                    prior,
                    tsys,
                    &self.w_prev_zone_ts,
                    &mut self.ds_w_prev_zone_ts,
                );
            }
        }
        if use_zone_hist {
            self.ztm = self.xmat;
            self.w_prev_zone_ts_temp = self.w_prev_zone_ts;
        } else {
            self.ztm = self.dsxmat;
            self.w_prev_zone_ts_temp = self.ds_w_prev_zone_ts;
        }
    }
}

/// A 0 or 1 flag as a bool, `None` for anything else.
fn flag(v: i32) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// Port of `double UpdateTemperatures(int which, int shorten, ...)` (`updtemp.c:9`): see the
/// oracle's shim for the arguments and what `which` picks.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn UpdateTemperatures(
    which: std::ffi::c_int,
    shorten: std::ffi::c_int,
    usezonehist: std::ffi::c_int,
    prior: f64,
    tsys: f64,
    nsys: std::ffi::c_int,
    nsyslast: std::ffi::c_int,
    hasnode: std::ffi::c_int,
    mat: f64,
    w: f64,
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    ds0: f64,
    ds1: f64,
    ds2: f64,
    ds3: f64,
    wp0: f64,
    wp1: f64,
    wp2: f64,
    wp3: f64,
    dsw0: f64,
    dsw1: f64,
    dsw2: f64,
    dsw3: f64,
) -> f64 {
    let checked = (|| Some((flag(shorten)?, flag(usezonehist)?, flag(hasnode)?)))();
    let (Some((shorten, usezonehist, hasnode)), Ok(which @ 0..=21)) =
        (checked, usize::try_from(which))
    else {
        return f64::NAN;
    };
    let mut z = Zone {
        mat,
        air_hum_rat: w,
        xmat: [x0, x1, x2, x3],
        dsxmat: [ds0, ds1, ds2, ds3],
        w_prev_zone_ts: [wp0, wp1, wp2, wp3],
        ds_w_prev_zone_ts: [dsw0, dsw1, dsw2, dsw3],
        ztm: [0.0; 4],
        w_prev_zone_ts_temp: [0.0; 4],
        node: hasnode.then_some([0.0; 4]),
    };
    z.update_temperatures(shorten, usezonehist, prior, tsys, nsys != nsyslast);
    let node = z.node.unwrap_or([0.0; 4]);
    match which {
        0 => z.mat,
        1 => z.air_hum_rat,
        2..=5 => z.dsxmat[which - 2],
        6..=9 => z.ds_w_prev_zone_ts[which - 6],
        10..=13 => z.ztm[which - 10],
        14..=17 => z.w_prev_zone_ts_temp[which - 14],
        _ => node[which - 18],
    }
}

/// Port of `double DownInterpolate5(int which, double old, double nw, double v0, double v1,
/// double v2)` (`updtemp.c:16`): see the oracle's shim.
pub fn DownInterpolate5(
    which: std::ffi::c_int,
    old: f64,
    nw: f64,
    v0: f64,
    v1: f64,
    v2: f64,
) -> f64 {
    match usize::try_from(which) {
        Ok(i @ 0..=4) => down_interpolate5(old, nw, [v0, v1, v2])[i],
        _ => f64::NAN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn down_interpolation_by_ratio() {
        assert_eq!(
            down_interpolate5(1.0, 0.5, [20.0, 21.0, 23.0]),
            [20.0, 20.5, 21.0, 22.0, 23.0]
        );
        let third = down_interpolate5(0.75, 0.25, [20.0, 23.0, 26.0]);
        assert_eq!(third, [20.0, 21.0, 22.0, 23.0, 24.0]);
        assert_eq!(
            down_interpolate5(1.0, 0.25, [20.0, 24.0, 0.0]),
            [20.0, 21.0, 22.0, 23.0, 24.0]
        );
        let mut new = [0.0; 4];
        assert_eq!(
            down_interpolate4(1.0, 0.5, &[20.0, 21.0, 23.0, 9.0], &mut new),
            20.0
        );
        assert_eq!(new, [20.0, 20.5, 21.0, 22.0]);
    }

    #[test]
    fn shortening_rolls_back_and_interpolates() {
        let args = |which, nsys| {
            UpdateTemperatures(
                which, 1, 0, 0.25, 0.125, nsys, 1, 1, 30.0, 0.02, 22.0, 21.0, 20.0, 19.0, 0.0, 0.0,
                0.0, 0.0, 0.008, 0.007, 0.006, 0.005, 0.0, 0.0, 0.0, 0.0,
            )
        };
        assert_eq!(args(0, 2), 22.0);
        assert_eq!(args(3, 2), 21.5);
        assert_eq!(args(11, 2), 21.5);
        // The same step count keeps the down-stepped history.
        assert_eq!(args(0, 1), 30.0);
        assert_eq!(args(3, 1), 0.0);
        assert_eq!(args(18, 1), 22.0);
        assert_eq!(args(20, 1), PsyHFnTdbW(22.0, 0.008));
    }

    #[test]
    fn guards() {
        assert!(DownInterpolate5(5, 1.0, 0.5, 0.0, 0.0, 0.0).is_nan());
        let bad = UpdateTemperatures(
            0, 2, 0, 0.25, 0.125, 2, 1, 1, 30.0, 0.02, 22.0, 21.0, 20.0, 19.0, 0.0, 0.0, 0.0, 0.0,
            0.008, 0.007, 0.006, 0.005, 0.0, 0.0, 0.0, 0.0,
        );
        assert!(bad.is_nan());
    }
}
