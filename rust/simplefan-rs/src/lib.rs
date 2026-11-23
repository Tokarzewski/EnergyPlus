//! C ABI for `simplefan.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SimpleFan(int which, int kind, int turn, int ems, double avail, double mdot, double mdotmax, double mdotmin, double volmax, double volmin, double dp, double eff, double moteff, double motinair, double rho, double tin, double win, double plf, double c0, double c1, double c2, double c3, double c4, double pr, double emsflow, double emspress, double emseff)` (`simplefan.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SimpleFan(
    which: std::ffi::c_int,
    kind: std::ffi::c_int,
    turn: std::ffi::c_int,
    ems: std::ffi::c_int,
    avail: f64,
    mdot: f64,
    mdotmax: f64,
    mdotmin: f64,
    volmax: f64,
    volmin: f64,
    dp: f64,
    eff: f64,
    moteff: f64,
    motinair: f64,
    rho: f64,
    tin: f64,
    win: f64,
    plf: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    pr: f64,
    emsflow: f64,
    emspress: f64,
    emseff: f64,
) -> f64 {
    port::SimpleFan(
        which, kind, turn, ems, avail, mdot, mdotmax, mdotmin, volmax, volmin, dp, eff, moteff,
        motinair, rho, tin, win, plf, c0, c1, c2, c3, c4, pr, emsflow, emspress, emseff,
    )
}

/// `SimpleFan` as `rust_anything_SimpleFan`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SimpleFan(
    which: std::ffi::c_int,
    kind: std::ffi::c_int,
    turn: std::ffi::c_int,
    ems: std::ffi::c_int,
    avail: f64,
    mdot: f64,
    mdotmax: f64,
    mdotmin: f64,
    volmax: f64,
    volmin: f64,
    dp: f64,
    eff: f64,
    moteff: f64,
    motinair: f64,
    rho: f64,
    tin: f64,
    win: f64,
    plf: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    pr: f64,
    emsflow: f64,
    emspress: f64,
    emseff: f64,
) -> f64 {
    port::SimpleFan(
        which, kind, turn, ems, avail, mdot, mdotmax, mdotmin, volmax, volmin, dp, eff, moteff,
        motinair, rho, tin, win, plf, c0, c1, c2, c3, c4, pr, emsflow, emspress, emseff,
    )
}
