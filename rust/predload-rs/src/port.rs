//! The Rust port of `predload.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneTempPredictorCorrector::ZoneSpaceHeatBalanceData::calcPredictedSystemLoad` for one zone:
//! the sensible loads to its heating and cooling set points under the zone air solution algorithm,
//! by thermostat type, with the RoomAirflowNetwork fraction, the IT equipment return air
//! adjustment, the deadband and staged thermostats, then the zone set point, setback and deadband
//! flags, and the demands with the load correction factor and zone multipliers
//! (`ZoneSystemSensibleDemand::reportSensibleLoadsZoneMultiplier`). Every expression keeps the C++
//! order. The oracle's guards (see its shim) return NaN here too, as do the loads EnergyPlus stops
//! on as fatal errors.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

/// `HVAC::SetptType`.
const SINGLE_HEAT: i32 = 1;
const SINGLE_COOL: i32 = 2;
const SINGLE_HEAT_COOL: i32 = 3;
const DUAL_HEAT_COOL: i32 = 4;

/// `DataHeatBalance::SolutionAlgo`.
const THIRD_ORDER: i32 = 0;
const ANALYTICAL_SOLUTION: i32 = 1;

/// ObjexxFCL `min` for doubles: `a < b ? a : b` (so a NaN `b` is returned).
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max` for doubles: `a < b ? b : a` (so a NaN `a` is returned).
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The zone terms `calcPredictedSystemLoad` reads.
struct Zone {
    ctrl: i32,
    algo: i32,
    rafn_frac: f64,
    temp_dep_load: f64,
    temp_ind_load: f64,
    air_power_cap: f64,
    t1: f64,
    setpt: f64,
    setpt_lo: f64,
    setpt_hi: f64,
    /// `AdjustedReturnTempByITE` when the zone has it and the simulation is past `BeginSimFlag`.
    ite_return_temp: Option<f64>,
    /// The system node temperature, when the zone has a node.
    node_temp: Option<f64>,
    /// The stage, when the zone has a staged thermostat.
    stage_num: Option<i32>,
}

/// What the call leaves behind.
struct Predicted {
    total_load: f64,
    load_to_heating_set_point: f64,
    load_to_cooling_set_point: f64,
    zone_set_point: f64,
    dead_band_or_set_back: bool,
}

impl Zone {
    /// The load to hold the zone at `sp` under the zone air solution algorithm.
    fn load_to(&self, sp: f64) -> f64 {
        match self.algo {
            THIRD_ORDER => self.temp_dep_load * sp - self.temp_ind_load,
            ANALYTICAL_SOLUTION => {
                if self.temp_dep_load == 0.0 {
                    self.air_power_cap * (sp - self.t1) - self.temp_ind_load
                } else {
                    let exp_700_ta = min(700.0, -self.temp_dep_load / self.air_power_cap).exp();
                    self.temp_dep_load * (sp - self.t1 * exp_700_ta) / (1.0 - exp_700_ta)
                        - self.temp_ind_load
                }
            }
            // EulerMethod.
            _ => self.air_power_cap * (sp - self.t1) + self.temp_dep_load * sp - self.temp_ind_load,
        }
    }

    /// The node temperature trapped into the deadband, or `sp` when the zone has no node.
    fn dead_band_set_point(&self, sp: f64) -> f64 {
        match self.node_temp {
            Some(t) => min(max(t, self.setpt_lo), self.setpt_hi),
            None => sp,
        }
    }

    /// `calcPredictedSystemLoad`; `None` where EnergyPlus stops on a fatal error.
    fn calc_predicted_system_load(&self) -> Option<Predicted> {
        let mut this_dead_band_or_set_back = false;
        let mut zone_set_point = 0.0;
        let mut total_load = 0.0;
        let mut load_to_heating_set_point = 0.0;
        let mut load_to_cooling_set_point = 0.0;
        match self.ctrl {
            SINGLE_HEAT => {
                load_to_heating_set_point = self.load_to(self.setpt);
                if self.rafn_frac > 0.0 {
                    load_to_heating_set_point /= self.rafn_frac;
                }
                total_load = load_to_heating_set_point;
                zone_set_point = self.setpt;
                load_to_cooling_set_point = load_to_heating_set_point;
                if total_load <= 0.0 {
                    this_dead_band_or_set_back = true;
                }
            }
            SINGLE_COOL => {
                load_to_cooling_set_point = self.load_to(self.setpt);
                // The C++ divides the heating load by the RoomAirflowNetwork fraction here (not the
                // cooling load), then overwrites it below, so the cooling load is never divided.
                if let Some(t) = self.ite_return_temp {
                    load_to_cooling_set_point = self.temp_dep_load * t - self.temp_ind_load;
                }
                total_load = load_to_cooling_set_point;
                zone_set_point = self.setpt;
                load_to_heating_set_point = load_to_cooling_set_point;
                if total_load >= 0.0 {
                    this_dead_band_or_set_back = true;
                }
            }
            SINGLE_HEAT_COOL | DUAL_HEAT_COOL => {
                let dual = self.ctrl == DUAL_HEAT_COOL;
                let (lo, hi) = if dual {
                    (self.setpt_lo, self.setpt_hi)
                } else {
                    (self.setpt, self.setpt)
                };
                load_to_heating_set_point = self.load_to(lo);
                load_to_cooling_set_point = self.load_to(hi);
                if !dual {
                    zone_set_point = self.setpt;
                }
                if self.rafn_frac > 0.0 {
                    load_to_heating_set_point /= self.rafn_frac;
                    load_to_cooling_set_point /= self.rafn_frac;
                }
                if let Some(t) = self.ite_return_temp {
                    load_to_cooling_set_point = self.temp_dep_load * t - self.temp_ind_load;
                }
                // Effective heating set point above the cooling one: a fatal error.
                if load_to_heating_set_point > load_to_cooling_set_point {
                    return None;
                }
                if load_to_heating_set_point > 0.0 && load_to_cooling_set_point > 0.0 {
                    total_load = load_to_heating_set_point;
                    if dual {
                        zone_set_point = self.setpt_lo;
                    }
                } else if load_to_heating_set_point < 0.0 && load_to_cooling_set_point < 0.0 {
                    total_load = load_to_cooling_set_point;
                    if dual {
                        zone_set_point = self.setpt_hi;
                    }
                } else if load_to_heating_set_point <= 0.0 && load_to_cooling_set_point >= 0.0 {
                    total_load = 0.0;
                    zone_set_point = self.dead_band_set_point(zone_set_point);
                    this_dead_band_or_set_back = true;
                } else {
                    // Loads that compare as none of these (NaN): a fatal error.
                    return None;
                }
            }
            // Uncontrolled, and an unset thermostat type.
            _ => {}
        }

        // Staged control zone.
        if let Some(stage_num) = self.stage_num {
            if stage_num == 0 {
                load_to_heating_set_point = 0.0;
                load_to_cooling_set_point = 0.0;
                total_load = 0.0;
                zone_set_point = self.dead_band_set_point(zone_set_point);
                this_dead_band_or_set_back = true;
            } else if stage_num < 0 {
                load_to_cooling_set_point = self.load_to(self.setpt_hi);
                total_load = load_to_cooling_set_point;
                zone_set_point = self.setpt_hi;
                load_to_heating_set_point = load_to_cooling_set_point;
                if total_load >= 0.0 {
                    this_dead_band_or_set_back = true;
                }
            } else {
                load_to_heating_set_point = self.load_to(self.setpt_lo);
                total_load = load_to_heating_set_point;
                zone_set_point = self.setpt_lo;
                load_to_cooling_set_point = load_to_heating_set_point;
                if total_load <= 0.0 {
                    this_dead_band_or_set_back = true;
                }
            }
        }

        Some(Predicted {
            total_load,
            load_to_heating_set_point,
            load_to_cooling_set_point,
            zone_set_point,
            dead_band_or_set_back: this_dead_band_or_set_back,
        })
    }
}

/// One result of `calcPredictedSystemLoad` for one zone (see the oracle's shim for the arguments
/// and `which`).
pub fn PredictedSystemLoad(
    which: i32,
    ctrl: i32,
    algo: i32,
    rafnfrac: f64,
    tempdep: f64,
    tempind: f64,
    aircap: f64,
    t1: f64,
    setpt: f64,
    setptlo: f64,
    setpthi: f64,
    hasite: i32,
    itetemp: f64,
    beginsim: i32,
    hasnode: i32,
    nodetemp: f64,
    staged: i32,
    stagenum: i32,
    setptlast: f64,
    lcf: f64,
    mult: i32,
    listmult: i32,
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    if !(0..=9).contains(&which)
        || ![hasite, beginsim, hasnode, staged].into_iter().all(flag)
        || !(-1..=4).contains(&ctrl)
        || !(0..=2).contains(&algo)
        || !(1..=10000).contains(&mult)
        || !(1..=10000).contains(&listmult)
    {
        return f64::NAN;
    }
    let z = Zone {
        ctrl,
        algo,
        rafn_frac: rafnfrac,
        temp_dep_load: tempdep,
        temp_ind_load: tempind,
        air_power_cap: aircap,
        t1,
        setpt,
        setpt_lo: setptlo,
        setpt_hi: setpthi,
        ite_return_temp: (hasite == 1 && beginsim == 0).then_some(itetemp),
        node_temp: (hasnode == 1).then_some(nodetemp),
        stage_num: (staged == 1).then_some(stagenum),
    };
    let Some(p) = z.calc_predicted_system_load() else {
        return f64::NAN;
    };
    let predicted_rate = p.total_load * lcf;
    let predicted_hsp_rate = p.load_to_heating_set_point * lcf;
    let predicted_csp_rate = p.load_to_cooling_set_point * lcf;
    let zone_mult_fac = f64::from(mult * listmult);
    let flag_value = |b: bool| if b { 1.0 } else { 0.0 };
    match which {
        0 => predicted_rate,
        1 => predicted_hsp_rate,
        2 => predicted_csp_rate,
        3 => predicted_rate * zone_mult_fac,
        4 => predicted_hsp_rate * zone_mult_fac,
        5 => predicted_csp_rate * zone_mult_fac,
        6 => p.zone_set_point,
        7 => {
            if hasnode == 1 {
                p.zone_set_point
            } else {
                0.0
            }
        }
        8 => flag_value(p.zone_set_point > setptlast),
        _ => flag_value(p.dead_band_or_set_back),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A dual set point zone (20 to 24 C) under the third order solution, with a node at 23 C.
    fn dual(which: i32, tempind: f64) -> f64 {
        PredictedSystemLoad(
            which, 4, 0, 0.0, 500.0, tempind, 2000.0, 21.0, 22.0, 20.0, 24.0, 0, 0.0, 0, 1, 23.0,
            0, 0, 20.0, 1.0, 2, 1,
        )
    }

    #[test]
    fn dual_set_point_deadband_holds_the_node_temperature() {
        // Loads 0 to heating and 2000 to cooling: deadband.
        assert_eq!(dual(0, 10000.0), 0.0);
        assert_eq!(dual(5, 10000.0), 4000.0);
        assert_eq!(dual(6, 10000.0), 23.0);
        assert_eq!(dual(8, 10000.0), 1.0);
        assert_eq!(dual(9, 10000.0), 1.0);
    }

    #[test]
    fn dual_set_point_heating() {
        // Loads 1000 to heating and 3000 to cooling: heat to 20 C.
        assert_eq!(dual(3, 9000.0), 2000.0);
        assert_eq!(dual(6, 9000.0), 20.0);
        assert_eq!(dual(9, 9000.0), 0.0);
    }

    #[test]
    fn crossed_set_points_and_guards_return_nan() {
        let crossed = PredictedSystemLoad(
            0, 4, 0, 0.0, 500.0, 10000.0, 2000.0, 21.0, 22.0, 25.0, 24.0, 0, 0.0, 0, 1, 23.0, 0, 0,
            20.0, 1.0, 1, 1,
        );
        assert!(crossed.is_nan());
        assert!(dual(10, 0.0).is_nan());
        assert!(
            PredictedSystemLoad(
                0, 5, 0, 0.0, 500.0, 0.0, 2000.0, 21.0, 22.0, 20.0, 24.0, 0, 0.0, 0, 1, 23.0, 0, 0,
                20.0, 1.0, 1, 1,
            )
            .is_nan()
        );
    }

    #[test]
    fn objexx_min_returns_a_nan_second_argument() {
        assert!(min(700.0, f64::NAN).is_nan());
        assert_eq!(max(1.0, f64::NAN), 1.0);
        assert!(max(f64::NAN, 1.0).is_nan());
    }
}
