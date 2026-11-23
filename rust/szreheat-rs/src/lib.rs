//! C ABI for `szreheat.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SZReheat(int hasoa, int hasfan, int deadband, double oafrac, double zonem, double inw, double load, double tocool, double toheat, double zonet, double retw, double reth, double oaw, double oah, double loopt, double fanin, double fanout, double mint, double maxt)` (`szreheat.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SZReheat(
    hasoa: std::ffi::c_int,
    hasfan: std::ffi::c_int,
    deadband: std::ffi::c_int,
    oafrac: f64,
    zonem: f64,
    inw: f64,
    load: f64,
    tocool: f64,
    toheat: f64,
    zonet: f64,
    retw: f64,
    reth: f64,
    oaw: f64,
    oah: f64,
    loopt: f64,
    fanin: f64,
    fanout: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::SZReheat(
        hasoa, hasfan, deadband, oafrac, zonem, inw, load, tocool, toheat, zonet, retw, reth, oaw,
        oah, loopt, fanin, fanout, mint, maxt,
    )
}

/// `SZReheat` as `rust_anything_SZReheat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SZReheat(
    hasoa: std::ffi::c_int,
    hasfan: std::ffi::c_int,
    deadband: std::ffi::c_int,
    oafrac: f64,
    zonem: f64,
    inw: f64,
    load: f64,
    tocool: f64,
    toheat: f64,
    zonet: f64,
    retw: f64,
    reth: f64,
    oaw: f64,
    oah: f64,
    loopt: f64,
    fanin: f64,
    fanout: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::SZReheat(
        hasoa, hasfan, deadband, oafrac, zonem, inw, load, tocool, toheat, zonet, retw, reth, oaw,
        oah, loopt, fanin, fanout, mint, maxt,
    )
}
