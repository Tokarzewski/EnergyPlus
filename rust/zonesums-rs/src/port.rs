//! The Rust port of `zonesums.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums` for one zone (no space) of one space without
//! heat transfer surfaces: the convective internal gains (with radiant heater, pool and return air
//! gains), the non-system air flow sums, and as the corrector the system air flow sums of a
//! controlled zone, a return plenum (with air terminal leaks) or a supply plenum, and parallel PIU
//! backdraft leakage. The surface sums are unit 40 (`sumhat-rs`). `PsyCpAirFnW` comes from
//! `psychrometrics-rs`. Every expression keeps the C++ order. The oracle's guards (see its shim)
//! return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

use psychrometrics_rs::port::PsyCpAirFnW;

/// The zone's kind of system air flow.
#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Uncontrolled,
    Controlled,
    ReturnPlenum,
    SupplyPlenum,
}

/// An internal gain device: convective and return air convective gain rates (W).
#[derive(Clone, Copy)]
pub struct Device {
    pub convect_gain_rate: f64,
    pub return_air_conv_gain_rate: f64,
}

/// A node's mass flow rate (kg/s) and temperature (C).
#[derive(Clone, Copy)]
pub struct Node {
    pub mass_flow_rate: f64,
    pub temp: f64,
}

/// An air distribution unit leaking to the return plenum.
pub struct AirDistUnit {
    pub up_stream_leak: bool,
    pub mass_flow_rate_up_str_lk: f64,
    pub inlet_temp: f64,
    pub down_stream_leak: bool,
    pub mass_flow_rate_dn_str_lk: f64,
    pub outlet_temp: f64,
}

/// A parallel PIU's backdraft damper leakage (kg/s) and its primary air inlet temperature (C).
pub struct Piu {
    pub leak_flow: f64,
    pub pri_air_in_temp: f64,
}

/// The zone, its space's gain devices and the heat balance terms the sums start from.
pub struct Zone<'a> {
    pub devices: &'a [Device],
    pub sum_conv_ht_rad_sys: f64,
    pub sum_conv_pool: f64,
    pub no_heat_to_return_air: bool,
    /// MCPI, MCPV, MCPM, MCPE, MCPC.
    pub mcp: [f64; 5],
    /// MCPTI, MCPTV, MCPTM, MCPTE, MCPTC.
    pub mcpt: [f64; 5],
    pub mdot_cp_oa: f64,
    pub out_dry_bulb_temp: f64,
    pub air_hum_rat: f64,
    pub kind: Kind,
    /// The first `ninlets` inlet nodes.
    pub inlets: &'a [Node],
    /// All three inlet nodes (the supply plenum reads the first whatever the inlet count).
    pub inlets_all: &'a [Node; 3],
    pub adu: Option<AirDistUnit>,
    pub piu: Option<Piu>,
    pub has_system_node: bool,
    pub zone_mult: i32,
}

/// The sums calcZoneOrSpaceSums sets.
pub struct Sums {
    pub sum_int_gain: f64,
    pub sum_mcp: f64,
    pub sum_mcp_t: f64,
    pub sum_sys_mcp: f64,
    pub sum_sys_mcp_t: f64,
}

/// `InternalHeatGains::zoneSumAllInternalConvectionGains` for the zone's one space.
fn zone_sum_all_internal_convection_gains(devices: &[Device]) -> f64 {
    let mut zone_sum_conv_gain_rate = 0.0;
    if !devices.is_empty() {
        let mut space_sum_conv_gain_rate = 0.0;
        for d in devices {
            space_sum_conv_gain_rate += d.convect_gain_rate;
        }
        zone_sum_conv_gain_rate += space_sum_conv_gain_rate;
    }
    zone_sum_conv_gain_rate
}

