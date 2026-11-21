//! C ABI for `sandia.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SandiaModuleTemperature(double Ibc, double Idc, double Ws, double Ta, double fd, double a, double b)` (`sandia.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaModuleTemperature(
    Ibc: f64,
    Idc: f64,
    Ws: f64,
    Ta: f64,
    fd: f64,
    a: f64,
    b: f64,
) -> f64 {
    port::SandiaModuleTemperature(Ibc, Idc, Ws, Ta, fd, a, b)
}

/// `SandiaModuleTemperature` as `rust_anything_SandiaModuleTemperature`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaModuleTemperature(
    Ibc: f64,
    Idc: f64,
    Ws: f64,
    Ta: f64,
    fd: f64,
    a: f64,
    b: f64,
) -> f64 {
    port::SandiaModuleTemperature(Ibc, Idc, Ws, Ta, fd, a, b)
}

/// `double SandiaTcellFromTmodule(double Tm, double Ibc, double Idc, double fd, double DT0)` (`sandia.c:25`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaTcellFromTmodule(Tm: f64, Ibc: f64, Idc: f64, fd: f64, DT0: f64) -> f64 {
    port::SandiaTcellFromTmodule(Tm, Ibc, Idc, fd, DT0)
}

/// `SandiaTcellFromTmodule` as `rust_anything_SandiaTcellFromTmodule`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaTcellFromTmodule(
    Tm: f64,
    Ibc: f64,
    Idc: f64,
    fd: f64,
    DT0: f64,
) -> f64 {
    port::SandiaTcellFromTmodule(Tm, Ibc, Idc, fd, DT0)
}

/// `double SandiaCellTemperature(double Ibc, double Idc, double Ws, double Ta, double fd, double a, double b, double DT0)` (`sandia.c:30`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaCellTemperature(
    Ibc: f64,
    Idc: f64,
    Ws: f64,
    Ta: f64,
    fd: f64,
    a: f64,
    b: f64,
    DT0: f64,
) -> f64 {
    port::SandiaCellTemperature(Ibc, Idc, Ws, Ta, fd, a, b, DT0)
}

/// `SandiaCellTemperature` as `rust_anything_SandiaCellTemperature`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaCellTemperature(
    Ibc: f64,
    Idc: f64,
    Ws: f64,
    Ta: f64,
    fd: f64,
    a: f64,
    b: f64,
    DT0: f64,
) -> f64 {
    port::SandiaCellTemperature(Ibc, Idc, Ws, Ta, fd, a, b, DT0)
}

/// `double SandiaEffectiveIrradiance(double Tc, double Isc, double Isc0, double aIsc)` (`sandia.c:35`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaEffectiveIrradiance(Tc: f64, Isc: f64, Isc0: f64, aIsc: f64) -> f64 {
    port::SandiaEffectiveIrradiance(Tc, Isc, Isc0, aIsc)
}

/// `SandiaEffectiveIrradiance` as `rust_anything_SandiaEffectiveIrradiance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaEffectiveIrradiance(
    Tc: f64,
    Isc: f64,
    Isc0: f64,
    aIsc: f64,
) -> f64 {
    port::SandiaEffectiveIrradiance(Tc, Isc, Isc0, aIsc)
}

/// `double AbsoluteAirMass(double SolZen, double Altitude)` (`sandia.c:40`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn AbsoluteAirMass(SolZen: f64, Altitude: f64) -> f64 {
    port::AbsoluteAirMass(SolZen, Altitude)
}

/// `AbsoluteAirMass` as `rust_anything_AbsoluteAirMass`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_AbsoluteAirMass(SolZen: f64, Altitude: f64) -> f64 {
    port::AbsoluteAirMass(SolZen, Altitude)
}

/// `double SandiaF1(double AMa, double a0, double a1, double a2, double a3, double a4)` (`sandia.c:45`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaF1(AMa: f64, a0: f64, a1: f64, a2: f64, a3: f64, a4: f64) -> f64 {
    port::SandiaF1(AMa, a0, a1, a2, a3, a4)
}

/// `SandiaF1` as `rust_anything_SandiaF1`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaF1(
    AMa: f64,
    a0: f64,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
) -> f64 {
    port::SandiaF1(AMa, a0, a1, a2, a3, a4)
}

/// `double SandiaF2(double IncAng, double b0, double b1, double b2, double b3, double b4, double b5)` (`sandia.c:50`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaF2(
    IncAng: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    b3: f64,
    b4: f64,
    b5: f64,
) -> f64 {
    port::SandiaF2(IncAng, b0, b1, b2, b3, b4, b5)
}

/// `SandiaF2` as `rust_anything_SandiaF2`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaF2(
    IncAng: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    b3: f64,
    b4: f64,
    b5: f64,
) -> f64 {
    port::SandiaF2(IncAng, b0, b1, b2, b3, b4, b5)
}

