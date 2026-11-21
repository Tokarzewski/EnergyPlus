//! The Rust port of `sandia.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Each function is the EnergyPlus Sandia photovoltaic module equation of the same name
//! (namespace `Photovoltaics`, `Photovoltaics.cc`, from King et al., SAND2004-3535). Every
//! formula keeps the C++ operation order with no FMA; `exp`, `ln`, `cos` and `powf` call the
//! same C library functions as `std::exp`, `std::log`, `std::cos` and `std::pow`. C has no error
//! channel: nothing panics, and bad inputs give the same infinity or NaN the C++ gives.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the C signatures pass every module coefficient as its own double"
)]

/// `Constant::Kelvin`.
const KELVIN: f64 = 273.15;
/// `Constant::DegToRad`, `Pi / 180.0` with `Pi = 3.14159265358979324` (which rounds to `PI`).
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
/// Boltzmann's constant over the electron charge, as the C++ writes them [J/K, C].
const BOLTZMANN: f64 = 1.38066e-23;
const ELECTRON_CHARGE: f64 = 1.60218e-19;

/// ObjexxFCL `pow_2`: `x * x`.
fn pow_2(x: f64) -> f64 {
    x * x
}

/// ObjexxFCL `pow_3`: `x * x * x`.
fn pow_3(x: f64) -> f64 {
    x * x * x
}

/// ObjexxFCL `pow_4`: the square of `x * x`.
fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

/// ObjexxFCL `pow_5`: `t * t * x` with `t = x * x`.
fn pow_5(x: f64) -> f64 {
    let t = x * x;
    t * t * x
}

/// `x` when it is above zero, else 0 (also for NaN), as `SandiaF1` and `SandiaF2` clip.
fn positive(x: f64) -> f64 {
    if x > 0.0 { x } else { 0.0 }
}

/// Port of `SandiaModuleTemperature` (`Photovoltaics.cc:1771`): back-of-module temperature.
pub fn SandiaModuleTemperature(
    Ibc: f64,
    Idc: f64,
    Ws: f64,
    Ta: f64,
    fd: f64,
    a: f64,
    b: f64,
) -> f64 {
    let e = Ibc + fd * Idc;
    e * (a + b * Ws).exp() + Ta
}

/// Port of `SandiaTcellFromTmodule` (`Photovoltaics.cc:1815`).
pub fn SandiaTcellFromTmodule(Tm: f64, Ibc: f64, Idc: f64, fd: f64, DT0: f64) -> f64 {
    let e = Ibc + fd * Idc;
    Tm + (e / 1000.0) * DT0
}

/// Port of `SandiaCellTemperature` (`Photovoltaics.cc:1856`): the two functions above in turn.
pub fn SandiaCellTemperature(
    Ibc: f64,
    Idc: f64,
    Ws: f64,
    Ta: f64,
    fd: f64,
    a: f64,
    b: f64,
    DT0: f64,
) -> f64 {
    let tm = SandiaModuleTemperature(Ibc, Idc, Ws, Ta, fd, a, b);
    SandiaTcellFromTmodule(tm, Ibc, Idc, fd, DT0)
}

/// Port of `SandiaEffectiveIrradiance` (`Photovoltaics.cc:1902`).
pub fn SandiaEffectiveIrradiance(Tc: f64, Isc: f64, Isc0: f64, aIsc: f64) -> f64 {
    Isc / (1.0 + aIsc * (Tc - 25.0)) / Isc0
}

/// Port of `AbsoluteAirMass` (`Photovoltaics.cc:1927`). From 89.9 degrees (and for NaN) the
/// relative air mass is held at 36.32.
pub fn AbsoluteAirMass(SolZen: f64, Altitude: f64) -> f64 {
    let am = if SolZen < 89.9 {
        1.0 / ((SolZen * DEG_TO_RAD).cos() + 0.5057 * (96.08 - SolZen).powf(-1.634))
    } else {
        36.32
    };
    (-0.0001184 * Altitude).exp() * am
}

/// Port of `SandiaF1` (`Photovoltaics.cc:1956`): the air-mass modifier, at least 0.
pub fn SandiaF1(AMa: f64, a0: f64, a1: f64, a2: f64, a3: f64, a4: f64) -> f64 {
    positive(a0 + a1 * AMa + a2 * pow_2(AMa) + a3 * pow_3(AMa) + a4 * pow_4(AMa))
}

/// Port of `SandiaF2` (`Photovoltaics.cc:1999`): the incidence-angle modifier, at least 0.
pub fn SandiaF2(IncAng: f64, b0: f64, b1: f64, b2: f64, b3: f64, b4: f64, b5: f64) -> f64 {
    positive(
        b0 + b1 * IncAng
            + b2 * pow_2(IncAng)
            + b3 * pow_3(IncAng)
            + b4 * pow_4(IncAng)
            + b5 * pow_5(IncAng),
    )
}

