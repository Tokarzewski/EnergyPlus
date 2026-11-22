//! The Rust port of `leaf.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Plain-number leaf helpers from eleven EnergyPlus modules: window interpolation
//! (`WindowManager`), water tank temperatures (`WaterThermalTanks`), glazing deflection sums
//! (`TARCOGCommon`), point-in-polygon tests (`OutputReportTabular`), ice storage
//! (`IceThermalStorage`), bounds (`PlantUtilities`), PVT glass transmittance
//! (`PhotovoltaicThermalCollectors`), the passive gap Nusselt number (`TranspiredCollector`),
//! `fclamp` (`UtilityRoutines`), pool make-up water (`SwimmingPool`) and micro-CHP engine
//! temperatures (`MicroCHPElectricGenerator`). Every formula keeps the C++ operation order with no
//! FMA. Nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the C++ signatures"
)]

const PI: f64 = std::f64::consts::PI; // `Constant::Pi`
const PI_OVR2: f64 = PI / 2.0;
const DEG_TO_RAD: f64 = PI / 180.0;

/// ObjexxFCL `max` and `min`: `a < b ? b : a` and `a < b ? a : b`.
fn ox_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
fn ox_min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// ObjexxFCL `pow_2`.
fn pow_2(x: f64) -> f64 {
    x * x
}

// ---- WindowManager.cc / WindowManager.hh (namespace Window)

/// Port of `Window::InterpolateBetweenTwoValues` (`WindowManager.cc:5374`).
pub fn InterpolateBetweenTwoValues(X: f64, X0: f64, X1: f64, F0: f64, F1: f64) -> f64 {
    F0 + ((X - X0) / (X1 - X0)) * (F1 - F0)
}

/// Port of `Window::InterpolateBetweenFourValues` (`WindowManager.cc:5395`): bilinear
/// interpolation.
pub fn InterpolateBetweenFourValues(
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
    let area = (X2 - X1) * (Y2 - Y1);
    (Fx1y1 / area) * (X2 - X) * (Y2 - Y)
        + (Fx2y1 / area) * (X - X1) * (Y2 - Y)
        + (Fx1y2 / area) * (X2 - X) * (Y - Y1)
        + (Fx2y2 / area) * (X - X1) * (Y - Y1)
}

/// Port of `Window::InterpSw` (`WindowManager.hh:376`). `std::clamp` is
/// `std::min(std::max(v, lo), hi)`, which keeps a NaN.
pub fn InterpSw(SwitchFac: f64, A: f64, B: f64) -> f64 {
    let lo = if SwitchFac < 0.0 { 0.0 } else { SwitchFac };
    let s = if 1.0 < lo { 1.0 } else { lo };
    (1.0 - s) * A + s * B
}

// ---- WaterThermalTanks.cc (static members of WaterThermalTankData)

/// The tank's `a` and `b` (`dT/dt = a + b T`) when it loses heat or has flow.
fn tank_ab(
    Ta: f64,
    T1: f64,
    T2: f64,
    m: f64,
    Cp: f64,
    m1: f64,
    m2: f64,
    UA: f64,
    Q: f64,
) -> (f64, f64) {
    let a = (Q / Cp + UA * Ta / Cp + m1 * T1 + m2 * T2) / m;
    let b = -(UA / Cp + m1 + m2) / m;
    (a, b)
}

/// Port of `WaterThermalTankData::CalcTimeNeeded` (`WaterThermalTanks.cc:7693`): the time for
/// the tank to go from `Ti` to `Tf`, 99999999.9 s when it never gets there.
pub fn CalcTimeNeeded(
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
    const INFINITY: f64 = 99999999.9;
    if Tf == Ti {
        return 0.0;
    }
    let t = if UA / Cp + m1 + m2 == 0.0 {
        if Q == 0.0 {
            INFINITY
        } else {
            (Tf - Ti) / (Q / (m * Cp))
        }
    } else {
        let (a, b) = tank_ab(Ta, T1, T2, m, Cp, m1, m2, UA, Q);
        let tm = -a / b;
        if tm == Ti || tm == Tf {
            INFINITY
        } else {
            let quotient = (Tf - tm) / (Ti - tm);
            if quotient <= 0.0 {
                INFINITY
            } else {
                quotient.ln() / b
            }
        }
    };
    if t < 0.0 { INFINITY } else { t }
}

