//! The Rust port of `vent.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The ventilation part of `ZoneEquipmentManager::CalcAirFlowSimple` for one zone with one
//! ventilation object: the design flow rate model with its fan power, fan electricity and intake
//! fan heating, the wind and stack open area model, the indoor, outdoor, delta temperature and
//! wind speed limits, hybrid ventilation closing, EMS overrides and the zone's ventilation sums.
//! The psychrometric functions come from `psychrometrics-rs`. Every expression keeps the C++
//! order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::{PsyCpAirFnW, PsyRhoAirFnPbTdbW, PsyTdbFnHW};

/// `Constant::AutoCalculate`.
const AUTO_CALCULATE: f64 = -99999.0;

/// `DataHeatBalance::VentilationType`.
#[derive(Clone, Copy, PartialEq)]
enum FanType {
    Natural,
    Intake,
    Exhaust,
    Balanced,
}

/// The ventilation object's inputs (schedules as their current values).
struct Ventilation {
    wind_and_stack: bool,
    /// `InfVentDensityBasis`: 0 outdoor, 1 standard, 2 indoor.
    density_basis: i32,
    avail: f64,
    design_level: f64,
    constant_term_coef: f64,
    temperature_term_coef: f64,
    velocity_term_coef: f64,
    velocity_sq_term_coef: f64,
    fan_type: FanType,
    fan_pressure: f64,
    fan_efficiency: f64,
    open_area: f64,
    open_area_frac: f64,
    open_eff: f64,
    disc_coef: f64,
    eff_angle: f64,
    dh: f64,
    min_indoor_temperature: f64,
    max_indoor_temperature: f64,
    min_outdoor_temperature: f64,
    max_outdoor_temperature: f64,
    del_temperature: f64,
    max_wind_speed: f64,
    hybrid_close: bool,
    ems_simple_vent_on: bool,
    ems_simple_vent_flow_rate: f64,
}

/// The zone and outdoor conditions.
struct Conditions {
    sys_timestep_loop: i32,
    mat: f64,
    air_hum_rat: f64,
    xmpt: f64,
    w_time_minus_p: f64,
    out_dry_bulb_temp: f64,
    wind_speed: f64,
    wind_dir: f64,
    out_baro_press: f64,
    out_hum_rat: f64,
    out_enthalpy: f64,
    /// The zone's outdoor air node's humidity ratio and enthalpy.
    linked: Option<(f64, f64)>,
    std_rho_air: f64,
    time_step_sys_sec: f64,
}

/// What the calculation sets.
#[derive(Default)]
struct Results {
    mcpv: f64,
    vamfl: f64,
    mcptv: f64,
    mcp: f64,
    fan_power: f64,
    air_temp: f64,
    ventil_fan_elec: f64,
}

