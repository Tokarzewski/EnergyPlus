//! The Rust port of `comfort.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Plain-number helpers from EnergyPlus `ThermalComfort.cc` (namespace `ThermalComfort`): Fanger's
//! PPD from PMV, the relative air velocity of an active occupant, and two saturation vapour
//! pressure fits. Nothing here panics.

#![forbid(unsafe_code)]

/// ObjexxFCL `pow_2`, `pow_3` and `pow_4`.
fn pow_2(x: f64) -> f64 {
    x * x
}
fn pow_3(x: f64) -> f64 {
    x * x * x
}
fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

/// Port of `ThermalComfort::CalcFangerPPD` (`ThermalComfort.cc:797`): the predicted percentage
/// dissatisfied, 0 to 100. Below `DataPrecisionGlobals::EXP_LowerLimit` (-20) the exponential is
/// taken as 0, and a NaN PMV gives 100.
pub fn CalcFangerPPD(PMV: f64) -> f64 {
    let exp_test = -0.03353 * pow_4(PMV) - 0.2179 * pow_2(PMV);
    let ppd = if exp_test > -20.0 {
        100.0 - 95.0 * exp_test.exp()
    } else {
        100.0
    };
    ppd.clamp(0.0, 100.0) // the C++ caps below 0 and above 100; `ppd` is never NaN here
}

/// Port of `ThermalComfort::CalcRelativeAirVelocity` (`ThermalComfort.cc:815`): air speed plus
/// 0.3 m/s per met above 1.
pub fn CalcRelativeAirVelocity(AirVel: f64, ActMet: f64) -> f64 {
    if ActMet > 1.0 {
        AirVel + 0.3 * (ActMet - 1.0)
    } else {
        AirVel
    }
}

/// Port of `ThermalComfort::CalcSatVapPressFromTemp` (`ThermalComfort.cc:2165`) [mbar].
pub fn CalcSatVapPressFromTemp(Temp: f64) -> f64 {
    let xt = Temp / 100.0;
    6.16796 + 358.1855 * pow_2(xt) - 550.3543 * pow_3(xt) + 1048.8115 * pow_4(xt)
}

/// Port of `ThermalComfort::CalcSatVapPressFromTempTorr` (`ThermalComfort.cc:2190`) [Torr].
pub fn CalcSatVapPressFromTempTorr(Temp: f64) -> f64 {
    (18.6686 - 4030.183 / (Temp + 235.0)).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        assert_eq!(
            (
                CalcFangerPPD(0.0),
                CalcFangerPPD(10.0),
                CalcFangerPPD(f64::NAN)
            ),
            (5.0, 100.0, 100.0)
        );
        assert_eq!(
            (
                CalcRelativeAirVelocity(0.1, 1.0),
                CalcRelativeAirVelocity(0.0, 2.0)
            ),
            (0.1, 0.3)
        );
    }
}