/// Port of `WaterThermalTankData::CalcTankTemp` (`WaterThermalTanks.cc:7790`): the tank
/// temperature after time `t`.
pub fn CalcTankTemp(
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
    if UA / Cp + m1 + m2 == 0.0 {
        let a = Q / (m * Cp);
        a * t + Ti
    } else {
        let (a, b) = tank_ab(Ta, T1, T2, m, Cp, m1, m2, UA, Q);
        (a / b + Ti) * (b * t).exp() - a / b
    }
}

/// Port of `WaterThermalTankData::CalcTempIntegral` (`WaterThermalTanks.cc:7846`): the integral
/// of the tank temperature over time `t` [C s].
pub fn CalcTempIntegral(
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
    if t == 0.0 {
        0.0
    } else if Tf == Ti {
        Tf * t
    } else if UA / Cp + m1 + m2 == 0.0 {
        let a = Q / (m * Cp);
        0.5 * a * t * t + Ti * t
    } else {
        let (a, b) = tank_ab(Ta, T1, T2, m, Cp, m1, m2, UA, Q);
        (a / b + Ti) * ((b * t).exp() - 1.0) / b - a * t / b
    }
}

// ---- TARCOGCommon.cc (namespace TARCOGCommon)

/// `TARCOGParams::MMax` and `NMax`: the odd terms up to 5 of the deflection sums.
const ODD_TERMS: [i32; 3] = [1, 3, 5];

/// Port of `TARCOGCommon::LDSumMax` (`TARCOGCommon.cc:72`).
pub fn LDSumMax(Width: f64, Height: f64) -> f64 {
    let mut sum = 0.0;
    for i in ODD_TERMS {
        let sin_i = (f64::from(i) * PI_OVR2).sin();
        let pow_i_w = pow_2(f64::from(i) / Width);
        for j in ODD_TERMS {
            sum += (sin_i * (f64::from(j) * PI_OVR2).sin())
                / (f64::from(i * j) * pow_2(pow_i_w + pow_2(f64::from(j) / Height)));
        }
    }
    sum
}

/// Port of `TARCOGCommon::LDSumMean` (`TARCOGCommon.cc:100`).
pub fn LDSumMean(Width: f64, Height: f64) -> f64 {
    const PI_SQUARED: f64 = PI * PI;
    let mut sum = 0.0;
    for i in ODD_TERMS {
        let pow_i_pi_2 = f64::from(i * i) * PI_SQUARED;
        let pow_i_w = pow_2(f64::from(i) / Width);
        for j in ODD_TERMS {
            sum += 4.0
                / (pow_i_pi_2 * f64::from(j * j) * pow_2(pow_i_w + pow_2(f64::from(j) / Height)));
        }
    }
    sum
}

/// Port of `TARCOGCommon::pos` (`TARCOGCommon.cc:407`): `x` if positive, else 0.
pub fn pos(x: f64) -> f64 {
    (x + x.abs()) / 2.0
}

// ---- OutputReportTabular.cc (namespace OutputReportTabular)

/// Port of `OutputReportTabular::WaterConversionFunct` (`OutputReportTabular.cc:9676`).
pub fn WaterConversionFunct(WaterTotal: f64, ConversionFactor: f64) -> f64 {
    WaterTotal / ConversionFactor
}

/// Port of `OutputReportTabular::isInTriangle` (`OutputReportTabular.cc:18514`): whether the
/// point is inside or on the triangle.
pub fn isInTriangle(
    qx: f64,
    qy: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
) -> bool {
    let f_ab = (qy - y1) * (x2 - x1) - (qx - x1) * (y2 - y1);
    let f_ca = (qy - y3) * (x1 - x3) - (qx - x3) * (y1 - y3);
    let f_bc = (qy - y2) * (x3 - x2) - (qx - x2) * (y3 - y2);
    (f_ab * f_bc) >= 0.0 && (f_bc * f_ca) >= 0.0
}

