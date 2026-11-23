//! The Rust port of `oscill.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `ZoneTempPredictorCorrector::DetectOscillatingZoneTemp` once its output variables are set up and
//! reported, for up to three zones: each zone's air temperature history shift, the test for three
//! swings of more than `HVAC::OscillateMagnitude` in alternating directions, the oscillation time
//! during occupancy and in the dead band, the facility values and their run period sums. Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]

/// `HVAC::OscillateMagnitude` (C).
const OSCILLATE_MAGNITUDE: f64 = 0.15;

/// One zone: its air temperature history (`ZoneTempHist` rows 1 to 4), the new zone air
/// temperature, whether it is occupied and whether it is in its dead band or set back.
struct Zone {
    hist: [f64; 4],
    zt: f64,
    occupied: bool,
    dead_band: bool,
}

/// One zone's oscillation times.
#[derive(Clone, Copy, Default)]
struct ZoneResult {
    oscillate: f64,
    during_occupancy: f64,
    in_dead_band: f64,
}

/// The facility values (any zone oscillating, during occupancy, in the dead band) and their run
/// period sums.
struct Facility {
    any: [f64; 3],
    annual: [f64; 3],
}

/// `DetectOscillatingZoneTemp`. `comfort` is whether the ASH55 comfort results (and so the zones'
/// occupancy) are allocated.
fn detect_oscillating_zone_temp(
    zones: &mut [Zone],
    results: &mut [ZoneResult],
    comfort: bool,
    tsys: f64,
    annual: [f64; 3],
) -> Facility {
    let neg_oscillate_magnitude = -OSCILLATE_MAGNITUDE;
    let mut any = [false; 3];
    for (z, r) in zones.iter_mut().zip(results.iter_mut()) {
        let mut is_oscillate = false;
        z.hist = [z.zt, z.hist[0], z.hist[1], z.hist[2]];
        let diff34 = z.hist[2] - z.hist[3];
        let diff23 = z.hist[1] - z.hist[2];
        let diff12 = z.hist[0] - z.hist[1];
        if diff12 > OSCILLATE_MAGNITUDE
            && diff23 < neg_oscillate_magnitude
            && diff34 > OSCILLATE_MAGNITUDE
        {
            is_oscillate = true;
        }
        if diff12 < neg_oscillate_magnitude
            && diff23 > OSCILLATE_MAGNITUDE
            && diff34 < neg_oscillate_magnitude
        {
            is_oscillate = true;
        }
        r.during_occupancy = 0.0;
        r.in_dead_band = 0.0;
        if is_oscillate {
            r.oscillate = tsys;
            any[0] = true;
            if comfort && z.occupied {
                r.during_occupancy = tsys;
                any[1] = true;
            }
            if z.dead_band {
                r.in_dead_band = tsys;
                any[2] = true;
            }
        } else {
            r.oscillate = 0.0;
        }
    }
    let any = any.map(|a| if a { tsys } else { 0.0 });
    Facility {
        any,
        annual: [annual[0] + any[0], annual[1] + any[1], annual[2] + any[2]],
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

/// Port of `double Oscillate(int which, int nzones, double tsys, int comfort, ...)` (`oscill.c:8`):
/// see the oracle's shim for the arguments and what `which` picks.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn Oscillate(
    which: std::ffi::c_int,
    nzones: std::ffi::c_int,
    tsys: f64,
    comfort: std::ffi::c_int,
    ha1: f64,
    hb1: f64,
    hc1: f64,
    zt1: f64,
    occ1: std::ffi::c_int,
    db1: std::ffi::c_int,
    ha2: f64,
    hb2: f64,
    hc2: f64,
    zt2: f64,
    occ2: std::ffi::c_int,
    db2: std::ffi::c_int,
    ha3: f64,
    hb3: f64,
    hc3: f64,
    zt3: f64,
    occ3: std::ffi::c_int,
    db3: std::ffi::c_int,
    sum1: f64,
    sum2: f64,
    sum3: f64,
) -> f64 {
    let checked = (|| {
        let flags = [
            flag(comfort)?,
            flag(occ1)?,
            flag(db1)?,
            flag(occ2)?,
            flag(db2)?,
            flag(occ3)?,
            flag(db3)?,
        ];
        let n = usize::try_from(nzones)
            .ok()
            .filter(|n| (1..=3).contains(n))?;
        let which = usize::try_from(which)
            .ok()
            .filter(|&w| w <= 26 && (w >= 21 || w / 7 < n))?;
        Some((flags, n, which))
    })();
    let Some(([comfort, occ1, db1, occ2, db2, occ3, db3], n, which)) = checked else {
        return f64::NAN;
    };
    let zone = |ha, hb, hc, zt, occupied, dead_band| Zone {
        hist: [ha, hb, hc, 0.0],
        zt,
        occupied,
        dead_band,
    };
    let mut zones = [
        zone(ha1, hb1, hc1, zt1, occ1, db1),
        zone(ha2, hb2, hc2, zt2, occ2, db2),
        zone(ha3, hb3, hc3, zt3, occ3, db3),
    ];
    let mut results = [ZoneResult::default(); 3];
    let f = detect_oscillating_zone_temp(
        &mut zones[..n],
        &mut results[..n],
        comfort,
        tsys,
        [sum1, sum2, sum3],
    );
    if which >= 24 {
        return f.annual[which - 24];
    }
    if which >= 21 {
        return f.any[which - 21];
    }
    let (z, r) = (&zones[which / 7], &results[which / 7]);
    match which % 7 {
        s @ 0..=3 => z.hist[s],
        4 => r.oscillate,
        5 => r.during_occupancy,
        _ => r.in_dead_band,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Zone 1 swinging up, down and up by `d`, occupied and in its dead band; zones 2 and 3 steady.
    fn swing(which: i32, d: f64, comfort: i32) -> f64 {
        Oscillate(
            which,
            3,
            0.25,
            comfort,
            21.0,
            21.0 + d,
            21.0,
            21.0 + d,
            1,
            1,
            22.0,
            22.0,
            22.0,
            22.0,
            0,
            0,
            23.0,
            23.0,
            23.0,
            23.0,
            0,
            0,
            1.0,
            2.0,
            3.0,
        )
    }

    #[test]
    fn oscillation_needs_three_large_swings() {
        // History after the shift: 21.3, 21, 21.3, 21: up, down, up by 0.3.
        assert_eq!(
            [swing(0, 0.3, 1), swing(1, 0.3, 1), swing(3, 0.3, 1)],
            [21.3, 21.0, 21.0]
        );
        assert_eq!(
            [swing(4, 0.3, 1), swing(5, 0.3, 1), swing(6, 0.3, 1)],
            [0.25, 0.25, 0.25]
        );
        assert_eq!(
            [swing(21, 0.3, 1), swing(24, 0.3, 1), swing(26, 0.3, 1)],
            [0.25, 1.25, 3.25]
        );
        assert_eq!(swing(5, 0.3, 0), 0.0);
        assert_eq!(swing(4, 0.1, 1), 0.0);
        assert_eq!(swing(11, 0.3, 1), 0.0);
    }

    #[test]
    fn guards() {
        assert!(swing(27, 0.3, 1).is_nan());
        assert!(swing(0, 0.3, 2).is_nan());
        let past = Oscillate(
            7, 1, 0.25, 0, 21.0, 21.0, 21.0, 21.0, 0, 0, 0.0, 0.0, 0.0, 0.0, 0, 0, 0.0, 0.0, 0.0,
            0.0, 0, 0, 0.0, 0.0, 0.0,
        );
        assert!(past.is_nan());
    }
}
