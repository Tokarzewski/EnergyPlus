//! The Rust port of `eql.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Plain-number helpers from EnergyPlus `WindowEquivalentLayer.cc` (namespace
//! `WindowEquivalentLayer`): insect-screen openness, venetian slat geometry, gap convection
//! (Rayleigh and Nusselt numbers), radiation coefficients, and a float comparison. Every formula
//! keeps the C++ operation order with no FMA. Nothing here panics.

#![forbid(unsafe_code)]

/// `Constant::Gravity` and `Constant::StefanBoltzmann`, as `DataGlobalConstants.hh` has them.
const GRAVITY: f64 = 9.807;
const STEFAN_BOLTZMANN: f64 = 5.6697E-8;

/// ObjexxFCL `max` and `min`: `a < b ? b : a` and `a < b ? a : b`.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `root_4`: `sqrt(sqrt(x))`.
fn root_4(x: f64) -> f64 {
    x.sqrt().sqrt()
}

/// Port of `IS_OPENNESS` (`WindowEquivalentLayer.cc:1259`): the openness of an insect screen of
/// wire diameter `D` and spacing `S`.
pub fn IS_OPENNESS(D: f64, S: f64) -> f64 {
    if S > 0.0 {
        let r = max(S - D, 0.0) / S;
        r * r
    } else {
        0.0
    }
}

/// Port of `IS_DSRATIO` (`WindowEquivalentLayer.cc:1274`): wire diameter over spacing.
pub fn IS_DSRATIO(OPENNESS: f64) -> f64 {
    if OPENNESS > 0.0 {
        1.0 - min(OPENNESS.sqrt(), 1.0)
    } else {
        0.0
    }
}

/// Port of `VB_SLAT_RADIUS_RATIO` (`WindowEquivalentLayer.cc:3597`): slat width over the radius
/// of its crown arc.
pub fn VB_SLAT_RADIUS_RATIO(W: f64, C: f64) -> f64 {
    if C <= 0.0 || W <= 0.0 {
        return 0.0;
    }
    let cx = min(C, W / 2.001);
    2.0 * W * cx / (cx * cx + W * W / 4.0)
}

/// Port of `FRA` (`WindowEquivalentLayer.cc:5748`): the Rayleigh number of a gap. As in the C++,
/// the quadratic terms of the heat capacity and viscosity use `BCP` and `BVISC`; `CCP` and
/// `CVISC` are unused.
#[expect(
    clippy::too_many_arguments,
    reason = "the C++ signature takes thirteen numbers"
)]
pub fn FRA(
    TM: f64,
    T: f64,
    DT: f64,
    AK: f64,
    BK: f64,
    CK: f64,
    ACP: f64,
    BCP: f64,
    CCP: f64,
    AVISC: f64,
    BVISC: f64,
    CVISC: f64,
    RHOGAS: f64,
) -> f64 {
    let _ = (CCP, CVISC);
    let z = 1.0;
    let k = AK + BK * TM + CK * TM * TM;
    let cp = ACP + BCP * TM + BCP * TM * TM;
    let visc = AVISC + BVISC * TM + BVISC * TM * TM;
    (GRAVITY * RHOGAS * RHOGAS * DT * T * T * T * cp) / (visc * k * TM * z * z)
}

/// Port of `FNU` (`WindowEquivalentLayer.cc:5787`): the Nusselt number of a gap, three fits by
/// Rayleigh number.
pub fn FNU(RA: f64) -> f64 {
    let ara = RA.abs();
    if ara <= 10000.0 {
        1.0 + 1.75967e-10 * ara.powf(2.2984755)
    } else if ara <= 50000.0 {
        0.028154 * ara.powf(0.413993)
    } else {
        0.0673838 * ara.powf(1.0 / 3.0)
    }
}

/// Port of `HRadPar` (`WindowEquivalentLayer.cc:5842`): the radiative coefficient between two
/// parallel surfaces, 0 when either emittance is at most 0.001.
pub fn HRadPar(T1: f64, T2: f64, E1: f64, E2: f64) -> f64 {
    if E1 > 0.001 && E2 > 0.001 {
        let dv = (1.0 / E1) + (1.0 / E2) - 1.0;
        (STEFAN_BOLTZMANN / dv) * (T1 + T2) * (T1 * T1 + T2 * T2)
    } else {
        0.0
    }
}

/// Port of `HIC_ASHRAE` (`WindowEquivalentLayer.cc:5872`): the ASHRAE indoor convection
/// coefficient for a surface of height `L`.
pub fn HIC_ASHRAE(L: f64, TG: f64, TI: f64) -> f64 {
    1.46 * root_4((TG - TI).abs() / max(L, 0.001))
}

/// Port of `VB_CriticalSlatAngle` (`WindowEquivalentLayer.cc:7392`).
pub fn VB_CriticalSlatAngle(OMEGA_DEG: f64) -> f64 {
    90.0 - OMEGA_DEG
}

/// Port of `FEQX` (`WindowEquivalentLayer.cc:7857`): whether two values are equal within an
/// absolute tolerance (at least 1e-10) or a relative one.
pub fn FEQX(a: f64, b: f64, tolF: f64, tolAbs: f64) -> bool {
    let d = (a - b).abs();
    d < max(tolAbs, 1e-10) || (2.0 * d / (a.abs() + b.abs())) < tolF
}

/// Port of `TRadC` (`WindowEquivalentLayer.cc:7878`): the radiant temperature of a radiosity [C].
pub fn TRadC(J: f64, Emiss: f64) -> f64 {
    root_4(J / (STEFAN_BOLTZMANN * max(Emiss, 0.001))) - 273.15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        assert_eq!(
            (
                IS_OPENNESS(0.001, 0.0),
                IS_DSRATIO(1.5),
                VB_SLAT_RADIUS_RATIO(0.0, 1.0)
            ),
            (0.0, 0.0, 0.0)
        );
        assert_eq!((HRadPar(300.0, 300.0, 0.001, 0.9), FNU(0.0)), (0.0, 1.0));
        assert!(FEQX(1.0, 1.0, 0.0, 0.0) && !FEQX(1.0, 2.0, 1e-6, 0.0));
    }
}