/// Port of `OutputReportTabular::isInQuadrilateral` (`OutputReportTabular.cc:18541`).
pub fn isInQuadrilateral(
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
    let in_abc = isInTriangle(qx, qy, ax, ay, bx, by, cx, cy);
    let in_cda = isInTriangle(qx, qy, cx, cy, dx, dy, ax, ay);
    in_abc || in_cda
}

// ---- IceThermalStorage.cc (namespace IceThermalStorage)

/// Port of `IceThermalStorage::CalcDetIceStorLMTDstar` (`IceThermalStorage.cc:1988`): the
/// log mean temperature difference over the nominal 10 C.
pub fn CalcDetIceStorLMTDstar(Tin: f64, Tout: f64, Tfr: f64) -> f64 {
    const TNOM: f64 = 10.0;
    const DELTA_TOF_MIN: f64 = 0.5;
    const DELTA_TIF_MIN: f64 = 1.0;
    let delta_tio = (Tin - Tout).abs();
    let delta_tif = ox_max_floor((Tin - Tfr).abs(), DELTA_TIF_MIN);
    let delta_tof = ox_max_floor((Tout - Tfr).abs(), DELTA_TOF_MIN);
    (delta_tio / (delta_tif / delta_tof).ln()) / TNOM
}

/// `if (d < min) d = min;`, which keeps a NaN.
fn ox_max_floor(d: f64, min: f64) -> f64 {
    if d < min { min } else { d }
}

/// Port of `IceThermalStorage::TempSItoIP` (`IceThermalStorage.cc:2058`).
pub fn TempSItoIP(Temp: f64) -> f64 {
    (Temp * 9.0 / 5.0) + 32.0
}

/// Port of `IceThermalStorage::TempIPtoSI` (`IceThermalStorage.cc:2063`).
pub fn TempIPtoSI(Temp: f64) -> f64 {
    (Temp - 32.0) * 5.0 / 9.0
}

// ---- PlantUtilities.cc (namespace PlantUtilities)

/// Port of `PlantUtilities::BoundValueToWithinTwoValues` (`PlantUtilities.cc:1518`), with
/// ObjexxFCL `max` then `min`.
pub fn BoundValueToWithinTwoValues(ValueToBound: f64, LowerBound: f64, UpperBound: f64) -> f64 {
    ox_min(ox_max(ValueToBound, LowerBound), UpperBound)
}

/// Port of `PlantUtilities::IntegerIsWithinTwoValues` (`PlantUtilities.cc:1543`).
pub fn IntegerIsWithinTwoValues(ValueToCheck: i32, LowerBound: i32, UpperBound: i32) -> bool {
    (LowerBound..=UpperBound).contains(&ValueToCheck)
}

// ---- PhotovoltaicThermalCollectors.cc (members of PVTCollectorStruct that read no fields)

/// Port of `PVTCollectorStruct::calc_taoalpha` (`PhotovoltaicThermalCollectors.cc:1720`): the
/// transmittance-absorptance of the cover at incidence angle `theta` [rad]. `std::pow(x, 2)` is
/// `x * x`, which is what the C++ compiles it to.
pub fn calc_taoalpha(
    theta: f64,
    glass_thickness: f64,
    refrac_index_glass: f64,
    k_glass: f64,
) -> f64 {
    let theta = if theta == 0.0 { 0.000000001 } else { theta };
    let theta_r = (theta.sin() / refrac_index_glass).asin();
    (-k_glass * glass_thickness / theta_r.cos()).exp()
        * (1.0
            - 0.5
                * ((pow_2((theta_r - theta).sin()) / pow_2((theta_r + theta).sin()))
                    + (pow_2((theta_r - theta).tan()) / pow_2((theta_r + theta).tan()))))
}

