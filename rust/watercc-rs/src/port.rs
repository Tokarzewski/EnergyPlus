//! The Rust port of `watercc.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `WaterCoils::CoolingCoil` with the simple analysis, the Coil:Cooling:Water model: a dry coil
//! (`CoilCompletelyDry`) when the entering air's dew point is at or below the water temperature,
//! otherwise a completely wet one (`CoilCompletelyWet`, an enthalpy-based counterflow exchanger
//! with `WetCoilOutletCondition`), both through `CoilOutletStreamCondition`'s effectiveness-NTU
//! relations for counter or cross flow. The water specific heat comes from `halfloop-rs`'s cached
//! lookup, the air properties from `psychrometrics-rs`. Every expression keeps the C++ order, and
//! `min` and `max` are ObjexxFCL's. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use general_rs::port::SafeDivide;
use halfloop_rs::port::water_cp;
use psychrometrics_rs::port::{
    PsyCpAirFnW, PsyHFnTdbW, PsyTdbFnHW, PsyTdpFnWPb, PsyTsatFnHPb, PsyWFnTdbH, PsyWFnTdpPb,
};
use std::ffi::c_int;

/// `WaterCoils::MinAirMassFlow` (kg/s).
pub const MIN_AIR_MASS_FLOW: f64 = 0.001;
/// `DataWaterCoils::CounterFlow`.
const COUNTER_FLOW: c_int = 1;

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `v` is finite and within `lo` to `hi`.
pub fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// The coil's inlet state and operating mode, as the routines read them from the coil object.
pub struct Coil {
    pub hex: c_int,
    pub mdot_air: f64,
    pub w_in: f64,
    pub mdot_water: f64,
    pub mdot_water_max: f64,
    pub cycling: bool,
    pub plr: f64,
    pub pb: f64,
}

impl Coil {
    /// The air and water mass flows, raised to the on-cycle ones for a cycling fan.
    fn flows(&self) -> (f64, f64) {
        if self.cycling {
            if self.plr > 0.0 {
                (
                    self.mdot_air / self.plr,
                    min(self.mdot_water / self.plr, self.mdot_water_max),
                )
            } else {
                (0.0, 0.0)
            }
        } else {
            (self.mdot_air, self.mdot_water)
        }
    }
}

/// `CoilOutletStreamCondition`: the outlet states of the two streams.
fn coil_outlet_stream_condition(
    hex: c_int,
    cap1: f64,
    in1: f64,
    cap2: f64,
    in2: f64,
    coil_ua: f64,
) -> (f64, f64) {
    const LARGE_NO: f64 = 1.0e10;
    const SMALL_NO: f64 = 1.0e-15;
    let min_cap = min(cap1, cap2);
    let max_cap = max(cap1, cap2);
    let ratio = if max_cap.abs() <= 1.0e-6 {
        1.0
    } else {
        min_cap / max_cap
    };
    let ntu = if min_cap.abs() <= 1.0e-6 {
        LARGE_NO
    } else {
        coil_ua / min_cap
    };
    let mut effectiveness;
    if ntu <= 0.0 {
        effectiveness = 0.0;
    } else if ratio < SMALL_NO {
        effectiveness = if ntu > 20.0 { 1.0 } else { 1.0 - (-ntu).exp() };
    } else if hex == COUNTER_FLOW {
        if (ratio - 1.0).abs() < SMALL_NO {
            effectiveness = ntu / (ntu + 1.0);
        } else {
            let e = if ntu * (1.0 - ratio) > 20.0 {
                0.0
            } else {
                (-ntu * (1.0 - ratio)).exp()
            };
            effectiveness = (1.0 - e) / (1.0 - ratio * e);
        }
    } else {
        let eta = ntu.powf(-0.22);
        if (ntu * ratio * eta) > 20.0 {
            let b = 1.0 / (ratio * eta);
            if b > 20.0 {
                effectiveness = 1.0;
            } else {
                effectiveness = 1.0 - (-b).exp();
                if effectiveness < 0.0 {
                    effectiveness = 0.0;
                }
            }
        } else {
            let d = ((-ntu * ratio * eta).exp() - 1.0) / (ratio * eta);
            // Not `contains`: a NaN `d` takes the C++ else branch.
            if d < -20.0 || d > 0.0 {
                effectiveness = 1.0;
            } else {
                effectiveness = 1.0 - (((-ntu * ratio * eta).exp() - 1.0) / (ratio * eta)).exp();
                if effectiveness < 0.0 {
                    effectiveness = 0.0;
                }
            }
        }
    }
    let max_heat_transfer = max(min_cap, SMALL_NO) * (in1 - in2);
    (
        in1 - effectiveness * max_heat_transfer / max(cap1, SMALL_NO),
        in2 + effectiveness * max_heat_transfer / max(cap2, SMALL_NO),
    )
}

