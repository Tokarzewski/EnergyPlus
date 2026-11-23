//! The Rust port of `ksu.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ThermalComfort::CalcThermalComfortKSU` for one People object: the KSU two-node model of the
//! body's core and skin over a one hour exposure, integrated by `RKG` (Runge-Kutta-Gill, 0.01 h
//! steps) over the thermoregulation rates of `DERIV`, and the thermal sensation vote it gives;
//! with `CalcRadTemp` for an enclosure averaged MRT and radiant heaters. Every expression keeps the
//! C++ order, and `max` is ObjexxFCL's. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use comfort_rs::port::CalcSatVapPressFromTemp;
use psychrometrics_rs::port::PsyRhFnTdbWPb;

/// `ThermalComfort::BodySurfArea` (m2).
const BODY_SURF_AREA: f64 = 1.8;
/// `ThermalComfort::RadSurfEff`.
const RAD_SURF_EFF: f64 = 0.72;
/// `ThermalComfort::ActLevelConv` (W/m2 per met).
const ACT_LEVEL_CONV: f64 = 58.2;
/// `Constant::Kelvin`.
const KELVIN: f64 = 273.15;

/// ObjexxFCL `max(a, b)`: `b` only when `a < b`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The `state.dataThermalComforts` fields the KSU model reads and writes.
#[derive(Default)]
struct Ksu {
    air_temp: f64,
    act_level: f64,
    work_eff: f64,
    core_temp: f64,
    skin_temp: f64,
    core_therm_cap: f64,
    skin_therm_cap: f64,
    hc: f64,
    h: f64,
    op_temp: f64,
    vap_press: f64,
    clo_body_rat: f64,
    clo_therm_eff: f64,
    clo_permeat_eff: f64,
    time: f64,
    accl_pattern: f64,
    core_temp_neut: f64,
    skin_temp_neut: f64,
    skin_wet_sweat: f64,
    skin_wet_tot: f64,
    evap_heat_loss: f64,
    evap_heat_loss_max: f64,
    evap_heat_loss_sweat: f64,
    evap_heat_loss_sweat_prev: f64,
    therm_cndct: f64,
}

