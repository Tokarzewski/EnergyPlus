//! C ABI for `plantiface.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double PlantIface(int which, int side, int newstep, int haspress, int oot0, double tin, double mdot, double minav, double maxav, double q, double p, double otemp, double omdot, double pump, double mass, double last, double prev, double tssec, double hist)` (`plantiface.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PlantIface(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    newstep: std::ffi::c_int,
    haspress: std::ffi::c_int,
    oot0: std::ffi::c_int,
    tin: f64,
    mdot: f64,
    minav: f64,
    maxav: f64,
    q: f64,
    p: f64,
    otemp: f64,
    omdot: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    tssec: f64,
    hist: f64,
) -> f64 {
    port::PlantIface(
        which, side, newstep, haspress, oot0, tin, mdot, minav, maxav, q, p, otemp, omdot, pump,
        mass, last, prev, tssec, hist,
    )
}

/// `PlantIface` as `rust_anything_PlantIface`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PlantIface(
    which: std::ffi::c_int,
    side: std::ffi::c_int,
    newstep: std::ffi::c_int,
    haspress: std::ffi::c_int,
    oot0: std::ffi::c_int,
    tin: f64,
    mdot: f64,
    minav: f64,
    maxav: f64,
    q: f64,
    p: f64,
    otemp: f64,
    omdot: f64,
    pump: f64,
    mass: f64,
    last: f64,
    prev: f64,
    tssec: f64,
    hist: f64,
) -> f64 {
    port::PlantIface(
        which, side, newstep, haspress, oot0, tin, mdot, minav, maxav, q, p, otemp, omdot, pump,
        mass, last, prev, tssec, hist,
    )
}
