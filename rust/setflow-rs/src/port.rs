//! The Rust port of `setflow.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `PlantUtilities::SetComponentFlowRate` for a component on a branch of up to three components
//! in series: its flow request bounded by the available and hardware limits (of the whole branch
//! when it is series active), the EMS on/off supervisory override, the locked loop side's inlet
//! flow, and whether a demand component's changed request needs the loop side simulated again.
//! Every expression keeps the C++ order, and `min` and `max` are ObjexxFCL's. The oracle's guards
//! (see its shim) return NaN here too.

#![forbid(unsafe_code)]

/// `DataBranchAirLoopPlant::MassFlowTolerance` (kg/s).
pub const MASS_FLOW_TOLERANCE: f64 = 0.000000001;

/// ObjexxFCL `min(a, b)`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The flow fields of one loop node.
#[derive(Clone, Copy, Default)]
pub struct Node {
    pub mass_flow_rate: f64,
    pub min: f64,
    pub max: f64,
    pub min_avail: f64,
    pub max_avail: f64,
    pub request: f64,
}

/// The branch and the calling component's settings.
pub struct Branch {
    /// Nodes 1 to `ncomp + 1`; component `c` runs from node `c` to node `c + 1`.
    pub nodes: [Node; 4],
    pub ncomp: usize,
    /// Each component's EMS load override, if on, is zero.
    pub ems_off: [bool; 3],
    pub locked: bool,
    pub autosize: bool,
    pub series_active: bool,
    pub demand: bool,
    pub sim_loop_side_needed: bool,
}

impl Branch {
    /// `SetComponentFlowRate` for the component from node `i` to node `o` (0-based).
    pub fn set_component_flow_rate(&mut self, comp_flow: &mut f64, i: usize, o: usize) {
        let mdot_old_request = self.nodes[i].request;
        self.nodes[i].request = *comp_flow;
        if self.demand {
            self.nodes[o].min_avail = max(self.nodes[i].min_avail, self.nodes[i].min);
            self.nodes[o].max_avail = min(self.nodes[i].max_avail, self.nodes[i].max);
        }
        self.nodes[o].min_avail = max(self.nodes[i].min_avail, self.nodes[i].min);
        // The oracle keeps the hardware maximum at 0 or more, so its severe error never runs.
        self.nodes[o].max_avail = min(self.nodes[i].max_avail, self.nodes[i].max);

        let ems_load_override = self.ems_off[..self.ncomp].iter().any(|&z| z);
        if !self.locked {
            if self.autosize {
                self.nodes[o].mass_flow_rate = *comp_flow;
                self.nodes[i].mass_flow_rate = self.nodes[o].mass_flow_rate;
            } else if self.series_active {
                let mut high_flow_request = 0.0;
                let mut hardware_max = self.nodes[i].max;
                let mut hardware_min = 0.0;
                let mut max_avail = self.nodes[i].max_avail;
                let mut min_avail = 0.0;
                for n in &self.nodes[..self.ncomp] {
                    high_flow_request = max(n.request, high_flow_request);
                    hardware_max = min(n.max, hardware_max);
                    hardware_min = max(n.min, hardware_min);
                    max_avail = min(n.max_avail, max_avail);
                    min_avail = max(n.min_avail, min_avail);
                }
                if ems_load_override {
                    hardware_max = 0.0;
                }
                *comp_flow = max(*comp_flow, high_flow_request);
                *comp_flow = max(*comp_flow, hardware_min);
                *comp_flow = max(*comp_flow, min_avail);
                *comp_flow = min(*comp_flow, hardware_max);
                *comp_flow = min(*comp_flow, max_avail);
                if *comp_flow < MASS_FLOW_TOLERANCE {
                    *comp_flow = 0.0;
                }
                self.nodes[o].mass_flow_rate = *comp_flow;
                self.nodes[i].mass_flow_rate = self.nodes[o].mass_flow_rate;
                for c in 0..self.ncomp {
                    self.nodes[c].mass_flow_rate = self.nodes[o].mass_flow_rate;
                    self.nodes[c + 1].mass_flow_rate = self.nodes[o].mass_flow_rate;
                }
            } else {
                let mut m = max(self.nodes[o].min_avail, *comp_flow);
                m = max(self.nodes[i].min, m);
                m = min(self.nodes[o].max_avail, m);
                m = min(self.nodes[i].max, m);
                if ems_load_override {
                    m = 0.0;
                }
                if m < MASS_FLOW_TOLERANCE {
                    m = 0.0;
                }
                self.nodes[o].mass_flow_rate = m;
                *comp_flow = m;
                self.nodes[i].mass_flow_rate = m;
            }
        } else {
            self.nodes[o].mass_flow_rate = self.nodes[i].mass_flow_rate;
            *comp_flow = self.nodes[o].mass_flow_rate;
        }

        if self.demand
            && (mdot_old_request > 0.0)
            && (*comp_flow > 0.0)
            && (mdot_old_request - self.nodes[i].request).abs() > MASS_FLOW_TOLERANCE
        {
            self.sim_loop_side_needed = true;
        }
    }
}

