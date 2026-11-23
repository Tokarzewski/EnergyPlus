//! The Rust port of `outsidetemp.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatBalanceSurfaceManager::CalcOutsideSurfTemp` for a heat transfer surface (not a tubular
//! daylighting dome): the outside face heat balance, picked by whether exterior movable insulation
//! is present and whether the construction conducts "quickly" (cross CTF term above 0.01, so the
//! inside heat balance is folded in), each with or without an other side conditions model and an
//! embedded source or sink; then the linearized radiation for reporting and, for a source or sink,
//! the radiant system coefficients. Each sum keeps the C++ order of terms. The oracle's guards
//! (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

/// One surface's terms, named after the state they come from.
struct Terms {
    cross0: f64,
    inside0: f64,
    outside0: f64,
    srcout0: f64,
    srcin0: f64,
    src: bool,
    hconvint: f64,
    hconvext: f64,
    hairext: f64,
    hskyext: f64,
    hgrdext: f64,
    hsrdext: f64,
    ctfout: f64,
    ctfin: f64,
    qswout: f64,
    qswin: f64,
    qsrc: f64,
    tempin: f64,
    qaddout: f64,
    qintgains: f64,
    qnetlwin: f64,
    mat: f64,
    qswmvins: f64,
    /// Sky, ground and surrounding surfaces temperatures as `CalcOutsideSurfTemp` settles them.
    tsky: f64,
    tground: f64,
    tsrd: f64,
    /// The other side conditions model's radiation temperature and coefficient, when there is one.
    osc: Option<(f64, f64)>,
}

/// What `CalcOutsideSurfTemp` sets: the outside face temperature, `SurfQdotRadOutRepPerArea`
/// and, for a source or sink, `RadSysToHBConstCoef`, `RadSysToHBTinCoef` and `RadSysToHBQsrcCoef`.
struct Outside {
    th11: f64,
    qrad_rep: f64,
    radsys: [f64; 3],
}

/// `CalcOutsideSurfTemp` for movable insulation coefficient `hmov` and exterior temperature `text`.
fn calc_outside_surf_temp(s: &Terms, hmov: f64, text: f64) -> Outside {
    let mov_insul_present = hmov > 0.0;
    let mut f1 = 0.0;
    let quick = s.cross0 > 0.01;
    if quick {
        f1 = s.cross0 / (s.inside0 + s.hconvint);
    }
    let (tsky, tground, tsrd) = (s.tsky, s.tground, s.tsrd);
    let th11 = if !mov_insul_present {
        match (s.osc, quick) {
            (None, false) => {
                let mut num = -s.ctfout
                    + s.qswout
                    + s.hsrdext * tsrd
                    + (s.hconvext + s.hairext) * text
                    + s.qaddout
                    + s.hskyext * tsky
                    + s.hgrdext * tground
                    + s.cross0 * s.tempin;
                if s.src {
                    num += s.srcout0 * s.qsrc;
                }
                num / (s.outside0 + s.hconvext + s.hairext + s.hskyext + s.hgrdext + s.hsrdext)
            }
            (Some((rad_temp, hrad)), false) => {
                let mut num = -s.ctfout
                    + s.hconvext * text
                    + s.qaddout
                    + hrad * rad_temp
                    + s.cross0 * s.tempin;
                if s.src {
                    num += s.srcout0 * s.qsrc;
                }
                num / (s.outside0 + s.hconvext + hrad)
            }
            (None, true) => {
                let mut num = -s.ctfout
                    + s.qswout
                    + s.hsrdext * tsrd
                    + (s.hconvext + s.hairext) * text
                    + s.qaddout
                    + s.hskyext * tsky
                    + s.hgrdext * tground;
                if s.src {
                    num += s.srcout0 * s.qsrc;
                }
                (num + f1 * (s.ctfin + s.qswin + s.qintgains + s.hconvint * s.mat + s.qnetlwin))
                    / (s.outside0 + s.hconvext + s.hairext + s.hskyext + s.hgrdext + s.hsrdext
                        - f1 * s.cross0)
            }
            (Some((rad_temp, hrad)), true) => {
                let mut num = -s.ctfout + s.hconvext * text + s.qaddout + hrad * rad_temp;
                let folded = if s.src {
                    num += s.srcout0 * s.qsrc;
                    s.ctfin
                        + s.qswin
                        + s.qintgains
                        + s.srcin0 * s.qsrc
                        + s.hconvint * s.mat
                        + s.qnetlwin
                } else {
                    s.ctfin + s.qswin + s.qintgains + s.hconvint * s.mat + s.qnetlwin
                };
                (num + f1 * folded) / (s.outside0 + s.hconvext + hrad - f1 * s.cross0)
            }
        }
    } else {
        let f2 = hmov / (hmov + s.hconvext + s.hairext + s.hskyext + s.hgrdext + s.hsrdext);
        let outside = s.qswmvins
            + (s.hconvext + s.hairext) * text
            + s.qaddout
            + s.hskyext * tsky
            + s.hgrdext * tground
            + s.hsrdext * tsrd;
        if quick {
            (-s.ctfout
                + s.qswout
                + f1 * (s.ctfin + s.qswin + s.qintgains + s.hconvint * s.mat + s.qnetlwin)
                + f2 * outside)
                / (s.outside0 + hmov - f2 * hmov - f1 * s.cross0)
        } else {
            (-s.ctfout + s.qswout + s.cross0 * s.tempin + f2 * outside)
                / (s.outside0 + hmov - f2 * hmov)
        }
    };
    let qrad_rep = -(s.hskyext * (th11 - tsky)
        + s.hairext * (th11 - text)
        + s.hgrdext * (th11 - tground)
        + s.hsrdext * (th11 - tsrd));
    let mut radsys = [0.0; 3];
    if s.src {
        let div = 1.0
            / (s.outside0 + s.hconvext + s.hairext + s.hskyext + s.hgrdext + s.hsrdext + s.hsrdext);
        radsys[0] = (-s.ctfout
            + s.qswout
            + s.hsrdext * tsrd
            + (s.hconvext + s.hairext) * text
            + s.hskyext * tsky
            + s.hgrdext * tground)
            * div;
        radsys[1] = s.cross0 * div;
        radsys[2] = s.srcout0 * div;
    }
    Outside {
        th11,
        qrad_rep,
        radsys,
    }
}

