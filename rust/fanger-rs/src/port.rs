//! The Rust port of `fanger.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ThermalComfort::CalcFangerPMV` (Fanger's predicted mean vote, with its clothing surface
//! temperature iteration) and `ThermalComfort::CalcStandardEffectiveTemp` (the Pierce two-node
//! model's hour of minute-by-minute temperature regulation, the effective temperature and the
//! standard effective temperature). `PsyPsatFnTemp` comes from `psychrometrics-rs` and
//! `CalcSatVapPressFromTempTorr` from `comfort-rs`. Every expression keeps the C++ order. The
//! oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use comfort_rs::port::CalcSatVapPressFromTempTorr;
use psychrometrics_rs::port::PsyPsatFnTemp;

/// `ThermalComfort::TAbsConv` (`Constant::Kelvin`).
const T_ABS_CONV: f64 = 273.15;
/// `ThermalComfort::ActLevelConv` (W/m2 per met).
const ACT_LEVEL_CONV: f64 = 58.2;
/// `ThermalComfort::BodySurfAreaPierce` (m2).
const BODY_SURF_AREA_PIERCE: f64 = 1.8258;
/// `ThermalComfort::StefanBoltz` (W/m2K4).
const STEFAN_BOLTZ: f64 = 5.6697e-8;

/// ObjexxFCL `pow_4`: `(x * x) * (x * x)`.
fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

/// ObjexxFCL `max` and `std::max` for doubles: `a < b ? b : a`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// What `CalcFangerPMV` returns and leaves in the state.
struct Pmv {
    pmv: f64,
    clo_surf_temp: f64,
    iter_num: i32,
    hc: f64,
}

/// `CalcFangerPMV`.
fn calc_fanger_pmv(
    air_temp: f64,
    rad_temp: f64,
    rel_hum: f64,
    air_vel: f64,
    act_level: f64,
    clo_unit: f64,
    work_eff: f64,
) -> Pmv {
    const MAX_ITER: i32 = 150;
    const STOP_ITER_CRIT: f64 = 0.00015;
    let mut vap_press = PsyPsatFnTemp(air_temp);
    vap_press *= rel_hum;
    let int_heat_prod = act_level - work_eff;
    let std_icl = 0.155 * clo_unit;
    let clo_body_rat = if std_icl < 0.078 {
        1.0 + 1.29 * std_icl
    } else {
        1.05 + 0.645 * std_icl
    };
    let abs_rad_temp = rad_temp + T_ABS_CONV;
    let abs_air_temp = air_temp + T_ABS_CONV;
    let clo_insul = std_icl * clo_body_rat;
    let p2 = clo_insul * 3.96;
    let p3 = clo_insul * 100.0;
    let p1 = clo_insul * abs_air_temp;
    let p4 = 308.7 - 0.028 * int_heat_prod + p2 * pow_4(abs_rad_temp / 100.0);

    let abs_clo_surf_temp = abs_air_temp + (35.5 - air_temp) / (3.5 * std_icl + 0.1);
    let mut xn = abs_clo_surf_temp / 100.0;
    let hc_for = 12.1 * air_vel.sqrt();
    let mut iter_num = 0;
    let mut xf = xn;
    let mut hc = 0.0;
    while ((xn - xf).abs() > STOP_ITER_CRIT || iter_num == 0) && iter_num < MAX_ITER {
        xf = (xf + xn) / 2.0;
        let hc_nat = 2.38 * (100.0 * xf - abs_air_temp).abs().sqrt().sqrt();
        hc = max(hc_for, hc_nat);
        xn = (p4 + p1 * hc - p2 * pow_4(xf)) / (100.0 + p3 * hc);
        iter_num += 1;
    }
    let abs_clo_surf_temp = 100.0 * xn;
    let clo_surf_temp = abs_clo_surf_temp - T_ABS_CONV;

    let rad_heat_loss =
        3.96 * clo_body_rat * (pow_4(abs_clo_surf_temp * 0.01) - pow_4(abs_rad_temp * 0.01));
    let conv_heat_loss = clo_body_rat * hc * (clo_surf_temp - air_temp);
    let dry_heat_loss = rad_heat_loss + conv_heat_loss;
    let mut evap_heat_loss_reg_comf = 0.0;
    if int_heat_prod > 58.2 {
        evap_heat_loss_reg_comf = 0.42 * (int_heat_prod - ACT_LEVEL_CONV);
    }
    let evap_heat_loss_diff = 3.05 * 0.001 * (5733.0 - 6.99 * int_heat_prod - vap_press);
    let evap_heat_loss = evap_heat_loss_reg_comf + evap_heat_loss_diff;
    let lat_resp_heat_loss = 1.7 * 0.00001 * act_level * (5867.0 - vap_press);
    let dry_resp_heat_loss = 0.0014 * act_level * (34.0 - air_temp);
    let resp_heat_loss = lat_resp_heat_loss + dry_resp_heat_loss;
    let therm_sens_trans_coef = 0.303 * (-0.036 * act_level).exp() + 0.028;
    let pmv =
        therm_sens_trans_coef * (int_heat_prod - evap_heat_loss - resp_heat_loss - dry_heat_loss);
    Pmv {
        pmv,
        clo_surf_temp,
        iter_num,
        hc,
    }
}