/// `CoilCompletelyDry`: outlet water and air temperatures, outlet humidity ratio and heat rate.
pub fn coil_completely_dry(
    c: &Coil,
    water_temp_in: f64,
    air_temp_in: f64,
    coil_ua: f64,
) -> [f64; 4] {
    let (air_mass_flow, water_mass_flow) = c.flows();
    let capacitance_air = air_mass_flow * PsyCpAirFnW(c.w_in);
    let cp = water_cp(water_temp_in);
    let capacitance_water = water_mass_flow * cp;
    let (outlet_water_temp, outlet_air_temp) = coil_outlet_stream_condition(
        c.hex,
        capacitance_water,
        water_temp_in,
        capacitance_air,
        air_temp_in,
        coil_ua,
    );
    let q = capacitance_air * (air_temp_in - outlet_air_temp);
    [outlet_water_temp, outlet_air_temp, c.w_in, q]
}

/// `WetCoilOutletCondition`: outlet air temperature and humidity ratio and the sensible rate.
fn wet_coil_outlet_condition(
    c: &Coil,
    air_temp_in: f64,
    enth_air_inlet: f64,
    enth_air_outlet: f64,
    ua_coil_external: f64,
) -> [f64; 3] {
    const SMALL_NO: f64 = 1.0e-9;
    let capacitance_air = c.mdot_air * PsyCpAirFnW(c.w_in);
    let mut effectiveness = if ua_coil_external > 0.0 {
        let ntu = if capacitance_air > 0.0 {
            ua_coil_external / capacitance_air
        } else {
            0.0
        };
        1.0 - (-ntu).exp()
    } else {
        0.0
    };
    effectiveness = max(effectiveness, SMALL_NO);
    let enth_air_condensate_temp =
        enth_air_inlet - (enth_air_inlet - enth_air_outlet) / effectiveness;
    let temp_condensation = PsyTsatFnHPb(enth_air_condensate_temp, c.pb);
    let temp_air_dew_point = PsyTdpFnWPb(c.w_in, c.pb);
    let (outlet_air_temp, outlet_air_hum_rat) = if (temp_air_dew_point - temp_condensation) > 0.1 {
        let t = air_temp_in - (air_temp_in - temp_condensation) * effectiveness;
        (t, PsyWFnTdbH(t, enth_air_outlet))
    } else {
        (PsyTdbFnHW(enth_air_outlet, c.w_in), c.w_in)
    };
    [
        outlet_air_temp,
        outlet_air_hum_rat,
        capacitance_air * (air_temp_in - outlet_air_temp),
    ]
}

/// `CoilCompletelyWet`: outlet water and air temperatures, outlet humidity ratio, total and
/// sensible rates, wet fraction and the surface temperature at the air inlet.
pub fn coil_completely_wet(
    c: &Coil,
    water_temp_in: f64,
    air_temp_in: f64,
    air_hum_rat: f64,
    [ua_internal_total, ua_external_total]: [f64; 2],
) -> [f64; 7] {
    const SMALL_NO: f64 = 1.0e-9;
    let surf_area_wet_fraction = 1.0;
    let air_side_resist = 1.0 / max(ua_external_total, SMALL_NO);
    let water_side_resist = 1.0 / max(ua_internal_total, SMALL_NO);
    let (air_mass_flow, water_mass_flow) = c.flows();
    let enth_air_inlet = PsyHFnTdbW(air_temp_in, air_hum_rat);
    let enth_sat_air_inlet_water_temp = PsyHFnTdbW(water_temp_in, PsyWFnTdpPb(water_temp_in, c.pb));
    let entering_air_dew_pt = PsyTdpFnWPb(air_hum_rat, c.pb);
    let intermediate_cp_sat =
        (PsyHFnTdbW(entering_air_dew_pt, PsyWFnTdpPb(entering_air_dew_pt, c.pb))
            - enth_sat_air_inlet_water_temp)
            / (entering_air_dew_pt - water_temp_in);
    let ua_coil_total_enth =
        1.0 / (intermediate_cp_sat * water_side_resist + air_side_resist * PsyCpAirFnW(0.0));
    let capacity_rate_air_wet = air_mass_flow;
    let cp = water_cp(water_temp_in);
    let capacity_rate_water_wet = water_mass_flow * (cp / intermediate_cp_sat);
    let (enth_air_outlet, enth_sat_air_outlet_water_temp) = coil_outlet_stream_condition(
        c.hex,
        capacity_rate_air_wet,
        enth_air_inlet,
        capacity_rate_water_wet,
        enth_sat_air_inlet_water_temp,
        ua_coil_total_enth,
    );
    let resist_ratio = water_side_resist
        / (water_side_resist + PsyCpAirFnW(0.0) / intermediate_cp_sat * air_side_resist);
    let enth_sat_air_coil_surface_entry_temp = enth_sat_air_outlet_water_temp
        + resist_ratio * (enth_air_inlet - enth_sat_air_outlet_water_temp);
    let air_inlet_coil_surf_temp = PsyTsatFnHPb(enth_sat_air_coil_surface_entry_temp, c.pb);
    let tot = air_mass_flow * (enth_air_inlet - enth_air_outlet);
    let outlet_water_temp = water_temp_in + tot / max(water_mass_flow, SMALL_NO) / cp;
    let [outlet_air_temp, outlet_air_hum_rat, sen] = wet_coil_outlet_condition(
        c,
        air_temp_in,
        enth_air_inlet,
        enth_air_outlet,
        ua_external_total,
    );
    [
        outlet_water_temp,
        outlet_air_temp,
        outlet_air_hum_rat,
        tot,
        sen,
        surf_area_wet_fraction,
        air_inlet_coil_surf_temp,
    ]
}

