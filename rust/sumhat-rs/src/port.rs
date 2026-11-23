//! The Rust port of `sumhat.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneTempPredictorCorrector::SpaceHeatBalanceData::calcSumHAT` for one heat transfer surface:
//! its convective heat gains to the zone air from a window (divider behind an interior shade or
//! blind, equivalent layer gain, shade gap convection, airflow window gains, with the airflow
//! window's return gain booked into the window heat gain reports), its frame and divider
//! convection, and its surface convection, added to the sum its reference air temperature asks
//! for. Every sum starts at 0 and adds in the C++ order. The oracle's guards (see its shim)
//! return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

/// `WinShadingType::IntShade` and `IntBlind`, the shading flags `ANY_INTERIOR_SHADE_BLIND` takes.
const INT_SHADE: i32 = 2;
const INT_BLIND: i32 = 6;

/// `SumHATOutput`.
#[derive(Default)]
struct SumHat {
    sum_int_gain: f64,
    sum_ha: f64,
    sum_hat_surf: f64,
    sum_hat_ref: f64,
}

/// The window heat gain and its report variables after the call.
#[derive(Default)]
struct WinReports {
    heat_gain: f64,
    gain_rep: f64,
    gain_rep_energy: f64,
    loss_rep: f64,
    loss_rep_energy: f64,
    transfer_rep_energy: f64,
}

/// A window's terms (`SurfWin...` arrays).
struct Window {
    shading: i32,
    eql: bool,
    divider_area: f64,
    divider_gain: f64,
    proj_div: f64,
    divider_temp: f64,
    frame_area: f64,
    proj_fr: f64,
    frame_temp: f64,
    other_conv: f64,
    nat_conv: f64,
    airflow: f64,
    conv_gain: f64,
    ret_gain: f64,
}

/// One surface and the zone terms `calcSumHAT` reads.
struct Surface {
    window: Option<Window>,
    area: f64,
    hconv: f64,
    tin: f64,
    tair_ref: i32,
    temp_eff: f64,
    no_heat_to_return_air: bool,
    sum_sys_mcp: f64,
    sum_sys_mcpt: f64,
    time_step_zone_sec: f64,
}

fn calc_sum_hat(s: &Surface, rep: &mut WinReports) -> SumHat {
    let mut results = SumHat::default();
    let mut ha = 0.0;
    let mut area = s.area;
    if let Some(w) = &s.window {
        let interior = w.shading == INT_SHADE || w.shading == INT_BLIND;
        if interior {
            area += w.divider_area;
            results.sum_int_gain += w.divider_gain;
        }
        if w.eql {
            results.sum_int_gain += w.other_conv;
        }
        if interior {
            results.sum_int_gain += w.nat_conv;
        }
        if w.airflow > 0.0 {
            results.sum_int_gain += w.conv_gain;
            if s.no_heat_to_return_air {
                results.sum_int_gain += w.ret_gain;
                rep.heat_gain += w.ret_gain;
                if rep.heat_gain >= 0.0 {
                    rep.gain_rep = rep.heat_gain;
                    rep.gain_rep_energy = rep.gain_rep * s.time_step_zone_sec;
                } else {
                    rep.loss_rep = -rep.heat_gain;
                    rep.loss_rep_energy = rep.loss_rep * s.time_step_zone_sec;
                }
                rep.transfer_rep_energy = rep.heat_gain * s.time_step_zone_sec;
            }
        }
        if w.frame_area > 0.0 {
            let ha_surf = s.hconv * w.frame_area * (1.0 + w.proj_fr);
            results.sum_hat_surf += ha_surf * w.frame_temp;
            ha += ha_surf;
        }
        if w.divider_area > 0.0 && !interior {
            let ha_surf = s.hconv * w.divider_area * (1.0 + 2.0 * w.proj_div);
            results.sum_hat_surf += ha_surf * w.divider_temp;
            ha += ha_surf;
        }
    }
    ha += s.hconv * area;
    results.sum_hat_surf += s.hconv * area * s.tin;
    match s.tair_ref {
        1 => results.sum_hat_ref += ha * s.temp_eff,
        2 if s.sum_sys_mcp > 0.0 => results.sum_hat_ref += ha * s.sum_sys_mcpt / s.sum_sys_mcp,
        // The zone mean air, the supply air before the system has flow, and an unset reference.
        _ => results.sum_ha += ha,
    }
    results
}