/// The value is a flow the oracle accepts.
fn flow_ok(v: f64) -> bool {
    v.is_finite() && (0.0..=1000.0).contains(&v)
}

/// Port of `double SetFlow(int which, int ncomp, int k, int lock, int autosize, int series, int
/// demand, int ems, double flow, ...)` (`setflow.c:9`), with each of nodes 1 to 4's mass flow,
/// hardware and available limits and request: see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn SetFlow(
    which: std::ffi::c_int,
    ncomp: std::ffi::c_int,
    k: std::ffi::c_int,
    lock: std::ffi::c_int,
    autosize: std::ffi::c_int,
    series: std::ffi::c_int,
    demand: std::ffi::c_int,
    ems: std::ffi::c_int,
    flow: f64,
    mdot1: f64,
    min1: f64,
    max1: f64,
    minav1: f64,
    maxav1: f64,
    req1: f64,
    mdot2: f64,
    min2: f64,
    max2: f64,
    minav2: f64,
    maxav2: f64,
    req2: f64,
    mdot3: f64,
    min3: f64,
    max3: f64,
    minav3: f64,
    maxav3: f64,
    req3: f64,
    mdot4: f64,
    min4: f64,
    max4: f64,
    minav4: f64,
    maxav4: f64,
    req4: f64,
) -> f64 {
    let raw = [
        [mdot1, min1, max1, minav1, maxav1, req1],
        [mdot2, min2, max2, minav2, maxav2, req2],
        [mdot3, min3, max3, minav3, maxav3, req3],
        [mdot4, min4, max4, minav4, maxav4, req4],
    ];
    let flag = |v: std::ffi::c_int| v == 0 || v == 1;
    let ok = (0..=17).contains(&which)
        && (1..=3).contains(&ncomp)
        && (1..=ncomp).contains(&k)
        && flag(lock)
        && flag(autosize)
        && flag(series)
        && flag(demand)
        && (0..=63).contains(&ems)
        && flow_ok(flow)
        && raw.iter().flatten().all(|&v| flow_ok(v));
    let (Ok(ncomp), Ok(k)) = (usize::try_from(ncomp), usize::try_from(k)) else {
        return f64::NAN;
    };
    if !ok {
        return f64::NAN;
    }
    let nodes = raw.map(
        |[mass_flow_rate, min, max, min_avail, max_avail, request]| Node {
            mass_flow_rate,
            min,
            max,
            min_avail,
            max_avail,
            request,
        },
    );
    let bit = |b: usize| (ems >> b) & 1 != 0;
    let mut branch = Branch {
        nodes,
        ncomp,
        ems_off: [bit(0) && bit(1), bit(2) && bit(3), bit(4) && bit(5)],
        locked: lock == 1,
        autosize: autosize == 1,
        series_active: series == 1,
        demand: demand == 1,
        sim_loop_side_needed: false,
    };
    let mut comp_flow = flow;
    branch.set_component_flow_rate(&mut comp_flow, k - 1, k);
    let Ok(which) = usize::try_from(which) else {
        return f64::NAN;
    };
    match which {
        0 => comp_flow,
        17 => f64::from(u8::from(branch.sim_loop_side_needed)),
        _ => {
            let n = branch.nodes[(which - 1) % 4];
            match (which - 1) / 4 {
                0 => n.mass_flow_rate,
                1 => n.min_avail,
                2 => n.max_avail,
                _ => n.request,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_is_bounded_by_the_branch() {
        let set = |which, lock, series, flow| {
            SetFlow(
                which, 2, 1, lock, 0, series, 1, 0, flow, 0.5, 0.0, 2.0, 0.0, 1.5, 0.3, 0.0, 0.0,
                2.0, 0.0, 2.0, 0.2, 0.0, 0.0, 1.0, 0.0, 1.0, 0.8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            )
        };
        assert_eq!(set(0, 0, 0, 1.0), 1.0);
        assert_eq!(set(0, 0, 0, 3.0), 1.5);
        assert_eq!(set(0, 1, 0, 3.0), 0.5);
        // The caller's own request is replaced first, so the downstream 0.2 is the highest.
        assert_eq!(set(0, 0, 1, 0.1), 0.2);
        assert_eq!(set(3, 0, 1, 0.1), 0.2);
        assert_eq!(set(17, 0, 0, 1.0), 1.0);
        assert!(set(18, 0, 0, 1.0).is_nan());
    }
}