/// `InternalHeatGains::zoneSumAllReturnAirConvectionGains` for the zone's one space, return node 0.
fn zone_sum_all_return_air_convection_gains(devices: &[Device]) -> f64 {
    let mut zone_sum_return_air_gain_rate = 0.0;
    if !devices.is_empty() {
        let mut space_sum_return_air_gain_rate = 0.0;
        for d in devices {
            space_sum_return_air_gain_rate += d.return_air_conv_gain_rate;
        }
        zone_sum_return_air_gain_rate += space_sum_return_air_gain_rate;
    }
    zone_sum_return_air_gain_rate
}

/// `calcZoneOrSpaceSums` for the zone (`spaceNum` 0).
pub fn calc_zone_or_space_sums(z: &Zone, corrector_flag: bool) -> Sums {
    let mut sum_sys_mcp = 0.0;
    let mut sum_sys_mcp_t = 0.0;
    let mut sum_int_gain = zone_sum_all_internal_convection_gains(z.devices);
    sum_int_gain += z.sum_conv_ht_rad_sys + z.sum_conv_pool;
    if z.no_heat_to_return_air {
        sum_int_gain += zone_sum_all_return_air_convection_gains(z.devices);
    }

    let [mcpi, mcpv, mcpm, mcpe, mcpc] = z.mcp;
    let [mcpti, mcptv, mcptm, mcpte, mcptc] = z.mcpt;
    let mut sum_mcp = mcpi + mcpv + mcpm + mcpe + mcpc + z.mdot_cp_oa;
    let mut sum_mcp_t = mcpti + mcptv + mcptm + mcpte + mcptc + z.mdot_cp_oa * z.out_dry_bulb_temp;

    if corrector_flag {
        match z.kind {
            Kind::Controlled => {
                for node in z.inlets {
                    let cp_air = PsyCpAirFnW(z.air_hum_rat);
                    let mass_flow_rate_cp_air = node.mass_flow_rate * cp_air;
                    sum_sys_mcp += mass_flow_rate_cp_air;
                    sum_sys_mcp_t += mass_flow_rate_cp_air * node.temp;
                }
            }
            Kind::ReturnPlenum => {
                let air_hum_rat = z.air_hum_rat;
                for node in z.inlets {
                    let mass_flow_rate_cp_air = node.mass_flow_rate * PsyCpAirFnW(air_hum_rat);
                    sum_sys_mcp += mass_flow_rate_cp_air;
                    sum_sys_mcp_t += mass_flow_rate_cp_air * node.temp;
                }
                if let Some(adu) = &z.adu {
                    if adu.up_stream_leak {
                        let mass_flow_rate_cp_air =
                            adu.mass_flow_rate_up_str_lk * PsyCpAirFnW(air_hum_rat);
                        sum_sys_mcp += mass_flow_rate_cp_air;
                        sum_sys_mcp_t += mass_flow_rate_cp_air * adu.inlet_temp;
                    }
                    if adu.down_stream_leak {
                        let mass_flow_rate_cp_air =
                            adu.mass_flow_rate_dn_str_lk * PsyCpAirFnW(air_hum_rat);
                        sum_sys_mcp += mass_flow_rate_cp_air;
                        sum_sys_mcp_t += mass_flow_rate_cp_air * adu.outlet_temp;
                    }
                }
            }
            Kind::SupplyPlenum => {
                // The supply plenum's inlet is the first inlet node, whatever the inlet count.
                let mass_flow_rate = z.inlets_all[0].mass_flow_rate;
                let cp_air = PsyCpAirFnW(z.air_hum_rat);
                sum_sys_mcp += mass_flow_rate * cp_air;
                sum_sys_mcp_t += mass_flow_rate * cp_air * z.inlets_all[0].temp;
            }
            Kind::Uncontrolled => {}
        }

        let zone_mult = f64::from(z.zone_mult);

        if let Some(piu) = &z.piu {
            let cp_air = PsyCpAirFnW(z.air_hum_rat);
            if piu.leak_flow > 0.0 {
                if z.has_system_node {
                    sum_sys_mcp += piu.leak_flow * cp_air;
                    sum_sys_mcp_t += piu.leak_flow * cp_air * piu.pri_air_in_temp;
                } else {
                    sum_mcp += piu.leak_flow * cp_air / zone_mult;
                    sum_mcp_t += piu.leak_flow * cp_air * piu.pri_air_in_temp / zone_mult;
                }
            }
        }

        sum_sys_mcp /= zone_mult;
        sum_sys_mcp_t /= zone_mult;
    }

    // calcSumHAT over a space without heat transfer surfaces adds nothing but 0.0.
    sum_int_gain += 0.0;
    Sums {
        sum_int_gain,
        sum_mcp,
        sum_mcp_t,
        sum_sys_mcp,
        sum_sys_mcp_t,
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

/// A count of 0 to 3, `None` for anything else.
fn count(v: i32) -> Option<usize> {
    usize::try_from(v).ok().filter(|&n| n <= 3)
}

/// Port of `double ZoneSums(...)` (`zonesums.c:8`): see the oracle's shim for the arguments and
/// what `which` picks.
pub fn ZoneSums(
    which: std::ffi::c_int,
    corrector: std::ffi::c_int,
    kind: std::ffi::c_int,
    ndev: std::ffi::c_int,
    cg1: f64,
    rg1: f64,
    cg2: f64,
    rg2: f64,
    cg3: f64,
    rg3: f64,
    radsys: f64,
    pool: f64,
    noheatret: std::ffi::c_int,
    mcpi: f64,
    mcpti: f64,
    mcpv: f64,
    mcptv: f64,
    mcpm: f64,
    mcptm: f64,
    mcpe: f64,
    mcpte: f64,
    mcpc: f64,
    mcptc: f64,
    mdotcpoa: f64,
    outdb: f64,
    airhumrat: f64,
    ninlets: std::ffi::c_int,
    m1: f64,
    t1: f64,
    m2: f64,
    t2: f64,
    m3: f64,
    t3: f64,
    hasadu: std::ffi::c_int,
    upleak: std::ffi::c_int,
    mup: f64,
    tup: f64,
    dnleak: std::ffi::c_int,
    mdn: f64,
    tdn: f64,
    haspiu: std::ffi::c_int,
    leak: f64,
    tpri: f64,
    hasnode: std::ffi::c_int,
    mult: std::ffi::c_int,
    listmult: std::ffi::c_int,
) -> f64 {
    let checked = (|| {
        let flags = [
            flag(corrector)?,
            flag(noheatret)?,
            flag(hasadu)?,
            flag(upleak)?,
            flag(dnleak)?,
            flag(haspiu)?,
            flag(hasnode)?,
        ];
        let kind = match kind {
            0 => Kind::Uncontrolled,
            1 => Kind::Controlled,
            2 => Kind::ReturnPlenum,
            3 => Kind::SupplyPlenum,
            _ => return None,
        };
        let multipliers = (1..=10000).contains(&mult) && (1..=10000).contains(&listmult);
        ((0..=4).contains(&which) && multipliers).then_some(())?;
        Some((flags, kind, count(ndev)?, count(ninlets)?))
    })();
    let Some((
        [
            corrector,
            noheatret,
            hasadu,
            upleak,
            dnleak,
            haspiu,
            hasnode,
        ],
        kind,
        ndev,
        ninlets,
    )) = checked
    else {
        return f64::NAN;
    };
    let dev = |c, r| Device {
        convect_gain_rate: c,
        return_air_conv_gain_rate: r,
    };
    let devices = [dev(cg1, rg1), dev(cg2, rg2), dev(cg3, rg3)];
    let node = |mass_flow_rate, temp| Node {
        mass_flow_rate,
        temp,
    };
    let inlets = [node(m1, t1), node(m2, t2), node(m3, t3)];
    let z = Zone {
        devices: &devices[..ndev],
        sum_conv_ht_rad_sys: radsys,
        sum_conv_pool: pool,
        no_heat_to_return_air: noheatret,
        mcp: [mcpi, mcpv, mcpm, mcpe, mcpc],
        mcpt: [mcpti, mcptv, mcptm, mcpte, mcptc],
        mdot_cp_oa: mdotcpoa,
        out_dry_bulb_temp: outdb,
        air_hum_rat: airhumrat,
        kind,
        inlets: &inlets[..ninlets],
        inlets_all: &inlets,
        adu: hasadu.then_some(AirDistUnit {
            up_stream_leak: upleak,
            mass_flow_rate_up_str_lk: mup,
            inlet_temp: tup,
            down_stream_leak: dnleak,
            mass_flow_rate_dn_str_lk: mdn,
            outlet_temp: tdn,
        }),
        piu: haspiu.then_some(Piu {
            leak_flow: leak,
            pri_air_in_temp: tpri,
        }),
        has_system_node: hasnode,
        zone_mult: mult * listmult,
    };
    let s = calc_zone_or_space_sums(&z, corrector);
    match which {
        0 => s.sum_int_gain,
        1 => s.sum_mcp,
        2 => s.sum_mcp_t,
        3 => s.sum_sys_mcp,
        _ => s.sum_sys_mcp_t,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A controlled office zone, corrector, with two gain devices (300 and 500 W, return air 50 W)
    /// and two supply inlets (0.2 kg/s at 14 C, 0.1 kg/s at 16 C) at humidity ratio 0.008.
    fn office(which: i32, kind: i32, noheatret: i32, haspiu: i32, hasnode: i32, mult: i32) -> f64 {
        ZoneSums(
            which, 1, kind, 2, 300.0, 50.0, 500.0, 0.0, 9.0, 9.0, 100.0, 0.0, noheatret, 20.0, 0.0,
            10.0, 200.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, -2.0, 0.008, 2, 0.2, 14.0, 0.1, 16.0,
            9.0, 9.0, 1, 1, 0.01, 18.0, 0, 0.0, 0.0, haspiu, 0.02, 13.0, hasnode, mult, 1,
        )
    }

    #[test]
    fn internal_gains() {
        assert_eq!(office(0, 1, 0, 0, 1, 1), 900.0);
        assert_eq!(office(0, 1, 1, 0, 1, 1), 950.0);
    }

    #[test]
    fn non_system_air_flow() {
        assert_eq!(office(1, 0, 0, 0, 1, 1), 35.0);
        assert_eq!(office(2, 0, 0, 0, 1, 1), 190.0);
    }

    #[test]
    fn controlled_and_plenum_system_sums() {
        let cp = PsyCpAirFnW(0.008);
        assert_eq!(office(3, 1, 0, 0, 1, 2), (0.2 * cp + 0.1 * cp) / 2.0);
        assert_eq!(office(3, 2, 0, 0, 1, 1), 0.2 * cp + 0.1 * cp + 0.01 * cp);
        assert_eq!(office(4, 3, 0, 0, 1, 1), 0.2 * cp * 14.0);
        assert_eq!(office(3, 0, 0, 0, 1, 1), 0.0);
    }

    #[test]
    fn piu_leakage_goes_to_system_or_zone() {
        let cp = PsyCpAirFnW(0.008);
        assert_eq!(office(3, 0, 0, 1, 1, 1), 0.02 * cp);
        assert_eq!(office(1, 0, 0, 1, 0, 2), 35.0 + 0.02 * cp / 2.0);
    }

    #[test]
    fn guards_return_nan() {
        assert!(office(5, 1, 0, 0, 1, 1).is_nan());
        assert!(office(0, 4, 0, 0, 1, 1).is_nan());
        assert!(office(0, 1, 2, 0, 1, 1).is_nan());
        assert!(office(0, 1, 0, 0, 1, 0).is_nan());
    }
}
