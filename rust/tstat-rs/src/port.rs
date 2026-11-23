//! The Rust port of `tstat.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneTempPredictorCorrector::CalcZoneAirTempSetPoints` for one thermostat: the zone set points
//! of each control type (single heating, single cooling, single heating and cooling, dual set
//! point), the operative temperature adjustment (`AdjustAirSetPointsforOpTempCntrl`), the
//! temperature and humidity overcooling (`AdjustCoolingSetPointforTempAndHumidityControl`) and the
//! EMS overrides (`OverrideAirSetPointsforEMSCntrl`). Every expression keeps the C++ order. The
//! oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

/// `HVAC::SetptType`.
const SINGLE_HEAT: i32 = 1;
const SINGLE_COOL: i32 = 2;
const SINGLE_HEAT_COOL: i32 = 3;
const DUAL_HEAT_COOL: i32 = 4;

/// ObjexxFCL `min` for doubles: `a < b ? a : b`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// The thermostat, its schedules' current values and the controls it reads.
struct Thermostat {
    ctype: i32,
    heat_used: bool,
    heat_sp: f64,
    cool_sp: f64,
    heat_cool_sp: f64,
    dual_heat_sp: f64,
    dual_cool_sp: f64,
    /// The radiative fraction and mean radiant temperature when operative temperature control
    /// applies (the model has it and the thermostat uses it).
    op_temp: Option<(f64, f64)>,
    /// The overcool range, control ratio, zone relative humidity and dehumidifying set point when
    /// temperature and humidity control applies.
    overcool: Option<(f64, f64, f64, f64)>,
    ems_heat: Option<f64>,
    ems_cool: Option<f64>,
}

/// The zone's set points and the thermostat's high and low set points.
struct SetPoints {
    setpt: f64,
    hi: f64,
    lo: f64,
    tstat_hi: f64,
    tstat_lo: f64,
}

impl Thermostat {
    /// `AdjustAirSetPointsforOpTempCntrl`.
    fn adjust_op_temp(&self, zone_air_set_point: f64) -> f64 {
        match self.op_temp {
            Some((this_mrt_fraction, this_mrt)) => {
                (zone_air_set_point - this_mrt_fraction * this_mrt) / (1.0 - this_mrt_fraction)
            }
            None => zone_air_set_point,
        }
    }

    /// `AdjustCoolingSetPointforTempAndHumidityControl`.
    fn adjust_cool_for_humidity(&self, sp: &mut SetPoints) {
        let Some((mut zone_overcool_range, zone_overcool_control_ratio, air_rel_hum, dehum)) =
            self.overcool
        else {
            return;
        };
        let max_allowed_overcool_range = sp.hi - sp.lo;
        if max_allowed_overcool_range > 0.0 {
            zone_overcool_range = min(zone_overcool_range, max_allowed_overcool_range);
        }
        let relative_humidity_diff = air_rel_hum - dehum;
        if relative_humidity_diff > 0.0 && zone_overcool_control_ratio > 0.0 {
            zone_overcool_range = min(
                zone_overcool_range,
                relative_humidity_diff / zone_overcool_control_ratio,
            );
            sp.hi -= zone_overcool_range;
        }
    }

