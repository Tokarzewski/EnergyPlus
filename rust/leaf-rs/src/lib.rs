//! C ABI for `leaf.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double InterpolateBetweenTwoValues(double X, double X0, double X1, double F0, double F1)` (`leaf.c:33`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn InterpolateBetweenTwoValues(X: f64, X0: f64, X1: f64, F0: f64, F1: f64) -> f64 {
    port::InterpolateBetweenTwoValues(X, X0, X1, F0, F1)
}

/// `InterpolateBetweenTwoValues` as `rust_anything_InterpolateBetweenTwoValues`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_InterpolateBetweenTwoValues(
    X: f64,
    X0: f64,
    X1: f64,
    F0: f64,
    F1: f64,
) -> f64 {
    port::InterpolateBetweenTwoValues(X, X0, X1, F0, F1)
}

/// `double InterpolateBetweenFourValues(double X, double Y, double X1, double X2, double Y1, double Y2, double Fx1y1, double Fx1y2, double Fx2y1, double Fx2y2)` (`leaf.c:38`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn InterpolateBetweenFourValues(
    X: f64,
    Y: f64,
    X1: f64,
    X2: f64,
    Y1: f64,
    Y2: f64,
    Fx1y1: f64,
    Fx1y2: f64,
    Fx2y1: f64,
    Fx2y2: f64,
) -> f64 {
    port::InterpolateBetweenFourValues(X, Y, X1, X2, Y1, Y2, Fx1y1, Fx1y2, Fx2y1, Fx2y2)
}

/// `InterpolateBetweenFourValues` as `rust_anything_InterpolateBetweenFourValues`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_InterpolateBetweenFourValues(
    X: f64,
    Y: f64,
    X1: f64,
    X2: f64,
    Y1: f64,
    Y2: f64,
    Fx1y1: f64,
    Fx1y2: f64,
    Fx2y1: f64,
    Fx2y2: f64,
) -> f64 {
    port::InterpolateBetweenFourValues(X, Y, X1, X2, Y1, Y2, Fx1y1, Fx1y2, Fx2y1, Fx2y2)
}

/// `double InterpSw(double SwitchFac, double A, double B)` (`leaf.c:43`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn InterpSw(SwitchFac: f64, A: f64, B: f64) -> f64 {
    port::InterpSw(SwitchFac, A, B)
}

/// `InterpSw` as `rust_anything_InterpSw`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_InterpSw(SwitchFac: f64, A: f64, B: f64) -> f64 {
    port::InterpSw(SwitchFac, A, B)
}

/// `double CalcTimeNeeded(double Ti, double Tf, double Ta, double T1, double T2, double m, double Cp, double m1, double m2, double UA, double Q)` (`leaf.c:48`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcTimeNeeded(
    Ti: f64,
    Tf: f64,
    Ta: f64,
    T1: f64,
    T2: f64,
    m: f64,
    Cp: f64,
    m1: f64,
    m2: f64,
    UA: f64,
    Q: f64,
) -> f64 {
    port::CalcTimeNeeded(Ti, Tf, Ta, T1, T2, m, Cp, m1, m2, UA, Q)
}

/// `CalcTimeNeeded` as `rust_anything_CalcTimeNeeded`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcTimeNeeded(
    Ti: f64,
    Tf: f64,
    Ta: f64,
    T1: f64,
    T2: f64,
    m: f64,
    Cp: f64,
    m1: f64,
    m2: f64,
    UA: f64,
    Q: f64,
) -> f64 {
    port::CalcTimeNeeded(Ti, Tf, Ta, T1, T2, m, Cp, m1, m2, UA, Q)
}

/// `double CalcTankTemp(double Ti, double Ta, double T1, double T2, double m, double Cp, double m1, double m2, double UA, double Q, double t)` (`leaf.c:53`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcTankTemp(
    Ti: f64,
    Ta: f64,
    T1: f64,
    T2: f64,
    m: f64,
    Cp: f64,
    m1: f64,
    m2: f64,
    UA: f64,
    Q: f64,
    t: f64,
) -> f64 {
    port::CalcTankTemp(Ti, Ta, T1, T2, m, Cp, m1, m2, UA, Q, t)
}

/// `CalcTankTemp` as `rust_anything_CalcTankTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcTankTemp(
    Ti: f64,
    Ta: f64,
    T1: f64,
    T2: f64,
    m: f64,
    Cp: f64,
    m1: f64,
    m2: f64,
    UA: f64,
    Q: f64,
    t: f64,
) -> f64 {
    port::CalcTankTemp(Ti, Ta, T1, T2, m, Cp, m1, m2, UA, Q, t)
}