/// What `CalcStandardEffectiveTemp` returns and leaves in the state.
struct Set {
    set: f64,
    eff_temp: f64,
    skin_temp: f64,
    core_temp: f64,
    skin_wet_tot: f64,
    dry_heat_loss_set: f64,
}

/// A person's surroundings and state, as `CalcStandardEffectiveTemp` takes them: air and mean
/// radiant temperatures (C), relative humidity (0 to 1), air velocity (m/s), activity (met),
/// clothing (clo) and work efficiency (W/m2).
#[derive(Clone, Copy)]
struct Person {
    air_temp: f64,
    rad_temp: f64,
    rel_hum: f64,
    air_vel: f64,
    act_met: f64,
    clo_unit: f64,
    work_eff: f64,
}

/// `CalcStandardEffectiveTemp`, at outdoor barometric pressure `out_baro_press`.
fn calc_standard_effective_temp(out_baro_press: f64, person: Person) -> Set {
    let Person {
        air_temp,
        rad_temp,
        rel_hum,
        mut air_vel,
        act_met,
        clo_unit,
        work_eff,
    } = person;
    const CLO_FAC: f64 = 0.25;
    const BODY_WEIGHT: f64 = 69.9;
    const SWEAT_CONT_CONST: f64 = 170.0;
    const DRI_COEFF_VASODILATION: f64 = 120.0;
    const DRI_COEFF_VASOCONSTRICTION: f64 = 0.5;
    const MAX_SKIN_BLOOD_FLOW: f64 = 90.0;
    const MIN_SKIN_BLOOD_FLOW: f64 = 0.5;
    const REG_SWEAT_MAX: f64 = 500.0;
    const SKIN_TEMP_SET: f64 = 33.7;
    const CORE_TEMP_SET: f64 = 36.8;
    const SKIN_BLOOD_FLOW_SET: f64 = 6.3;
    const SKIN_MASS_RAT_SET: f64 = 0.1;

    if air_vel < 0.1 {
        air_vel = 0.1;
    }
    let vap_press = rel_hum * CalcSatVapPressFromTempTorr(air_temp);
    let mut act_level = ACT_LEVEL_CONV * act_met;
    let p_in_atmospheres = out_baro_press / 101325.0;
    let r_clo = clo_unit * 0.155;
    let tot_clo_fac = 1.0 + 0.15 * clo_unit;
    let lewis_ratio = 2.2 / p_in_atmospheres;
    let mut skin_temp = SKIN_TEMP_SET;
    let mut core_temp = CORE_TEMP_SET;
    let mut skin_blood_flow = SKIN_BLOOD_FLOW_SET;
    let mut skin_mass_rat = SKIN_MASS_RAT_SET;
    let (evap_eff, clo_insul) = if clo_unit <= 0.0 {
        (0.38 * air_vel.powf(-0.29), 1.0)
    } else {
        (0.59 * air_vel.powf(-0.08), 0.45)
    };
    let corrected_hc = 3.0 * p_in_atmospheres.powf(0.53);
    let forced_hc = 8.600001 * (air_vel * p_in_atmospheres).powf(0.53);
    let hc = max(corrected_hc, forced_hc);
    let mut hr = 4.7;
    let mut evap_heat_loss = 0.1 * act_met;
    let mut r_air = 1.0 / (tot_clo_fac * (hc + hr));
    let mut op_temp = (hr * rad_temp + hc * air_temp) / (hc + hr);
    let act_level_start = act_level;
    let avg_body_temp_set =
        SKIN_MASS_RAT_SET * SKIN_TEMP_SET + (1.0 - SKIN_MASS_RAT_SET) * CORE_TEMP_SET;
    let mut h = 0.0;
    let mut dry_heat_loss = 0.0;
    let mut sat_skin_vap_press = 0.0;
    let mut skin_wet_tot = 0.0;

    for _ in 1..=60 {
        let mut clo_surf_temp = (r_air * skin_temp + r_clo * op_temp) / (r_air + r_clo);
        loop {
            hr = 4.0 * STEFAN_BOLTZ * ((clo_surf_temp + rad_temp) / 2.0 + 273.15).powf(3.0) * 0.72;
            r_air = 1.0 / (tot_clo_fac * (hc + hr));
            op_temp = (hr * rad_temp + hc * air_temp) / (hc + hr);
            let clo_surf_temp_new = (r_air * skin_temp + r_clo * op_temp) / (r_air + r_clo);
            let converged = (clo_surf_temp_new - clo_surf_temp).abs() <= 0.01;
            clo_surf_temp = clo_surf_temp_new;
            if converged {
                break;
            }
        }
        h = hr + hc;
        dry_heat_loss = (skin_temp - op_temp) / (r_air + r_clo);
        let lat_resp_heat_loss = 0.0023 * act_level * (44.0 - vap_press);
        let dry_resp_heat_loss = 0.0014 * act_level * (34.0 - air_temp);
        let resp_heat_loss = lat_resp_heat_loss + dry_resp_heat_loss;
        let heat_flow = (core_temp - skin_temp) * (5.28 + 1.163 * skin_blood_flow);
        let core_heat_storage = act_level - heat_flow - resp_heat_loss - work_eff;
        let skin_heat_storage = heat_flow - dry_heat_loss - evap_heat_loss;
        let core_therm_cap = 0.97 * (1.0 - skin_mass_rat) * BODY_WEIGHT;
        let skin_therm_cap = 0.97 * skin_mass_rat * BODY_WEIGHT;
        let core_temp_change = core_heat_storage * BODY_SURF_AREA_PIERCE / (core_therm_cap * 60.0);
        let skin_temp_change =
            (skin_heat_storage * BODY_SURF_AREA_PIERCE) / (skin_therm_cap * 60.0);
        core_temp += core_temp_change;
        skin_temp += skin_temp_change;
        let avg_body_temp = skin_mass_rat * skin_temp + (1.0 - skin_mass_rat) * core_temp;

        let skin_signal = skin_temp - SKIN_TEMP_SET;
        let (skin_therm_sig_warm, skin_therm_sig_cold) = if skin_signal > 0.0 {
            (skin_signal, 0.0)
        } else {
            (0.0, -skin_signal)
        };
        let core_signal = core_temp - CORE_TEMP_SET;
        let (core_therm_sig_warm, core_therm_sig_cold) = if core_signal > 0.0 {
            (core_signal, 0.0)
        } else {
            (0.0, -core_signal)
        };
        let body_signal = avg_body_temp - avg_body_temp_set;
        let body_therm_sig_warm = if body_signal > 0.0 { body_signal } else { 0.0 };

        let vasodilation_fac = DRI_COEFF_VASODILATION * core_therm_sig_warm;
        let vasoconstrict_fac = DRI_COEFF_VASOCONSTRICTION * skin_therm_sig_cold;
        skin_blood_flow = (SKIN_BLOOD_FLOW_SET + vasodilation_fac) / (1.0 + vasoconstrict_fac);
        // The C++ two ifs: below the minimum, then above the maximum (NaN stays NaN).
        skin_blood_flow = skin_blood_flow.clamp(MIN_SKIN_BLOOD_FLOW, MAX_SKIN_BLOOD_FLOW);
        skin_mass_rat = 0.0417737 + 0.7451832 / (skin_blood_flow + 0.585417);

        let mut reg_sweat =
            SWEAT_CONT_CONST * body_therm_sig_warm * (skin_therm_sig_warm / 10.7).exp();
        if reg_sweat > REG_SWEAT_MAX {
            reg_sweat = REG_SWEAT_MAX;
        }
        let mut evap_heat_loss_reg_sweat = 0.68 * reg_sweat;
        let shiv_response = 19.4 * skin_therm_sig_cold * core_therm_sig_cold;
        act_level = act_level_start + shiv_response;

        let air_evap_heat_resist = 1.0 / (lewis_ratio * tot_clo_fac * hc);
        let clo_evap_heat_resist = r_clo / (lewis_ratio * clo_insul);
        let tot_evap_heat_resist = air_evap_heat_resist + clo_evap_heat_resist;
        sat_skin_vap_press = CalcSatVapPressFromTempTorr(skin_temp);
        let evap_heat_loss_max = (sat_skin_vap_press - vap_press) / tot_evap_heat_resist;
        let mut skin_wet_sweat = evap_heat_loss_reg_sweat / evap_heat_loss_max;
        let mut skin_wet_diff = (1.0 - skin_wet_sweat) * 0.06;
        let mut evap_heat_loss_diff = skin_wet_diff * evap_heat_loss_max;
        evap_heat_loss = evap_heat_loss_reg_sweat + evap_heat_loss_diff;
        skin_wet_tot = evap_heat_loss / evap_heat_loss_max;
        if skin_wet_tot >= evap_eff {
            skin_wet_tot = evap_eff;
            skin_wet_sweat = (skin_wet_tot - 0.06) / 0.94;
            evap_heat_loss_reg_sweat = skin_wet_sweat * evap_heat_loss_max;
            skin_wet_diff = (1.0 - skin_wet_sweat) * 0.06;
            evap_heat_loss_diff = skin_wet_diff * evap_heat_loss_max;
            evap_heat_loss = evap_heat_loss_reg_sweat + evap_heat_loss_diff;
        }
        if evap_heat_loss_max < 0.0 {
            evap_heat_loss = 0.0;
            skin_wet_tot = evap_eff;
        }
    }

    // Step 3: heat transfer indices in the real environment.
    let effect_skin_heat_loss = dry_heat_loss + evap_heat_loss;
    let clo_body_rat = 1.0 + CLO_FAC * clo_unit;
    let effect_clo_unit = clo_unit - (clo_body_rat - 1.0) / (0.155 * clo_body_rat * h);
    let effect_clo_therm_eff = 1.0 / (1.0 + 0.155 * hc * effect_clo_unit);
    let clo_permeat_eff = 1.0 / (1.0 + (0.155 / clo_insul) * hc * effect_clo_unit);
    let mut et = skin_temp - effect_skin_heat_loss / (h * effect_clo_therm_eff);
    loop {
        let std_vap_press_et = CalcSatVapPressFromTempTorr(et);
        let energy_bal_err_et = effect_skin_heat_loss
            - h * effect_clo_therm_eff * (skin_temp - et)
            - skin_wet_tot
                * lewis_ratio
                * hc
                * clo_permeat_eff
                * (sat_skin_vap_press - std_vap_press_et / 2.0);
        if energy_bal_err_et >= 0.0 {
            break;
        }
        et += 0.1;
    }

    let std_hr = hr;
    let mut std_hc = if act_met <= 0.85 {
        3.0
    } else {
        5.66 * (act_met - 0.85).powf(0.39)
    };
    if std_hc <= 3.0 {
        std_hc = 3.0;
    }
    let std_h = std_hc + std_hr;
    let std_clo_unit = 1.52 / (act_met - work_eff / ACT_LEVEL_CONV + 0.6944) - 0.1835;
    let std_r_clo = 0.155 * std_clo_unit;
    let std_clo_body_rat = 1.0 + CLO_FAC * std_clo_unit;
    let std_effect_clo_therm_eff = 1.0 / (1.0 + 0.155 * std_clo_body_rat * std_h * std_clo_unit);
    let std_clo_insul = clo_insul * std_hc / std_h * (1.0 - std_effect_clo_therm_eff)
        / (std_hc / std_h - clo_insul * std_effect_clo_therm_eff);
    let std_r_evap = 1.0 / (lewis_ratio * std_clo_body_rat * std_hc);
    let std_r_evap_clo = std_r_clo / (lewis_ratio * std_clo_insul);
    let std_h_evap = 1.0 / (std_r_evap + std_r_evap_clo);
    let std_r_air = 1.0 / (std_clo_body_rat * std_h);
    let std_h_dry = 1.0 / (std_r_air + std_r_clo);

    let std_effect_skin_heat_loss = dry_heat_loss + evap_heat_loss;
    let mut old_set = ((skin_temp - std_effect_skin_heat_loss / std_h_dry) * 100.0).round() / 100.0;
    let delta = 0.0001;
    let mut err: f64 = 100.0;
    let bal = |t: f64| {
        let std_vap_press_set = CalcSatVapPressFromTempTorr(t);
        std_effect_skin_heat_loss
            - std_h_dry * (skin_temp - t)
            - skin_wet_tot * std_h_evap * (sat_skin_vap_press - std_vap_press_set / 2.0)
    };
    while err.abs() > 0.01 {
        let energy_bal_err_set_1 = bal(old_set);
        let energy_bal_err_set_2 = bal(old_set + delta);
        let new_set =
            old_set - delta * energy_bal_err_set_1 / (energy_bal_err_set_2 - energy_bal_err_set_1);
        err = new_set - old_set;
        old_set = new_set;
    }
    let set = old_set;
    Set {
        set,
        eff_temp: et,
        skin_temp,
        core_temp,
        skin_wet_tot,
        dry_heat_loss_set: std_h * std_effect_clo_therm_eff * (skin_temp - set),
    }
}

