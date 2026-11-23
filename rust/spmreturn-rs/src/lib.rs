//! C ABI for `spmreturn.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double RetWater(int hot, int ttype, double mdot, double tsup, double tret, double target, double retsp, double mint, double maxt)` (`spmreturn.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn RetWater(
    hot: std::ffi::c_int,
    ttype: std::ffi::c_int,
    mdot: f64,
    tsup: f64,
    tret: f64,
    target: f64,
    retsp: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::RetWater(hot, ttype, mdot, tsup, tret, target, retsp, mint, maxt)
}

/// `RetWater` as `rust_anything_RetWater`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_RetWater(
    hot: std::ffi::c_int,
    ttype: std::ffi::c_int,
    mdot: f64,
    tsup: f64,
    tret: f64,
    target: f64,
    retsp: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::RetWater(hot, ttype, mdot, tsup, tret, target, retsp, mint, maxt)
}

/// `double SysNode(int ctrl, double ref, double lowref, double highref, double splow, double sphigh)` (`spmreturn.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SysNode(
    ctrl: std::ffi::c_int,
    r#ref: f64,
    lowref: f64,
    highref: f64,
    splow: f64,
    sphigh: f64,
) -> f64 {
    port::SysNode(ctrl, r#ref, lowref, highref, splow, sphigh)
}

/// `SysNode` as `rust_anything_SysNode`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SysNode(
    ctrl: std::ffi::c_int,
    r#ref: f64,
    lowref: f64,
    highref: f64,
    splow: f64,
    sphigh: f64,
) -> f64 {
    port::SysNode(ctrl, r#ref, lowref, highref, splow, sphigh)
}