/// Port of `double WaterCoolCoil(int which, int cycling, int design, int hex, ...)` (`watercc.c:8`):
/// `CoolingCoil` with the simple analysis, see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn WaterCoolCoil(
    which: c_int,
    cycling: c_int,
    design: c_int,
    hex: c_int,
    avail: f64,
    uatot: f64,
    uaint: f64,
    uaext: f64,
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
    if which > 7
        || !flag(cycling)
        || !flag(design)
        || (hex != 1 && hex != 2)
        || !within(avail, 0.0, 1.0)
        || ![uatot, uaint, uaext].iter().all(|&u| within(u, 0.0, 1.0e7))
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
    let c = Coil {
        hex,
        mdot_air: mdota,
        w_in: win,
        mdot_water: mdotw,
        mdot_water_max: mdotwmax,
        cycling: cycling == 1,
        plr,
        pb,
    };
    let air_mass_flow = if c.cycling && plr > 0.0 {
        mdota / plr
    } else {
        mdota
    };
    // The shim zeroes the rates, which a coil that does not run leaves as they were.
    let (t_air_out, w_out, t_water_out, mut tot, mut sen, frac, h_water_out);
    if (avail > 0.0
        && mdotw > 0.0
        && air_mass_flow >= MIN_AIR_MASS_FLOW
        && desvol > 0.0
        && mdotwmax > 0.0)
        || design == 1
    {
        let air_dew_point_temp = PsyTdpFnWPb(win, pb);
        if air_dew_point_temp <= twin {
            [t_water_out, t_air_out, w_out, tot] = coil_completely_dry(&c, twin, tain, uatot);
            sen = tot;
            frac = 0.0;
        } else {
            let wet = coil_completely_wet(&c, twin, tain, win, [uaint, uaext]);
            [t_water_out, t_air_out, w_out, tot, sen, frac] =
                [wet[0], wet[1], wet[2], wet[3], wet[4], wet[5]];
        }
        if c.cycling {
            tot *= plr;
            sen *= plr;
        }
        h_water_out = hwin + SafeDivide(tot, mdotw);
    } else {
        (t_water_out, t_air_out, w_out, h_water_out, frac) = (twin, tain, win, hwin, 0.0);
        (tot, sen) = (0.0, 0.0);
    }
    [
        t_air_out,
        w_out,
        t_water_out,
        tot,
        sen,
        frac,
        h_water_out,
        PsyHFnTdbW(t_air_out, w_out),
    ][which]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARGS: [f64; 14] = [
        1.0, 5000.0, 12000.0, 9000.0, 2.0, 2.4, 26.0, 0.011, 2.0, 3.0, 7.0, 29330.0, 1.0, 101325.0,
    ];

    fn coil(which: c_int, args: [f64; 14]) -> f64 {
        let a = args;
        WaterCoolCoil(
            which, 0, 0, 2, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10],
            a[11], a[12], a[13],
        )
    }

    #[test]
    fn wet_coil_cools_and_dries_the_air() {
        assert!(coil(0, ARGS) < 26.0);
        assert!(coil(1, ARGS) < 0.011);
        assert_eq!(coil(5, ARGS), 1.0);
        assert!(coil(3, ARGS) >= coil(4, ARGS));
        assert!(coil(8, ARGS).is_nan());
    }

    #[test]
    fn dry_air_leaves_a_dry_coil() {
        let mut a = ARGS;
        a[7] = 0.004;
        assert_eq!(coil(1, a), 0.004);
        assert_eq!(coil(5, a), 0.0);
        assert_eq!(coil(3, a), coil(4, a));
    }

    #[test]
    fn no_water_flow_passes_the_inlets_through() {
        let mut a = ARGS;
        a[8] = 0.0;
        assert_eq!(coil(0, a), 26.0);
        assert_eq!(coil(2, a), 7.0);
        assert_eq!(coil(3, a), 0.0);
    }
}
