//! C ABI for `psychrometrics.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double PsyRhoAirFnPbTdbW(double pb, double tdb, double dw)` (`psychrometrics.c:44`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhoAirFnPbTdbW(pb: f64, tdb: f64, dw: f64) -> f64 {
    port::PsyRhoAirFnPbTdbW(pb, tdb, dw)
}

/// `PsyRhoAirFnPbTdbW` as `rust_anything_PsyRhoAirFnPbTdbW`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhoAirFnPbTdbW(pb: f64, tdb: f64, dw: f64) -> f64 {
    port::PsyRhoAirFnPbTdbW(pb, tdb, dw)
}

/// `double PsyRhoAirFnPbTdbW_fast(double pb, double tdb, double dw)` (`psychrometrics.c:49`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhoAirFnPbTdbW_fast(pb: f64, tdb: f64, dw: f64) -> f64 {
    port::PsyRhoAirFnPbTdbW_fast(pb, tdb, dw)
}

/// `PsyRhoAirFnPbTdbW_fast` as `rust_anything_PsyRhoAirFnPbTdbW_fast`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhoAirFnPbTdbW_fast(pb: f64, tdb: f64, dw: f64) -> f64 {
    port::PsyRhoAirFnPbTdbW_fast(pb, tdb, dw)
}

/// `double PsyHfgAirFnWTdb(double w, double T)` (`psychrometrics.c:54`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyHfgAirFnWTdb(w: f64, T: f64) -> f64 {
    port::PsyHfgAirFnWTdb(w, T)
}

/// `PsyHfgAirFnWTdb` as `rust_anything_PsyHfgAirFnWTdb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyHfgAirFnWTdb(w: f64, T: f64) -> f64 {
    port::PsyHfgAirFnWTdb(w, T)
}

/// `double PsyHgAirFnWTdb(double w, double T)` (`psychrometrics.c:59`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyHgAirFnWTdb(w: f64, T: f64) -> f64 {
    port::PsyHgAirFnWTdb(w, T)
}

/// `PsyHgAirFnWTdb` as `rust_anything_PsyHgAirFnWTdb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyHgAirFnWTdb(w: f64, T: f64) -> f64 {
    port::PsyHgAirFnWTdb(w, T)
}

/// `double PsyHFnTdbW(double TDB, double dW)` (`psychrometrics.c:64`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyHFnTdbW(TDB: f64, dW: f64) -> f64 {
    port::PsyHFnTdbW(TDB, dW)
}

/// `PsyHFnTdbW` as `rust_anything_PsyHFnTdbW`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyHFnTdbW(TDB: f64, dW: f64) -> f64 {
    port::PsyHFnTdbW(TDB, dW)
}

/// `double PsyHFnTdbW_fast(double TDB, double dW)` (`psychrometrics.c:69`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyHFnTdbW_fast(TDB: f64, dW: f64) -> f64 {
    port::PsyHFnTdbW_fast(TDB, dW)
}

/// `PsyHFnTdbW_fast` as `rust_anything_PsyHFnTdbW_fast`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyHFnTdbW_fast(TDB: f64, dW: f64) -> f64 {
    port::PsyHFnTdbW_fast(TDB, dW)
}

/// `double PsyCpAirFnW(double dw)` (`psychrometrics.c:74`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyCpAirFnW(dw: f64) -> f64 {
    port::PsyCpAirFnW(dw)
}

/// `PsyCpAirFnW` as `rust_anything_PsyCpAirFnW`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyCpAirFnW(dw: f64) -> f64 {
    port::PsyCpAirFnW(dw)
}

/// `double PsyCpAirFnW_fast(double dw)` (`psychrometrics.c:79`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyCpAirFnW_fast(dw: f64) -> f64 {
    port::PsyCpAirFnW_fast(dw)
}

/// `PsyCpAirFnW_fast` as `rust_anything_PsyCpAirFnW_fast`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyCpAirFnW_fast(dw: f64) -> f64 {
    port::PsyCpAirFnW_fast(dw)
}

/// `double PsyTdbFnHW(double H, double dW)` (`psychrometrics.c:84`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTdbFnHW(H: f64, dW: f64) -> f64 {
    port::PsyTdbFnHW(H, dW)
}

/// `PsyTdbFnHW` as `rust_anything_PsyTdbFnHW`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTdbFnHW(H: f64, dW: f64) -> f64 {
    port::PsyTdbFnHW(H, dW)
}

/// `double PsyRhovFnTdbRhLBnd0C(double Tdb, double RH)` (`psychrometrics.c:89`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhovFnTdbRhLBnd0C(Tdb: f64, RH: f64) -> f64 {
    port::PsyRhovFnTdbRhLBnd0C(Tdb, RH)
}