impl Ksu {
    /// `DERIV`: the rates of change of the core and skin temperatures (C/h). Like the C++, it
    /// reads the core and skin temperatures from the state, not from `RKG`'s `Y`.
    fn deriv(&mut self, temp_change: &mut [f64; 2]) {
        let core_signal_warm = self.core_temp - 36.98;
        let skin_signal_warm = self.skin_temp - 33.8;
        let skin_signal_cold = 32.1 - self.skin_temp;
        let core_signal_skin_sens = self.core_temp - 35.15;
        let core_signal_warm_max = max(0.0, core_signal_warm);
        let skin_signal_warm_max = max(0.0, skin_signal_warm);
        let skin_signal_cold_max = max(0.0, skin_signal_cold);

        let mut core_temp_sweat = self.core_temp;
        if core_temp_sweat > 38.29 {
            core_temp_sweat = 38.29;
        }
        let core_signal_sweat_warm = core_temp_sweat - self.core_temp_neut;
        let mut skin_temp_sweat = self.skin_temp;
        if skin_temp_sweat > 36.1 {
            skin_temp_sweat = 36.1;
        }
        let skin_signal_sweat_warm = skin_temp_sweat - self.skin_temp_neut;
        let core_signal_sweat_max = max(0.0, core_signal_sweat_warm);
        let skin_signal_sweat_max = max(0.0, skin_signal_sweat_warm);
        let mut skin_signal_sweat_cold = 33.37 - self.skin_temp;
        if self.skin_temp_neut < 33.37 {
            skin_signal_sweat_cold = self.skin_temp_neut - self.skin_temp;
        }
        let skin_signal_sweat_cold_max = max(0.0, skin_signal_sweat_cold);

        let core_signal_shiv_max = max(0.0, 36.9 - self.core_temp);
        let skin_signal_shiv_max = max(0.0, 32.5 - self.skin_temp);
        let mut shiv_response =
            20.0 * core_signal_shiv_max * skin_signal_shiv_max + 5.0 * skin_signal_shiv_max;
        if self.core_temp >= 37.1 {
            shiv_response = 0.0;
        }

        let weigh_fac = 260.0 + 70.0 * self.accl_pattern;
        let sweat_ctrl_fac = 1.0 + 0.05 * skin_signal_sweat_cold_max.powf(2.4);
        let evap_heat_loss_dry_sweat = ((weigh_fac * core_signal_sweat_max
            + 0.1 * weigh_fac * skin_signal_sweat_max)
            * (skin_signal_sweat_max / 8.5).exp())
            / sweat_ctrl_fac;

        let skin_vap_press = CalcSatVapPressFromTemp(self.skin_temp);
        self.evap_heat_loss_max =
            2.2 * self.hc * (skin_vap_press - self.vap_press) * self.clo_permeat_eff;
        if self.evap_heat_loss_max > 0.0 {
            self.skin_wet_sweat = evap_heat_loss_dry_sweat / self.evap_heat_loss_max;
            let evap_heat_loss_diff = 0.408 * (skin_vap_press - self.vap_press);
            self.evap_heat_loss = self.skin_wet_sweat * self.evap_heat_loss_max
                + (1.0 - self.skin_wet_sweat) * evap_heat_loss_diff;
            self.skin_wet_tot = self.evap_heat_loss / self.evap_heat_loss_max;
            if self.time == 0.0 {
                self.evap_heat_loss_sweat = evap_heat_loss_dry_sweat;
                self.evap_heat_loss_sweat_prev = evap_heat_loss_dry_sweat;
            }
            if self.skin_wet_tot > 0.4 {
                self.sweat_iteration(evap_heat_loss_dry_sweat, skin_vap_press);
            } else {
                self.evap_heat_loss_sweat = evap_heat_loss_dry_sweat;
            }
        } else {
            self.skin_wet_sweat = 1.0;
            self.skin_wet_tot = 1.0;
            self.evap_heat_loss_sweat = 0.5 * evap_heat_loss_dry_sweat;
            self.evap_heat_loss = self.evap_heat_loss_sweat;
        }

        let skin_cndct_dilation = 42.45 * core_signal_warm_max
            + 8.15 * core_signal_skin_sens.powf(0.8) * skin_signal_warm_max;
        let skin_cndct_constriction = 1.0 + 0.4 * skin_signal_cold_max;
        self.therm_cndct = 5.3 + (6.75 + skin_cndct_dilation) / skin_cndct_constriction;
        let skin_cndct_max = 75.0 + 10.0 * self.accl_pattern;
        if self.therm_cndct > skin_cndct_max {
            self.therm_cndct = skin_cndct_max;
        }

        let act_level_tot = self.act_level + shiv_response;
        let int_heat_prod_tot = act_level_tot - self.work_eff;
        let lat_resp_heat_loss = 0.0023 * act_level_tot * (44.0 - self.vap_press);
        let dry_resp_heat_loss = 0.0014 * act_level_tot * (34.0 - self.air_temp);
        let resp_heat_loss = lat_resp_heat_loss + dry_resp_heat_loss;
        let heat_flow = self.therm_cndct * (self.core_temp - self.skin_temp);
        temp_change[0] = (int_heat_prod_tot - resp_heat_loss - heat_flow) / self.core_therm_cap;
        if self.evap_heat_loss > self.evap_heat_loss_max {
            self.evap_heat_loss = self.evap_heat_loss_max;
        }
        let dry_heat_loss =
            self.h * self.clo_body_rat * self.clo_therm_eff * (self.skin_temp - self.op_temp);
        temp_change[1] = (heat_flow - self.evap_heat_loss - dry_heat_loss) / self.skin_therm_cap;
    }

