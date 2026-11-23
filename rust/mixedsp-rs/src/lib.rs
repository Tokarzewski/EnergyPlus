//! C ABI for `mixedsp.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double MixedSP(int which, int layout, double refsp, double reft, double fanin, double fanout, double coilin, double coilout, double mincoil, double oat)` (`mixedsp.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn MixedSP(
    which: std::ffi::c_int,
    layout: std::ffi::c_int,
    refsp: f64,
    reft: f64,
    fanin: f64,
    fanout: f64,
    coilin: f64,
    coilout: f64,
    mincoil: f64,
    oat: f64,
) -> f64 {
    port::MixedSP(
        which, layout, refsp, reft, fanin, fanout, coilin, coilout, mincoil, oat,
    )
}

/// `MixedSP` as `rust_anything_MixedSP`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_MixedSP(
    which: std::ffi::c_int,
    layout: std::ffi::c_int,
    refsp: f64,
    reft: f64,
    fanin: f64,
    fanout: f64,
    coilin: f64,
    coilout: f64,
    mincoil: f64,
    oat: f64,
) -> f64 {
    port::MixedSP(
        which, layout, refsp, reft, fanin, fanout, coilin, coilout, mincoil, oat,
    )
}
