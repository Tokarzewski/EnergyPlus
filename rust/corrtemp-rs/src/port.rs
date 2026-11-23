//! The Rust port of `corrtemp.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneSpaceHeatBalanceData::correctAirTemp` for one fully mixed zone (no space): the air power
//! capacity, the zone heat balance sums (unit 48, `zonesums-rs`), the heat balance coefficients
//! with and without system air flow, the new zone air temperature under each zone air solution
//! algorithm, the sensible load and its air system heating and cooling rates and energies, the
//! humidity correction (unit 43, `corrhum-rs`) with the new relative humidity and the system
//! node's conditions, and the returned temperature change. The psychrometric functions come from
//! `psychrometrics-rs`. Every expression keeps the C++ order. The oracle's guards (see its shim)
//! return NaN here too, as does the negative air density EnergyPlus stops on as a fatal error.

#![forbid(unsafe_code)]

use corrhum_rs::port::Zone as MoistZone;
use psychrometrics_rs::port::{PsyCpAirFnW, PsyHFnTdbW, PsyRhFnTdbWPb, PsyRhoAirFnPbTdbW};
use zonesums_rs::port::{Device, Kind, Node, Zone as SumsZone, calc_zone_or_space_sums};

/// `DataHeatBalance::SolutionAlgo`.
const THIRD_ORDER: i32 = 0;
const ANALYTICAL_SOLUTION: i32 = 1;

/// ObjexxFCL `min` for doubles: `a < b ? a : b`.
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `max` for doubles: `a < b ? b : a`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The zone terms `correctAirTemp` reads besides those of its sums and humidity correction.
struct Zone {
    use_zone_time_step_history: bool,
    algo: i32,
    has_system_node: bool,
    zone_mult: i32,
    volume: f64,
    zone_vol_cap_multp_sens: f64,
    time_step_sys_sec: f64,
    out_baro_press: f64,
    mat: f64,
    air_hum_rat: f64,
    xmat: [f64; 3],
    dsxmat: [f64; 3],
    w_prev_zone_ts: [f64; 3],
    ds_w_prev_zone_ts: [f64; 3],
    t1: f64,
    node_mass_flow_rate: f64,
    non_air_system_response: f64,
    sys_dep_zone_loads_lagged: f64,
}

/// What `correctAirTemp` sets, in the shim's `which` order.
struct Corrected {
    zt: f64,
    temp_dep_coef: f64,
    temp_ind_coef: f64,
    air_power_cap: f64,
    air_sys_heat_rate: f64,
    air_sys_cool_rate: f64,
    air_sys_heat_energy: f64,
    air_sys_cool_energy: f64,
    air_hum_rat: f64,
    air_rel_hum: f64,
    node_temp: f64,
    node_hum_rat: f64,
    node_enthalpy: f64,
    temp_change: f64,
}

/// The zone air temperature from the heat balance coefficients (`algo` 0 to 2).
fn solve(algo: i32, tic: f64, tdc: f64, apc: f64, ztm: &[f64; 3], t1: f64) -> f64 {
    match algo {
        THIRD_ORDER => {
            (tic + apc * (3.0 * ztm[0] - (3.0 / 2.0) * ztm[1] + (1.0 / 3.0) * ztm[2]))
                / ((11.0 / 6.0) * apc + tdc)
        }
        ANALYTICAL_SOLUTION => {
            if tdc == 0.0 {
                t1 + tic / apc
            } else {
                (t1 - tic / tdc) * min(700.0, -tdc / apc).exp() + tic / tdc
            }
        }
        // EulerMethod.
        _ => (apc * t1 + tic) / (apc + tdc),
    }
}

