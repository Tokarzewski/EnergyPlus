//! The Rust port of `convection.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Each function is the EnergyPlus convection correlation of the same name (namespace `Convect`,
//! `ConvectionCoefficients.cc`), the overload that takes plain numbers. Every formula keeps the
//! C++ operation order with no FMA, `std::pow` is `powf` (both call the C library's `pow`), and
//! ObjexxFCL's `pow_N` and `max` are the helpers below. C has no error channel: nothing panics,
//! and a zero or negative input gives the same infinity or NaN the C++ gives.

#![forbid(unsafe_code)]

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

/// ObjexxFCL `pow_6`: the square of `x * x * x`.
fn pow_6(x: f64) -> f64 {
    let t = x * x * x;
    t * t
}

/// ObjexxFCL `max`, `a < b ? b : a`: a NaN `a` is returned, a NaN `b` is ignored.
fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// The cube root of `|cofpow|`, negated when `cofpow` is negative, as the Beausoleil-Morrison
/// models take it.
fn signed_cbrt(cofpow: f64) -> f64 {
    let hc = cofpow.abs().powf(1.0 / 3.0);
    if cofpow < 0.0 { -hc } else { hc }
}

/// Port of `Windward` (`ConvectionCoefficients.cc:697`): whether a surface faces into the wind.
pub fn Windward(CosTilt: f64, Azimuth: f64, WindDirection: f64) -> bool {
    if CosTilt.abs() >= 0.98 {
        return true;
    }
    let mut diff = (WindDirection - Azimuth).abs();
    if (diff - 180.0) > 0.001 {
        diff -= 360.0;
    }
    (diff.abs() - 90.0) <= 0.001
}

/// Port of `CalcASHRAESimpleIntConvCoeff` (`ConvectionCoefficients.cc:1824`).
pub fn CalcASHRAESimpleIntConvCoeff(Tsurf: f64, Tamb: f64, cosTilt: f64) -> f64 {
    if cosTilt.abs() < 0.3827 {
        return 3.076; // vertical
    }
    let delta_temp_cos_tilt = (Tamb - Tsurf) * cosTilt;
    let (enhanced, reduced) = if cosTilt.abs() >= 0.9239 {
        (4.040, 0.948)
    } else {
        (3.870, 2.281)
    };
    if delta_temp_cos_tilt > 0.0 {
        enhanced
    } else if delta_temp_cos_tilt < 0.0 {
        reduced
    } else {
        3.076
    }
}

/// Port of `CalcASHRAETARPNatural` (`ConvectionCoefficients.cc:1902`).
pub fn CalcASHRAETARPNatural(Tsurf: f64, Tamb: f64, cosTilt: f64) -> f64 {
    let delta_temp = Tsurf - Tamb;
    if delta_temp == 0.0 || cosTilt == 0.0 {
        return CalcASHRAEVerticalWall(delta_temp);
    }
    if (delta_temp < 0.0 && cosTilt < 0.0) || (delta_temp > 0.0 && cosTilt > 0.0) {
        return CalcWaltonUnstableHorizontalOrTilt(delta_temp, cosTilt);
    }
    CalcWaltonStableHorizontalOrTilt(delta_temp, cosTilt)
}

/// Port of `CalcASHRAEVerticalWall` (`ConvectionCoefficients.hh:390`).
pub fn CalcASHRAEVerticalWall(DeltaTemp: f64) -> f64 {
    1.31 * DeltaTemp.abs().powf(1.0 / 3.0)
}

/// Port of `CalcWaltonUnstableHorizontalOrTilt` (`ConvectionCoefficients.hh:408`).
pub fn CalcWaltonUnstableHorizontalOrTilt(DeltaTemp: f64, CosineTilt: f64) -> f64 {
    9.482 * DeltaTemp.abs().powf(1.0 / 3.0) / (7.238 - CosineTilt.abs())
}