    /// `DERIV`'s iteration for the sweat when the total skin wettedness is over 0.4.
    fn sweat_iteration(&mut self, evap_heat_loss_dry_sweat: f64, skin_vap_press: f64) {
        let mut iter_num = 0;
        let mut err_prev = 0.0;
        if self.skin_wet_sweat > 1.0 {
            self.skin_wet_sweat = 1.0;
        }
        loop {
            let evap_heat_loss_sweat_est = self.evap_heat_loss_sweat_prev;
            self.skin_wet_sweat = evap_heat_loss_sweat_est / self.evap_heat_loss_max;
            if self.skin_wet_sweat > 1.0 {
                self.skin_wet_sweat = 1.0;
            }
            let evap_heat_loss_diff = 0.408 * (skin_vap_press - self.vap_press);
            self.evap_heat_loss =
                (1.0 - self.skin_wet_tot) * evap_heat_loss_diff + self.evap_heat_loss_sweat;
            self.skin_wet_tot = self.evap_heat_loss / self.evap_heat_loss_max;
            if self.skin_wet_tot > 1.0 {
                self.skin_wet_tot = 1.0;
            }
            let skin_wet_signal = max(0.0, self.skin_wet_tot - 0.4);
            let sweat_supp_fac = 0.5 + 0.5 * (-5.6 * skin_wet_signal).exp();
            let evap_heat_loss_sweat_est_new = sweat_supp_fac * evap_heat_loss_dry_sweat;
            if iter_num == 0 {
                self.evap_heat_loss_sweat = evap_heat_loss_sweat_est_new;
            }
            let err = evap_heat_loss_sweat_est - evap_heat_loss_sweat_est_new;
            if iter_num != 0 {
                if (err_prev * err) < 0.0 {
                    self.evap_heat_loss_sweat =
                        (evap_heat_loss_sweat_est + evap_heat_loss_sweat_est_new) / 2.0;
                }
                if (err_prev * err) >= 0.0 {
                    self.evap_heat_loss_sweat = evap_heat_loss_sweat_est_new;
                }
            }
            if (err.abs() <= 0.5) || (iter_num >= 10) {
                break;
            }
            iter_num += 1;
            self.evap_heat_loss_sweat_prev = self.evap_heat_loss_sweat;
            err_prev = err;
        }
    }

    /// `RKG`: one Runge-Kutta-Gill step of `h` hours over `y`, with `dy` and `c` its work arrays.
    fn rkg(&mut self, h: f64, y: &mut [f64; 2], dy: &mut [f64; 2], c: &mut [f64; 2]) {
        const A: [f64; 2] = [0.29289321881345, 1.70710678118654];
        let h2 = 0.5 * h;
        self.deriv(dy);
        for i in 0..2 {
            let b = h2 * dy[i] - c[i];
            y[i] += b;
            c[i] += 3.0 * b - h2 * dy[i];
        }
        self.time += h2;
        for a in A {
            self.deriv(dy);
            for i in 0..2 {
                let b = a * (h * dy[i] - c[i]);
                y[i] += b;
                c[i] += 3.0 * b - a * h * dy[i];
            }
        }
        self.time += h2;
        self.deriv(dy);
        for i in 0..2 {
            let b = (h * dy[i] - 2.0 * c[i]) / 6.0;
            y[i] += b;
            c[i] += 3.0 * b - h2 * dy[i];
        }
        self.deriv(dy);
    }
}

/// One People object's comfort inputs (see the oracle's shim).
struct Inputs {
    out_baro: f64,
    ztav: f64,
    hum_rat: f64,
    mrt: f64,
    q_rad: f64,
    act: f64,
    eff: f64,
    clo: f64,
    vel: f64,
}