impl Zone {
    /// `correctAirTemp`; `None` where EnergyPlus stops on a fatal error.
    fn correct_air_temp(&self, sums_zone: &SumsZone, moist: MoistZone) -> Option<Corrected> {
        let zone_mult = f64::from(self.zone_mult);
        let time_step_sys_sec = self.time_step_sys_sec;
        let (ztm, w_prev_zone_ts_temp) = if !self.use_zone_time_step_history {
            (self.dsxmat, self.ds_w_prev_zone_ts)
        } else {
            (self.xmat, self.w_prev_zone_ts)
        };
        let rho_air = PsyRhoAirFnPbTdbW(self.out_baro_press, self.mat, self.air_hum_rat);
        // A negative air density is a fatal error.
        if rho_air < 0.0 {
            return None;
        }
        let air_power_cap =
            self.volume * self.zone_vol_cap_multp_sens * rho_air * PsyCpAirFnW(self.air_hum_rat)
                / time_step_sys_sec;

        let sums = calc_zone_or_space_sums(sums_zone, true);
        // calcSumHAT over a space without heat transfer surfaces.
        let (sum_ha, sum_ha_tsurf, sum_ha_tref) = (0.0, 0.0, 0.0);

        let temp_dep_coef;
        let temp_ind_coef;
        let zt_new;
        let mut node_temp = 0.0;
        let sn_load;
        if self.has_system_node {
            temp_dep_coef = sum_ha + sums.sum_mcp + sums.sum_sys_mcp;
            temp_ind_coef = sums.sum_int_gain + sum_ha_tsurf - sum_ha_tref
                + sums.sum_mcp_t
                + sums.sum_sys_mcp_t
                + (self.non_air_system_response / zone_mult + self.sys_dep_zone_loads_lagged);
            zt_new = solve(
                self.algo,
                temp_ind_coef,
                temp_dep_coef,
                air_power_cap,
                &ztm,
                self.t1,
            );
            // Fully mixed.
            node_temp = zt_new;
            let cp_air = PsyCpAirFnW(self.air_hum_rat);
            let zone_enthalpy_in = sums.sum_sys_mcp_t;
            sn_load = zone_enthalpy_in
                - (self.node_mass_flow_rate / zone_mult) * cp_air * node_temp
                + self.non_air_system_response / zone_mult
                + self.sys_dep_zone_loads_lagged;
        } else {
            temp_dep_coef = sum_ha + sums.sum_mcp;
            temp_ind_coef = sums.sum_int_gain + sum_ha_tsurf - sum_ha_tref + sums.sum_mcp_t;
            zt_new = solve(
                self.algo,
                temp_ind_coef,
                temp_dep_coef,
                air_power_cap,
                &ztm,
                self.t1,
            );
            sn_load = 0.0;
        }

        // reportZoneAirSystemSensibleLoads.
        let air_sys_heat_rate = max(sn_load, 0.0);
        let air_sys_cool_rate = min(sn_load, 0.0).abs();

        let moist = MoistZone {
            zt: zt_new,
            w_prev_zone_ts_temp,
            ..moist
        };
        let air_hum_rat = moist.correct_hum_rat()?;
        let (node_hum_rat, node_enthalpy) = if self.has_system_node {
            (air_hum_rat, PsyHFnTdbW(zt_new, air_hum_rat))
        } else {
            (0.0, 0.0)
        };
        let air_rel_hum = 100.0 * PsyRhFnTdbWPb(zt_new, air_hum_rat, self.out_baro_press);

        let temp_change = match self.algo {
            THIRD_ORDER => max(0.0, (zt_new - ztm[0]).abs()),
            // AnalyticalSolution and EulerMethod.
            _ => max(0.0, (zt_new - self.t1).abs()),
        };
        Some(Corrected {
            zt: zt_new,
            temp_dep_coef,
            temp_ind_coef,
            air_power_cap,
            air_sys_heat_rate,
            air_sys_cool_rate,
            air_sys_heat_energy: air_sys_heat_rate * time_step_sys_sec,
            air_sys_cool_energy: air_sys_cool_rate * time_step_sys_sec,
            air_hum_rat,
            air_rel_hum,
            node_temp,
            node_hum_rat,
            node_enthalpy,
            temp_change,
        })
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

/// A count from 0 to 2, `None` for anything else.
fn count(v: i32) -> Option<usize> {
    usize::try_from(v).ok().filter(|&n| n <= 2)
}

/// Port of `double CorrectAirTemp(int which, int usezonehist, int algo, ...)` (`corrtemp.c:8`):
/// see the oracle's shim for the arguments and what `which` picks.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn CorrectAirTemp(
    which: std::ffi::c_int,
    usezonehist: std::ffi::c_int,
    algo: std::ffi::c_int,
    hasnode: std::ffi::c_int,
    controlled: std::ffi::c_int,
    mult: std::ffi::c_int,
    listmult: std::ffi::c_int,
    volume: f64,
    capsens: f64,
    capmoist: f64,
    tsyssec: f64,
    outbaro: f64,
    outhumrat: f64,
    mat: f64,
    airhumrat: f64,
    xmat0: f64,
    xmat1: f64,
    xmat2: f64,
    dsxmat0: f64,
    dsxmat1: f64,
    dsxmat2: f64,
    wprev0: f64,
    wprev1: f64,
    wprev2: f64,
    dswprev0: f64,
    dswprev1: f64,
    dswprev2: f64,
    t1: f64,
    w1: f64,
    ndev: std::ffi::c_int,
    cg1: f64,
    rg1: f64,
    cg2: f64,
    rg2: f64,
    noheatret: std::ffi::c_int,
    radsys: f64,
    mcpi: f64,
    mcpti: f64,
    mcpv: f64,
    mcptv: f64,
    mdotcpoa: f64,
    outdb: f64,
    ninlets: std::ffi::c_int,
    m1: f64,
    tin1: f64,
    win1: f64,
    m2: f64,
    tin2: f64,
    win2: f64,
    nodemflow: f64,
    nonair: f64,
    sysdeplag: f64,
    latentgain: f64,
    oamfl: f64,
    vamfl: f64,
    mdotoa: f64,
) -> f64 {
    let checked = (|| {
        let flags = (
            flag(usezonehist)?,
            flag(hasnode)?,
            flag(controlled)?,
            flag(noheatret)?,
        );
        let counts = (count(ndev)?, count(ninlets)?);
        let ok = (0..=13).contains(&which)
            && (0..=2).contains(&algo)
            && (1..=10000).contains(&mult)
            && (1..=10000).contains(&listmult);
        ok.then_some((flags, counts))
    })();
    let Some(((usezonehist, hasnode, controlled, noheatret), (ndev, ninlets))) = checked else {
        return f64::NAN;
    };
    let zone_mult = mult * listmult;
    let devices = [
        Device {
            convect_gain_rate: cg1,
            return_air_conv_gain_rate: rg1,
        },
        Device {
            convect_gain_rate: cg2,
            return_air_conv_gain_rate: rg2,
        },
    ];
    let unused = Node {
        mass_flow_rate: 0.0,
        temp: 0.0,
    };
    let inlets = [
        Node {
            mass_flow_rate: m1,
            temp: tin1,
        },
        Node {
            mass_flow_rate: m2,
            temp: tin2,
        },
        unused,
    ];
    let n = if controlled { ninlets } else { 0 };
    let sums_zone = SumsZone {
        devices: &devices[..ndev],
        sum_conv_ht_rad_sys: radsys,
        sum_conv_pool: 0.0,
        no_heat_to_return_air: noheatret,
        mcp: [mcpi, mcpv, 0.0, 0.0, 0.0],
        mcpt: [mcpti, mcptv, 0.0, 0.0, 0.0],
        mdot_cp_oa: mdotcpoa,
        out_dry_bulb_temp: outdb,
        air_hum_rat: airhumrat,
        kind: if controlled {
            Kind::Controlled
        } else {
            Kind::Uncontrolled
        },
        inlets: &inlets[..n],
        inlets_all: &inlets,
        adu: None,
        piu: None,
        has_system_node: hasnode,
        zone_mult,
    };
    let moist_inlets = [(m1, win1), (m2, win2)];
    let moist = MoistZone {
        inlets: &moist_inlets[..n],
        zone_mult,
        zt: 0.0,
        out_baro_press: outbaro,
        air_hum_rat: airhumrat,
        latent_gain: latentgain,
        sum_latent_ht_rad_sys: 0.0,
        sum_latent_pool: 0.0,
        time_step_sys_sec: tsyssec,
        oamfl,
        vamfl,
        ctmfl: 0.0,
        eamfl: 0.0,
        eamfl_x_hum_rat: 0.0,
        sum_hm_a_ra_w: 0.0,
        sum_hm_a_ra: 0.0,
        mixing_mass_flow_x_hum_rat: 0.0,
        mixing_mass_flow_zone: 0.0,
        mdot_oa: mdotoa,
        out_hum_rat: outhumrat,
        volume,
        zone_vol_cap_multp_moist: capmoist,
        algo,
        w_prev_zone_ts_temp: [0.0; 3],
        w1,
    };
    let z = Zone {
        use_zone_time_step_history: usezonehist,
        algo,
        has_system_node: hasnode,
        zone_mult,
        volume,
        zone_vol_cap_multp_sens: capsens,
        time_step_sys_sec: tsyssec,
        out_baro_press: outbaro,
        mat,
        air_hum_rat: airhumrat,
        xmat: [xmat0, xmat1, xmat2],
        dsxmat: [dsxmat0, dsxmat1, dsxmat2],
        w_prev_zone_ts: [wprev0, wprev1, wprev2],
        ds_w_prev_zone_ts: [dswprev0, dswprev1, dswprev2],
        t1,
        node_mass_flow_rate: nodemflow,
        non_air_system_response: nonair,
        sys_dep_zone_loads_lagged: sysdeplag,
    };
    let Some(r) = z.correct_air_temp(&sums_zone, moist) else {
        return f64::NAN;
    };
    match which {
        0 => r.zt,
        1 => r.temp_dep_coef,
        2 => r.temp_ind_coef,
        3 => r.air_power_cap,
        4 => r.air_sys_heat_rate,
        5 => r.air_sys_cool_rate,
        6 => r.air_sys_heat_energy,
        7 => r.air_sys_cool_energy,
        8 => r.air_hum_rat,
        9 => r.air_rel_hum,
        10 => r.node_temp,
        11 => r.node_hum_rat,
        12 => r.node_enthalpy,
        _ => r.temp_change,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 300 m3 controlled zone at 22 C with two devices, infiltration, ventilation and two
    /// supply air inlets, under algorithm `algo` from the zone (`zh` 1) or system time step
    /// history.
    fn zone(which: i32, algo: i32, zh: i32, hasnode: i32) -> f64 {
        CorrectAirTemp(
            which, zh, algo, hasnode, 1, 1, 1, 300.0, 1.0, 1.0, 600.0, 101325.0, 0.006, 22.0,
            0.008, 22.0, 21.9, 21.8, 22.05, 22.0, 21.95, 0.008, 0.0079, 0.0078, 0.0081, 0.008,
            0.0079, 22.0, 0.008, 2, 500.0, 20.0, 150.0, 0.0, 0, 0.0, 30.0, 150.0, 10.0, 60.0, 5.0,
            5.0, 2, 0.3, 14.0, 0.007, 0.1, 30.0, 0.006, 0.4, 100.0, 50.0, 0.0001, 0.02, 0.01,
            0.005,
        )
    }

    #[test]
    fn cold_supply_air_cools_the_zone() {
        for algo in 0..=2 {
            let zt = zone(0, algo, 1, 1);
            assert!(zt < 22.0 && zt > 14.0, "{zt}");
            // The system node follows the zone air, and the load is cooling.
            assert_eq!(zone(10, algo, 1, 1), zt);
            assert_eq!(zone(4, algo, 1, 1), 0.0);
            assert!(zone(5, algo, 1, 1) > 0.0);
            assert_eq!(zone(7, algo, 1, 1), zone(5, algo, 1, 1) * 600.0);
        }
    }

    #[test]
    fn history_choice_and_uncontrolled_branch() {
        assert_ne!(zone(0, 0, 1, 1), zone(0, 0, 0, 1));
        // Without a system node: no load and the node untouched.
        assert_eq!(zone(5, 0, 1, 0), 0.0);
        assert_eq!(zone(10, 0, 1, 0), 0.0);
        assert_eq!(zone(13, 0, 1, 1), (zone(0, 0, 1, 1) - 22.0).abs());
    }

    #[test]
    fn guards_and_fatal_density() {
        assert!(zone(14, 0, 1, 1).is_nan());
        assert!(zone(0, 3, 1, 1).is_nan());
        assert!(zone(0, 0, 2, 1).is_nan());
        let cold = CorrectAirTemp(
            0, 1, 0, 1, 1, 1, 1, 300.0, 1.0, 1.0, 600.0, 101325.0, 0.006, -400.0, 0.008, 22.0,
            21.9, 21.8, 22.05, 22.0, 21.95, 0.008, 0.0079, 0.0078, 0.0081, 0.008, 0.0079, 22.0,
            0.008, 0, 0.0, 0.0, 0.0, 0.0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        );
        assert!(cold.is_nan());
    }
}