/// Port of `CalcWaltonStableHorizontalOrTilt` (`ConvectionCoefficients.hh:432`).
pub fn CalcWaltonStableHorizontalOrTilt(DeltaTemp: f64, CosineTilt: f64) -> f64 {
    1.810 * DeltaTemp.abs().powf(1.0 / 3.0) / (1.382 + CosineTilt.abs())
}

/// Port of `CalcAlamdariHammondUnstableHorizontal` (`ConvectionCoefficients.cc:5018`).
pub fn CalcAlamdariHammondUnstableHorizontal(DeltaTemp: f64, HydraulicDiameter: f64) -> f64 {
    (pow_6(1.4 * (DeltaTemp.abs() / HydraulicDiameter).powf(0.25)) + (1.63 * pow_2(DeltaTemp)))
        .powf(1.0 / 6.0)
}

/// Port of `CalcAlamdariHammondStableHorizontal` (`ConvectionCoefficients.cc:5055`).
pub fn CalcAlamdariHammondStableHorizontal(DeltaTemp: f64, HydraulicDiameter: f64) -> f64 {
    0.6 * (DeltaTemp.abs() / pow_2(HydraulicDiameter)).powf(0.2)
}

/// Port of `CalcAlamdariHammondVerticalWall` (`ConvectionCoefficients.cc:5094`).
pub fn CalcAlamdariHammondVerticalWall(DeltaTemp: f64, Height: f64) -> f64 {
    (pow_6(1.5 * (DeltaTemp.abs() / Height).powf(0.25)) + (1.23 * pow_2(DeltaTemp))).powf(1.0 / 6.0)
}

/// Port of `CalcKhalifaEq3WallAwayFromHeat` (`ConvectionCoefficients.cc:5134`).
pub fn CalcKhalifaEq3WallAwayFromHeat(DeltaTemp: f64) -> f64 {
    2.07 * DeltaTemp.abs().powf(0.23)
}

/// Port of `CalcKhalifaEq4CeilingAwayFromHeat` (`ConvectionCoefficients.cc:5158`).
pub fn CalcKhalifaEq4CeilingAwayFromHeat(DeltaTemp: f64) -> f64 {
    2.72 * DeltaTemp.abs().powf(0.13)
}

/// Port of `CalcKhalifaEq5WallsNearHeat` (`ConvectionCoefficients.cc:5179`).
pub fn CalcKhalifaEq5WallsNearHeat(DeltaTemp: f64) -> f64 {
    1.98 * DeltaTemp.abs().powf(0.32)
}

/// Port of `CalcKhalifaEq6NonHeatedWalls` (`ConvectionCoefficients.cc:5200`).
pub fn CalcKhalifaEq6NonHeatedWalls(DeltaTemp: f64) -> f64 {
    2.30 * DeltaTemp.abs().powf(0.24)
}

/// Port of `CalcKhalifaEq7Ceiling` (`ConvectionCoefficients.cc:5221`).
pub fn CalcKhalifaEq7Ceiling(DeltaTemp: f64) -> f64 {
    3.10 * DeltaTemp.abs().powf(0.17)
}

/// Port of `CalcAwbiHattonHeatedFloor` (`ConvectionCoefficients.cc:5242`). Below a diameter
/// of 1 m the C++ divides by `std::pow(1.0, 0.076)`, which is exactly 1.
pub fn CalcAwbiHattonHeatedFloor(DeltaTemp: f64, HydraulicDiameter: f64) -> f64 {
    if HydraulicDiameter > 1.0 {
        return 2.175 * DeltaTemp.abs().powf(0.308) / HydraulicDiameter.powf(0.076);
    }
    2.175 * DeltaTemp.abs().powf(0.308)
}

/// Port of `CalcAwbiHattonHeatedWall` (`ConvectionCoefficients.cc:5269`).
pub fn CalcAwbiHattonHeatedWall(DeltaTemp: f64, HydraulicDiameter: f64) -> f64 {
    1.823 * DeltaTemp.abs().powf(0.293) / max(HydraulicDiameter, 1.0).powf(0.121)
}