/// Port of `PVTCollectorStruct::calc_k_taoalpha` (`PhotovoltaicThermalCollectors.cc:1745`): the
/// incidence angle modifier.
pub fn calc_k_taoalpha(
    theta: f64,
    glass_thickness: f64,
    refrac_index_glass: f64,
    k_glass: f64,
) -> f64 {
    calc_taoalpha(theta, glass_thickness, refrac_index_glass, k_glass)
        / calc_taoalpha(0.0, glass_thickness, refrac_index_glass, k_glass)
}

// ---- TranspiredCollector.cc (namespace TranspiredCollector)

/// Port of `TranspiredCollector::PassiveGapNusseltNumber` (`TranspiredCollector.cc:1942`): the
/// Nusselt number of a tilted gap (ISO 15099, after Hollands and ElSherbiny).
pub fn PassiveGapNusseltNumber(AspRat: f64, Tilt: f64, Tso: f64, Tsi: f64, Gr: f64) -> f64 {
    const PR: f64 = 0.71;
    let tiltr = Tilt * DEG_TO_RAD;
    let ra = Gr * PR;
    let gnu901 = if ra <= 1.0e4 {
        1.0 + 1.7596678e-10 * ra.powf(2.2984755)
    } else if ra > 1.0e4 && ra <= 5.0e4 {
        0.028154 * ra.powf(0.4134)
    } else if ra > 5.0e4 {
        0.0673838 * ra.powf(1.0 / 3.0)
    } else {
        0.0 // a NaN Rayleigh number takes none of the three fits
    };
    let gnu902 = 0.242 * (ra / AspRat).powf(0.272);
    let gnu90 = ox_max(gnu901, gnu902);
    if Tso > Tsi {
        return 1.0 + (gnu90 - 1.0) * tiltr.sin();
    }
    if Tilt >= 60.0 {
        let g = 0.5 * (1.0 + (ra / 3160.0).powf(20.6)).powf(-0.1);
        let x = 0.0936 * ra.powf(0.314) / (1.0 + g);
        let x3 = x * x * x; // ObjexxFCL `pow_7`: `t = x * x * x; t * t * x`
        let gnu601a = 1.0 + x3 * x3 * x;
        let gnu601 = gnu601a.powf(0.142857);
        let gnu602 = (0.104 + 0.175 / AspRat) * ra.powf(0.283);
        let gnu60 = ox_max(gnu601, gnu602);
        return ((90.0 - Tilt) * gnu60 + (Tilt - 60.0) * gnu90) / 30.0;
    }
    let cra = ra * tiltr.cos();
    let a = 1.0 - 1708.0 / cra;
    let b = (cra / 5830.0).powf(0.33333) - 1.0;
    let gnua = (a.abs() + a) / 2.0;
    let gnub = (b.abs() + b) / 2.0;
    let ang = 1708.0 * (1.8 * tiltr).sin().powf(1.6);
    1.0 + 1.44 * gnua * (1.0 - ang / cra) + gnub
}

// ---- UtilityRoutines.hh