/// One result of `calcSumHAT` for a space of one surface (see the oracle's shim for the arguments
/// and `which`).
pub fn SumHAT(
    which: i32,
    window: i32,
    shading: i32,
    eql: i32,
    area: f64,
    hconv: f64,
    tin: f64,
    tairref: i32,
    tempeff: f64,
    dividerarea: f64,
    dividergain: f64,
    projdiv: f64,
    dividertemp: f64,
    framearea: f64,
    projfr: f64,
    frametemp: f64,
    otherconv: f64,
    natconv: f64,
    airflow: f64,
    convgain: f64,
    retgain: f64,
    noheatret: i32,
    heatgain0: f64,
    controlled: i32,
    sysmcp: f64,
    sysmcpt: f64,
    tszsec: f64,
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    if !(0..=9).contains(&which)
        || ![window, eql, noheatret, controlled].into_iter().all(flag)
        || !(-1..=13).contains(&shading)
        || !(-1..=2).contains(&tairref)
        || (tairref == 2 && controlled == 0)
    {
        return f64::NAN;
    }
    let s = Surface {
        window: (window == 1).then_some(Window {
            shading,
            eql: eql == 1,
            divider_area: dividerarea,
            divider_gain: dividergain,
            proj_div: projdiv,
            divider_temp: dividertemp,
            frame_area: framearea,
            proj_fr: projfr,
            frame_temp: frametemp,
            other_conv: otherconv,
            nat_conv: natconv,
            airflow,
            conv_gain: convgain,
            ret_gain: retgain,
        }),
        area,
        hconv,
        tin,
        tair_ref: tairref,
        temp_eff: tempeff,
        no_heat_to_return_air: noheatret == 1,
        sum_sys_mcp: sysmcp,
        sum_sys_mcpt: sysmcpt,
        time_step_zone_sec: tszsec,
    };
    let mut rep = WinReports {
        heat_gain: heatgain0,
        ..WinReports::default()
    };
    let r = calc_sum_hat(&s, &mut rep);
    match which {
        0 => r.sum_int_gain,
        1 => r.sum_ha,
        2 => r.sum_hat_surf,
        3 => r.sum_hat_ref,
        4 => rep.heat_gain,
        5 => rep.gain_rep,
        6 => rep.gain_rep_energy,
        7 => rep.loss_rep,
        8 => rep.loss_rep_energy,
        _ => rep.transfer_rep_energy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 10 m2 wall with h = 2 at 18 C, referred to `tairref`.
    fn wall(which: i32, tairref: i32) -> f64 {
        SumHAT(
            which, 0, 0, 0, 10.0, 2.0, 18.0, tairref, 25.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0, 0.0, 1, 100.0, 2000.0, 900.0,
        )
    }

    #[test]
    fn wall_convection_goes_to_the_reference_sum() {
        assert_eq!(wall(1, 0), 20.0);
        assert_eq!(wall(2, 0), 360.0);
        assert_eq!(wall(3, 1), 500.0);
        // Supply air at 2000 / 100 = 20 C.
        assert_eq!(wall(3, 2), 400.0);
        assert_eq!(wall(1, -1), 20.0);
    }

    #[test]
    fn airflow_window_books_return_gain() {
        // 1 m2 window, airflow to the return in a zone with no heat to return air.
        let w = |which: i32| {
            SumHAT(
                which, 1, 0, 0, 1.0, 2.0, 18.0, 0, 25.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.01, 30.0, 50.0, 1, -80.0, 1, 0.0, 0.0, 900.0,
            )
        };
        assert_eq!(w(0), 80.0);
        assert_eq!(w(4), -30.0);
        assert_eq!(w(7), 30.0);
        assert_eq!(w(9), -27000.0);
    }

    #[test]
    fn guards_return_nan() {
        assert!(wall(10, 0).is_nan());
        assert!(wall(0, 3).is_nan());
        assert!(
            SumHAT(
                0, 0, 0, 0, 10.0, 2.0, 18.0, 2, 25.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0, 0.0, 0, 100.0, 2000.0, 900.0,
            )
            .is_nan()
        );
    }
}