/// The ventilation loop of `CalcAirFlowSimple` for the one object, `None` on a negative air density
/// (a fatal error).
fn calc_ventilation(v: &Ventilation, c: &Conditions) -> Option<Results> {
    let mut r = Results::default();
    let mut mixing_mat = c.mat;
    let mut mixing_hum_rat = c.air_hum_rat;
    if c.sys_timestep_loop == 1 {
        mixing_mat = c.xmpt;
        mixing_hum_rat = c.w_time_minus_p;
    }
    let this_mixing_mat = mixing_mat;
    r.fan_power = 0.0;
    r.mcp = 0.0;

    let temp_ext = c.out_dry_bulb_temp;
    let wind_speed_ext = c.wind_speed;
    let wind_dir_ext = c.wind_dir;
    let (hum_rat_ext, enthalpy_ext) = c.linked.unwrap_or((c.out_hum_rat, c.out_enthalpy));

    let air_density = match v.density_basis {
        1 => c.std_rho_air,
        2 => PsyRhoAirFnPbTdbW(c.out_baro_press, this_mixing_mat, mixing_hum_rat),
        _ => PsyRhoAirFnPbTdbW(c.out_baro_press, temp_ext, hum_rat_ext),
    };
    if v.density_basis != 1 && air_density < 0.0 {
        return None;
    }

    let cp_air = PsyCpAirFnW(hum_rat_ext);

    let hybrid_control_zone_mat = mixing_mat;
    let ems = v.ems_simple_vent_on;
    let skip = (hybrid_control_zone_mat < v.min_indoor_temperature && !ems)
        || (hybrid_control_zone_mat > v.max_indoor_temperature && !ems)
        || ((hybrid_control_zone_mat - temp_ext) < v.del_temperature && !ems)
        || (temp_ext < v.min_outdoor_temperature && !ems)
        || (temp_ext > v.max_outdoor_temperature && !ems)
        || (wind_speed_ext > v.max_wind_speed && !ems)
        || (v.hybrid_close && !ems);
    if skip {
        return Some(r);
    }

    // thisMCPV, thisVAMFL and thisMCPTV, which both models set.
    let (this_mcpv, this_vamfl, this_mcptv) = if !v.wind_and_stack {
        let mut vvf = v.design_level * v.avail;
        if ems {
            vvf = v.ems_simple_vent_flow_rate;
        }
        if vvf < 0.0 {
            vvf = 0.0;
        }
        r.mcp = vvf
            * air_density
            * cp_air
            * (v.constant_term_coef
                + (temp_ext - this_mixing_mat).abs() * v.temperature_term_coef
                + wind_speed_ext
                    * (v.velocity_term_coef + wind_speed_ext * v.velocity_sq_term_coef));
        if r.mcp < 0.0 {
            r.mcp = 0.0;
        }
        let vamfl_temp = r.mcp / cp_air;
        if v.fan_efficiency > 0.0 {
            r.fan_power = vamfl_temp * v.fan_pressure / (v.fan_efficiency * air_density);
            if v.fan_type == FanType::Balanced {
                r.fan_power *= 2.0;
            }
            r.ventil_fan_elec += r.fan_power * c.time_step_sys_sec;
        }
        if v.fan_type == FanType::Intake || v.fan_type == FanType::Balanced {
            let outlet_air_enthalpy = if vamfl_temp == 0.0 {
                enthalpy_ext
            } else if r.fan_power > 0.0 {
                if v.fan_type == FanType::Balanced {
                    enthalpy_ext + r.fan_power / vamfl_temp / 2.0
                } else {
                    enthalpy_ext + r.fan_power / vamfl_temp
                }
            } else {
                enthalpy_ext
            };
            r.air_temp = PsyTdbFnHW(outlet_air_enthalpy, hum_rat_ext);
        } else {
            r.air_temp = temp_ext;
        }
        (r.mcp, vamfl_temp, r.mcp * r.air_temp)
    } else {
        let cw = if v.open_eff != AUTO_CALCULATE {
            v.open_eff
        } else {
            let angle = 180.0 - ((wind_dir_ext - v.eff_angle).abs() - 180.0).abs();
            if angle > 90.0 {
                0.0
            } else {
                const SLOPE: f64 = (0.3 - 0.55) / (45.0 - 0.0);
                const INTERCEPT: f64 = 0.55;
                INTERCEPT + angle * SLOPE
            }
        };
        let cd = if v.disc_coef != AUTO_CALCULATE {
            v.disc_coef
        } else {
            0.40 + 0.0045 * (temp_ext - this_mixing_mat).abs()
        };
        let qw = cw * v.open_area * v.open_area_frac * wind_speed_ext;
        let qst = cd
            * v.open_area
            * v.open_area_frac
            * (2.0 * 9.81 * v.dh * (temp_ext - this_mixing_mat).abs() / (this_mixing_mat + 273.15))
                .sqrt();
        let mut vvf = (qw * qw + qst * qst).sqrt();
        if ems {
            vvf = v.ems_simple_vent_flow_rate;
        }
        if vvf < 0.0 {
            vvf = 0.0;
        }
        r.mcp = vvf * air_density * cp_air;
        if r.mcp < 0.0 {
            r.mcp = 0.0;
        }
        r.air_temp = temp_ext;
        (r.mcp, r.mcp / cp_air, r.mcp * r.air_temp)
    };
    r.mcpv += this_mcpv;
    r.vamfl += this_vamfl;
    r.mcptv += this_mcptv;
    Some(r)
}