/// `double CalcTempIntegral(double Ti, double Tf, double Ta, double T1, double T2, double m, double Cp, double m1, double m2, double UA, double Q, double t)` (`leaf.c:58`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcTempIntegral(
    Ti: f64,
    Tf: f64,
    Ta: f64,
    T1: f64,
    T2: f64,
    m: f64,
    Cp: f64,
    m1: f64,
    m2: f64,
    UA: f64,
    Q: f64,
    t: f64,
) -> f64 {
    port::CalcTempIntegral(Ti, Tf, Ta, T1, T2, m, Cp, m1, m2, UA, Q, t)
}

/// `CalcTempIntegral` as `rust_anything_CalcTempIntegral`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcTempIntegral(
    Ti: f64,
    Tf: f64,
    Ta: f64,
    T1: f64,
    T2: f64,
    m: f64,
    Cp: f64,
    m1: f64,
    m2: f64,
    UA: f64,
    Q: f64,
    t: f64,
) -> f64 {
    port::CalcTempIntegral(Ti, Tf, Ta, T1, T2, m, Cp, m1, m2, UA, Q, t)
}

/// `double LDSumMax(double Width, double Height)` (`leaf.c:63`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn LDSumMax(Width: f64, Height: f64) -> f64 {
    port::LDSumMax(Width, Height)
}

/// `LDSumMax` as `rust_anything_LDSumMax`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_LDSumMax(Width: f64, Height: f64) -> f64 {
    port::LDSumMax(Width, Height)
}

/// `double LDSumMean(double Width, double Height)` (`leaf.c:68`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn LDSumMean(Width: f64, Height: f64) -> f64 {
    port::LDSumMean(Width, Height)
}

/// `LDSumMean` as `rust_anything_LDSumMean`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_LDSumMean(Width: f64, Height: f64) -> f64 {
    port::LDSumMean(Width, Height)
}

/// `double pos(double x)` (`leaf.c:73`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn pos(x: f64) -> f64 {
    port::pos(x)
}

/// `pos` as `rust_anything_pos`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_pos(x: f64) -> f64 {
    port::pos(x)
}

/// `double WaterConversionFunct(double WaterTotal, double ConversionFactor)` (`leaf.c:78`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterConversionFunct(WaterTotal: f64, ConversionFactor: f64) -> f64 {
    port::WaterConversionFunct(WaterTotal, ConversionFactor)
}

/// `WaterConversionFunct` as `rust_anything_WaterConversionFunct`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterConversionFunct(
    WaterTotal: f64,
    ConversionFactor: f64,
) -> f64 {
    port::WaterConversionFunct(WaterTotal, ConversionFactor)
}

/// `bool isInTriangle(double qx, double qy, double x1, double y1, double x2, double y2, double x3, double y3)` (`leaf.c:83`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn isInTriangle(
    qx: f64,
    qy: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> bool {
    port::isInTriangle(qx, qy, x1, y1, x2, y2, x3, y3)
}

/// `isInTriangle` as `rust_anything_isInTriangle`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_isInTriangle(
    qx: f64,
    qy: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> bool {
    port::isInTriangle(qx, qy, x1, y1, x2, y2, x3, y3)
}

/// `bool isInQuadrilateral(double qx, double qy, double ax, double ay, double bx, double by, double cx, double cy, double dx, double dy)` (`leaf.c:88`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn isInQuadrilateral(
    qx: f64,
    qy: f64,
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    cx: f64,
    cy: f64,
    dx: f64,
    dy: f64,
) -> bool {
    port::isInQuadrilateral(qx, qy, ax, ay, bx, by, cx, cy, dx, dy)
}

/// `isInQuadrilateral` as `rust_anything_isInQuadrilateral`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_isInQuadrilateral(
    qx: f64,
    qy: f64,
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    cx: f64,
    cy: f64,
    dx: f64,
    dy: f64,
) -> bool {
    port::isInQuadrilateral(qx, qy, ax, ay, bx, by, cx, cy, dx, dy)
}

/// `double CalcDetIceStorLMTDstar(double Tin, double Tout, double Tfr)` (`leaf.c:93`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CalcDetIceStorLMTDstar(Tin: f64, Tout: f64, Tfr: f64) -> f64 {
    port::CalcDetIceStorLMTDstar(Tin, Tout, Tfr)
}

/// `CalcDetIceStorLMTDstar` as `rust_anything_CalcDetIceStorLMTDstar`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CalcDetIceStorLMTDstar(Tin: f64, Tout: f64, Tfr: f64) -> f64 {
    port::CalcDetIceStorLMTDstar(Tin, Tout, Tfr)
}

/// `double TempSItoIP(double Temp)` (`leaf.c:98`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn TempSItoIP(Temp: f64) -> f64 {
    port::TempSItoIP(Temp)
}

/// `TempSItoIP` as `rust_anything_TempSItoIP`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_TempSItoIP(Temp: f64) -> f64 {
    port::TempSItoIP(Temp)
}

