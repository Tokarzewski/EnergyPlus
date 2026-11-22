//! The Rust port of `glhesim.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The ground heat exchanger's time stepping from EnergyPlus `GroundHeatExchangers/Base.cc`:
//! `GLHEBase::calcGroundHeatExchanger` and `calcAggregateLoad` (sub-hourly, hourly and monthly
//! load aggregation with g-function superposition), for a vertical borehole (`GLHEVert`'s
//! `getAnnualTimeConstant`, `getGFunc` and `calcHXResistance`), run for a number of time steps as
//! the oracle runs it. The HX resistance comes from `vert-fluid-rs` and `interpGFunc` from
//! `vert-gfunc-rs`. Water's specific heat goes through a copy of the glycol's specific heat cache
//! (`EP_cache_GlycolSpecificHeat`), because over many time steps it can return the value of an
//! earlier, nearby temperature. Nothing here panics: the history arrays are read through `at`,
//! which gives NaN outside them (the oracle's domain keeps every index inside).

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

use std::collections::HashMap;

use fluidproperties_rs::port::WaterSpecificHeat_raw;
use vert_fluid_rs::port::hx_resistance;
use vert_gfunc_rs::port::interp;

const PI: f64 = std::f64::consts::PI; // `Constant::Pi`
const HRS_PER_MONTH: f64 = 730.0; // `hrsPerMonth`
const MAX_TS_IN_HR: f64 = 60.0; // `maxTSinHr`
const SUB_AGG: i32 = 15;
const AGG: i32 = 192;

/// `GlycolProps::getSpecificHeat` for water (`Num` 1) with its cache: the key is the top 36 bits
/// of `T + 1000`, the slot its low 20 bits, and every slot starts with key 1000.
#[derive(Default)]
struct CpCache(HashMap<u64, (u64, f64)>);

impl CpCache {
    fn get(&mut self, t: f64) -> f64 {
        let tag = (t + 1000.0 * 1.0).to_bits() >> (64 - 12 - 24);
        let slot = self.0.entry(tag & ((1 << 20) - 1)).or_insert((1000, 0.0));
        if slot.0 != tag {
            *slot = (tag, WaterSpecificHeat_raw(t));
        }
        slot.1
    }
}

/// A 1-based history array (`Array1D`).
fn hist(n: usize) -> Vec<f64> {
    vec![0.0; n + 1]
}

fn at(v: &[f64], i: i32) -> f64 {
    usize::try_from(i)
        .ok()
        .and_then(|i| v.get(i))
        .copied()
        .unwrap_or(f64::NAN)
}

fn at_i(v: &[i32], i: i32) -> i32 {
    usize::try_from(i)
        .ok()
        .and_then(|i| v.get(i))
        .copied()
        .unwrap_or(0)
}

fn set(v: &mut [f64], i: i32, x: f64) {
    if let Some(e) = usize::try_from(i).ok().and_then(|i| v.get_mut(i)) {
        *e = x;
    }
}

/// `std::shift_right(begin, end, 1)` on a 1-based array: element 1 keeps its value.
fn shift<T: Copy>(v: &mut [T]) {
    if v.len() > 2 {
        v.copy_within(1..v.len() - 1, 2);
    }
}

/// The borehole and plant inputs that do not change between time steps.
struct Bore {
    bh: [f64; 4],
    soil_k: f64,
    diffusivity: f64,
    pipe: [f64; 3],
    num_bh: i32,
    total_tube_length: f64,
    bh_radius: f64,
    g_ref_ratio: f64,
    lntts: Vec<f64>,
    gfnc: Vec<f64>,
}

impl Bore {
    /// `GLHEVert::getGFunc` (`Vertical.cc:1464`).
    fn g_func(&self, time: f64) -> f64 {
        let mut g = interp(&self.lntts, &self.gfnc, time.ln());
        let bh_length = self.bh[0];
        if self.bh_radius / bh_length != self.g_ref_ratio {
            g -= (self.bh_radius / (bh_length * self.g_ref_ratio)).ln();
        }
        g
    }
}

