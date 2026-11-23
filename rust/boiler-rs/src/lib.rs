//! C ABI for `boiler.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double Boiler(int which, int lock, int flowmode, int dual, int series, int run, int curve, int tempmode, double myload, double nomcap, double nomeff, double minplr, double maxplr, double tlimit, double desmdot, double parelec, double parfuel, double tin, double mdotin, double hwmax, double minav, double maxav, double tset, double tsetlo, double c0, double c1, double c2, double c3, double c4, double c5)` (`boiler.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Boiler(
    which: std::ffi::c_int,
    lock: std::ffi::c_int,
    flowmode: std::ffi::c_int,
    dual: std::ffi::c_int,
    series: std::ffi::c_int,
    run: std::ffi::c_int,
    curve: std::ffi::c_int,
    tempmode: std::ffi::c_int,
    myload: f64,
    nomcap: f64,
    nomeff: f64,
    minplr: f64,
    maxplr: f64,
    tlimit: f64,
    desmdot: f64,
    parelec: f64,
    parfuel: f64,
    tin: f64,
    mdotin: f64,
    hwmax: f64,
    minav: f64,
    maxav: f64,
    tset: f64,
    tsetlo: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
) -> f64 {
    port::Boiler(
        which, lock, flowmode, dual, series, run, curve, tempmode, myload, nomcap, nomeff, minplr,
        maxplr, tlimit, desmdot, parelec, parfuel, tin, mdotin, hwmax, minav, maxav, tset, tsetlo,
        c0, c1, c2, c3, c4, c5,
    )
}

/// `Boiler` as `rust_anything_Boiler`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Boiler(
    which: std::ffi::c_int,
    lock: std::ffi::c_int,
    flowmode: std::ffi::c_int,
    dual: std::ffi::c_int,
    series: std::ffi::c_int,
    run: std::ffi::c_int,
    curve: std::ffi::c_int,
    tempmode: std::ffi::c_int,
    myload: f64,
    nomcap: f64,
    nomeff: f64,
    minplr: f64,
    maxplr: f64,
    tlimit: f64,
    desmdot: f64,
    parelec: f64,
    parfuel: f64,
    tin: f64,
    mdotin: f64,
    hwmax: f64,
    minav: f64,
    maxav: f64,
    tset: f64,
    tsetlo: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
) -> f64 {
    port::Boiler(
        which, lock, flowmode, dual, series, run, curve, tempmode, myload, nomcap, nomeff, minplr,
        maxplr, tlimit, desmdot, parelec, parfuel, tin, mdotin, hwmax, minav, maxav, tset, tsetlo,
        c0, c1, c2, c3, c4, c5,
    )
}