/// Port of `SandiaImp` (`Photovoltaics.cc:2042`): current at the maximum power point.
pub fn SandiaImp(Tc: f64, Ee: f64, Imp0: f64, aImp: f64, C0: f64, C1: f64) -> f64 {
    Imp0 * (C0 * Ee + C1 * pow_2(Ee)) * (1.0 + aImp * (Tc - 25.0))
}

/// Port of `SandiaIsc` (`Photovoltaics.cc:2075`): short-circuit current.
pub fn SandiaIsc(
    Tc: f64,
    Isc0: f64,
    Ibc: f64,
    Idc: f64,
    F1: f64,
    F2: f64,
    fd: f64,
    aIsc: f64,
) -> f64 {
    Isc0 * F1 * ((Ibc * F2 + fd * Idc) / 1000.0) * (1.0 + aIsc * (Tc - 25.0))
}

/// Port of `SandiaIx` (`Photovoltaics.cc:2114`): current at half the open-circuit voltage.
pub fn SandiaIx(Tc: f64, Ee: f64, Ix0: f64, aIsc: f64, aImp: f64, C4: f64, C5: f64) -> f64 {
    Ix0 * (C4 * Ee + C5 * pow_2(Ee)) * (1.0 + ((aIsc + aImp) / 2.0 * (Tc - 25.0)))
}

/// Port of `SandiaIxx` (`Photovoltaics.cc:2145`).
pub fn SandiaIxx(Tc: f64, Ee: f64, Ixx0: f64, aImp: f64, C6: f64, C7: f64) -> f64 {
    Ixx0 * (C6 * Ee + C7 * pow_2(Ee)) * (1.0 + aImp * (Tc - 25.0))
}

/// The diode thermal voltage, `n k (Tc + 273.15) / q`.
fn thermal_voltage(Tc: f64, DiodeFactor: f64) -> f64 {
    DiodeFactor * ((BOLTZMANN * (Tc + KELVIN)) / ELECTRON_CHARGE)
}

/// Port of `SandiaVmp` (`Photovoltaics.cc:2175`): voltage at the maximum power point, 0 without
/// effective irradiance.
pub fn SandiaVmp(
    Tc: f64,
    Ee: f64,
    Vmp0: f64,
    NcellSer: f64,
    DiodeFactor: f64,
    BVmp0: f64,
    mBVmp: f64,
    C2: f64,
    C3: f64,
) -> f64 {
    if Ee > 0.0 {
        let dtc = thermal_voltage(Tc, DiodeFactor);
        let bvmp_ee = BVmp0 + mBVmp * (1.0 - Ee);
        Vmp0 + C2 * NcellSer * dtc * Ee.ln()
            + C3 * NcellSer * pow_2(dtc * Ee.ln())
            + bvmp_ee * (Tc - 25.0)
    } else {
        0.0
    }
}

/// Port of `SandiaVoc` (`Photovoltaics.cc:2223`): open-circuit voltage, 0 without effective
/// irradiance.
pub fn SandiaVoc(
    Tc: f64,
    Ee: f64,
    Voc0: f64,
    NcellSer: f64,
    DiodeFactor: f64,
    BVoc0: f64,
    mBVoc: f64,
) -> f64 {
    if Ee > 0.0 {
        let dtc = thermal_voltage(Tc, DiodeFactor);
        let bvoc_ee = BVoc0 + mBVoc * (1.0 - Ee);
        Voc0 + NcellSer * dtc * Ee.ln() + bvoc_ee * (Tc - 25.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branches() {
        assert_eq!(AbsoluteAirMass(90.0, 0.0), 36.32);
        assert_eq!(AbsoluteAirMass(f64::NAN, 0.0), 36.32);
        assert_eq!(SandiaF1(1.5, -1.0, 0.0, 0.0, 0.0, 0.0), 0.0);
        assert_eq!(SandiaF2(0.0, f64::NAN, 0.0, 0.0, 0.0, 0.0, 0.0), 0.0);
        assert_eq!(
            SandiaVmp(25.0, 0.0, 40.0, 72.0, 1.2, -0.2, 0.0, 0.0, -5.0),
            0.0
        );
        assert_eq!(SandiaVoc(25.0, f64::NAN, 50.0, 72.0, 1.2, -0.2, 0.0), 0.0);
        assert_eq!(SandiaVoc(25.0, 1.0, 50.0, 72.0, 1.2, -0.2, 0.0), 50.0);
    }
}
