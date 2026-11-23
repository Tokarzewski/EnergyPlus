//! The Rust port of `outsurf.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatBalanceSurfaceManager::CalcOutsideSurfTemp` for one exterior surface (not a tubular
//! daylighting device dome): the boundary temperatures (sky, ground, surrounding surfaces), the
//! outside face temperature from the heat balance case the surface falls in (slow or quick
//! conduction, with or without movable insulation, an other side conditions model or an embedded
//! source or sink), the outside radiation report, and the radiant system coefficients. Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

/// The first CTF terms of the surface's construction.
struct Ctf {
    cross: f64,
    inside: f64,
    outside: f64,
    source_out: f64,
    source_in: f64,
    /// `SourceSinkPresent`.
    source_sink: bool,
}

/// The surface terms `CalcOutsideSurfTemp` reads.
struct Surface {
    ctf: Ctf,
    h_conv_int: f64,
    h_conv_ext: f64,
    h_air_ext: f64,
    h_sky_ext: f64,
    h_grd_ext: f64,
    h_srd_surf_ext: f64,
    q_additional_heat_source_outside: f64,
    ctf_const_out_part: f64,
    ctf_const_in_part: f64,
    opaq_q_rad_sw_out_abs: f64,
    opaq_q_rad_sw_in_abs: f64,
    temp_in: f64,
    q_src_hist: f64,
    q_dot_rad_int_gains_in_per_area: f64,
    mat: f64,
    q_dot_rad_net_lw_in_per_area: f64,
    q_rad_sw_out_mv_ins: f64,
    /// The other side conditions model's radiation temperature and coefficient, when it has one.
    oscm: Option<(f64, f64)>,
}

/// What the call leaves behind.
struct Outside {
    th11: f64,
    q_dot_rad_out_rep_per_area: f64,
    /// `RadSysToHBConstCoef`, `RadSysToHBTinCoef` and `RadSysToHBQsrcCoef`, for a surface with a
    /// source or sink.
    rad_sys: Option<[f64; 3]>,
}

