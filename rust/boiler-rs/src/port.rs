//! The Rust port of `boiler.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `Boilers::BoilerSpecs::CalcBoilerModel`: the hot water boiler's load, flow and outlet
//! temperature for constant, not modulated or leaving set point modulated flow (the flow through
//! `SetComponentFlowRate` from `setflow-rs`) or on a locked loop side, the outlet temperature
//! limit, the part load ratio, the normalised efficiency curve (a quadratic in the part load ratio
//! or a biquadratic with the entering or leaving water temperature, as `Curve::value` evaluates
//! them), the fuel use and the parasitics. The water specific heat comes from
//! `fluidproperties-rs`. Every expression keeps the C++ order, and `min`, `max` and `clamp` are
//! the standard library's. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use fluidproperties_rs::port::WaterSpecificHeat;
use setflow_rs::port::{Branch, Node};

thread_local! {
    /// The last temperature's water specific heat (bits of the temperature, value).
    static CP_CACHE: std::cell::Cell<(u64, f64)> = const { std::cell::Cell::new((u64::MAX, 0.0)) };
}

/// `GlycolProps::getSpecificHeat` for water, which caches its results by temperature; this keeps
/// the last one (the value is the table's either way).
fn water_cp(t: f64) -> f64 {
    let key = t.to_bits();
    let (k, v) = CP_CACHE.get();
    if k == key {
        return v;
    }
    let cp = WaterSpecificHeat(t);
    CP_CACHE.set((key, cp));
    cp
}

/// `std::min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}

/// `std::max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `std::clamp(v, lo, hi)`.
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    if v < lo {
        lo
    } else if hi < v {
        hi
    } else {
        v
    }
}

/// The flow mode.
#[derive(Clone, Copy, PartialEq)]
enum FlowMode {
    Constant,
    NotModulated,
    LeavingSetpointModulated,
}

/// The efficiency curve: none, a quadratic in the part load ratio (input 0 to 1) or a biquadratic
/// in it and the water temperature (input 5 to 95 C), with its temperature mode.
#[derive(Clone, Copy, PartialEq)]
enum EffCurve {
    None,
    Quadratic,
    BiQuadratic,
}

/// One boiler as `InitBoiler` leaves it.
struct Boiler {
    nom_cap: f64,
    nom_eff: f64,
    min_plr: f64,
    max_plr: f64,
    temp_up_limit: f64,
    des_mass_flow: f64,
    parasitic_elec_load: f64,
    parasitic_fuel_capacity: f64,
    flow_mode: FlowMode,
    curve: EffCurve,
    /// `TempMode`: 0 not set, 1 entering, 2 leaving boiler temperature.
    temp_mode: i32,
    coeff: [f64; 6],
}

/// The results of a call.
#[derive(Default)]
struct Out {
    load: f64,
    mass_flow: f64,
    outlet_temp: f64,
    plr: f64,
    eff: f64,
    fuel_used: f64,
    parasitic_elec_power: f64,
    parasitic_fuel_rate: f64,
}

impl Boiler {
    /// `Curve::value` of the efficiency curve.
    fn curve_value(&self, v1: f64, v2: f64) -> f64 {
        let c = &self.coeff;
        let v1 = clamp(v1, 0.0, 1.0);
        if self.curve == EffCurve::Quadratic {
            return c[0] + v1 * (c[1] + v1 * c[2]);
        }
        let v2 = clamp(v2, 5.0, 95.0);
        c[0] + v1 * (c[1] + v1 * c[2]) + v2 * (c[3] + v2 * c[4]) + v1 * v2 * c[5]
    }