/// `PsyRhovFnTdbRhLBnd0C` as `rust_anything_PsyRhovFnTdbRhLBnd0C`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhovFnTdbRhLBnd0C(Tdb: f64, RH: f64) -> f64 {
    port::PsyRhovFnTdbRhLBnd0C(Tdb, RH)
}

/// `double PsyRhovFnTdbWPb(double Tdb, double dW, double PB)` (`psychrometrics.c:94`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhovFnTdbWPb(Tdb: f64, dW: f64, PB: f64) -> f64 {
    port::PsyRhovFnTdbWPb(Tdb, dW, PB)
}

/// `PsyRhovFnTdbWPb` as `rust_anything_PsyRhovFnTdbWPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhovFnTdbWPb(Tdb: f64, dW: f64, PB: f64) -> f64 {
    port::PsyRhovFnTdbWPb(Tdb, dW, PB)
}

/// `double PsyRhovFnTdbWPb_fast(double Tdb, double dW, double PB)` (`psychrometrics.c:99`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhovFnTdbWPb_fast(Tdb: f64, dW: f64, PB: f64) -> f64 {
    port::PsyRhovFnTdbWPb_fast(Tdb, dW, PB)
}

/// `PsyRhovFnTdbWPb_fast` as `rust_anything_PsyRhovFnTdbWPb_fast`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhovFnTdbWPb_fast(Tdb: f64, dW: f64, PB: f64) -> f64 {
    port::PsyRhovFnTdbWPb_fast(Tdb, dW, PB)
}

/// `double PsyRhFnTdbRhovLBnd0C(double Tdb, double Rhovapor)` (`psychrometrics.c:104`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhFnTdbRhovLBnd0C(Tdb: f64, Rhovapor: f64) -> f64 {
    port::PsyRhFnTdbRhovLBnd0C(Tdb, Rhovapor)
}

/// `PsyRhFnTdbRhovLBnd0C` as `rust_anything_PsyRhFnTdbRhovLBnd0C`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhFnTdbRhovLBnd0C(Tdb: f64, Rhovapor: f64) -> f64 {
    port::PsyRhFnTdbRhovLBnd0C(Tdb, Rhovapor)
}

/// `double PsyTwbFnTdbWPb(double TDB, double dW, double Patm)` (`psychrometrics.c:109`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTwbFnTdbWPb(TDB: f64, dW: f64, Patm: f64) -> f64 {
    port::PsyTwbFnTdbWPb(TDB, dW, Patm)
}

/// `PsyTwbFnTdbWPb` as `rust_anything_PsyTwbFnTdbWPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTwbFnTdbWPb(TDB: f64, dW: f64, Patm: f64) -> f64 {
    port::PsyTwbFnTdbWPb(TDB, dW, Patm)
}

/// `double PsyTwbFnTdbWPb_raw(double TDB, double dW, double Patm)` (`psychrometrics.c:114`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTwbFnTdbWPb_raw(TDB: f64, dW: f64, Patm: f64) -> f64 {
    port::PsyTwbFnTdbWPb_raw(TDB, dW, Patm)
}

/// `PsyTwbFnTdbWPb_raw` as `rust_anything_PsyTwbFnTdbWPb_raw`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTwbFnTdbWPb_raw(TDB: f64, dW: f64, Patm: f64) -> f64 {
    port::PsyTwbFnTdbWPb_raw(TDB, dW, Patm)
}

/// `double PsyVFnTdbWPb(double TDB, double dW, double PB)` (`psychrometrics.c:119`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyVFnTdbWPb(TDB: f64, dW: f64, PB: f64) -> f64 {
    port::PsyVFnTdbWPb(TDB, dW, PB)
}

/// `PsyVFnTdbWPb` as `rust_anything_PsyVFnTdbWPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyVFnTdbWPb(TDB: f64, dW: f64, PB: f64) -> f64 {
    port::PsyVFnTdbWPb(TDB, dW, PB)
}

/// `double PsyWFnTdbH(double TDB, double H)` (`psychrometrics.c:124`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyWFnTdbH(TDB: f64, H: f64) -> f64 {
    port::PsyWFnTdbH(TDB, H)
}

/// `PsyWFnTdbH` as `rust_anything_PsyWFnTdbH`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyWFnTdbH(TDB: f64, H: f64) -> f64 {
    port::PsyWFnTdbH(TDB, H)
}

/// `double PsyPsatFnTemp(double T)` (`psychrometrics.c:129`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyPsatFnTemp(T: f64) -> f64 {
    port::PsyPsatFnTemp(T)
}

/// `PsyPsatFnTemp` as `rust_anything_PsyPsatFnTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyPsatFnTemp(T: f64) -> f64 {
    port::PsyPsatFnTemp(T)
}

