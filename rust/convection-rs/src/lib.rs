//! C ABI for `convection.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `bool Windward(double CosTilt, double Azimuth, double WindDirection)` (`convection.c:43`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Windward(CosTilt: f64, Azimuth: f64, WindDirection: f64) -> bool {
    port::Windward(CosTilt, Azimuth, WindDirection)
}

/// `Windward` as `rust_anything_Windward`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Windward(CosTilt: f64, Azimuth: f64, WindDirection: f64) -> bool {
    port::Windward(CosTilt, Azimuth, WindDirection)
}

/// `double CalcASHRAESimpleIntConvCoeff(double Tsurf, double Tamb, double cosTilt)` (`convection.c:48`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcASHRAESimpleIntConvCoeff(Tsurf: f64, Tamb: f64, cosTilt: f64) -> f64 {
    port::CalcASHRAESimpleIntConvCoeff(Tsurf, Tamb, cosTilt)
}

/// `CalcASHRAESimpleIntConvCoeff` as `rust_anything_CalcASHRAESimpleIntConvCoeff`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcASHRAESimpleIntConvCoeff(
    Tsurf: f64,
    Tamb: f64,
    cosTilt: f64,
) -> f64 {
    port::CalcASHRAESimpleIntConvCoeff(Tsurf, Tamb, cosTilt)
}

/// `double CalcASHRAETARPNatural(double Tsurf, double Tamb, double cosTilt)` (`convection.c:53`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcASHRAETARPNatural(Tsurf: f64, Tamb: f64, cosTilt: f64) -> f64 {
    port::CalcASHRAETARPNatural(Tsurf, Tamb, cosTilt)
}

/// `CalcASHRAETARPNatural` as `rust_anything_CalcASHRAETARPNatural`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcASHRAETARPNatural(Tsurf: f64, Tamb: f64, cosTilt: f64) -> f64 {
    port::CalcASHRAETARPNatural(Tsurf, Tamb, cosTilt)
}

/// `double CalcASHRAEVerticalWall(double DeltaTemp)` (`convection.c:58`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcASHRAEVerticalWall(DeltaTemp: f64) -> f64 {
    port::CalcASHRAEVerticalWall(DeltaTemp)
}

/// `CalcASHRAEVerticalWall` as `rust_anything_CalcASHRAEVerticalWall`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcASHRAEVerticalWall(DeltaTemp: f64) -> f64 {
    port::CalcASHRAEVerticalWall(DeltaTemp)
}

/// `double CalcWaltonUnstableHorizontalOrTilt(double DeltaTemp, double CosineTilt)` (`convection.c:63`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcWaltonUnstableHorizontalOrTilt(DeltaTemp: f64, CosineTilt: f64) -> f64 {
    port::CalcWaltonUnstableHorizontalOrTilt(DeltaTemp, CosineTilt)
}

/// `CalcWaltonUnstableHorizontalOrTilt` as `rust_anything_CalcWaltonUnstableHorizontalOrTilt`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcWaltonUnstableHorizontalOrTilt(
    DeltaTemp: f64,
    CosineTilt: f64,
) -> f64 {
    port::CalcWaltonUnstableHorizontalOrTilt(DeltaTemp, CosineTilt)
}

/// `double CalcWaltonStableHorizontalOrTilt(double DeltaTemp, double CosineTilt)` (`convection.c:68`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcWaltonStableHorizontalOrTilt(DeltaTemp: f64, CosineTilt: f64) -> f64 {
    port::CalcWaltonStableHorizontalOrTilt(DeltaTemp, CosineTilt)
}

/// `CalcWaltonStableHorizontalOrTilt` as `rust_anything_CalcWaltonStableHorizontalOrTilt`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcWaltonStableHorizontalOrTilt(
    DeltaTemp: f64,
    CosineTilt: f64,
) -> f64 {
    port::CalcWaltonStableHorizontalOrTilt(DeltaTemp, CosineTilt)
}

/// `double CalcAlamdariHammondUnstableHorizontal(double DeltaTemp, double HydraulicDiameter)` (`convection.c:73`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcAlamdariHammondUnstableHorizontal(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
) -> f64 {
    port::CalcAlamdariHammondUnstableHorizontal(DeltaTemp, HydraulicDiameter)
}