    /// `CalcBoilerModel` on the branch (node 0 inlet, node 1 outlet) with inlet temperature `tin`
    /// and the outlet's set points.
    #[expect(
        clippy::too_many_arguments,
        reason = "the routine's inputs, as in the C++ state"
    )]
    fn calc(
        &self,
        branch: &mut Branch,
        my_load: f64,
        run: bool,
        dual: bool,
        tin: f64,
        tset: f64,
        tset_lo: f64,
    ) -> Out {
        let mut o = Out::default();
        let cp = water_cp(tin);
        if my_load <= 0.0 || !run {
            if branch.series_active {
                o.mass_flow = branch.nodes[0].mass_flow_rate;
            }
            return o;
        }
        o.load = my_load;
        if !branch.locked {
            if self.flow_mode == FlowMode::Constant || self.flow_mode == FlowMode::NotModulated {
                o.mass_flow = self.des_mass_flow;
                branch.set_component_flow_rate(&mut o.mass_flow, 0, 1);
                let delta = if (o.mass_flow != 0.0) && (my_load > 0.0) {
                    o.load / o.mass_flow / cp
                } else {
                    0.0
                };
                o.outlet_temp = delta + tin;
            } else {
                let delta = if dual { tset_lo - tin } else { tset - tin };
                o.outlet_temp = delta + tin;
                if (delta > 0.0) && (o.load > 0.0) {
                    o.mass_flow = o.load / cp / delta;
                    o.mass_flow = min(self.des_mass_flow, o.mass_flow);
                } else {
                    o.mass_flow = 0.0;
                }
                branch.set_component_flow_rate(&mut o.mass_flow, 0, 1);
            }
        } else {
            o.mass_flow = branch.nodes[0].mass_flow_rate;
            if (my_load > 0.0) && (o.mass_flow > 0.0) {
                o.load = my_load;
                if o.load > self.nom_cap * self.max_plr {
                    o.load = self.nom_cap * self.max_plr;
                }
                if o.load < self.nom_cap * self.min_plr {
                    o.load = self.nom_cap * self.min_plr;
                }
                o.outlet_temp = tin + o.load / (o.mass_flow * cp);
            } else {
                o.load = 0.0;
                o.outlet_temp = tin;
            }
        }

        if o.outlet_temp > self.temp_up_limit {
            o.load = 0.0;
            o.outlet_temp = tin;
        }
        o.plr = o.load / self.nom_cap;
        o.plr = min(o.plr, self.max_plr);
        o.plr = max(o.plr, self.min_plr);

        let theor_fuel_use = o.load / self.nom_eff;
        let eff_curve_output = match (self.curve, self.temp_mode) {
            (EffCurve::None, _) | (EffCurve::BiQuadratic, 0) => 1.0,
            (EffCurve::Quadratic, _) => self.curve_value(o.plr, 0.0),
            (EffCurve::BiQuadratic, 1) => self.curve_value(o.plr, tin),
            (EffCurve::BiQuadratic, _) => self.curve_value(o.plr, o.outlet_temp),
        };
        o.eff = eff_curve_output * self.nom_eff;
        // The oracle runs with the warm-up flag on, which skips the curve warnings and the resets
        // of the curve output to 0.01 and 1.1 that come with them.
        o.fuel_used = theor_fuel_use / eff_curve_output;
        if o.load > 0.0 {
            o.parasitic_elec_power = self.parasitic_elec_load * o.plr;
        }
        o.parasitic_fuel_rate = self.parasitic_fuel_capacity * (1.0 - o.plr);
        o
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double Boiler(int which, int lock, int flowmode, int dual, int series, int run, int
/// curve, int tempmode, double myload, ...)` (`boiler.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn Boiler(
    which: std::ffi::c_int,
    lock: std::ffi::c_int,
    flowmode: std::ffi::c_int,
    dual: std::ffi::c_int,
    series: std::ffi::c_int,
    run: std::ffi::c_int,
    curve: std::ffi::c_int,
    tempmode: std::ffi::c_int,
    myload: f64,
    nomcap: f64,
    nomeff: f64,
    minplr: f64,
    maxplr: f64,
    tlimit: f64,
    desmdot: f64,
    parelec: f64,
    parfuel: f64,
    tin: f64,
    mdotin: f64,
    hwmax: f64,
    minav: f64,
    maxav: f64,
    tset: f64,
    tsetlo: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
) -> f64 {
    let flag = |v: std::ffi::c_int| v == 0 || v == 1;
    let coeff = [c0, c1, c2, c3, c4, c5];
    let ok = (0..=10).contains(&which)
        && flag(lock)
        && flag(dual)
        && flag(series)
        && flag(run)
        && (0..=2).contains(&flowmode)
        && (0..=2).contains(&curve)
        && (0..=2).contains(&tempmode)
        && within(myload, -1.0e8, 1.0e8)
        && within(nomcap, 1.0, 1.0e8)
        && within(nomeff, 0.1, 1.2)
        && within(minplr, 0.0, 1.0)
        && within(maxplr, minplr, 1.5)
        && within(tlimit, 20.0, 120.0)
        && within(desmdot, 0.0, 1000.0)
        && within(parelec, 0.0, 1.0e5)
        && within(parfuel, 0.0, 1.0e5)
        && within(tin, 1.0, 99.0)
        && within(mdotin, 0.0, 1000.0)
        && within(hwmax, 0.0, 1000.0)
        && within(minav, 0.0, 1000.0)
        && within(maxav, 0.0, 1000.0)
        && within(tset, -20.0, 150.0)
        && within(tsetlo, -20.0, 150.0)
        && coeff.iter().all(|&c| within(c, -5.0, 5.0));
    if !ok {
        return f64::NAN;
    }
    let boiler = Boiler {
        nom_cap: nomcap,
        nom_eff: nomeff,
        min_plr: minplr,
        max_plr: maxplr,
        temp_up_limit: tlimit,
        des_mass_flow: desmdot,
        parasitic_elec_load: parelec,
        parasitic_fuel_capacity: parfuel,
        flow_mode: match flowmode {
            0 => FlowMode::Constant,
            1 => FlowMode::NotModulated,
            _ => FlowMode::LeavingSetpointModulated,
        },
        curve: match curve {
            0 => EffCurve::None,
            1 => EffCurve::Quadratic,
            _ => EffCurve::BiQuadratic,
        },
        temp_mode: tempmode,
        coeff,
    };
    let inlet = Node {
        mass_flow_rate: mdotin,
        min: 0.0,
        max: hwmax,
        min_avail: minav,
        max_avail: maxav,
        request: 0.0,
    };
    let mut branch = Branch {
        nodes: [inlet, Node::default(), Node::default(), Node::default()],
        ncomp: 1,
        ems_off: [false; 3],
        locked: lock == 1,
        autosize: false,
        series_active: series == 1,
        demand: false,
        sim_loop_side_needed: false,
    };
    let o = boiler.calc(&mut branch, myload, run == 1, dual == 1, tin, tset, tsetlo);
    match which {
        0 => o.load,
        1 => o.mass_flow,
        2 => o.outlet_temp,
        3 => o.plr,
        4 => o.eff,
        5 => o.fuel_used,
        6 => o.parasitic_elec_power,
        7 => o.parasitic_fuel_rate,
        8 => branch.nodes[0].mass_flow_rate,
        9 => branch.nodes[1].mass_flow_rate,
        _ => branch.nodes[1].max_avail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_flow_boiler_heats_the_water() {
        let b = |which, lock, myload| {
            Boiler(
                which, lock, 0, 0, 0, 1, 1, 0, myload, 100_000.0, 0.8, 0.0, 1.0, 95.0, 2.0, 100.0,
                50.0, 60.0, 1.0, 2.0, 0.0, 2.0, 80.0, 70.0, 0.97, 0.06, -0.03, 0.0, 0.0, 0.0,
            )
        };
        assert_eq!(b(1, 0, 50_000.0), 2.0);
        let t = b(2, 0, 50_000.0);
        assert!(t > 65.0 && t < 67.0);
        assert_eq!(b(3, 0, 50_000.0), 0.5);
        assert!(b(5, 0, 50_000.0) > 50_000.0 / 0.8);
        assert_eq!(b(0, 0, 0.0), 0.0);
        assert_eq!(b(1, 1, 50_000.0), 1.0);
        assert!(b(11, 0, 50_000.0).is_nan());
    }
}