/// The state `GLHEBase` keeps between time steps, as the constructor and the oracle set it up.
struct Glhe {
    cp: CpCache,
    qn_monthly_agg: Vec<f64>,
    qn_hr: Vec<f64>,
    qn_sub_hr: Vec<f64>,
    last_hour_n: Vec<i32>,
    prev_time_steps: Vec<f64>,
    prev_hour: i32,
    last_qn_sub_hr: f64,
    prev_n: i32,
    update_cur_sim_time: bool,
    n: i32,
    current_sim_time: f64,
    loc_hour_of_day: i32,
    loc_day_of_sim: i32,
    bh_temp: f64,
    outlet_temp: f64,
    ave_fluid_temp: f64,
    qglhe: f64,
    hx_resistance: f64,
    tout_new: f64,
}

impl Glhe {
    fn new() -> Self {
        let sub = ((SUB_AGG + 1) as f64 * MAX_TS_IN_HR + 1.0) as usize;
        Glhe {
            cp: CpCache::default(),
            qn_monthly_agg: hist(12),
            qn_hr: hist((730 + AGG + SUB_AGG) as usize),
            qn_sub_hr: hist(sub),
            last_hour_n: vec![0; SUB_AGG as usize + 2],
            prev_time_steps: hist(sub),
            prev_hour: 1,
            last_qn_sub_hr: 0.0,
            prev_n: 1,
            update_cur_sim_time: true,
            n: 1,
            current_sim_time: 0.0,
            loc_hour_of_day: 0,
            loc_day_of_sim: 0,
            bh_temp: 0.0,
            outlet_temp: 0.0,
            ave_fluid_temp: 0.0,
            qglhe: 0.0,
            hx_resistance: 0.0,
            tout_new: 19.375,
        }
    }

    /// `GLHEBase::calcAggregateLoad` (`Base.cc:382`).
    fn aggregate(&mut self) {
        if self.current_sim_time <= 0.0 {
            return;
        }
        let pts = &self.prev_time_steps;
        if self.prev_hour != self.loc_hour_of_day {
            let mut sum = 0.0;
            let mut j = 1;
            while j <= self.n - at_i(&self.last_hour_n, 1) {
                sum += at(&self.qn_sub_hr, j) * (at(pts, j) - at(pts, j + 1)).abs();
                j += 1;
            }
            if at(pts, 1) != at(pts, j) {
                sum /= (at(pts, 1) - at(pts, j)).abs();
            } else {
                sum /= 0.05;
            }
            shift(&mut self.qn_hr);
            set(&mut self.qn_hr, 1, sum);
            shift(&mut self.last_hour_n);
            self.last_hour_n[1] = self.n;
        }
        let hour = (self.loc_day_of_sim - 1) * 24 + self.loc_hour_of_day;
        if f64::from(hour) % HRS_PER_MONTH == 0.0 && self.prev_hour != self.loc_hour_of_day {
            let month =
                (f64::from(self.loc_day_of_sim * 24 + self.loc_hour_of_day) / HRS_PER_MONTH) as i32;
            let mut sum = 0.0;
            for j in 1..=HRS_PER_MONTH as i32 {
                sum += at(&self.qn_hr, j);
            }
            set(&mut self.qn_monthly_agg, month, sum / HRS_PER_MONTH);
        }
        self.prev_hour = self.loc_hour_of_day;
    }

