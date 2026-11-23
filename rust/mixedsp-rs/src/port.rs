//! The Rust port of `mixedsp.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `SetPointManager::SPMMixedAir::calculate`: the mixed air set point is the reference node's set
//! point less the fan's temperature rise; with a cooling coil that is cooling while it is colder
//! outdoors than the coil's minimum outlet temperature, the freeze check raises it so the coil
//! outlet stays at that minimum, for blow through and draw through layouts. Every expression keeps
//! the C++ order, and `max` is ObjexxFCL's. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

/// ObjexxFCL `max(a, b)`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The nodes and the outdoor temperature the manager reads.
struct Inputs {
    ref_set_point: f64,
    ref_temp: f64,
    fan_in: f64,
    fan_out: f64,
    /// The cooling coil's inlet and outlet temperatures, if the manager names its nodes.
    coil: Option<(f64, f64)>,
    /// The reference node is the coil's outlet node.
    ref_is_coil_out: bool,
    min_cool_coil_out_temp: f64,
    out_dry_bulb: f64,
}

/// `SPMMixedAir::calculate`: the set point and whether the freeze check is on.
fn calculate(x: &Inputs) -> (f64, bool) {
    let mut set_pt = x.ref_set_point - (x.fan_out - x.fan_in);
    let mut freeze_check = false;
    if let Some((coil_in, coil_out)) = x.coil {
        let dt_fan = x.fan_out - x.fan_in;
        let dt_cool_coil = coil_in - coil_out;
        if dt_cool_coil > 0.0 && x.min_cool_coil_out_temp > x.out_dry_bulb {
            freeze_check = true;
            if x.ref_temp == coil_out {
                set_pt = max(x.ref_set_point, x.min_cool_coil_out_temp) - dt_fan + dt_cool_coil;
            } else if !x.ref_is_coil_out {
                set_pt = max(x.ref_set_point - dt_fan, x.min_cool_coil_out_temp) + dt_cool_coil;
            } else {
                set_pt = max(x.ref_set_point, x.min_cool_coil_out_temp) + dt_cool_coil;
            }
        }
    }
    (set_pt, freeze_check)
}

/// Port of `double MixedSP(int which, int layout, double refsp, double reft, double fanin, double
/// fanout, double coilin, double coilout, double mincoil, double oat)` (`mixedsp.c:9`): see the
/// oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn MixedSP(
    which: std::ffi::c_int,
    layout: std::ffi::c_int,
    refsp: f64,
    reft: f64,
    fanin: f64,
    fanout: f64,
    coilin: f64,
    coilout: f64,
    mincoil: f64,
    oat: f64,
) -> f64 {
    let ts = [refsp, reft, fanin, fanout, coilin, coilout, mincoil, oat];
    if !(0..=1).contains(&which)
        || !(0..=3).contains(&layout)
        || !ts
            .iter()
            .all(|&t| t.is_finite() && (-60.0..=80.0).contains(&t))
    {
        return f64::NAN;
    }
    let ref_is_coil_out = layout == 3;
    let (set_pt, freeze_check) = calculate(&Inputs {
        ref_set_point: refsp,
        // The reference node is the coil's outlet node in layout 3.
        ref_temp: if ref_is_coil_out { coilout } else { reft },
        fan_in: fanin,
        fan_out: fanout,
        coil: (layout != 0).then_some((coilin, coilout)),
        ref_is_coil_out,
        min_cool_coil_out_temp: mincoil,
        out_dry_bulb: oat,
    });
    if which == 0 {
        set_pt
    } else {
        f64::from(u8::from(freeze_check))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fan_rise_comes_off_the_set_point() {
        assert_eq!(
            MixedSP(0, 0, 13.0, 13.0, 12.0, 13.0, 0.0, 0.0, 7.2, 20.0),
            12.0
        );
        assert_eq!(
            MixedSP(1, 1, 13.0, 13.0, 12.0, 13.0, 20.0, 12.0, 7.2, 20.0),
            0.0
        );
        assert_eq!(
            MixedSP(0, 2, 5.0, 9.0, 12.0, 13.0, 15.0, 6.0, 7.2, 2.0),
            7.2 + 9.0
        );
        assert_eq!(
            MixedSP(1, 2, 5.0, 9.0, 12.0, 13.0, 15.0, 6.0, 7.2, 2.0),
            1.0
        );
        assert!(MixedSP(2, 0, 13.0, 13.0, 12.0, 13.0, 0.0, 0.0, 7.2, 20.0).is_nan());
    }
}
