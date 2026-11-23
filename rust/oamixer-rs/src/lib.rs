//! C ABI for `oamixer.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double OAMixer(int which, double rett, double retw, double reth, double retp, double retm, double oat, double oaw, double oah, double oap, double oam, double relm)` (`oamixer.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn OAMixer(
    which: std::ffi::c_int,
    rett: f64,
    retw: f64,
    reth: f64,
    retp: f64,
    retm: f64,
    oat: f64,
    oaw: f64,
    oah: f64,
    oap: f64,
    oam: f64,
    relm: f64,
) -> f64 {
    port::OAMixer(
        which, rett, retw, reth, retp, retm, oat, oaw, oah, oap, oam, relm,
    )
}

/// `OAMixer` as `rust_anything_OAMixer`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_OAMixer(
    which: std::ffi::c_int,
    rett: f64,
    retw: f64,
    reth: f64,
    retp: f64,
    retm: f64,
    oat: f64,
    oaw: f64,
    oah: f64,
    oap: f64,
    oam: f64,
    relm: f64,
) -> f64 {
    port::OAMixer(
        which, rett, retw, reth, retp, retm, oat, oaw, oah, oap, oam, relm,
    )
}
