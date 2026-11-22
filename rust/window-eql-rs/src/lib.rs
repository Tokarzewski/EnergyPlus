//! C ABI for `eql.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double IS_OPENNESS(double D, double S)` (`eql.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IS_OPENNESS(D: f64, S: f64) -> f64 {
    port::IS_OPENNESS(D, S)
}

/// `IS_OPENNESS` as `rust_anything_IS_OPENNESS`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IS_OPENNESS(D: f64, S: f64) -> f64 {
    port::IS_OPENNESS(D, S)
}

/// `double IS_DSRATIO(double OPENNESS)` (`eql.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IS_DSRATIO(OPENNESS: f64) -> f64 {
    port::IS_DSRATIO(OPENNESS)
}

/// `IS_DSRATIO` as `rust_anything_IS_DSRATIO`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IS_DSRATIO(OPENNESS: f64) -> f64 {
    port::IS_DSRATIO(OPENNESS)
}

/// `double VB_SLAT_RADIUS_RATIO(double W, double C)` (`eql.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VB_SLAT_RADIUS_RATIO(W: f64, C: f64) -> f64 {
    port::VB_SLAT_RADIUS_RATIO(W, C)
}

/// `VB_SLAT_RADIUS_RATIO` as `rust_anything_VB_SLAT_RADIUS_RATIO`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VB_SLAT_RADIUS_RATIO(W: f64, C: f64) -> f64 {
    port::VB_SLAT_RADIUS_RATIO(W, C)
}

/// `double FRA(double TM, double T, double DT, double AK, double BK, double CK, double ACP, double BCP, double CCP, double AVISC, double BVISC, double CVISC, double RHOGAS)` (`eql.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FRA(
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
    port::FRA(
        TM, T, DT, AK, BK, CK, ACP, BCP, CCP, AVISC, BVISC, CVISC, RHOGAS,
    )
}

/// `FRA` as `rust_anything_FRA`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FRA(
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
    port::FRA(
        TM, T, DT, AK, BK, CK, ACP, BCP, CCP, AVISC, BVISC, CVISC, RHOGAS,
    )
}

/// `double FNU(double RA)` (`eql.c:37`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FNU(RA: f64) -> f64 {
    port::FNU(RA)
}

/// `FNU` as `rust_anything_FNU`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FNU(RA: f64) -> f64 {
    port::FNU(RA)
}

/// `double HRadPar(double T1, double T2, double E1, double E2)` (`eql.c:42`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn HRadPar(T1: f64, T2: f64, E1: f64, E2: f64) -> f64 {
    port::HRadPar(T1, T2, E1, E2)
}

/// `HRadPar` as `rust_anything_HRadPar`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_HRadPar(T1: f64, T2: f64, E1: f64, E2: f64) -> f64 {
    port::HRadPar(T1, T2, E1, E2)
}

/// `double HIC_ASHRAE(double L, double TG, double TI)` (`eql.c:47`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn HIC_ASHRAE(L: f64, TG: f64, TI: f64) -> f64 {
    port::HIC_ASHRAE(L, TG, TI)
}

/// `HIC_ASHRAE` as `rust_anything_HIC_ASHRAE`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_HIC_ASHRAE(L: f64, TG: f64, TI: f64) -> f64 {
    port::HIC_ASHRAE(L, TG, TI)
}

/// `double VB_CriticalSlatAngle(double OMEGA_DEG)` (`eql.c:52`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VB_CriticalSlatAngle(OMEGA_DEG: f64) -> f64 {
    port::VB_CriticalSlatAngle(OMEGA_DEG)
}

/// `VB_CriticalSlatAngle` as `rust_anything_VB_CriticalSlatAngle`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VB_CriticalSlatAngle(OMEGA_DEG: f64) -> f64 {
    port::VB_CriticalSlatAngle(OMEGA_DEG)
}

/// `bool FEQX(double a, double b, double tolF, double tolAbs)` (`eql.c:57`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FEQX(a: f64, b: f64, tolF: f64, tolAbs: f64) -> bool {
    port::FEQX(a, b, tolF, tolAbs)
}

/// `FEQX` as `rust_anything_FEQX`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FEQX(a: f64, b: f64, tolF: f64, tolAbs: f64) -> bool {
    port::FEQX(a, b, tolF, tolAbs)
}

/// `double TRadC(double J, double Emiss)` (`eql.c:62`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn TRadC(J: f64, Emiss: f64) -> f64 {
    port::TRadC(J, Emiss)
}

/// `TRadC` as `rust_anything_TRadC`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_TRadC(J: f64, Emiss: f64) -> f64 {
    port::TRadC(J, Emiss)
}
