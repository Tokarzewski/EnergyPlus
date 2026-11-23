//! The Rust port of `infil.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The infiltration part of `ZoneEquipmentManager::CalcAirFlowSimple` for one zone with up to two
//! infiltration objects: the design flow rate, Sherman-Grimsrud (effective leakage area) and AIM-2
//! (flow coefficient) models, the outdoor, standard and indoor density bases, EMS overrides, and
//! the zone's infiltration sums. The psychrometric functions come from `psychrometrics-rs`. Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::{PsyCpAirFnW, PsyRhoAirFnPbTdbW};

/// ObjexxFCL `pow_2`.
fn pow_2(x: f64) -> f64 {
    x * x
}

/// `DataHeatBalance::InfiltrationModelType`.
#[derive(Clone, Copy)]
enum Model {
    DesignFlowRate,
    ShermanGrimsrud,
    Aim2,
}

/// `DataHeatBalance::InfVentDensityBasis`.
#[derive(Clone, Copy)]
enum Density {
    Outdoor,
    Standard,
    Indoor,
}

/// `DataHeatBalance::InfiltrationData`: the inputs, with the schedule's current value.
struct Infiltration {
    model: Model,
    density_basis: Density,
    schedule_frac: f64,
    design_level: f64,
    constant_term_coef: f64,
    temperature_term_coef: f64,
    velocity_term_coef: f64,
    velocity_sq_term_coef: f64,
    leakage_area: f64,
    basic_stack_coefficient: f64,
    basic_wind_coefficient: f64,
    flow_coefficient: f64,
    aim2_stack_coefficient: f64,
    aim2_wind_coefficient: f64,
    pressure_exponent: f64,
    shelter_factor: f64,
    ems_override_on: bool,
    ems_air_flow_rate_value: f64,
}

/// What the calculation sets on an infiltration object.
#[derive(Clone, Copy, Default)]
struct Flows {
    volume_flow_rate: f64,
    mass_flow_rate: f64,
    mcpi_temp: f64,
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
    met_wind_speed: f64,
    out_baro_press: f64,
    out_hum_rat: f64,
    linked_hum_rat: Option<f64>,
    std_rho_air: f64,
}

/// One infiltration object's flows, `None` on a negative air density (a fatal error).
fn infiltration_flows(
    inf: &Infiltration,
    c: &Conditions,
    mixing_mat: f64,
    mixing_hum_rat: f64,
) -> Option<Flows> {
    let temp_int = mixing_mat;
    let temp_ext = c.out_dry_bulb_temp;
    let mut wind_speed_ext = c.wind_speed;
    let hum_rat_ext = c.linked_hum_rat.unwrap_or(c.out_hum_rat);

    let air_density = match inf.density_basis {
        Density::Standard => c.std_rho_air,
        Density::Indoor => PsyRhoAirFnPbTdbW(c.out_baro_press, temp_int, mixing_hum_rat),
        Density::Outdoor => PsyRhoAirFnPbTdbW(c.out_baro_press, temp_ext, hum_rat_ext),
    };
    if !matches!(inf.density_basis, Density::Standard) && air_density < 0.0 {
        return None;
    }

    let cp_air = PsyCpAirFnW(hum_rat_ext);
    let mut mcpi_temp;
    let mut f = Flows::default();
    let schedule_frac = inf.schedule_frac;
    if schedule_frac > 0.0 {
        match inf.model {
            Model::DesignFlowRate => {
                let mut ivf = inf.design_level * schedule_frac;
                if ivf < 0.0 {
                    ivf = 0.0;
                }
                mcpi_temp = ivf
                    * air_density
                    * cp_air
                    * (inf.constant_term_coef
                        + (temp_ext - temp_int).abs() * inf.temperature_term_coef
                        + wind_speed_ext
                            * (inf.velocity_term_coef
                                + wind_speed_ext * inf.velocity_sq_term_coef));
                if mcpi_temp < 0.0 {
                    mcpi_temp = 0.0;
                }
                f.volume_flow_rate = mcpi_temp / air_density / cp_air;
            }
            Model::ShermanGrimsrud => {
                wind_speed_ext = c.met_wind_speed;
                let mut ivf = schedule_frac * inf.leakage_area / 1000.0
                    * (inf.basic_stack_coefficient * (temp_ext - temp_int).abs()
                        + inf.basic_wind_coefficient * pow_2(wind_speed_ext))
                    .sqrt();
                if ivf < 0.0 {
                    ivf = 0.0;
                }
                mcpi_temp = ivf * air_density * cp_air;
                if mcpi_temp < 0.0 {
                    mcpi_temp = 0.0;
                }
                f.volume_flow_rate = mcpi_temp / air_density / cp_air;
            }
            Model::Aim2 => {
                let mut ivf = schedule_frac
                    * (pow_2(
                        inf.flow_coefficient
                            * inf.aim2_stack_coefficient
                            * (temp_ext - temp_int).abs().powf(inf.pressure_exponent),
                    ) + pow_2(
                        inf.flow_coefficient
                            * inf.aim2_wind_coefficient
                            * (inf.shelter_factor * wind_speed_ext)
                                .powf(2.0 * inf.pressure_exponent),
                    ))
                    .sqrt();
                if ivf < 0.0 {
                    ivf = 0.0;
                }
                mcpi_temp = ivf * air_density * cp_air;
                if mcpi_temp < 0.0 {
                    mcpi_temp = 0.0;
                }
                f.volume_flow_rate = mcpi_temp / air_density / cp_air;
            }
        }
    } else {
        f.volume_flow_rate = 0.0;
        mcpi_temp = 0.0;
    }

    f.mass_flow_rate = f.volume_flow_rate * air_density;

    if inf.ems_override_on {
        let mut ivf = inf.ems_air_flow_rate_value;
        if ivf < 0.0 {
            ivf = 0.0;
        }
        mcpi_temp = ivf * air_density * cp_air;
        if mcpi_temp < 0.0 {
            mcpi_temp = 0.0;
        }
    }
    f.mcpi_temp = mcpi_temp;
    Some(f)
}