/// `CalcAlamdariHammondUnstableHorizontal` as `rust_anything_CalcAlamdariHammondUnstableHorizontal`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcAlamdariHammondUnstableHorizontal(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
) -> f64 {
    port::CalcAlamdariHammondUnstableHorizontal(DeltaTemp, HydraulicDiameter)
}

/// `double CalcAlamdariHammondStableHorizontal(double DeltaTemp, double HydraulicDiameter)` (`convection.c:78`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcAlamdariHammondStableHorizontal(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
) -> f64 {
    port::CalcAlamdariHammondStableHorizontal(DeltaTemp, HydraulicDiameter)
}

/// `CalcAlamdariHammondStableHorizontal` as `rust_anything_CalcAlamdariHammondStableHorizontal`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcAlamdariHammondStableHorizontal(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
) -> f64 {
    port::CalcAlamdariHammondStableHorizontal(DeltaTemp, HydraulicDiameter)
}

/// `double CalcAlamdariHammondVerticalWall(double DeltaTemp, double Height)` (`convection.c:83`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcAlamdariHammondVerticalWall(DeltaTemp: f64, Height: f64) -> f64 {
    port::CalcAlamdariHammondVerticalWall(DeltaTemp, Height)
}

/// `CalcAlamdariHammondVerticalWall` as `rust_anything_CalcAlamdariHammondVerticalWall`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcAlamdariHammondVerticalWall(
    DeltaTemp: f64,
    Height: f64,
) -> f64 {
    port::CalcAlamdariHammondVerticalWall(DeltaTemp, Height)
}

/// `double CalcKhalifaEq3WallAwayFromHeat(double DeltaTemp)` (`convection.c:88`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcKhalifaEq3WallAwayFromHeat(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq3WallAwayFromHeat(DeltaTemp)
}

/// `CalcKhalifaEq3WallAwayFromHeat` as `rust_anything_CalcKhalifaEq3WallAwayFromHeat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcKhalifaEq3WallAwayFromHeat(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq3WallAwayFromHeat(DeltaTemp)
}

/// `double CalcKhalifaEq4CeilingAwayFromHeat(double DeltaTemp)` (`convection.c:93`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcKhalifaEq4CeilingAwayFromHeat(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq4CeilingAwayFromHeat(DeltaTemp)
}

/// `CalcKhalifaEq4CeilingAwayFromHeat` as `rust_anything_CalcKhalifaEq4CeilingAwayFromHeat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcKhalifaEq4CeilingAwayFromHeat(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq4CeilingAwayFromHeat(DeltaTemp)
}

/// `double CalcKhalifaEq5WallsNearHeat(double DeltaTemp)` (`convection.c:98`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcKhalifaEq5WallsNearHeat(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq5WallsNearHeat(DeltaTemp)
}

/// `CalcKhalifaEq5WallsNearHeat` as `rust_anything_CalcKhalifaEq5WallsNearHeat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcKhalifaEq5WallsNearHeat(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq5WallsNearHeat(DeltaTemp)
}

/// `double CalcKhalifaEq6NonHeatedWalls(double DeltaTemp)` (`convection.c:103`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcKhalifaEq6NonHeatedWalls(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq6NonHeatedWalls(DeltaTemp)
}

/// `CalcKhalifaEq6NonHeatedWalls` as `rust_anything_CalcKhalifaEq6NonHeatedWalls`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcKhalifaEq6NonHeatedWalls(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq6NonHeatedWalls(DeltaTemp)
}

/// `double CalcKhalifaEq7Ceiling(double DeltaTemp)` (`convection.c:108`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcKhalifaEq7Ceiling(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq7Ceiling(DeltaTemp)
}

/// `CalcKhalifaEq7Ceiling` as `rust_anything_CalcKhalifaEq7Ceiling`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcKhalifaEq7Ceiling(DeltaTemp: f64) -> f64 {
    port::CalcKhalifaEq7Ceiling(DeltaTemp)
}

/// `double CalcAwbiHattonHeatedFloor(double DeltaTemp, double HydraulicDiameter)` (`convection.c:113`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcAwbiHattonHeatedFloor(DeltaTemp: f64, HydraulicDiameter: f64) -> f64 {
    port::CalcAwbiHattonHeatedFloor(DeltaTemp, HydraulicDiameter)
}