/// The Alamdari-Hammond vertical-wall term under the square root of the mixed wall models.
fn bm_wall_natural(DeltaTemp: f64, Height: f64) -> f64 {
    (pow_6(1.5 * (DeltaTemp.abs() / Height).powf(0.25)) + (1.23 * pow_2(DeltaTemp)).powf(1.0 / 6.0))
        .sqrt()
}

/// `((SurfTemp - SupplyAirTemp) / |DeltaTemp|) * (a + b * ACH^0.8)`, the forced term of the
/// mixed models.
fn bm_forced(DeltaTemp: f64, SurfTemp: f64, SupplyAirTemp: f64, ach: f64, a: f64, b: f64) -> f64 {
    ((SurfTemp - SupplyAirTemp) / DeltaTemp.abs()) * (a + b * ach.powf(0.8))
}

/// Port of `CalcBeausoleilMorrisonMixedAssistedWall` (`ConvectionCoefficients.cc:5289`).
pub fn CalcBeausoleilMorrisonMixedAssistedWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    let forced = bm_forced(
        DeltaTemp,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
        -0.199,
        0.190,
    );
    signed_cbrt(bm_wall_natural(DeltaTemp, Height) + pow_3(forced))
}

/// Port of `CalcBeausoleilMorrisonMixedOpposingWall` (`ConvectionCoefficients.cc:5347`).
pub fn CalcBeausoleilMorrisonMixedOpposingWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    let (mut hc1, mut hc2) = (9.999, 9.999);
    if Height != 0.0 {
        let forced = bm_forced(
            DeltaTemp,
            SurfTemp,
            SupplyAirTemp,
            AirChangeRate,
            -0.199,
            0.190,
        );
        hc1 = signed_cbrt(bm_wall_natural(DeltaTemp, Height) - pow_3(forced));
        hc2 = 0.8 * CalcAlamdariHammondVerticalWall(DeltaTemp, Height);
    }
    let hc3 = 0.8
        * ((SurfTemp - SupplyAirTemp) / DeltaTemp.abs())
        * (-0.199 + 0.190 * AirChangeRate.powf(0.8));
    max(max(hc1, hc2), hc3)
}

/// The stable floor and ceiling models: Alamdari-Hammond stable horizontal plus a forced term.
fn bm_stable(dt: f64, dh: f64, ts: f64, tsup: f64, ach: f64, a: f64, b: f64) -> f64 {
    let natural = 0.6 * (dt.abs() / dh).powf(0.2);
    signed_cbrt(pow_3(natural) + pow_3(bm_forced(dt, ts, tsup, ach, a, b)))
}

/// The unstable floor and ceiling models: Alamdari-Hammond unstable horizontal plus a forced term.
fn bm_unstable(dt: f64, dh: f64, ts: f64, tsup: f64, ach: f64, a: f64, b: f64) -> f64 {
    let natural =
        (pow_6(1.4 * (dt.abs() / dh).powf(0.25)) + pow_6(1.63 * dt.abs().powf(1.0 / 3.0))).sqrt();
    signed_cbrt(natural + pow_3(bm_forced(dt, ts, tsup, ach, a, b)))
}

/// Port of `CalcBeausoleilMorrisonMixedStableFloor` (`ConvectionCoefficients.cc:5417`).
pub fn CalcBeausoleilMorrisonMixedStableFloor(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    bm_stable(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
        0.159,
        0.116,
    )
}

/// Port of `CalcBeausoleilMorrisonMixedUnstableFloor` (`ConvectionCoefficients.cc:5545`).
pub fn CalcBeausoleilMorrisonMixedUnstableFloor(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    bm_unstable(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
        0.159,
        0.116,
    )
}