/// The zone's MCPI, OAMFL and MCPTI and each object's flows, `None` on a fatal error.
fn calc_air_flow_simple(infs: &[Infiltration], c: &Conditions) -> Option<([f64; 3], [Flows; 2])> {
    let (mut mcpi, mut oamfl, mut mcpti) = (0.0, 0.0, 0.0);
    let (mut mixing_mat, mut mixing_hum_rat) = (c.mat, c.air_hum_rat);
    if c.sys_timestep_loop == 1 {
        mixing_mat = c.xmpt;
        mixing_hum_rat = c.w_time_minus_p;
    }
    let mut flows = [Flows::default(); 2];
    for (inf, out) in infs.iter().zip(flows.iter_mut()) {
        let f = infiltration_flows(inf, c, mixing_mat, mixing_hum_rat)?;
        let cp_air = PsyCpAirFnW(c.linked_hum_rat.unwrap_or(c.out_hum_rat));
        *out = f;
        mcpi += f.mcpi_temp;
        oamfl += f.mcpi_temp / cp_air;
        mcpti += f.mcpi_temp * c.out_dry_bulb_temp;
    }
    // The thermal chimney terms, reset to zero above and with no thermal chimney.
    mcpi += 0.0;
    oamfl += 0.0;
    mcpti += 0.0;
    Some(([mcpi, oamfl, mcpti], flows))
}

/// An enum index of 0 to 2, `None` for anything else.
fn enum3(v: i32) -> Option<usize> {
    usize::try_from(v).ok().filter(|&n| n <= 2)
}

/// A 0 or 1 flag as a bool, `None` for anything else.
fn flag(v: i32) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// An infiltration object from its arguments, `None` when a guard fails.
fn infiltration(
    model: i32,
    density: i32,
    [frac, dl, c0, ct, cv, cvv, la, cs, cw, fc, a_s, aw, n, sf]: [f64; 14],
    ems: i32,
    emsv: f64,
) -> Option<Infiltration> {
    Some(Infiltration {
        model: [Model::DesignFlowRate, Model::ShermanGrimsrud, Model::Aim2][enum3(model)?],
        density_basis: [Density::Outdoor, Density::Standard, Density::Indoor][enum3(density)?],
        schedule_frac: frac,
        design_level: dl,
        constant_term_coef: c0,
        temperature_term_coef: ct,
        velocity_term_coef: cv,
        velocity_sq_term_coef: cvv,
        leakage_area: la,
        basic_stack_coefficient: cs,
        basic_wind_coefficient: cw,
        flow_coefficient: fc,
        aim2_stack_coefficient: a_s,
        aim2_wind_coefficient: aw,
        pressure_exponent: n,
        shelter_factor: sf,
        ems_override_on: flag(ems)?,
        ems_air_flow_rate_value: emsv,
    })
}