/// Port of `fclamp` (`UtilityRoutines.hh:654`).
pub fn fclamp(v: f64, min: f64, max: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

// ---- SwimmingPool.cc (namespace SwimmingPool)

/// Port of `SwimmingPool::MakeUpWaterVolFlowFunct` (`SwimmingPool.cc:1160`).
pub fn MakeUpWaterVolFlowFunct(MakeUpWaterMassFlowRate: f64, Density: f64) -> f64 {
    MakeUpWaterMassFlowRate / Density
}

/// Port of `SwimmingPool::MakeUpWaterVolFunct` (`SwimmingPool.cc:1165`).
pub fn MakeUpWaterVolFunct(MakeUpWaterMass: f64, Density: f64) -> f64 {
    MakeUpWaterMass / Density
}

// ---- MicroCHPElectricGenerator.cc (namespace MicroCHPElectricGenerator)

/// The engine's `a` and `b` (`dT/dt = a + b T`). The C++ writes `-1.0 * x`, which is `-x`.
fn engine_ab(
    TcwOut: f64,
    MCeng: f64,
    UAHX: f64,
    UAskin: f64,
    Troom: f64,
    Qgenss: f64,
) -> (f64, f64) {
    let a = (UAHX * TcwOut / MCeng) + (UAskin * Troom / MCeng) + (Qgenss / MCeng);
    let b = (-UAHX / MCeng) + (-UAskin / MCeng);
    (a, b)
}

/// The cooling water's `a` and `b`.
fn coolant_ab(TcwIn: f64, MCcw: f64, UAHX: f64, MdotCpcw: f64, Teng: f64) -> (f64, f64) {
    let a = (MdotCpcw * TcwIn / MCcw) + (UAHX * Teng / MCcw);
    let b = (-MdotCpcw / MCcw) + (-UAHX / MCcw);
    (a, b)
}

/// Port of `MicroCHPElectricGenerator::FuncDetermineEngineTemp` (`MicroCHPElectricGenerator.cc:1205`).
pub fn FuncDetermineEngineTemp(
    TcwOut: f64,
    MCeng: f64,
    UAHX: f64,
    UAskin: f64,
    Troom: f64,
    Qgenss: f64,
    TengLast: f64,
    time: f64,
) -> f64 {
    let (a, b) = engine_ab(TcwOut, MCeng, UAHX, UAskin, Troom, Qgenss);
    (TengLast + a / b) * (b * time).exp() - a / b
}

/// Port of `MicroCHPElectricGenerator::FuncDetermineCoolantWaterExitTemp`
/// (`MicroCHPElectricGenerator.cc:1235`); past `Constant::MaxEXPArg` the steady state.
pub fn FuncDetermineCoolantWaterExitTemp(
    TcwIn: f64,
    MCcw: f64,
    UAHX: f64,
    MdotCpcw: f64,
    Teng: f64,
    TcwoutLast: f64,
    time: f64,
) -> f64 {
    const MAX_EXP_ARG: f64 = 709.78;
    let (a, b) = coolant_ab(TcwIn, MCcw, UAHX, MdotCpcw, Teng);
    if b * time < -MAX_EXP_ARG {
        return -a / b;
    }
    (TcwoutLast + a / b) * (b * time).exp() - a / b
}

/// Port of `MicroCHPElectricGenerator::CheckMicroCHPThermalBalance`
/// (`MicroCHPElectricGenerator.cc:1267`).
pub fn CheckMicroCHPThermalBalance(
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
    let (a, b) = engine_ab(TcwOut, MCeng, UAHX, UAskin, Troom, Qgenss);
    let d_teng_d_time = a + b * Teng;
    let (c, d) = coolant_ab(TcwIn, MCcw, UAHX, MdotCpcw, Teng);
    let d_cool_out_t_d_time = c + d * TcwOut;
    let mag_imbal_eng =
        UAHX * (TcwOut - Teng) + UAskin * (Troom - Teng) + Qgenss - MCeng * d_teng_d_time;
    let mag_imbal_cooling =
        MdotCpcw * (TcwIn - TcwOut) + UAHX * (Teng - TcwOut) - MCcw * d_cool_out_t_d_time;
    let threshold = NomHeatGen / 10000000.0;
    (threshold > mag_imbal_eng) && (threshold > mag_imbal_cooling)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        assert_eq!(InterpolateBetweenTwoValues(0.5, 0.0, 1.0, 0.0, 2.0), 1.0);
        assert!(InterpSw(f64::NAN, 1.0, 2.0).is_nan());
        assert_eq!(
            (InterpSw(-1.0, 1.0, 2.0), InterpSw(2.0, 1.0, 2.0)),
            (1.0, 2.0)
        );
        assert_eq!(
            CalcTimeNeeded(20.0, 20.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            0.0
        );
        assert_eq!((pos(-1.0), pos(2.0), TempSItoIP(100.0)), (0.0, 2.0, 212.0));
        assert!(isInQuadrilateral(
            0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0
        ));
        assert!(!isInTriangle(2.0, 2.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0));
        assert!(IntegerIsWithinTwoValues(0, 0, 0) && !IntegerIsWithinTwoValues(1, 0, 0));
        assert_eq!(calc_k_taoalpha(0.0, 0.003, 1.526, 4.0), 1.0);
    }
}