    /// `GLHEBase::calcGroundHeatExchanger` (`Base.cc:64`) for one time step; WarmupFlag is on, so
    /// the design-day reset never fires.
    fn step(
        &mut self,
        b: &Bore,
        clock: (i32, i32, i32, f64),
        inlet_temp: f64,
        mass_flow: f64,
        temp_ground: f64,
    ) {
        let (day, hour, ts, ts_zone) = clock;
        let cp = self.cp.get(inlet_temp);
        let k_ground = 2.0 * PI * b.soil_k;
        let time_ss_factor = (b.bh[0] * b.bh[0] / (9.0 * b.diffusivity)) / 3600.0 / 8760.0 * 8760.0;
        let gf = |t: f64| b.g_func(t / time_ss_factor) / k_ground;

        if day == 1 && self.update_cur_sim_time {
            self.prev_time_steps.fill(0.0);
            self.qn_hr.fill(0.0);
            self.qn_monthly_agg.fill(0.0);
            self.qn_sub_hr.fill(0.0);
            self.last_hour_n.fill(1);
            self.n = 1;
            self.update_cur_sim_time = false;
        }
        let cst = f64::from((day - 1) * 24 + hour - 1) + f64::from(ts - 1) * ts_zone + 0.0;
        self.current_sim_time = cst;
        self.loc_hour_of_day = (cst % 24.0 + 1.0) as i32;
        self.loc_day_of_sim = (cst / 24.0 + 1.0) as i32;
        if day > 1 {
            self.update_cur_sim_time = true;
        }
        if cst <= 0.0 {
            self.prev_time_steps.fill(0.0);
            self.aggregate();
            return;
        }
        if at(&self.prev_time_steps, 1) != cst {
            shift(&mut self.prev_time_steps);
            set(&mut self.prev_time_steps, 1, cst);
            self.n += 1;
        }
        if self.n != self.prev_n {
            self.prev_n = self.n;
            shift(&mut self.qn_sub_hr);
            set(&mut self.qn_sub_hr, 1, self.last_qn_sub_hr);
        }
        self.aggregate();
        let hx = hx_resistance(b.bh, b.soil_k, b.pipe, mass_flow, b.num_bh, inlet_temp, cp);
        self.hx_resistance = hx;

        let (qs, qh, qm, pts) = (
            &self.qn_sub_hr,
            &self.qn_hr,
            &self.qn_monthly_agg,
            &self.prev_time_steps,
        );
        let len = b.total_tube_length;
        let tmp;
        let fluid_ave;
        let mut sum_total = 0.0;
        let icst = cst as i32;
        if self.n == 1 {
            if mass_flow <= 0.0 {
                tmp = 0.0;
                fluid_ave = temp_ground;
                self.tout_new = inlet_temp;
            } else {
                let g = b.g_func(cst / time_ss_factor);
                let c_1 = len / (2.0 * mass_flow * cp);
                tmp = (temp_ground - inlet_temp) / (g / k_ground + hx + c_1);
                fluid_ave = temp_ground - tmp * hx;
                self.tout_new = temp_ground - tmp * (g / k_ground + hx - c_1);
            }
        } else {
            // The sub-hourly superposition over `limit` steps; `tail` is the last step's load
            // difference.
            let sub_hourly = |limit: i32, tail: &dyn Fn(i32) -> f64| {
                let mut s = 0.0;
                for i in 1..=limit {
                    let r = gf(cst - at(pts, i + 1));
                    if i == limit {
                        s += tail(i) * r;
                        break;
                    }
                    s += (at(qs, i) - at(qs, i + 1)) * r;
                }
                s
            };
            if cst < HRS_PER_MONTH + f64::from(AGG) + f64::from(SUB_AGG) {
                let index_n = if icst < SUB_AGG {
                    icst + 1
                } else {
                    SUB_AGG + 1
                };
                let limit = self.n - at_i(&self.last_hour_n, index_n);
                let sub_sum = sub_hourly(limit, &|i| {
                    if icst >= SUB_AGG {
                        at(qs, i) - at(qh, index_n)
                    } else {
                        at(qs, i)
                    }
                });
                let mut hour_sum = 0.0;
                for i in SUB_AGG + 1..=icst {
                    if i == icst {
                        hour_sum += at(qh, i) * gf(cst);
                        break;
                    }
                    hour_sum +=
                        (at(qh, i) - at(qh, i + 1)) * gf(cst - f64::from(icst) + f64::from(i));
                }
                sum_total = sub_sum + hour_sum;
            } else {
                let months = ((cst + 1.0) / HRS_PER_MONTH) as i32;
                let current_month = if cst
                    < (f64::from(months) * HRS_PER_MONTH) + f64::from(AGG) + f64::from(SUB_AGG)
                {
                    months - 1
                } else {
                    months
                };
                let mut month_sum = 0.0;
                for i in 1..=current_month {
                    if i == 1 {
                        month_sum += at(qm, i) * gf(cst);
                        continue;
                    }
                    month_sum +=
                        (at(qm, i) - at(qm, i - 1)) * gf(cst - f64::from(i - 1) * HRS_PER_MONTH);
                }
                let limit = (cst - f64::from(current_month) * HRS_PER_MONTH) as i32;
                let mut hour_sum = 0.0;
                for i in 1 + SUB_AGG..=limit {
                    let r = gf(cst - f64::from(icst) + f64::from(i));
                    if i == limit {
                        hour_sum += (at(qh, i) - at(qm, current_month)) * r;
                        break;
                    }
                    hour_sum += (at(qh, i) - at(qh, i + 1)) * r;
                }
                let limit = self.n - at_i(&self.last_hour_n, SUB_AGG + 1);
                let sub_sum = sub_hourly(limit, &|i| at(qs, i) - at(qh, SUB_AGG + 1));
                sum_total = month_sum + hour_sum + sub_sum;
            }
            let rq = gf(cst - at(pts, 2));
            if mass_flow <= 0.0 {
                tmp = 0.0;
                fluid_ave = temp_ground - sum_total;
                self.tout_new = inlet_temp;
            } else {
                let c0 = rq;
                let c1 = temp_ground - (sum_total - at(qs, 1) * rq);
                let c2 = len / (2.0 * mass_flow * cp);
                let c3 = mass_flow * cp / len;
                tmp = (c1 - inlet_temp) / (hx + c0 - c2 + (1.0 / c3));
                fluid_ave = c1 - (c0 + hx) * tmp;
                self.tout_new = c1 + (c2 - c0 - hx) * tmp;
            }
        }
        self.bh_temp = temp_ground - sum_total;
        self.last_qn_sub_hr = tmp;
        self.outlet_temp = self.tout_new;
        self.qglhe = tmp * len;
        self.ave_fluid_temp = fluid_ave;
    }
}