/// `double PsyPsatFnTemp_raw(double T)` (`psychrometrics.c:134`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyPsatFnTemp_raw(T: f64) -> f64 {
    port::PsyPsatFnTemp_raw(T)
}

/// `PsyPsatFnTemp_raw` as `rust_anything_PsyPsatFnTemp_raw`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyPsatFnTemp_raw(T: f64) -> f64 {
    port::PsyPsatFnTemp_raw(T)
}

/// `double PsyTsatFnHPb(double H, double PB)` (`psychrometrics.c:139`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTsatFnHPb(H: f64, PB: f64) -> f64 {
    port::PsyTsatFnHPb(H, PB)
}

/// `PsyTsatFnHPb` as `rust_anything_PsyTsatFnHPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTsatFnHPb(H: f64, PB: f64) -> f64 {
    port::PsyTsatFnHPb(H, PB)
}

/// `double PsyTsatFnHPb_raw(double H, double PB)` (`psychrometrics.c:144`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTsatFnHPb_raw(H: f64, PB: f64) -> f64 {
    port::PsyTsatFnHPb_raw(H, PB)
}

/// `PsyTsatFnHPb_raw` as `rust_anything_PsyTsatFnHPb_raw`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTsatFnHPb_raw(H: f64, PB: f64) -> f64 {
    port::PsyTsatFnHPb_raw(H, PB)
}

/// `double PsyRhovFnTdbRh(double Tdb, double RH)` (`psychrometrics.c:149`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhovFnTdbRh(Tdb: f64, RH: f64) -> f64 {
    port::PsyRhovFnTdbRh(Tdb, RH)
}

/// `PsyRhovFnTdbRh` as `rust_anything_PsyRhovFnTdbRh`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhovFnTdbRh(Tdb: f64, RH: f64) -> f64 {
    port::PsyRhovFnTdbRh(Tdb, RH)
}

/// `double PsyRhFnTdbRhov(double Tdb, double Rhovapor)` (`psychrometrics.c:154`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhFnTdbRhov(Tdb: f64, Rhovapor: f64) -> f64 {
    port::PsyRhFnTdbRhov(Tdb, Rhovapor)
}

/// `PsyRhFnTdbRhov` as `rust_anything_PsyRhFnTdbRhov`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhFnTdbRhov(Tdb: f64, Rhovapor: f64) -> f64 {
    port::PsyRhFnTdbRhov(Tdb, Rhovapor)
}

/// `double PsyRhFnTdbWPb(double TDB, double dW, double PB)` (`psychrometrics.c:159`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyRhFnTdbWPb(TDB: f64, dW: f64, PB: f64) -> f64 {
    port::PsyRhFnTdbWPb(TDB, dW, PB)
}

/// `PsyRhFnTdbWPb` as `rust_anything_PsyRhFnTdbWPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyRhFnTdbWPb(TDB: f64, dW: f64, PB: f64) -> f64 {
    port::PsyRhFnTdbWPb(TDB, dW, PB)
}

/// `double PsyWFnTdpPb(double TDP, double PB)` (`psychrometrics.c:164`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyWFnTdpPb(TDP: f64, PB: f64) -> f64 {
    port::PsyWFnTdpPb(TDP, PB)
}

/// `PsyWFnTdpPb` as `rust_anything_PsyWFnTdpPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyWFnTdpPb(TDP: f64, PB: f64) -> f64 {
    port::PsyWFnTdpPb(TDP, PB)
}

/// `double PsyWFnTdbRhPb(double TDB, double RH, double PB)` (`psychrometrics.c:169`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyWFnTdbRhPb(TDB: f64, RH: f64, PB: f64) -> f64 {
    port::PsyWFnTdbRhPb(TDB, RH, PB)
}

/// `PsyWFnTdbRhPb` as `rust_anything_PsyWFnTdbRhPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyWFnTdbRhPb(TDB: f64, RH: f64, PB: f64) -> f64 {
    port::PsyWFnTdbRhPb(TDB, RH, PB)
}

/// `double PsyWFnTdbTwbPb(double TDB, double TWBin, double PB)` (`psychrometrics.c:174`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyWFnTdbTwbPb(TDB: f64, TWBin: f64, PB: f64) -> f64 {
    port::PsyWFnTdbTwbPb(TDB, TWBin, PB)
}

/// `PsyWFnTdbTwbPb` as `rust_anything_PsyWFnTdbTwbPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyWFnTdbTwbPb(TDB: f64, TWBin: f64, PB: f64) -> f64 {
    port::PsyWFnTdbTwbPb(TDB, TWBin, PB)
}

/// `double PsyHFnTdbRhPb(double TDB, double RH, double PB)` (`psychrometrics.c:179`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyHFnTdbRhPb(TDB: f64, RH: f64, PB: f64) -> f64 {
    port::PsyHFnTdbRhPb(TDB, RH, PB)
}

