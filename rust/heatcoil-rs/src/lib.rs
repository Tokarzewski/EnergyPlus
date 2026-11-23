//! C ABI for `heatcoil.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double HeatCoil(int which, int fuel, int cycling, int sensed, int plf, double avail, double mdot, double tin, double win, double tset, double qreq, double plr, double eff, double cap, double parelec, double parfuel, double c0, double c1, double c2)` (`heatcoil.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn HeatCoil(
    which: std::ffi::c_int,
    fuel: std::ffi::c_int,
    cycling: std::ffi::c_int,
    sensed: std::ffi::c_int,
    plf: std::ffi::c_int,
    avail: f64,
    mdot: f64,
    tin: f64,
    win: f64,
    tset: f64,
    qreq: f64,
    plr: f64,
    eff: f64,
    cap: f64,
    parelec: f64,
    parfuel: f64,
    c0: f64,
    c1: f64,
    c2: f64,
) -> f64 {
    port::HeatCoil(
        which, fuel, cycling, sensed, plf, avail, mdot, tin, win, tset, qreq, plr, eff, cap,
        parelec, parfuel, c0, c1, c2,
    )
}

/// `HeatCoil` as `rust_anything_HeatCoil`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_HeatCoil(
    which: std::ffi::c_int,
    fuel: std::ffi::c_int,
    cycling: std::ffi::c_int,
    sensed: std::ffi::c_int,
    plf: std::ffi::c_int,
    avail: f64,
    mdot: f64,
    tin: f64,
    win: f64,
    tset: f64,
    qreq: f64,
    plr: f64,
    eff: f64,
    cap: f64,
    parelec: f64,
    parfuel: f64,
    c0: f64,
    c1: f64,
    c2: f64,
) -> f64 {
    port::HeatCoil(
        which, fuel, cycling, sensed, plf, avail, mdot, tin, win, tset, qreq, plr, eff, cap,
        parelec, parfuel, c0, c1, c2,
    )
}
