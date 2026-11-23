//! The Rust port of `condupd.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `PlantUtilities::UpdateChillerComponentCondenserSide` and `UpdateComponentHeatRecoverySide`:
//! when the model's flow or temperatures differ from the nodes' (or the inlet has no flow but heat
//! is moved), or on the first HVAC iteration, the outlet temperature is the inlet's plus the heat
//! over the flow times the water specific heat, and the component's loop side and every connected
//! loop it puts demand on are flagged to be simulated again; otherwise the side's flag is cleared.
//! The two differ only in the condenser side taking the heat's magnitude in the no-flow check. The
//! water specific heat comes from `fluidproperties-rs`, cached by temperature as the C++ caches
//! it. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

use fluidproperties_rs::port::WaterSpecificHeat;

/// `DataBranchAirLoopPlant::MassFlowTolerance` (kg/s).
const MASS_FLOW_TOLERANCE: f64 = 0.000000001;

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

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// A node's mass flow (kg/s) and temperature (C).
#[derive(Clone, Copy)]
struct NodeState {
    m: f64,
    t: f64,
}

/// What the model reports: heat rate (W), inlet and outlet temperatures (C) and mass flow (kg/s).
#[derive(Clone, Copy)]
struct Model {
    q: f64,
    tin: f64,
    tout: f64,
    m: f64,
}

/// The outlet temperature and the three simulate flags (own side, remote 1, remote 2).
fn update(
    hr: bool,
    first: bool,
    sim0: bool,
    demands: &[bool],
    model: Model,
    inlet: NodeState,
    outlet: NodeState,
) -> (f64, [bool; 3]) {
    let heat_moved = if hr {
        model.q > 0.0
    } else {
        model.q.abs() > 0.0
    };
    let did_anything_change = inlet.m != model.m
        || outlet.m != model.m
        || inlet.t != model.tin
        || outlet.t != model.tout
        || (inlet.m == 0.0 && heat_moved);
    let mut out_t = outlet.t;
    let mut flags = [sim0, false, false];
    if did_anything_change || first {
        if inlet.m > MASS_FLOW_TOLERANCE {
            let cp = water_cp(model.tin);
            out_t = inlet.t + model.q / (inlet.m * cp);
        }
        flags[0] = true;
        for (k, &d) in demands.iter().enumerate() {
            if d {
                flags[k + 1] = true;
            }
        }
    } else {
        flags[0] = false;
    }
    (out_t, flags)
}

/// Port of `double CondUpd(int which, int hr, int first, int sim0, int nconn, ...)`
/// (`condupd.c:9`): `UpdateChillerComponentCondenserSide` and `UpdateComponentHeatRecoverySide`,
/// see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature, one argument per field the shim sets"
)]
pub fn CondUpd(
    which: std::ffi::c_int,
    hr: std::ffi::c_int,
    first: std::ffi::c_int,
    sim0: std::ffi::c_int,
    nconn: std::ffi::c_int,
    dem1: std::ffi::c_int,
    dem2: std::ffi::c_int,
    q: f64,
    tmin: f64,
    tmout: f64,
    mmod: f64,
    inm: f64,
    intemp: f64,
    outm: f64,
    outt: f64,
) -> f64 {
    let flag = |v: std::ffi::c_int| v == 0 || v == 1;
    let (Ok(which), Ok(nconn)) = (usize::try_from(which), usize::try_from(nconn)) else {
        return f64::NAN;
    };
    if which > 3
        || !flag(hr)
        || !flag(first)
        || !flag(sim0)
        || nconn > 2
        || !flag(dem1)
        || !flag(dem2)
        || !within(q, -1.0e8, 1.0e8)
        || !within(tmin, 1.0, 99.0)
        || !within(tmout, -20.0, 150.0)
        || !within(mmod, 0.0, 1000.0)
        || !within(inm, 0.0, 1000.0)
        || !within(intemp, -20.0, 150.0)
        || !within(outm, 0.0, 1000.0)
        || !within(outt, -20.0, 150.0)
    {
        return f64::NAN;
    }
    let demands = [dem1 == 1, dem2 == 1];
    let (out_t, flags) = update(
        hr == 1,
        first == 1,
        sim0 == 1,
        &demands[..nconn],
        Model {
            q,
            tin: tmin,
            tout: tmout,
            m: mmod,
        },
        NodeState { m: inm, t: intemp },
        NodeState { m: outm, t: outt },
    );
    match which {
        0 => out_t,
        k => f64::from(u8::from(flags[k - 1])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_change_heats_the_outlet_and_flags_the_demanded_loops() {
        let f = |which| {
            CondUpd(
                which, 0, 0, 0, 2, 1, 0, 50000.0, 30.0, 35.0, 2.0, 2.0, 30.0, 2.0, 34.0,
            )
        };
        assert_eq!(f(0), 30.0 + 50000.0 / (2.0 * WaterSpecificHeat(30.0)));
        assert_eq!([f(1), f(2), f(3)], [1.0, 1.0, 0.0]);
    }

    #[test]
    fn nothing_changed_clears_the_flag() {
        let f = |which, hr, q| {
            CondUpd(
                which, hr, 0, 1, 1, 1, 1, q, 30.0, 35.0, 0.0, 0.0, 30.0, 0.0, 35.0,
            )
        };
        assert_eq!(f(1, 0, 0.0), 0.0);
        assert_eq!(f(0, 0, 0.0), 35.0);
        assert_eq!(f(1, 0, -10.0), 1.0);
        assert_eq!(f(1, 1, -10.0), 0.0);
        assert!(f(4, 0, 0.0).is_nan());
    }
}