/// Port of `CalcBeausoleilMorrisonMixedStableCeiling` (`ConvectionCoefficients.cc:5601`).
pub fn CalcBeausoleilMorrisonMixedStableCeiling(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    bm_stable(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
        -0.166,
        0.484,
    )
}

/// Port of `CalcBeausoleilMorrisonMixedUnstableCeiling` (`ConvectionCoefficients.cc:5655`).
pub fn CalcBeausoleilMorrisonMixedUnstableCeiling(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    bm_unstable(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
        -0.166,
        0.484,
    )
}

/// Port of `CalcFohannoPolidoriVerticalWall` (`ConvectionCoefficients.cc:5710`).
pub fn CalcFohannoPolidoriVerticalWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    QdotConv: f64,
) -> f64 {
    const G: f64 = 9.81; // gravity [m/s2]
    const V: f64 = 15.89e-6; // kinematic viscosity of air [m2/s]
    const K: f64 = 0.0263; // conductivity of air [W/m-K]
    const PR: f64 = 0.71; // Prandtl number
    const KELVIN: f64 = 273.15;
    let beta_film = 1.0 / (KELVIN + SurfTemp + 0.5 * DeltaTemp);
    let ra_h = (G * beta_film * QdotConv * pow_4(Height) * PR) / (K * pow_2(V));
    if ra_h <= 6.3e09 {
        return 1.332 * (DeltaTemp.abs() / Height).powf(0.25);
    }
    1.235 * (0.0467 * Height).exp() * DeltaTemp.abs().powf(0.316)
}

/// Port of `CalcKaradagChilledCeiling` (`ConvectionCoefficients.cc:5761`).
pub fn CalcKaradagChilledCeiling(DeltaTemp: f64) -> f64 {
    3.1 * DeltaTemp.abs().powf(0.22)
}

/// Port of `CalcGoldsteinNovoselacCeilingDiffuserFloor` (`ConvectionCoefficients.cc:5904`).
pub fn CalcGoldsteinNovoselacCeilingDiffuserFloor(
    AirSystemFlowRate: f64,
    ZoneExtPerimLength: f64,
) -> f64 {
    if ZoneExtPerimLength > 0.0 {
        return 0.048 * (AirSystemFlowRate / ZoneExtPerimLength).powf(0.8);
    }
    9.999
}

/// Port of `CalcMoWITTNatural` (`ConvectionCoefficients.cc:6025`).
pub fn CalcMoWITTNatural(DeltaTemp: f64) -> f64 {
    0.84 * DeltaTemp.abs().powf(1.0 / 3.0)
}

/// Port of `CalcMoWITTForcedWindward` (`ConvectionCoefficients.cc:6031`).
pub fn CalcMoWITTForcedWindward(WindAtZ: f64) -> f64 {
    3.26 * WindAtZ.powf(0.89)
}

/// Port of `CalcMoWITTForcedLeeward` (`ConvectionCoefficients.cc:6038`).
pub fn CalcMoWITTForcedLeeward(WindAtZ: f64) -> f64 {
    3.55 * WindAtZ.powf(0.617)
}

/// Port of `CalcMoWITTWindward` (`ConvectionCoefficients.cc:6045`).
pub fn CalcMoWITTWindward(DeltaTemp: f64, WindAtZ: f64) -> f64 {
    (pow_2(CalcMoWITTNatural(DeltaTemp)) + pow_2(CalcMoWITTForcedWindward(WindAtZ))).sqrt()
}

/// Port of `CalcMoWITTLeeward` (`ConvectionCoefficients.cc:6065`).
pub fn CalcMoWITTLeeward(DeltaTemp: f64, WindAtZ: f64) -> f64 {
    (pow_2(CalcMoWITTNatural(DeltaTemp)) + pow_2(CalcMoWITTForcedLeeward(WindAtZ))).sqrt()
}