/// A 0 or 1 flag as a bool, `None` for anything else.
fn flag(v: i32) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// Port of `double Ventilation(...)` (`vent.c:8`): see the oracle's shim for the arguments and
/// what `which` picks.
pub fn Ventilation(
    which: std::ffi::c_int,
    nvent: std::ffi::c_int,
    r#mod: std::ffi::c_int,
    den: std::ffi::c_int,
    avail: f64,
    dl: f64,
    c0: f64,
    ct: f64,
    cv: f64,
    cvv: f64,
    fantype: std::ffi::c_int,
    fanp: f64,
    fane: f64,
    openarea: f64,
    openfrac: f64,
    openeff: f64,
    disc: f64,
    effangle: f64,
    dh: f64,
    scheds: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    minot: f64,
    maxot: f64,
    delt: f64,
    maxwind: f64,
    hybrid: std::ffi::c_int,
    ems: std::ffi::c_int,
    emsflow: f64,
    systs: std::ffi::c_int,
    mat: f64,
    hum: f64,
    xmpt: f64,
    wprev: f64,
    outdb: f64,
    wind: f64,
    winddir: f64,
    outbaro: f64,
    outhum: f64,
    outenth: f64,
    linked: std::ffi::c_int,
    linkedhum: f64,
    linkedenth: f64,
    stdrho: f64,
    tsyssec: f64,
) -> f64 {
    let checked = (|| {
        let fan_type = [
            FanType::Natural,
            FanType::Intake,
            FanType::Exhaust,
            FanType::Balanced,
        ]
        .get(usize::try_from(fantype).ok()?)
        .copied()?;
        let flags = [
            flag(nvent)?,
            flag(r#mod)?,
            flag(hybrid)?,
            flag(scheds)?,
            flag(ems)?,
            flag(linked)?,
        ];
        ((0..=7).contains(&which) && (0..=2).contains(&den)).then_some((fan_type, flags))
    })();
    let Some((fan_type, [nvent, wind_and_stack, hybrid_close, scheds, ems, linked])) = checked
    else {
        return f64::NAN;
    };
    if scheds && (mint > maxt || minot > maxot) {
        return f64::NAN;
    }
    // Without a ventilation object the minimum indoor temperature keeps the oracle's setting.
    let min_indoor = if scheds && !nvent { 0.0 } else { mint };
    if !nvent {
        return if which == 7 { min_indoor } else { 0.0 };
    }
    let v = Ventilation {
        wind_and_stack,
        density_basis: den,
        avail,
        design_level: dl,
        constant_term_coef: c0,
        temperature_term_coef: ct,
        velocity_term_coef: cv,
        velocity_sq_term_coef: cvv,
        fan_type,
        fan_pressure: fanp,
        fan_efficiency: fane,
        open_area: openarea,
        open_area_frac: openfrac,
        open_eff: openeff,
        disc_coef: disc,
        eff_angle: effangle,
        dh,
        min_indoor_temperature: mint,
        max_indoor_temperature: maxt,
        min_outdoor_temperature: minot,
        max_outdoor_temperature: maxot,
        del_temperature: delt,
        max_wind_speed: maxwind,
        hybrid_close,
        ems_simple_vent_on: ems,
        ems_simple_vent_flow_rate: emsflow,
    };
    let c = Conditions {
        sys_timestep_loop: systs,
        mat,
        air_hum_rat: hum,
        xmpt,
        w_time_minus_p: wprev,
        out_dry_bulb_temp: outdb,
        wind_speed: wind,
        wind_dir: winddir,
        out_baro_press: outbaro,
        out_hum_rat: outhum,
        out_enthalpy: outenth,
        linked: linked.then_some((linkedhum, linkedenth)),
        std_rho_air: stdrho,
        time_step_sys_sec: tsyssec,
    };
    let Some(r) = calc_ventilation(&v, &c) else {
        return f64::NAN;
    };
    match which {
        0 => r.mcpv,
        1 => r.vamfl,
        2 => r.mcptv,
        3 => r.mcp,
        4 => r.fan_power,
        5 => r.air_temp,
        6 => r.ventil_fan_elec,
        _ => min_indoor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A design flow rate exhaust fan (0.5 m3/s, constant coefficient, 125 Pa at 0.5 efficiency)
    /// at standard density 1.2, 22 C indoors and 15 C outdoors, minimum indoor temperature `mint`.
    fn exhaust(which: i32, fantype: i32, mint: f64, ems: i32) -> f64 {
        Ventilation(
            which, 1, 0, 1, 1.0, 0.5, 1.0, 0.0, 0.0, 0.0, fantype, 125.0, 0.5, 1.0, 1.0, -99999.0,
            -99999.0, 0.0, 1.0, 0, mint, 100.0, -100.0, 100.0, -100.0, 40.0, 0, ems, 0.2, 2, 22.0,
            0.008, 22.0, 0.008, 15.0, 3.0, 0.0, 101325.0, 0.006, 30000.0, 0, 0.0, 0.0, 1.2, 600.0,
        )
    }

    #[test]
    fn design_flow_rate_with_fan() {
        let cp = PsyCpAirFnW(0.006);
        assert_eq!(exhaust(3, 2, -100.0, 0), 0.5 * 1.2 * cp * 1.0);
        let vamfl = 0.5 * 1.2 * cp * 1.0 / cp;
        assert_eq!(exhaust(4, 2, -100.0, 0), vamfl * 125.0 / (0.5 * 1.2));
        assert_eq!(
            exhaust(6, 2, -100.0, 0),
            vamfl * 125.0 / (0.5 * 1.2) * 600.0
        );
        assert_eq!(exhaust(5, 2, -100.0, 0), 15.0);
        let vamfl_cp = 0.5 * 1.2 * cp * 1.0 / cp;
        let fan = vamfl_cp * 125.0 / (0.5 * 1.2);
        assert_eq!(
            exhaust(5, 1, -100.0, 0),
            PsyTdbFnHW(30000.0 + fan / vamfl_cp, 0.006)
        );
    }

    #[test]
    fn limits_and_ems() {
        assert_eq!(exhaust(0, 2, 23.0, 0), 0.0);
        let cp = PsyCpAirFnW(0.006);
        assert_eq!(exhaust(3, 2, 23.0, 1), 0.2 * 1.2 * cp * 1.0);
        assert_eq!(exhaust(7, 2, 23.0, 0), 23.0);
    }

    #[test]
    fn wind_and_stack_autocalculated() {
        let t = Ventilation(
            3, 1, 1, 1, 1.0, 0.5, 1.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, 2.0, 0.5, -99999.0, -99999.0,
            0.0, 1.0, 0, -100.0, 100.0, -100.0, 100.0, -100.0, 40.0, 0, 0, 0.0, 2, 22.0, 0.008,
            22.0, 0.008, 15.0, 3.0, 0.0, 101325.0, 0.006, 30000.0, 0, 0.0, 0.0, 1.2, 600.0,
        );
        let qw: f64 = 0.55 * 2.0 * 0.5 * 3.0;
        let qst = (0.40 + 0.0045 * 7.0)
            * 2.0
            * 0.5
            * (2.0 * 9.81 * 1.0 * 7.0 / (22.0 + 273.15_f64)).sqrt();
        assert_eq!(t, (qw * qw + qst * qst).sqrt() * 1.2 * PsyCpAirFnW(0.006));
    }

    #[test]
    fn guards_return_nan() {
        assert!(exhaust(8, 2, -100.0, 0).is_nan());
        assert!(exhaust(0, 4, -100.0, 0).is_nan());
        assert!(exhaust(0, 2, -100.0, 2).is_nan());
    }
}