/// `CalcRadTemp` for an enclosure averaged MRT, with the radiant heater gain `q` (W) to people.
fn calc_rad_temp(mrt: f64, q: f64) -> f64 {
    const AREA_EFF: f64 = 1.8;
    const STEFAN_BOLTZMANN_CONST: f64 = 5.6697e-8;
    let mut rad_temp = mrt;
    let q_to_person = q + 0.0 + 0.0 + 0.0 + 0.0;
    if q_to_person > 0.0 {
        rad_temp += KELVIN;
        let t2 = rad_temp * rad_temp;
        rad_temp = (t2 * t2 + (q_to_person / AREA_EFF / STEFAN_BOLTZMANN_CONST))
            .sqrt()
            .sqrt();
        rad_temp -= KELVIN;
    }
    rad_temp
}

/// `CalcThermalComfortKSU` for one People object: the thermal sensation vote, the MRT and the
/// operative temperature it reports.
fn calc_thermal_comfort_ksu(p: &Inputs) -> (f64, f64, f64) {
    const CLO_EMISS: f64 = 0.8;
    const TSV_MAX: f64 = 4.0;
    const TIME_EXPOS: f64 = 1.0;
    const BODY_WT: f64 = 70.0;
    let mut s = Ksu {
        air_temp: p.ztav,
        ..Ksu::default()
    };
    let rad_temp = calc_rad_temp(p.mrt, p.q_rad);
    let rel_hum = PsyRhFnTdbWPb(s.air_temp, p.hum_rat, p.out_baro);
    s.act_level = p.act / BODY_SURF_AREA;
    s.work_eff = p.eff * s.act_level;
    let clo_unit = p.clo;
    let mut air_vel = p.vel;
    let int_heat_prod = s.act_level - s.work_eff;
    s.core_temp = 37.0;
    s.skin_temp = 31.0;
    s.core_therm_cap = 0.9 * BODY_WT * 0.97 / BODY_SURF_AREA;
    s.skin_therm_cap = 0.1 * BODY_WT * 0.97 / BODY_SURF_AREA;
    if air_vel < 0.137 {
        air_vel = 0.137;
    }
    s.hc = 8.3 * air_vel.sqrt();
    let emiss_avg = RAD_SURF_EFF * CLO_EMISS + (1.0 - RAD_SURF_EFF) * 1.0;
    let hr = emiss_avg * (3.87 + 0.031 * rad_temp);
    s.h = hr + s.hc;
    s.op_temp = (s.hc * s.air_temp + hr * rad_temp) / s.h;
    s.vap_press = CalcSatVapPressFromTemp(s.air_temp);
    s.vap_press *= rel_hum;
    s.clo_body_rat = 1.0 + 0.2 * clo_unit;
    s.clo_therm_eff = 1.0 / (1.0 + 0.155 * s.h * s.clo_body_rat * clo_unit);
    s.clo_permeat_eff = 1.0 / (1.0 + 0.143 * s.hc * clo_unit);
    let int_heat_prod_met_max = max(1.0, int_heat_prod / ACT_LEVEL_CONV);
    let therm_cndct_neut = 12.05 * (0.2266 * (int_heat_prod_met_max - 1.0)).exp();
    let skin_wet_neut = 0.02 + 0.4 * (1.0 - (-0.6 * (int_heat_prod_met_max - 1.0)).exp());
    let therm_cndct_min = (therm_cndct_neut - 5.3) * 0.26074074 + 5.3;
    let them_cndct_75_fac = 1.0 / (75.0 - therm_cndct_neut);
    let them_cndct_fac = 1.0 / (therm_cndct_neut - therm_cndct_min);

    // One day: the unacclimated man.
    let day_num: f64 = 1.0;
    s.time = 0.0;
    let time_change = 0.01;
    let mut temp = [s.core_temp, s.skin_temp];
    let mut coeff = [0.0; 2];
    let mut temp_change = [0.0; 2];
    s.accl_pattern = 1.0 - (-0.12 * (day_num - 1.0)).exp();
    s.core_temp_neut = 36.9 - 0.6 * s.accl_pattern;
    s.skin_temp_neut = 33.8 - 1.6 * s.accl_pattern;
    s.act_level -= 0.07 * s.act_level * s.accl_pattern;
    let skin_temp_neut_fac = 1.0 / (1.0 - skin_wet_neut);
    s.deriv(&mut temp_change);
    let mut ksu_tsv;
    loop {
        let skin_wet_fac = (s.skin_wet_sweat - skin_wet_neut) * skin_temp_neut_fac;
        let vasodilation_fac = (s.therm_cndct - therm_cndct_neut) * them_cndct_75_fac;
        let vasoconstrict_fac = (therm_cndct_neut - s.therm_cndct) * them_cndct_fac;
        if vasodilation_fac < 0.0 {
            ksu_tsv = -1.46153 * vasoconstrict_fac
                + 3.74721 * (vasoconstrict_fac * vasoconstrict_fac)
                - 6.168856 * (vasoconstrict_fac * vasoconstrict_fac * vasoconstrict_fac);
        } else {
            ksu_tsv = (5.0 - 6.56 * (rel_hum - 0.50)) * skin_wet_fac;
            if ksu_tsv > TSV_MAX {
                ksu_tsv = TSV_MAX;
            }
        }
        s.core_temp = temp[0];
        s.skin_temp = temp[1];
        s.evap_heat_loss_sweat_prev = s.evap_heat_loss_sweat;
        s.rkg(time_change, &mut temp, &mut temp_change, &mut coeff);
        if s.time > TIME_EXPOS {
            break;
        }
    }
    (ksu_tsv, rad_temp, (rad_temp + s.air_temp) / 2.0)
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double KsuTsv(int which, double outbaro, double ztav, double w, double mrt, double
/// qrad, double act, double eff, double clo, double vel)` (`ksu.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn KsuTsv(
    which: std::ffi::c_int,
    outbaro: f64,
    ztav: f64,
    w: f64,
    mrt: f64,
    qrad: f64,
    act: f64,
    eff: f64,
    clo: f64,
    vel: f64,
) -> f64 {
    let ok = (0..=2).contains(&which)
        && within(outbaro, 60000.0, 110000.0)
        && within(ztav, -10.0, 60.0)
        && within(w, 0.0, 0.05)
        && within(mrt, -10.0, 60.0)
        && within(qrad, 0.0, 2000.0)
        && within(act, 50.0, 900.0)
        && within(eff, 0.0, 0.5)
        && within(clo, 0.0, 3.0)
        && within(vel, 0.0, 5.0);
    if !ok {
        return f64::NAN;
    }
    let inputs = Inputs {
        out_baro: outbaro,
        ztav,
        hum_rat: w,
        mrt,
        q_rad: qrad,
        act,
        eff,
        clo,
        vel,
    };
    let (tsv, mrt, op_temp) = calc_thermal_comfort_ksu(&inputs);
    match which {
        0 => tsv,
        1 => mrt,
        _ => op_temp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warmer_feels_warmer() {
        let tsv = |at: f64| KsuTsv(0, 101325.0, at, 0.008, at, 0.0, 120.0, 0.0, 0.5, 0.1);
        assert!(tsv(18.0) < tsv(24.0));
        assert!(tsv(24.0) < tsv(32.0));
        assert!(tsv(40.0) <= 4.0);
        assert_eq!(
            KsuTsv(1, 101325.0, 22.0, 0.008, 21.0, 0.0, 120.0, 0.0, 0.5, 0.1),
            21.0
        );
        assert!(KsuTsv(1, 101325.0, 22.0, 0.008, 21.0, 100.0, 120.0, 0.0, 0.5, 0.1) > 21.0);
        assert!(KsuTsv(3, 101325.0, 22.0, 0.008, 21.0, 0.0, 120.0, 0.0, 0.5, 0.1).is_nan());
    }
}