impl Surface {
    /// `CalcOutsideSurfTemp` with movable insulation coefficient `h_mov_insul` and boundary
    /// temperatures `temp_ext`, `t_sky`, `t_ground` and `t_srd_surfs`.
    fn calc_outside_surf_temp(
        &self,
        h_mov_insul: f64,
        temp_ext: f64,
        t_sky: f64,
        t_ground: f64,
        t_srd_surfs: f64,
    ) -> Outside {
        let c = &self.ctf;
        let mov_insul_present = h_mov_insul > 0.0;
        let quick_conduction_surf = c.cross > 0.01;
        let f1 = if quick_conduction_surf {
            c.cross / (c.inside + self.h_conv_int)
        } else {
            0.0
        };
        let (hce, hae, hsk, hgr, hsr) = (
            self.h_conv_ext,
            self.h_air_ext,
            self.h_sky_ext,
            self.h_grd_ext,
            self.h_srd_surf_ext,
        );
        let cco = self.ctf_const_out_part;
        let qadd = self.q_additional_heat_source_outside;
        let src = c.source_out * self.q_src_hist;
        // The inside terms a quick conduction surface brings in.
        let inside = |src_in: f64| {
            if c.source_sink && self.oscm.is_some() {
                self.ctf_const_in_part
                    + self.opaq_q_rad_sw_in_abs
                    + self.q_dot_rad_int_gains_in_per_area
                    + src_in
                    + self.h_conv_int * self.mat
                    + self.q_dot_rad_net_lw_in_per_area
            } else {
                self.ctf_const_in_part
                    + self.opaq_q_rad_sw_in_abs
                    + self.q_dot_rad_int_gains_in_per_area
                    + self.h_conv_int * self.mat
                    + self.q_dot_rad_net_lw_in_per_area
            }
        };
        let th11 = if !mov_insul_present {
            match self.oscm {
                None => {
                    let sum = -cco
                        + self.opaq_q_rad_sw_out_abs
                        + hsr * t_srd_surfs
                        + (hce + hae) * temp_ext
                        + qadd
                        + hsk * t_sky
                        + hgr * t_ground;
                    let den = c.outside + hce + hae + hsk + hgr + hsr;
                    if quick_conduction_surf {
                        let sum = if c.source_sink { sum + src } else { sum };
                        (sum + f1 * inside(0.0)) / (den - f1 * c.cross)
                    } else {
                        let sum = sum + c.cross * self.temp_in;
                        let sum = if c.source_sink { sum + src } else { sum };
                        sum / den
                    }
                }
                Some((rad_temp, h_rad)) => {
                    let sum = -cco + hce * temp_ext + qadd + h_rad * rad_temp;
                    let den = c.outside + hce + h_rad;
                    if quick_conduction_surf {
                        let sum = if c.source_sink { sum + src } else { sum };
                        (sum + f1 * inside(c.source_in * self.q_src_hist)) / (den - f1 * c.cross)
                    } else {
                        let sum = sum + c.cross * self.temp_in;
                        let sum = if c.source_sink { sum + src } else { sum };
                        sum / den
                    }
                }
            }
        } else {
            let f2 = h_mov_insul / (h_mov_insul + hce + hae + hsk + hgr + hsr);
            let outer = self.q_rad_sw_out_mv_ins
                + (hce + hae) * temp_ext
                + qadd
                + hsk * t_sky
                + hgr * t_ground
                + hsr * t_srd_surfs;
            if quick_conduction_surf {
                (-cco + self.opaq_q_rad_sw_out_abs + f1 * inside(0.0) + f2 * outer)
                    / (c.outside + h_mov_insul - f2 * h_mov_insul - f1 * c.cross)
            } else {
                (-cco + self.opaq_q_rad_sw_out_abs + c.cross * self.temp_in + f2 * outer)
                    / (c.outside + h_mov_insul - f2 * h_mov_insul)
            }
        };

        let q_dot_rad_out_rep_per_area = -(hsk * (th11 - t_sky)
            + hae * (th11 - temp_ext)
            + hgr * (th11 - t_ground)
            + hsr * (th11 - t_srd_surfs));

        let rad_sys = c.source_sink.then(|| {
            let rad_sys_div = 1.0 / (c.outside + hce + hae + hsk + hgr + hsr + hsr);
            [
                (-cco
                    + self.opaq_q_rad_sw_out_abs
                    + hsr * t_srd_surfs
                    + (hce + hae) * temp_ext
                    + hsk * t_sky
                    + hgr * t_ground)
                    * rad_sys_div,
                c.cross * rad_sys_div,
                c.source_out * rad_sys_div,
            ]
        });
        Outside {
            th11,
            q_dot_rad_out_rep_per_area,
            rad_sys,
        }
    }
}