/// `CalcAwbiHattonHeatedFloor` as `rust_anything_CalcAwbiHattonHeatedFloor`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcAwbiHattonHeatedFloor(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
) -> f64 {
    port::CalcAwbiHattonHeatedFloor(DeltaTemp, HydraulicDiameter)
}

/// `double CalcAwbiHattonHeatedWall(double DeltaTemp, double HydraulicDiameter)` (`convection.c:118`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcAwbiHattonHeatedWall(DeltaTemp: f64, HydraulicDiameter: f64) -> f64 {
    port::CalcAwbiHattonHeatedWall(DeltaTemp, HydraulicDiameter)
}

/// `CalcAwbiHattonHeatedWall` as `rust_anything_CalcAwbiHattonHeatedWall`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcAwbiHattonHeatedWall(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
) -> f64 {
    port::CalcAwbiHattonHeatedWall(DeltaTemp, HydraulicDiameter)
}

/// `double CalcBeausoleilMorrisonMixedAssistedWall(double DeltaTemp, double Height, double SurfTemp, double SupplyAirTemp, double AirChangeRate)` (`convection.c:123`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcBeausoleilMorrisonMixedAssistedWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedAssistedWall(
        DeltaTemp,
        Height,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `CalcBeausoleilMorrisonMixedAssistedWall` as `rust_anything_CalcBeausoleilMorrisonMixedAssistedWall`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcBeausoleilMorrisonMixedAssistedWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedAssistedWall(
        DeltaTemp,
        Height,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `double CalcBeausoleilMorrisonMixedOpposingWall(double DeltaTemp, double Height, double SurfTemp, double SupplyAirTemp, double AirChangeRate)` (`convection.c:128`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcBeausoleilMorrisonMixedOpposingWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedOpposingWall(
        DeltaTemp,
        Height,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `CalcBeausoleilMorrisonMixedOpposingWall` as `rust_anything_CalcBeausoleilMorrisonMixedOpposingWall`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcBeausoleilMorrisonMixedOpposingWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedOpposingWall(
        DeltaTemp,
        Height,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `double CalcBeausoleilMorrisonMixedStableFloor(double DeltaTemp, double HydraulicDiameter, double SurfTemp, double SupplyAirTemp, double AirChangeRate)` (`convection.c:133`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcBeausoleilMorrisonMixedStableFloor(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedStableFloor(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `CalcBeausoleilMorrisonMixedStableFloor` as `rust_anything_CalcBeausoleilMorrisonMixedStableFloor`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcBeausoleilMorrisonMixedStableFloor(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedStableFloor(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `double CalcBeausoleilMorrisonMixedUnstableFloor(double DeltaTemp, double HydraulicDiameter, double SurfTemp, double SupplyAirTemp, double AirChangeRate)` (`convection.c:138`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcBeausoleilMorrisonMixedUnstableFloor(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedUnstableFloor(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `CalcBeausoleilMorrisonMixedUnstableFloor` as `rust_anything_CalcBeausoleilMorrisonMixedUnstableFloor`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcBeausoleilMorrisonMixedUnstableFloor(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedUnstableFloor(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `double CalcBeausoleilMorrisonMixedStableCeiling(double DeltaTemp, double HydraulicDiameter, double SurfTemp, double SupplyAirTemp, double AirChangeRate)` (`convection.c:143`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcBeausoleilMorrisonMixedStableCeiling(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedStableCeiling(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `CalcBeausoleilMorrisonMixedStableCeiling` as `rust_anything_CalcBeausoleilMorrisonMixedStableCeiling`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcBeausoleilMorrisonMixedStableCeiling(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedStableCeiling(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `double CalcBeausoleilMorrisonMixedUnstableCeiling(double DeltaTemp, double HydraulicDiameter, double SurfTemp, double SupplyAirTemp, double AirChangeRate)` (`convection.c:148`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcBeausoleilMorrisonMixedUnstableCeiling(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedUnstableCeiling(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `CalcBeausoleilMorrisonMixedUnstableCeiling` as `rust_anything_CalcBeausoleilMorrisonMixedUnstableCeiling`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcBeausoleilMorrisonMixedUnstableCeiling(
    DeltaTemp: f64,
    HydraulicDiameter: f64,
    SurfTemp: f64,
    SupplyAirTemp: f64,
    AirChangeRate: f64,
) -> f64 {
    port::CalcBeausoleilMorrisonMixedUnstableCeiling(
        DeltaTemp,
        HydraulicDiameter,
        SurfTemp,
        SupplyAirTemp,
        AirChangeRate,
    )
}

/// `double CalcFohannoPolidoriVerticalWall(double DeltaTemp, double Height, double SurfTemp, double QdotConv)` (`convection.c:153`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcFohannoPolidoriVerticalWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    QdotConv: f64,
) -> f64 {
    port::CalcFohannoPolidoriVerticalWall(DeltaTemp, Height, SurfTemp, QdotConv)
}

/// `CalcFohannoPolidoriVerticalWall` as `rust_anything_CalcFohannoPolidoriVerticalWall`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcFohannoPolidoriVerticalWall(
    DeltaTemp: f64,
    Height: f64,
    SurfTemp: f64,
    QdotConv: f64,
) -> f64 {
    port::CalcFohannoPolidoriVerticalWall(DeltaTemp, Height, SurfTemp, QdotConv)
}

/// `double CalcKaradagChilledCeiling(double DeltaTemp)` (`convection.c:158`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcKaradagChilledCeiling(DeltaTemp: f64) -> f64 {
    port::CalcKaradagChilledCeiling(DeltaTemp)
}

/// `CalcKaradagChilledCeiling` as `rust_anything_CalcKaradagChilledCeiling`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcKaradagChilledCeiling(DeltaTemp: f64) -> f64 {
    port::CalcKaradagChilledCeiling(DeltaTemp)
}

/// `double CalcGoldsteinNovoselacCeilingDiffuserFloor(double AirSystemFlowRate, double ZoneExtPerimLength)` (`convection.c:163`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcGoldsteinNovoselacCeilingDiffuserFloor(
    AirSystemFlowRate: f64,
    ZoneExtPerimLength: f64,
) -> f64 {
    port::CalcGoldsteinNovoselacCeilingDiffuserFloor(AirSystemFlowRate, ZoneExtPerimLength)
}

/// `CalcGoldsteinNovoselacCeilingDiffuserFloor` as `rust_anything_CalcGoldsteinNovoselacCeilingDiffuserFloor`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcGoldsteinNovoselacCeilingDiffuserFloor(
    AirSystemFlowRate: f64,
    ZoneExtPerimLength: f64,
) -> f64 {
    port::CalcGoldsteinNovoselacCeilingDiffuserFloor(AirSystemFlowRate, ZoneExtPerimLength)
}

/// `double CalcMoWITTNatural(double DeltaTemp)` (`convection.c:168`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcMoWITTNatural(DeltaTemp: f64) -> f64 {
    port::CalcMoWITTNatural(DeltaTemp)
}

/// `CalcMoWITTNatural` as `rust_anything_CalcMoWITTNatural`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcMoWITTNatural(DeltaTemp: f64) -> f64 {
    port::CalcMoWITTNatural(DeltaTemp)
}

/// `double CalcMoWITTForcedWindward(double WindAtZ)` (`convection.c:173`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcMoWITTForcedWindward(WindAtZ: f64) -> f64 {
    port::CalcMoWITTForcedWindward(WindAtZ)
}

/// `CalcMoWITTForcedWindward` as `rust_anything_CalcMoWITTForcedWindward`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcMoWITTForcedWindward(WindAtZ: f64) -> f64 {
    port::CalcMoWITTForcedWindward(WindAtZ)
}

/// `double CalcMoWITTForcedLeeward(double WindAtZ)` (`convection.c:178`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcMoWITTForcedLeeward(WindAtZ: f64) -> f64 {
    port::CalcMoWITTForcedLeeward(WindAtZ)
}

/// `CalcMoWITTForcedLeeward` as `rust_anything_CalcMoWITTForcedLeeward`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcMoWITTForcedLeeward(WindAtZ: f64) -> f64 {
    port::CalcMoWITTForcedLeeward(WindAtZ)
}

/// `double CalcMoWITTWindward(double DeltaTemp, double WindAtZ)` (`convection.c:183`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcMoWITTWindward(DeltaTemp: f64, WindAtZ: f64) -> f64 {
    port::CalcMoWITTWindward(DeltaTemp, WindAtZ)
}

/// `CalcMoWITTWindward` as `rust_anything_CalcMoWITTWindward`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcMoWITTWindward(DeltaTemp: f64, WindAtZ: f64) -> f64 {
    port::CalcMoWITTWindward(DeltaTemp, WindAtZ)
}

/// `double CalcMoWITTLeeward(double DeltaTemp, double WindAtZ)` (`convection.c:188`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcMoWITTLeeward(DeltaTemp: f64, WindAtZ: f64) -> f64 {
    port::CalcMoWITTLeeward(DeltaTemp, WindAtZ)
}

/// `CalcMoWITTLeeward` as `rust_anything_CalcMoWITTLeeward`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcMoWITTLeeward(DeltaTemp: f64, WindAtZ: f64) -> f64 {
    port::CalcMoWITTLeeward(DeltaTemp, WindAtZ)
}

/// `double CalcNusseltJurges(double WindAtZ)` (`convection.c:193`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcNusseltJurges(WindAtZ: f64) -> f64 {
    port::CalcNusseltJurges(WindAtZ)
}

/// `CalcNusseltJurges` as `rust_anything_CalcNusseltJurges`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcNusseltJurges(WindAtZ: f64) -> f64 {
    port::CalcNusseltJurges(WindAtZ)
}

/// `double CalcMcAdams(double WindAtZ)` (`convection.c:198`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcMcAdams(WindAtZ: f64) -> f64 {
    port::CalcMcAdams(WindAtZ)
}

/// `CalcMcAdams` as `rust_anything_CalcMcAdams`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcMcAdams(WindAtZ: f64) -> f64 {
    port::CalcMcAdams(WindAtZ)
}

/// `double CalcMitchell(double WindAtZ, double LengthScale)` (`convection.c:203`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcMitchell(WindAtZ: f64, LengthScale: f64) -> f64 {
    port::CalcMitchell(WindAtZ, LengthScale)
}

/// `CalcMitchell` as `rust_anything_CalcMitchell`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcMitchell(WindAtZ: f64, LengthScale: f64) -> f64 {
    port::CalcMitchell(WindAtZ, LengthScale)
}

