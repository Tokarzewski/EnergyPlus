//! C ABI for `watercc.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double WaterCoolCoil(int which, int cycling, int design, int hex, double avail, double uatot, double uaint, double uaext, double desvol, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr, double pb)` (`watercc.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterCoolCoil(
    which: std::ffi::c_int,
    cycling: std::ffi::c_int,
    design: std::ffi::c_int,
    hex: std::ffi::c_int,
    avail: f64,
    uatot: f64,
    uaint: f64,
    uaext: f64,
    desvol: f64,
    mdota: f64,
    tain: f64,
    win: f64,
    mdotw: f64,
    mdotwmax: f64,
    twin: f64,
    hwin: f64,
    plr: f64,
    pb: f64,
) -> f64 {
    port::WaterCoolCoil(
        which, cycling, design, hex, avail, uatot, uaint, uaext, desvol, mdota, tain, win, mdotw,
        mdotwmax, twin, hwin, plr, pb,
    )
}

/// `WaterCoolCoil` as `rust_anything_WaterCoolCoil`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterCoolCoil(
    which: std::ffi::c_int,
    cycling: std::ffi::c_int,
    design: std::ffi::c_int,
    hex: std::ffi::c_int,
    avail: f64,
    uatot: f64,
    uaint: f64,
    uaext: f64,
    desvol: f64,
    mdota: f64,
    tain: f64,
    win: f64,
    mdotw: f64,
    mdotwmax: f64,
    twin: f64,
    hwin: f64,
    plr: f64,
    pb: f64,
) -> f64 {
    port::WaterCoolCoil(
        which, cycling, design, hex, avail, uatot, uaint, uaext, desvol, mdota, tain, win, mdotw,
        mdotwmax, twin, hwin, plr, pb,
    )
}