pub fn GSRun(
    which: i32,
    steps: i32,
    tsPerHour: i32,
    tGround: f64,
    tIn0: f64,
    tInAmp: f64,
    flow0: f64,
    offEvery: i32,
    gA: f64,
    gB: f64,
    gRefRatio: f64,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    numBH: i32,
) -> f64 {
    if !(0..=4).contains(&which)
        || !(1..=6000).contains(&steps)
        || !(1..=60).contains(&tsPerHour)
        || numBH < 1
    {
        return f64::NAN;
    }
    let bh_radius = bhDiameter / 2.0;
    let b = Bore {
        bh: [bhLength, bhDiameter, uTubeDist, groutK],
        soil_k: soilK,
        diffusivity: soilK / soilRhoCp,
        pipe: [outDia, thickness, kPipe],
        num_bh: numBH,
        total_tube_length: f64::from(numBH) * bhLength,
        bh_radius,
        g_ref_ratio: if gRefRatio > 0.0 {
            gRefRatio
        } else {
            bh_radius / bhLength
        },
        lntts: (0..=36).map(|k| -15.0 + 0.5 * f64::from(k)).collect(),
        gfnc: (0..=36)
            .map(|k| gA * f64::from(k) + gB * f64::from(k) * f64::from(k))
            .collect(),
    };
    let ts_zone = 1.0 / f64::from(tsPerHour);
    let mut g = Glhe::new();
    for s in 0..steps {
        let hour = s / tsPerHour;
        let clock = (hour / 24 + 1, hour % 24 + 1, s % tsPerHour + 1, ts_zone);
        let inlet = tIn0 + tInAmp * f64::from((7 * s) % 13 - 6) / 6.0;
        let flow = if offEvery > 0 && s % offEvery == offEvery - 1 {
            0.0
        } else {
            flow0
        };
        g.step(&b, clock, inlet, flow, tGround);
    }
    match which {
        0 => g.outlet_temp,
        1 => g.ave_fluid_temp,
        2 => g.bh_temp,
        3 => g.qglhe,
        _ => g.hx_resistance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_returns_the_first_nearby_temperature() {
        let mut c = CpCache::default();
        let a = c.get(20.0);
        assert_eq!(a, WaterSpecificHeat_raw(20.0));
        // 20 + 1e-9 shares the key with 20, so the cache returns 20's value.
        assert_eq!(c.get(20.0 + 1e-9), a);
    }

    #[test]
    fn shift_keeps_the_first_element() {
        let mut v = [0, 1, 2, 3];
        shift(&mut v);
        assert_eq!(v, [0, 1, 1, 2]);
    }
}