/// Port of `double Infiltration(...)` (`infil.c:8`): see the oracle's shim for the arguments and
/// what `which` picks.
pub fn Infiltration(
    which: std::ffi::c_int,
    ninf: std::ffi::c_int,
    mod1: std::ffi::c_int,
    den1: std::ffi::c_int,
    frac1: f64,
    dl1: f64,
    c01: f64,
    ct1: f64,
    cv1: f64,
    cvv1: f64,
    la1: f64,
    cs1: f64,
    cw1: f64,
    fc1: f64,
    as1: f64,
    aw1: f64,
    n1: f64,
    sf1: f64,
    ems1: std::ffi::c_int,
    emsv1: f64,
    mod2: std::ffi::c_int,
    den2: std::ffi::c_int,
    frac2: f64,
    dl2: f64,
    c02: f64,
    ct2: f64,
    cv2: f64,
    cvv2: f64,
    la2: f64,
    cs2: f64,
    cw2: f64,
    fc2: f64,
    as2: f64,
    aw2: f64,
    n2: f64,
    sf2: f64,
    ems2: std::ffi::c_int,
    emsv2: f64,
    systs: std::ffi::c_int,
    mat: f64,
    hum: f64,
    xmpt: f64,
    wprev: f64,
    outdb: f64,
    wind: f64,
    metwind: f64,
    outbaro: f64,
    outhum: f64,
    linked: std::ffi::c_int,
    linkedhum: f64,
    stdrho: f64,
) -> f64 {
    let checked = (|| {
        let i1 = infiltration(
            mod1,
            den1,
            [
                frac1, dl1, c01, ct1, cv1, cvv1, la1, cs1, cw1, fc1, as1, aw1, n1, sf1,
            ],
            ems1,
            emsv1,
        )?;
        let i2 = infiltration(
            mod2,
            den2,
            [
                frac2, dl2, c02, ct2, cv2, cvv2, la2, cs2, cw2, fc2, as2, aw2, n2, sf2,
            ],
            ems2,
            emsv2,
        )?;
        let linked = flag(linked)?;
        let n = enum3(ninf)?;
        (0..=8).contains(&which).then_some((i1, i2, linked, n))
    })();
    let Some((i1, i2, linked, n)) = checked else {
        return f64::NAN;
    };
    let c = Conditions {
        sys_timestep_loop: systs,
        mat,
        air_hum_rat: hum,
        xmpt,
        w_time_minus_p: wprev,
        out_dry_bulb_temp: outdb,
        wind_speed: wind,
        met_wind_speed: metwind,
        out_baro_press: outbaro,
        out_hum_rat: outhum,
        linked_hum_rat: linked.then_some(linkedhum),
        std_rho_air: stdrho,
    };
    let infs = [i1, i2];
    let Some((zone, flows)) = calc_air_flow_simple(&infs[..n], &c) else {
        return f64::NAN;
    };
    match which {
        0..=2 => zone[which as usize],
        3 => flows[0].volume_flow_rate,
        4 => flows[0].mass_flow_rate,
        5 => flows[0].mcpi_temp,
        6 => flows[1].volume_flow_rate,
        7 => flows[1].mass_flow_rate,
        _ => flows[1].mcpi_temp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One design flow rate object (0.05 m3/s, constant coefficient only) at standard density
    /// 1.2, later system time step, outdoor humidity ratio 0.005.
    fn design(which: i32, frac: f64, ems: i32, den: i32) -> f64 {
        Infiltration(
            which, 1, 0, den, frac, 0.05, 1.0, 0.0, 0.0, 0.0, 500.0, 0.000145, 0.000174, 0.05,
            0.069, 0.128, 0.67, 1.0, ems, 0.1, 0, 0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 2, 21.0, 0.008, 20.0, 0.007, -5.0, 3.0, 4.0, 101325.0,
            0.005, 0, 0.0, 1.2,
        )
    }

    #[test]
    fn design_flow_rate() {
        let cp = PsyCpAirFnW(0.005);
        assert_eq!(design(5, 1.0, 0, 1), 0.05 * 1.2 * cp * 1.0);
        assert_eq!(design(0, 1.0, 0, 1), 0.05 * 1.2 * cp * 1.0);
        assert_eq!(design(2, 1.0, 0, 1), 0.05 * 1.2 * cp * 1.0 * -5.0);
        assert_eq!(design(4, 1.0, 0, 1), design(3, 1.0, 0, 1) * 1.2);
    }

    #[test]
    fn schedule_off_and_ems() {
        assert_eq!(design(0, 0.0, 0, 1), 0.0);
        let cp = PsyCpAirFnW(0.005);
        assert_eq!(design(5, 0.0, 1, 1), 0.1 * 1.2 * cp);
        assert_eq!(design(3, 0.0, 1, 1), 0.0);
    }

    #[test]
    fn no_second_object_and_outdoor_density() {
        assert_eq!(design(8, 1.0, 0, 1), 0.0);
        let rho = PsyRhoAirFnPbTdbW(101325.0, -5.0, 0.005);
        assert_eq!(design(4, 1.0, 0, 0), design(3, 1.0, 0, 0) * rho);
    }

    #[test]
    fn guards_return_nan() {
        assert!(design(9, 1.0, 0, 1).is_nan());
        assert!(design(0, 1.0, 2, 1).is_nan());
        assert!(design(0, 1.0, 0, 3).is_nan());
    }
}