    /// `CalcZoneAirTempSetPoints` for this thermostat, then its EMS overrides.
    fn calc_set_points(&self, sp: &mut SetPoints) {
        match self.ctype {
            SINGLE_HEAT => {
                if self.heat_used {
                    sp.setpt = self.heat_sp;
                    sp.tstat_lo = sp.setpt;
                    sp.setpt = self.adjust_op_temp(sp.setpt);
                    sp.lo = sp.setpt;
                }
            }
            SINGLE_COOL => {
                sp.setpt = self.cool_sp;
                sp.tstat_hi = sp.setpt;
                sp.setpt = self.adjust_op_temp(sp.setpt);
                sp.hi = sp.setpt;
                self.adjust_cool_for_humidity(sp);
            }
            SINGLE_HEAT_COOL => {
                sp.setpt = self.adjust_op_temp(self.heat_cool_sp);
                sp.hi = sp.setpt;
                sp.lo = sp.setpt;
            }
            DUAL_HEAT_COOL => {
                sp.hi = self.dual_cool_sp;
                sp.tstat_hi = sp.hi;
                sp.hi = self.adjust_op_temp(sp.hi);
                sp.lo = self.dual_heat_sp;
                sp.tstat_lo = sp.lo;
                sp.lo = self.adjust_op_temp(sp.lo);
                self.adjust_cool_for_humidity(sp);
            }
            // Uncontrolled.
            _ => {}
        }

        if let Some(v) = self.ems_heat {
            match self.ctype {
                SINGLE_HEAT | SINGLE_HEAT_COOL => {
                    sp.setpt = v;
                    sp.lo = v;
                }
                DUAL_HEAT_COOL => sp.lo = v,
                _ => {}
            }
        }
        if let Some(v) = self.ems_cool {
            match self.ctype {
                SINGLE_COOL | SINGLE_HEAT_COOL => {
                    sp.setpt = v;
                    sp.hi = v;
                }
                DUAL_HEAT_COOL => sp.hi = v,
                _ => {}
            }
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

/// Port of `double TstatSetPoints(int which, int ctype, ...)` (`tstat.c:8`): see the oracle's shim
/// for the arguments and what `which` picks.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn TstatSetPoints(
    which: std::ffi::c_int,
    ctype: std::ffi::c_int,
    heatused: std::ffi::c_int,
    heatsp: f64,
    coolsp: f64,
    hcsp: f64,
    dhsp: f64,
    dcsp: f64,
    setpt0: f64,
    hi0: f64,
    lo0: f64,
    anyop: std::ffi::c_int,
    opctrl: std::ffi::c_int,
    frac: f64,
    mrt: f64,
    anyhum: std::ffi::c_int,
    ovctrl: std::ffi::c_int,
    ovrange: f64,
    ovratio: f64,
    relhum: f64,
    dehum: f64,
    emsheat: std::ffi::c_int,
    emsheatval: f64,
    emscool: std::ffi::c_int,
    emscoolval: f64,
) -> f64 {
    let checked = (|| {
        let flags = (
            flag(heatused)?,
            flag(anyop)?,
            flag(anyhum)?,
            flag(emsheat)?,
            flag(emscool)?,
        );
        let ok = (0..=5).contains(&which)
            && (0..=4).contains(&ctype)
            && (0..=2).contains(&opctrl)
            && (0..=2).contains(&ovctrl);
        ok.then_some(flags)
    })();
    let Some((heatused, anyop, anyhum, emsheat, emscool)) = checked else {
        return f64::NAN;
    };
    // TempCtrl::None is 0; the constant and scheduled values are the same.
    let t = Thermostat {
        ctype,
        heat_used: heatused,
        heat_sp: heatsp,
        cool_sp: coolsp,
        heat_cool_sp: hcsp,
        dual_heat_sp: dhsp,
        dual_cool_sp: dcsp,
        op_temp: (anyop && opctrl != 0).then_some((frac, mrt)),
        overcool: (anyhum && ovctrl != 0).then_some((ovrange, ovratio, relhum, dehum)),
        ems_heat: emsheat.then_some(emsheatval),
        ems_cool: emscool.then_some(emscoolval),
    };
    let mut sp = SetPoints {
        setpt: setpt0,
        hi: hi0,
        lo: lo0,
        tstat_hi: 0.0,
        tstat_lo: 0.0,
    };
    t.calc_set_points(&mut sp);
    match which {
        0 => sp.setpt,
        1 => sp.hi,
        2 => sp.lo,
        3 => sp.tstat_hi,
        4 => sp.tstat_lo,
        _ => f64::from(ctype),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A thermostat of type `ctype` at 20 C heating and 24 C cooling, no adjustments.
    fn plain(which: i32, ctype: i32, anyop: i32, anyhum: i32) -> f64 {
        TstatSetPoints(
            which, ctype, 1, 20.0, 24.0, 21.0, 20.0, 24.0, 5.0, 6.0, 7.0, anyop, 1, 0.5, 30.0,
            anyhum, 1, 2.0, 10.0, 70.0, 50.0, 0, 0.0, 0, 0.0,
        )
    }

    #[test]
    fn control_types() {
        // Uncontrolled keeps the set points.
        assert_eq!(
            [plain(0, 0, 0, 0), plain(1, 0, 0, 0), plain(2, 0, 0, 0)],
            [5.0, 6.0, 7.0]
        );
        assert_eq!(plain(2, 1, 0, 0), 20.0);
        assert_eq!(plain(1, 2, 0, 0), 24.0);
        assert_eq!([plain(1, 3, 0, 0), plain(2, 3, 0, 0)], [21.0, 21.0]);
        assert_eq!(
            [plain(1, 4, 0, 0), plain(2, 4, 0, 0), plain(3, 4, 0, 0)],
            [24.0, 20.0, 24.0]
        );
        assert_eq!(plain(5, 4, 0, 0), 4.0);
    }

    #[test]
    fn operative_and_overcool_adjustments() {
        // (20 - 0.5 * 30) / 0.5 = 10.
        assert_eq!(plain(2, 4, 1, 0), 10.0);
        // 20 points over the dehumidifying set point at 10 per degree: 2 degrees, the range.
        assert_eq!(plain(1, 4, 0, 1), 22.0);
        assert!(plain(6, 4, 0, 0).is_nan());
    }
}