/// `double CalcWindSurfaceTheta(double WindDir, double SurfAzimuth)` (`convection.c:208`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcWindSurfaceTheta(WindDir: f64, SurfAzimuth: f64) -> f64 {
    port::CalcWindSurfaceTheta(WindDir, SurfAzimuth)
}

/// `CalcWindSurfaceTheta` as `rust_anything_CalcWindSurfaceTheta`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcWindSurfaceTheta(WindDir: f64, SurfAzimuth: f64) -> f64 {
    port::CalcWindSurfaceTheta(WindDir, SurfAzimuth)
}

/// `double CalcEmmelVertical(double WindAt10m, double WindDir, double SurfAzimuth)` (`convection.c:213`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcEmmelVertical(WindAt10m: f64, WindDir: f64, SurfAzimuth: f64) -> f64 {
    port::CalcEmmelVertical(WindAt10m, WindDir, SurfAzimuth)
}

/// `CalcEmmelVertical` as `rust_anything_CalcEmmelVertical`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcEmmelVertical(
    WindAt10m: f64,
    WindDir: f64,
    SurfAzimuth: f64,
) -> f64 {
    port::CalcEmmelVertical(WindAt10m, WindDir, SurfAzimuth)
}

/// `double CalcEmmelRoof(double WindAt10m, double WindDir, double LongAxisOutwardAzimuth)` (`convection.c:218`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcEmmelRoof(WindAt10m: f64, WindDir: f64, LongAxisOutwardAzimuth: f64) -> f64 {
    port::CalcEmmelRoof(WindAt10m, WindDir, LongAxisOutwardAzimuth)
}

/// `CalcEmmelRoof` as `rust_anything_CalcEmmelRoof`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcEmmelRoof(
    WindAt10m: f64,
    WindDir: f64,
    LongAxisOutwardAzimuth: f64,
) -> f64 {
    port::CalcEmmelRoof(WindAt10m, WindDir, LongAxisOutwardAzimuth)
}