/// `double TempIPtoSI(double Temp)` (`leaf.c:103`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn TempIPtoSI(Temp: f64) -> f64 {
    port::TempIPtoSI(Temp)
}

/// `TempIPtoSI` as `rust_anything_TempIPtoSI`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_TempIPtoSI(Temp: f64) -> f64 {
    port::TempIPtoSI(Temp)
}

/// `double BoundValueToWithinTwoValues(double ValueToBound, double LowerBound, double UpperBound)` (`leaf.c:108`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BoundValueToWithinTwoValues(
    ValueToBound: f64,
    LowerBound: f64,
    UpperBound: f64,
) -> f64 {
    port::BoundValueToWithinTwoValues(ValueToBound, LowerBound, UpperBound)
}

/// `BoundValueToWithinTwoValues` as `rust_anything_BoundValueToWithinTwoValues`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BoundValueToWithinTwoValues(
    ValueToBound: f64,
    LowerBound: f64,
    UpperBound: f64,
) -> f64 {
    port::BoundValueToWithinTwoValues(ValueToBound, LowerBound, UpperBound)
}

/// `bool IntegerIsWithinTwoValues(int ValueToCheck, int LowerBound, int UpperBound)` (`leaf.c:113`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IntegerIsWithinTwoValues(
    ValueToCheck: std::ffi::c_int,
    LowerBound: std::ffi::c_int,
    UpperBound: std::ffi::c_int,
) -> bool {
    port::IntegerIsWithinTwoValues(ValueToCheck, LowerBound, UpperBound)
}

/// `IntegerIsWithinTwoValues` as `rust_anything_IntegerIsWithinTwoValues`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IntegerIsWithinTwoValues(
    ValueToCheck: std::ffi::c_int,
    LowerBound: std::ffi::c_int,
    UpperBound: std::ffi::c_int,
) -> bool {
    port::IntegerIsWithinTwoValues(ValueToCheck, LowerBound, UpperBound)
}

/// `double calc_taoalpha(double theta, double glass_thickness, double refrac_index_glass, double k_glass)` (`leaf.c:118`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn calc_taoalpha(
    theta: f64,
    glass_thickness: f64,
    refrac_index_glass: f64,
    k_glass: f64,
) -> f64 {
    port::calc_taoalpha(theta, glass_thickness, refrac_index_glass, k_glass)
}

/// `calc_taoalpha` as `rust_anything_calc_taoalpha`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_calc_taoalpha(
    theta: f64,
    glass_thickness: f64,
    refrac_index_glass: f64,
    k_glass: f64,
) -> f64 {
    port::calc_taoalpha(theta, glass_thickness, refrac_index_glass, k_glass)
}

/// `double calc_k_taoalpha(double theta, double glass_thickness, double refrac_index_glass, double k_glass)` (`leaf.c:123`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn calc_k_taoalpha(
    theta: f64,
    glass_thickness: f64,
    refrac_index_glass: f64,
    k_glass: f64,
) -> f64 {
    port::calc_k_taoalpha(theta, glass_thickness, refrac_index_glass, k_glass)
}

/// `calc_k_taoalpha` as `rust_anything_calc_k_taoalpha`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_calc_k_taoalpha(
    theta: f64,
    glass_thickness: f64,
    refrac_index_glass: f64,
    k_glass: f64,
) -> f64 {
    port::calc_k_taoalpha(theta, glass_thickness, refrac_index_glass, k_glass)
}

/// `double PassiveGapNusseltNumber(double AspRat, double Tilt, double Tso, double Tsi, double Gr)` (`leaf.c:128`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PassiveGapNusseltNumber(
    AspRat: f64,
    Tilt: f64,
    Tso: f64,
    Tsi: f64,
    Gr: f64,
) -> f64 {
    port::PassiveGapNusseltNumber(AspRat, Tilt, Tso, Tsi, Gr)
}

/// `PassiveGapNusseltNumber` as `rust_anything_PassiveGapNusseltNumber`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PassiveGapNusseltNumber(
    AspRat: f64,
    Tilt: f64,
    Tso: f64,
    Tsi: f64,
    Gr: f64,
) -> f64 {
    port::PassiveGapNusseltNumber(AspRat, Tilt, Tso, Tsi, Gr)
}

/// `double fclamp(double v, double min, double max)` (`leaf.c:133`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn fclamp(v: f64, min: f64, max: f64) -> f64 {
    port::fclamp(v, min, max)
}

/// `fclamp` as `rust_anything_fclamp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_fclamp(v: f64, min: f64, max: f64) -> f64 {
    port::fclamp(v, min, max)
}

/// `double MakeUpWaterVolFlowFunct(double MakeUpWaterMassFlowRate, double Density)` (`leaf.c:138`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn MakeUpWaterVolFlowFunct(MakeUpWaterMassFlowRate: f64, Density: f64) -> f64 {
    port::MakeUpWaterVolFlowFunct(MakeUpWaterMassFlowRate, Density)
}