/// The outside face temperature of surface 1 and what `CalcOutsideSurfTemp` sets with it (see
/// the oracle's shim for the arguments and `which`).
pub fn OutsideSurfTemp(
    which: i32,
    cross0: f64,
    inside0: f64,
    outside0: f64,
    srcout0: f64,
    srcin0: f64,
    src: i32,
    hconvint: f64,
    hconvext: f64,
    hairext: f64,
    hskyext: f64,
    hgrdext: f64,
    hsrdext: f64,
    ctfout: f64,
    ctfin: f64,
    qswout: f64,
    qswin: f64,
    qsrc: f64,
    tempin: f64,
    qaddout: f64,
    qintgains: f64,
    qnetlwin: f64,
    mat: f64,
    qswmvins: f64,
    hmov: f64,
    text: f64,
    skytemp: f64,
    odb: f64,
    hassrd: i32,
    srdtemp: f64,
    usegnd: i32,
    gndtemp: f64,
    osc: i32,
    osctrad: f64,
    oschrad: f64,
) -> f64 {
    let flag = |v: i32| (0..=1).contains(&v);
    if !(flag(src) && flag(hassrd) && flag(usegnd) && flag(osc)) || (src == 1 && hmov > 0.0) {
        return f64::NAN;
    }
    // The sky and ground temperatures from the weather, overridden as CalcOutsideSurfTemp does: a
    // surrounding surfaces list without schedules leaves them and brings its own temperature.
    let tsrd = if hassrd == 1 { srdtemp } else { 0.0 };
    let tground = if usegnd == 1 { gndtemp } else { odb };
    let s = Terms {
        cross0,
        inside0,
        outside0,
        srcout0,
        srcin0,
        src: src == 1,
        hconvint,
        hconvext,
        hairext,
        hskyext,
        hgrdext,
        hsrdext,
        ctfout,
        ctfin,
        qswout,
        qswin,
        qsrc,
        tempin,
        qaddout,
        qintgains,
        qnetlwin,
        mat,
        qswmvins,
        tsky: skytemp,
        tground,
        tsrd,
        osc: (osc == 1).then_some((osctrad, oschrad)),
    };
    let o = calc_outside_surf_temp(&s, hmov, text);
    match which {
        0 => o.th11,
        1 => o.qrad_rep,
        2..=4 => o.radsys[(which - 2) as usize],
        5 => 0.0,
        _ => f64::NAN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A wall with cross term `cross0`, both other CTF terms 2, inside convection 3, outside
    /// convection 10, sky 1 and nothing else, outside air at 0 C, sky at -10 C, inside face at
    /// 20 C, zone air at 20 C, no history.
    fn wall(which: i32, cross0: f64, hmov: f64) -> f64 {
        OutsideSurfTemp(
            which, cross0, 2.0, 2.0, 0.0, 0.0, 0, 3.0, 10.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 20.0, 0.0, 0.0, 0.0, 20.0, 0.0, hmov, 0.0, -10.0, 0.0, 0, 0.0, 0, 0.0, 0,
            0.0, 0.0,
        )
    }

    #[test]
    fn slow_wall_balances_conduction_against_air_and_sky() {
        // (10 * 0 + 1 * -10 + 0.005 * 20) / (2 + 10 + 1)
        let t = wall(0, 0.005, 0.0);
        assert!((t - (-10.0 + 0.1) / 13.0).abs() < 1e-12, "{t}");
        // Reported radiation: -(1 * (t + 10)).
        assert!((wall(1, 0.005, 0.0) + (t + 10.0)).abs() < 1e-12);
    }

    #[test]
    fn quick_wall_folds_in_the_inside_balance() {
        // F1 = 1 / (2 + 3); inside: 3 * 20 = 60, so (-10 + 0.2 * 60) / (13 - 0.2 * 1).
        let t = wall(0, 1.0, 0.0);
        assert!((t - (-10.0 + 12.0) / 12.8).abs() < 1e-12, "{t}");
    }

    #[test]
    fn movable_insulation_shelters_the_face() {
        // F2 = 1 / (1 + 10 + 1); (0.005 * 20 + F2 * -10) / (2 + 1 - F2).
        let f2 = 1.0 / 12.0;
        let t = wall(0, 0.005, 1.0);
        assert!((t - (0.1 + f2 * -10.0) / (3.0 - f2)).abs() < 1e-12, "{t}");
    }

    #[test]
    fn no_source_leaves_radiant_coefficients_at_zero() {
        assert_eq!(wall(2, 1.0, 0.0), 0.0);
        assert_eq!(wall(5, 1.0, 0.0), 0.0);
    }

    #[test]
    fn guards_return_nan() {
        assert!(wall(9, 1.0, 0.0).is_nan());
        let t = OutsideSurfTemp(
            0, 1.0, 2.0, 2.0, 0.5, 0.5, 1, 3.0, 10.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            20.0, 0.0, 0.0, 0.0, 20.0, 0.0, 1.0, 0.0, -10.0, 0.0, 0, 0.0, 0, 0.0, 0, 0.0, 0.0,
        );
        assert!(t.is_nan());
    }
}