/// `PsyHFnTdbRhPb` as `rust_anything_PsyHFnTdbRhPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyHFnTdbRhPb(TDB: f64, RH: f64, PB: f64) -> f64 {
    port::PsyHFnTdbRhPb(TDB, RH, PB)
}

/// `double PsyTsatFnPb(double Press)` (`psychrometrics.c:184`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTsatFnPb(Press: f64) -> f64 {
    port::PsyTsatFnPb(Press)
}

/// `PsyTsatFnPb` as `rust_anything_PsyTsatFnPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTsatFnPb(Press: f64) -> f64 {
    port::PsyTsatFnPb(Press)
}

/// `double PsyTsatFnPb_raw(double Press)` (`psychrometrics.c:189`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTsatFnPb_raw(Press: f64) -> f64 {
    port::PsyTsatFnPb_raw(Press)
}

/// `PsyTsatFnPb_raw` as `rust_anything_PsyTsatFnPb_raw`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTsatFnPb_raw(Press: f64) -> f64 {
    port::PsyTsatFnPb_raw(Press)
}

/// `double PsyTdpFnWPb(double W, double PB)` (`psychrometrics.c:194`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTdpFnWPb(W: f64, PB: f64) -> f64 {
    port::PsyTdpFnWPb(W, PB)
}

/// `PsyTdpFnWPb` as `rust_anything_PsyTdpFnWPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTdpFnWPb(W: f64, PB: f64) -> f64 {
    port::PsyTdpFnWPb(W, PB)
}

/// `double PsyTdpFnTdbTwbPb(double TDB, double TWB, double PB)` (`psychrometrics.c:199`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyTdpFnTdbTwbPb(TDB: f64, TWB: f64, PB: f64) -> f64 {
    port::PsyTdpFnTdbTwbPb(TDB, TWB, PB)
}

/// `PsyTdpFnTdbTwbPb` as `rust_anything_PsyTdpFnTdbTwbPb`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyTdpFnTdbTwbPb(TDB: f64, TWB: f64, PB: f64) -> f64 {
    port::PsyTdpFnTdbTwbPb(TDB, TWB, PB)
}

/// `double CPCW(double Temperature)` (`psychrometrics.c:204`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CPCW(Temperature: f64) -> f64 {
    port::CPCW(Temperature)
}

/// `CPCW` as `rust_anything_CPCW`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CPCW(Temperature: f64) -> f64 {
    port::CPCW(Temperature)
}

/// `double CPHW(double Temperature)` (`psychrometrics.c:209`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CPHW(Temperature: f64) -> f64 {
    port::CPHW(Temperature)
}

/// `CPHW` as `rust_anything_CPHW`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CPHW(Temperature: f64) -> f64 {
    port::CPHW(Temperature)
}

/// `double RhoH2O(double TB)` (`psychrometrics.c:214`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn RhoH2O(TB: f64) -> f64 {
    port::RhoH2O(TB)
}

/// `RhoH2O` as `rust_anything_RhoH2O`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_RhoH2O(TB: f64) -> f64 {
    port::RhoH2O(TB)
}

/// `double PsyDeltaHSenFnTdb2Tdb1W(double TDB2, double TDB1, double W)` (`psychrometrics.c:219`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyDeltaHSenFnTdb2Tdb1W(TDB2: f64, TDB1: f64, W: f64) -> f64 {
    port::PsyDeltaHSenFnTdb2Tdb1W(TDB2, TDB1, W)
}

/// `PsyDeltaHSenFnTdb2Tdb1W` as `rust_anything_PsyDeltaHSenFnTdb2Tdb1W`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyDeltaHSenFnTdb2Tdb1W(TDB2: f64, TDB1: f64, W: f64) -> f64 {
    port::PsyDeltaHSenFnTdb2Tdb1W(TDB2, TDB1, W)
}

/// `double PsyDeltaHSenFnTdb2W2Tdb1W1(double TDB2, double W2, double TDB1, double W1)` (`psychrometrics.c:224`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PsyDeltaHSenFnTdb2W2Tdb1W1(TDB2: f64, W2: f64, TDB1: f64, W1: f64) -> f64 {
    port::PsyDeltaHSenFnTdb2W2Tdb1W1(TDB2, W2, TDB1, W1)
}

/// `PsyDeltaHSenFnTdb2W2Tdb1W1` as `rust_anything_PsyDeltaHSenFnTdb2W2Tdb1W1`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PsyDeltaHSenFnTdb2W2Tdb1W1(
    TDB2: f64,
    W2: f64,
    TDB1: f64,
    W1: f64,
) -> f64 {
    port::PsyDeltaHSenFnTdb2W2Tdb1W1(TDB2, W2, TDB1, W1)
}
