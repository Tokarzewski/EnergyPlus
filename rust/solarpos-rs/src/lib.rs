//! C ABI for `solarpos.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SkyEmissivity(int model, double osky, double drybulb, double dewpoint, double relhum)` (`solarpos.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SkyEmissivity(
    model: std::ffi::c_int,
    osky: f64,
    drybulb: f64,
    dewpoint: f64,
    relhum: f64,
) -> f64 {
    port::SkyEmissivity(model, osky, drybulb, dewpoint, relhum)
}

/// `SkyEmissivity` as `rust_anything_SkyEmissivity`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SkyEmissivity(
    model: std::ffi::c_int,
    osky: f64,
    drybulb: f64,
    dewpoint: f64,
    relhum: f64,
) -> f64 {
    port::SkyEmissivity(model, osky, drybulb, dewpoint, relhum)
}

/// `double TauModel(int which, int tau2017, double etr, double cosZen, double taub, double taud)` (`solarpos.c:19`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn TauModel(
    which: std::ffi::c_int,
    tau2017: std::ffi::c_int,
    etr: f64,
    cosZen: f64,
    taub: f64,
    taud: f64,
) -> f64 {
    port::TauModel(which, tau2017, etr, cosZen, taub, taud)
}

/// `TauModel` as `rust_anything_TauModel`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_TauModel(
    which: std::ffi::c_int,
    tau2017: std::ffi::c_int,
    etr: f64,
    cosZen: f64,
    taub: f64,
    taud: f64,
) -> f64 {
    port::TauModel(which, tau2017, etr, cosZen, taub, taud)
}

/// `double DailySolar(int which, int doy, double lat)` (`solarpos.c:24`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn DailySolar(which: std::ffi::c_int, doy: std::ffi::c_int, lat: f64) -> f64 {
    port::DailySolar(which, doy, lat)
}

/// `DailySolar` as `rust_anything_DailySolar`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_DailySolar(
    which: std::ffi::c_int,
    doy: std::ffi::c_int,
    lat: f64,
) -> f64 {
    port::DailySolar(which, doy, lat)
}

/// `double SunCosines(int which, double time, double eqt, double sind, double cosd, double lat, double lon, double tzm)` (`solarpos.c:29`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SunCosines(
    which: std::ffi::c_int,
    time: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> f64 {
    port::SunCosines(which, time, eqt, sind, cosd, lat, lon, tzm)
}

/// `SunCosines` as `rust_anything_SunCosines`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SunCosines(
    which: std::ffi::c_int,
    time: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> f64 {
    port::SunCosines(which, time, eqt, sind, cosd, lat, lon, tzm)
}

/// `double SunUpDown(int which, int tsInHour, double curTime, double ts1off, double eqt, double sind, double cosd, double lat, double lon, double tzm)` (`solarpos.c:34`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SunUpDown(
    which: std::ffi::c_int,
    tsInHour: std::ffi::c_int,
    curTime: f64,
    ts1off: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> f64 {
    port::SunUpDown(
        which, tsInHour, curTime, ts1off, eqt, sind, cosd, lat, lon, tzm,
    )
}

/// `SunUpDown` as `rust_anything_SunUpDown`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SunUpDown(
    which: std::ffi::c_int,
    tsInHour: std::ffi::c_int,
    curTime: f64,
    ts1off: f64,
    eqt: f64,
    sind: f64,
    cosd: f64,
    lat: f64,
    lon: f64,
    tzm: f64,
) -> f64 {
    port::SunUpDown(
        which, tsInHour, curTime, ts1off, eqt, sind, cosd, lat, lon, tzm,
    )
}

/// `double WaterMains(int doy, double lat, double annualAvg, double maxDiff, double mult, double offset)` (`solarpos.c:39`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterMains(
    doy: std::ffi::c_int,
    lat: f64,
    annualAvg: f64,
    maxDiff: f64,
    mult: f64,
    offset: f64,
) -> f64 {
    port::WaterMains(doy, lat, annualAvg, maxDiff, mult, offset)
}

/// `WaterMains` as `rust_anything_WaterMains`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterMains(
    doy: std::ffi::c_int,
    lat: f64,
    annualAvg: f64,
    maxDiff: f64,
    mult: f64,
    offset: f64,
) -> f64 {
    port::WaterMains(doy, lat, annualAvg, maxDiff, mult, offset)
}

/// `double LumEff(int which, int month, double cosZen, double dif, double beam, double elev, double dewpoint)` (`solarpos.c:44`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn LumEff(
    which: std::ffi::c_int,
    month: std::ffi::c_int,
    cosZen: f64,
    dif: f64,
    beam: f64,
    elev: f64,
    dewpoint: f64,
) -> f64 {
    port::LumEff(which, month, cosZen, dif, beam, elev, dewpoint)
}

/// `LumEff` as `rust_anything_LumEff`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_LumEff(
    which: std::ffi::c_int,
    month: std::ffi::c_int,
    cosZen: f64,
    dif: f64,
    beam: f64,
    elev: f64,
    dewpoint: f64,
) -> f64 {
    port::LumEff(which, month, cosZen, dif, beam, elev, dewpoint)
}
