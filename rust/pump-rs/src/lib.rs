//! C ABI for `pump.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double Pump(int which, int ptype, int lock, int series, int press, int ems, int nbank, double flowreq, double mmax, double mmin, double nomvol, double nompow, double c0, double c1, double c2, double c3, double motor, double peff, double frac, double pdrop, double emsp, double tin, double pin, double mdotin, double hwmax, double minav, double maxav)` (`pump.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Pump(
    which: std::ffi::c_int,
    ptype: std::ffi::c_int,
    lock: std::ffi::c_int,
    series: std::ffi::c_int,
    press: std::ffi::c_int,
    ems: std::ffi::c_int,
    nbank: std::ffi::c_int,
    flowreq: f64,
    mmax: f64,
    mmin: f64,
    nomvol: f64,
    nompow: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    motor: f64,
    peff: f64,
    frac: f64,
    pdrop: f64,
    emsp: f64,
    tin: f64,
    pin: f64,
    mdotin: f64,
    hwmax: f64,
    minav: f64,
    maxav: f64,
) -> f64 {
    port::Pump(
        which, ptype, lock, series, press, ems, nbank, flowreq, mmax, mmin, nomvol, nompow, c0, c1,
        c2, c3, motor, peff, frac, pdrop, emsp, tin, pin, mdotin, hwmax, minav, maxav,
    )
}

/// `Pump` as `rust_anything_Pump`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Pump(
    which: std::ffi::c_int,
    ptype: std::ffi::c_int,
    lock: std::ffi::c_int,
    series: std::ffi::c_int,
    press: std::ffi::c_int,
    ems: std::ffi::c_int,
    nbank: std::ffi::c_int,
    flowreq: f64,
    mmax: f64,
    mmin: f64,
    nomvol: f64,
    nompow: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    motor: f64,
    peff: f64,
    frac: f64,
    pdrop: f64,
    emsp: f64,
    tin: f64,
    pin: f64,
    mdotin: f64,
    hwmax: f64,
    minav: f64,
    maxav: f64,
) -> f64 {
    port::Pump(
        which, ptype, lock, series, press, ems, nbank, flowreq, mmax, mmin, nomvol, nompow, c0, c1,
        c2, c3, motor, peff, frac, pdrop, emsp, tin, pin, mdotin, hwmax, minav, maxav,
    )
}
