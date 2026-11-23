//! C ABI for `waterhc.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double WaterHeatCoil(int which, int cycling, int design, double ua, double avail, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr)` (`waterhc.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WaterHeatCoil(
    which: std::ffi::c_int,
    cycling: std::ffi::c_int,
    design: std::ffi::c_int,
    ua: f64,
    avail: f64,
    mdota: f64,
    tain: f64,
    win: f64,
    mdotw: f64,
    mdotwmax: f64,
    twin: f64,
    hwin: f64,
    plr: f64,
) -> f64 {
    port::WaterHeatCoil(
        which, cycling, design, ua, avail, mdota, tain, win, mdotw, mdotwmax, twin, hwin, plr,
    )
}

/// `WaterHeatCoil` as `rust_anything_WaterHeatCoil`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WaterHeatCoil(
    which: std::ffi::c_int,
    cycling: std::ffi::c_int,
    design: std::ffi::c_int,
    ua: f64,
    avail: f64,
    mdota: f64,
    tain: f64,
    win: f64,
    mdotw: f64,
    mdotwmax: f64,
    twin: f64,
    hwin: f64,
    plr: f64,
) -> f64 {
    port::WaterHeatCoil(
        which, cycling, design, ua, avail, mdota, tain, win, mdotw, mdotwmax, twin, hwin, plr,
    )
}