/// `double SandiaImp(double Tc, double Ee, double Imp0, double aImp, double C0, double C1)` (`sandia.c:55`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaImp(Tc: f64, Ee: f64, Imp0: f64, aImp: f64, C0: f64, C1: f64) -> f64 {
    port::SandiaImp(Tc, Ee, Imp0, aImp, C0, C1)
}

/// `SandiaImp` as `rust_anything_SandiaImp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaImp(
    Tc: f64,
    Ee: f64,
    Imp0: f64,
    aImp: f64,
    C0: f64,
    C1: f64,
) -> f64 {
    port::SandiaImp(Tc, Ee, Imp0, aImp, C0, C1)
}

/// `double SandiaIsc(double Tc, double Isc0, double Ibc, double Idc, double F1, double F2, double fd, double aIsc)` (`sandia.c:60`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaIsc(
    Tc: f64,
    Isc0: f64,
    Ibc: f64,
    Idc: f64,
    F1: f64,
    F2: f64,
    fd: f64,
    aIsc: f64,
) -> f64 {
    port::SandiaIsc(Tc, Isc0, Ibc, Idc, F1, F2, fd, aIsc)
}

/// `SandiaIsc` as `rust_anything_SandiaIsc`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaIsc(
    Tc: f64,
    Isc0: f64,
    Ibc: f64,
    Idc: f64,
    F1: f64,
    F2: f64,
    fd: f64,
    aIsc: f64,
) -> f64 {
    port::SandiaIsc(Tc, Isc0, Ibc, Idc, F1, F2, fd, aIsc)
}

/// `double SandiaIx(double Tc, double Ee, double Ix0, double aIsc, double aImp, double C4, double C5)` (`sandia.c:65`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaIx(
    Tc: f64,
    Ee: f64,
    Ix0: f64,
    aIsc: f64,
    aImp: f64,
    C4: f64,
    C5: f64,
) -> f64 {
    port::SandiaIx(Tc, Ee, Ix0, aIsc, aImp, C4, C5)
}

/// `SandiaIx` as `rust_anything_SandiaIx`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaIx(
    Tc: f64,
    Ee: f64,
    Ix0: f64,
    aIsc: f64,
    aImp: f64,
    C4: f64,
    C5: f64,
) -> f64 {
    port::SandiaIx(Tc, Ee, Ix0, aIsc, aImp, C4, C5)
}

/// `double SandiaIxx(double Tc, double Ee, double Ixx0, double aImp, double C6, double C7)` (`sandia.c:70`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaIxx(Tc: f64, Ee: f64, Ixx0: f64, aImp: f64, C6: f64, C7: f64) -> f64 {
    port::SandiaIxx(Tc, Ee, Ixx0, aImp, C6, C7)
}

/// `SandiaIxx` as `rust_anything_SandiaIxx`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaIxx(
    Tc: f64,
    Ee: f64,
    Ixx0: f64,
    aImp: f64,
    C6: f64,
    C7: f64,
) -> f64 {
    port::SandiaIxx(Tc, Ee, Ixx0, aImp, C6, C7)
}

/// `double SandiaVmp(double Tc, double Ee, double Vmp0, double NcellSer, double DiodeFactor, double BVmp0, double mBVmp, double C2, double C3)` (`sandia.c:75`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaVmp(
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
    port::SandiaVmp(Tc, Ee, Vmp0, NcellSer, DiodeFactor, BVmp0, mBVmp, C2, C3)
}

/// `SandiaVmp` as `rust_anything_SandiaVmp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaVmp(
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
    port::SandiaVmp(Tc, Ee, Vmp0, NcellSer, DiodeFactor, BVmp0, mBVmp, C2, C3)
}

/// `double SandiaVoc(double Tc, double Ee, double Voc0, double NcellSer, double DiodeFactor, double BVoc0, double mBVoc)` (`sandia.c:80`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SandiaVoc(
    Tc: f64,
    Ee: f64,
    Voc0: f64,
    NcellSer: f64,
    DiodeFactor: f64,
    BVoc0: f64,
    mBVoc: f64,
) -> f64 {
    port::SandiaVoc(Tc, Ee, Voc0, NcellSer, DiodeFactor, BVoc0, mBVoc)
}

/// `SandiaVoc` as `rust_anything_SandiaVoc`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SandiaVoc(
    Tc: f64,
    Ee: f64,
    Voc0: f64,
    NcellSer: f64,
    DiodeFactor: f64,
    BVoc0: f64,
    mBVoc: f64,
) -> f64 {
    port::SandiaVoc(Tc, Ee, Voc0, NcellSer, DiodeFactor, BVoc0, mBVoc)
}