/// Port of `CalcNusseltJurges` (`ConvectionCoefficients.cc:6136`).
pub fn CalcNusseltJurges(WindAtZ: f64) -> f64 {
    5.8 + 3.94 * WindAtZ
}

/// Port of `CalcMcAdams` (`ConvectionCoefficients.cc:6157`).
pub fn CalcMcAdams(WindAtZ: f64) -> f64 {
    5.8 + 3.8 * WindAtZ
}

/// Port of `CalcMitchell` (`ConvectionCoefficients.cc:6177`).
pub fn CalcMitchell(WindAtZ: f64, LengthScale: f64) -> f64 {
    8.6 * WindAtZ.powf(0.6) / LengthScale.powf(0.4)
}

/// Port of `CalcWindSurfaceTheta` (`ConvectionCoefficients.cc:6214`): the angle between the wind
/// and the surface normal, 0 to 180 degrees. Rust's `%` on `f64` is C's `fmod`.
pub fn CalcWindSurfaceTheta(WindDir: f64, SurfAzimuth: f64) -> f64 {
    let theta = (WindDir % 360.0 - SurfAzimuth % 360.0).abs();
    if theta > 180.0 {
        (theta - 360.0).abs()
    } else {
        theta
    }
}

/// The Emmel forced-convection fit for the sector the wind angle falls in: `(a, b)` of
/// `a * U10^b` for 0-22.5, 22.5-67.5, 67.5-112.5, 112.5-157.5 and above 157.5 degrees.
fn emmel(WindAt10m: f64, theta: f64, fits: [(f64, f64); 5]) -> f64 {
    // A NaN angle passes every test, as in the C++, and takes the last fit.
    let sector = [22.5, 67.5, 112.5, 157.5]
        .iter()
        .position(|&edge| theta <= edge)
        .unwrap_or(4);
    let (a, b) = fits[sector];
    a * WindAt10m.powf(b)
}

/// Port of `CalcEmmelVertical` (`ConvectionCoefficients.cc:6273`).
pub fn CalcEmmelVertical(WindAt10m: f64, WindDir: f64, SurfAzimuth: f64) -> f64 {
    let fits = [
        (5.15, 0.81),
        (3.34, 0.84),
        (4.78, 0.71),
        (4.05, 0.77),
        (3.54, 0.76),
    ];
    emmel(WindAt10m, CalcWindSurfaceTheta(WindDir, SurfAzimuth), fits)
}

/// Port of `CalcEmmelRoof` (`ConvectionCoefficients.cc:6308`).
pub fn CalcEmmelRoof(WindAt10m: f64, WindDir: f64, LongAxisOutwardAzimuth: f64) -> f64 {
    let fits = [
        (5.11, 0.78),
        (4.60, 0.79),
        (3.67, 0.85),
        (4.60, 0.79),
        (5.11, 0.78),
    ];
    emmel(
        WindAt10m,
        CalcWindSurfaceTheta(WindDir, LongAxisOutwardAzimuth),
        fits,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branches_and_helpers() {
        assert_eq!(CalcASHRAESimpleIntConvCoeff(20.0, 25.0, 0.0), 3.076);
        assert_eq!(CalcASHRAESimpleIntConvCoeff(20.0, 25.0, 1.0), 4.040);
        assert_eq!(CalcASHRAESimpleIntConvCoeff(25.0, 20.0, 0.5), 2.281);
        assert!(Windward(1.0, 0.0, 180.0));
        assert!(!Windward(0.0, 0.0, 180.0));
        assert_eq!(CalcWindSurfaceTheta(350.0, 10.0), 20.0);
        assert_eq!(CalcGoldsteinNovoselacCeilingDiffuserFloor(1.0, 0.0), 9.999);
        assert!(max(f64::NAN, 1.0).is_nan());
        assert_eq!(max(1.0, f64::NAN), 1.0);
        assert_eq!(CalcEmmelRoof(0.0, 0.0, 0.0), 0.0);
    }
}