/// Port of `double FangerPMV(int which, double at, double rt, double rh, double vel, double act,
/// double clo, double work)` (`fanger.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn FangerPMV(
    which: std::ffi::c_int,
    at: f64,
    rt: f64,
    rh: f64,
    vel: f64,
    act: f64,
    clo: f64,
    work: f64,
) -> f64 {
    if !(0..=3).contains(&which) {
        return f64::NAN;
    }
    let p = calc_fanger_pmv(at, rt, rh, vel, act, clo, work);
    match which {
        0 => p.pmv,
        1 => p.clo_surf_temp,
        2 => f64::from(p.iter_num),
        _ => p.hc,
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double StandardEffectiveTemp(int which, double outbaro, double at, double rt, double
/// rh, double vel, double act, double clo, double work)` (`fanger.c:16`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn StandardEffectiveTemp(
    which: std::ffi::c_int,
    outbaro: f64,
    at: f64,
    rt: f64,
    rh: f64,
    vel: f64,
    act: f64,
    clo: f64,
    work: f64,
) -> f64 {
    let ok = (0..=5).contains(&which)
        && within(outbaro, 60000.0, 110000.0)
        && within(at, -40.0, 60.0)
        && within(rt, -40.0, 60.0)
        && within(rh, 0.0, 1.0)
        && within(vel, 0.0, 5.0)
        && within(act, 0.7, 4.0)
        && within(clo, 0.0, 3.0)
        && within(work, 0.0, act * 58.2 / 2.0);
    if !ok {
        return f64::NAN;
    }
    let person = Person {
        air_temp: at,
        rad_temp: rt,
        rel_hum: rh,
        air_vel: vel,
        act_met: act,
        clo_unit: clo,
        work_eff: work,
    };
    let s = calc_standard_effective_temp(outbaro, person);
    match which {
        0 => s.set,
        1 => s.eff_temp,
        2 => s.skin_temp,
        3 => s.core_temp,
        4 => s.skin_wet_tot,
        _ => s.dry_heat_loss_set,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neutral_office_is_near_zero_pmv_and_24_set() {
        // 1.2 met, 0.5 clo, still air at 24.5 C and 50 %.
        let pmv = FangerPMV(0, 24.5, 24.5, 0.5, 0.1, 1.2 * 58.2, 0.5, 0.0);
        assert!(pmv.abs() < 0.5, "{pmv}");
        assert!(FangerPMV(2, 24.5, 24.5, 0.5, 0.1, 1.2 * 58.2, 0.5, 0.0) >= 1.0);
        let set = StandardEffectiveTemp(0, 101325.0, 24.5, 24.5, 0.5, 0.1, 1.2, 0.5, 0.0);
        assert!((set - 24.5).abs() < 2.0, "{set}");
    }

    #[test]
    fn warmer_is_higher_and_guards() {
        let cool = FangerPMV(0, 20.0, 20.0, 0.5, 0.1, 70.0, 0.5, 0.0);
        let warm = FangerPMV(0, 28.0, 28.0, 0.5, 0.1, 70.0, 0.5, 0.0);
        assert!(warm > cool);
        assert!(FangerPMV(4, 20.0, 20.0, 0.5, 0.1, 70.0, 0.5, 0.0).is_nan());
        assert!(StandardEffectiveTemp(0, 101325.0, 24.0, 24.0, 1.5, 0.1, 1.2, 0.5, 0.0).is_nan());
        assert!(
            StandardEffectiveTemp(0, 101325.0, f64::NAN, 24.0, 0.5, 0.1, 1.2, 0.5, 0.0).is_nan()
        );
    }
}