/// One result of `CalcOutsideSurfTemp` for one exterior wall (see the oracle's shim for the
/// arguments and `which`).
pub fn OutsideSurfTemp(
    which: i32,
    ctfcross: f64,
    ctfinside: f64,
    ctfoutside: f64,
    ctfsourceout: f64,
    ctfsourcein: f64,
    sourcesink: i32,
    hconvint: f64,
    hmovinsul: f64,
    tempext: f64,
    skytemp: f64,
    outdrybulb: f64,
    hassrd: i32,
    srdskyon: i32,
    srdskyval: f64,
    srdgndon: i32,
    srdgndval: f64,
    srdsurftemp: f64,
    usegndsurf: i32,
    gndsurftavg: f64,
    oscm: i32,
    oscmtrad: f64,
    oscmhrad: f64,
    hconvext: f64,
    hairext: f64,
    hskyext: f64,
    hgrdext: f64,
    hsrdsurfext: f64,
    qaddout: f64,
    ctfconstout: f64,
    qradswoutabs: f64,
    tempin: f64,
    qsrchist: f64,
    ctfconstin: f64,
    qradswinabs: f64,
    qdotradintgainsin: f64,
    mat: f64,
    qdotradnetlwin: f64,
    qradswoutmvins: f64,
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    if !(0..=4).contains(&which)
        || ![sourcesink, hassrd, srdskyon, srdgndon, usegndsurf, oscm]
            .into_iter()
            .all(flag)
        || (sourcesink == 1 && hmovinsul > 0.0)
    {
        return f64::NAN;
    }
    // The boundary temperatures.
    let mut t_sky = skytemp;
    let mut t_ground = outdrybulb;
    let mut t_srd_surfs = 0.0;
    if hassrd == 1 {
        if srdskyon == 1 {
            t_sky = srdskyval;
        }
        if srdgndon == 1 {
            t_ground = srdgndval;
        }
        t_srd_surfs = srdsurftemp;
    }
    if usegndsurf == 1 {
        t_ground = gndsurftavg;
    }
    let s = Surface {
        ctf: Ctf {
            cross: ctfcross,
            inside: ctfinside,
            outside: ctfoutside,
            source_out: ctfsourceout,
            source_in: ctfsourcein,
            source_sink: sourcesink == 1,
        },
        h_conv_int: hconvint,
        h_conv_ext: hconvext,
        h_air_ext: hairext,
        h_sky_ext: hskyext,
        h_grd_ext: hgrdext,
        h_srd_surf_ext: hsrdsurfext,
        q_additional_heat_source_outside: qaddout,
        ctf_const_out_part: ctfconstout,
        ctf_const_in_part: ctfconstin,
        opaq_q_rad_sw_out_abs: qradswoutabs,
        opaq_q_rad_sw_in_abs: qradswinabs,
        temp_in: tempin,
        q_src_hist: qsrchist,
        q_dot_rad_int_gains_in_per_area: qdotradintgainsin,
        mat,
        q_dot_rad_net_lw_in_per_area: qdotradnetlwin,
        q_rad_sw_out_mv_ins: qradswoutmvins,
        oscm: (oscm == 1).then_some((oscmtrad, oscmhrad)),
    };
    let o = s.calc_outside_surf_temp(hmovinsul, tempext, t_sky, t_ground, t_srd_surfs);
    match which {
        0 => o.th11,
        1 => o.q_dot_rad_out_rep_per_area,
        _ => o
            .rad_sys
            .map_or(0.0, |r| r[usize::try_from(which - 2).unwrap_or(0)]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A slow conducting wall with only outside convection (h = 10) to 5 C and the CTF outside
    /// term 2 with constant part `cco`, and cross term `cross` to a 20 C inside face.
    fn wall(which: i32, cross: f64, cco: f64) -> f64 {
        OutsideSurfTemp(
            which, cross, 3.0, 2.0, 0.0, 0.0, 0, 3.0, 0.0, 5.0, -10.0, 5.0, 0, 0, 0.0, 0, 0.0, 0.0,
            0, 0.0, 0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0, cco, 0.0, 20.0, 0.0, 0.0, 0.0, 0.0,
            21.0, 0.0, 0.0,
        )
    }

    #[test]
    fn slow_conduction_heat_balance() {
        // (-(-10) + 10 * 5 + 0.005 * 20) / (2 + 10).
        assert_eq!(wall(0, 0.005, -10.0), (10.0 + 50.0 + 0.005 * 20.0) / 12.0);
        // No radiation coefficients: no radiation report, and no radiant system coefficients.
        assert_eq!(wall(1, 0.005, -10.0), 0.0);
        assert_eq!(wall(2, 0.005, -10.0), 0.0);
    }

    #[test]
    fn quick_conduction_brings_in_the_inside_face() {
        let f1 = 1.0 / (3.0 + 3.0);
        let want = (10.0 + 50.0 + f1 * (3.0 * 21.0)) / (12.0 - f1 * 1.0);
        assert_eq!(wall(0, 1.0, -10.0), want);
    }

    #[test]
    fn guards_return_nan() {
        assert!(wall(5, 0.0, 0.0).is_nan());
        let src_with_insulation = OutsideSurfTemp(
            0, 0.0, 3.0, 2.0, 0.1, 0.1, 1, 3.0, 1.0, 5.0, -10.0, 5.0, 0, 0, 0.0, 0, 0.0, 0.0, 0,
            0.0, 0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 0.0, 0.0, 0.0,
            21.0, 0.0, 0.0,
        );
        assert!(src_with_insulation.is_nan());
    }
}