/// `MakeUpWaterVolFlowFunct` as `rust_anything_MakeUpWaterVolFlowFunct`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_MakeUpWaterVolFlowFunct(
    MakeUpWaterMassFlowRate: f64,
    Density: f64,
) -> f64 {
    port::MakeUpWaterVolFlowFunct(MakeUpWaterMassFlowRate, Density)
}

/// `double MakeUpWaterVolFunct(double MakeUpWaterMass, double Density)` (`leaf.c:143`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn MakeUpWaterVolFunct(MakeUpWaterMass: f64, Density: f64) -> f64 {
    port::MakeUpWaterVolFunct(MakeUpWaterMass, Density)
}

/// `MakeUpWaterVolFunct` as `rust_anything_MakeUpWaterVolFunct`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_MakeUpWaterVolFunct(MakeUpWaterMass: f64, Density: f64) -> f64 {
    port::MakeUpWaterVolFunct(MakeUpWaterMass, Density)
}

/// `double FuncDetermineEngineTemp(double TcwOut, double MCeng, double UAHX, double UAskin, double Troom, double Qgenss, double TengLast, double time)` (`leaf.c:148`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FuncDetermineEngineTemp(
    TcwOut: f64,
    MCeng: f64,
    UAHX: f64,
    UAskin: f64,
    Troom: f64,
    Qgenss: f64,
    TengLast: f64,
    time: f64,
) -> f64 {
    port::FuncDetermineEngineTemp(TcwOut, MCeng, UAHX, UAskin, Troom, Qgenss, TengLast, time)
}

/// `FuncDetermineEngineTemp` as `rust_anything_FuncDetermineEngineTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FuncDetermineEngineTemp(
    TcwOut: f64,
    MCeng: f64,
    UAHX: f64,
    UAskin: f64,
    Troom: f64,
    Qgenss: f64,
    TengLast: f64,
    time: f64,
) -> f64 {
    port::FuncDetermineEngineTemp(TcwOut, MCeng, UAHX, UAskin, Troom, Qgenss, TengLast, time)
}

/// `double FuncDetermineCoolantWaterExitTemp(double TcwIn, double MCcw, double UAHX, double MdotCpcw, double Teng, double TcwoutLast, double time)` (`leaf.c:153`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FuncDetermineCoolantWaterExitTemp(
    TcwIn: f64,
    MCcw: f64,
    UAHX: f64,
    MdotCpcw: f64,
    Teng: f64,
    TcwoutLast: f64,
    time: f64,
) -> f64 {
    port::FuncDetermineCoolantWaterExitTemp(TcwIn, MCcw, UAHX, MdotCpcw, Teng, TcwoutLast, time)
}

/// `FuncDetermineCoolantWaterExitTemp` as `rust_anything_FuncDetermineCoolantWaterExitTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FuncDetermineCoolantWaterExitTemp(
    TcwIn: f64,
    MCcw: f64,
    UAHX: f64,
    MdotCpcw: f64,
    Teng: f64,
    TcwoutLast: f64,
    time: f64,
) -> f64 {
    port::FuncDetermineCoolantWaterExitTemp(TcwIn, MCcw, UAHX, MdotCpcw, Teng, TcwoutLast, time)
}

/// `bool CheckMicroCHPThermalBalance(double NomHeatGen, double TcwIn, double TcwOut, double Teng, double Troom, double UAHX, double UAskin, double Qgenss, double MCeng, double MCcw, double MdotCpcw)` (`leaf.c:158`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CheckMicroCHPThermalBalance(
    NomHeatGen: f64,
    TcwIn: f64,
    TcwOut: f64,
    Teng: f64,
    Troom: f64,
    UAHX: f64,
    UAskin: f64,
    Qgenss: f64,
    MCeng: f64,
    MCcw: f64,
    MdotCpcw: f64,
) -> bool {
    port::CheckMicroCHPThermalBalance(
        NomHeatGen, TcwIn, TcwOut, Teng, Troom, UAHX, UAskin, Qgenss, MCeng, MCcw, MdotCpcw,
    )
}

/// `CheckMicroCHPThermalBalance` as `rust_anything_CheckMicroCHPThermalBalance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CheckMicroCHPThermalBalance(
    NomHeatGen: f64,
    TcwIn: f64,
    TcwOut: f64,
    Teng: f64,
    Troom: f64,
    UAHX: f64,
    UAskin: f64,
    Qgenss: f64,
    MCeng: f64,
    MCcw: f64,
    MdotCpcw: f64,
) -> bool {
    port::CheckMicroCHPThermalBalance(
        NomHeatGen, TcwIn, TcwOut, Teng, Troom, UAHX, UAskin, Qgenss, MCeng, MCcw, MdotCpcw,
    )
}
