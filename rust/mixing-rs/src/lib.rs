//! C ABI for `mixing.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double Mixing(int which, int nmix, int hybrid, double saved, double td, int hastd, int hmint, double mint, int hmaxt, double maxt, int hmins, double mins, int hmaxs, double maxs, int hmino, double mino, int hmaxo, double maxo, int systs, double mat1, double hum1, double xmpt1, double wprev1, double mat2, double hum2, double xmpt2, double wprev2, double outdb, double outbaro)` (`mixing.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Mixing(
    which: std::ffi::c_int,
    nmix: std::ffi::c_int,
    hybrid: std::ffi::c_int,
    saved: f64,
    td: f64,
    hastd: std::ffi::c_int,
    hmint: std::ffi::c_int,
    mint: f64,
    hmaxt: std::ffi::c_int,
    maxt: f64,
    hmins: std::ffi::c_int,
    mins: f64,
    hmaxs: std::ffi::c_int,
    maxs: f64,
    hmino: std::ffi::c_int,
    mino: f64,
    hmaxo: std::ffi::c_int,
    maxo: f64,
    systs: std::ffi::c_int,
    mat1: f64,
    hum1: f64,
    xmpt1: f64,
    wprev1: f64,
    mat2: f64,
    hum2: f64,
    xmpt2: f64,
    wprev2: f64,
    outdb: f64,
    outbaro: f64,
) -> f64 {
    port::Mixing(
        which, nmix, hybrid, saved, td, hastd, hmint, mint, hmaxt, maxt, hmins, mins, hmaxs, maxs,
        hmino, mino, hmaxo, maxo, systs, mat1, hum1, xmpt1, wprev1, mat2, hum2, xmpt2, wprev2,
        outdb, outbaro,
    )
}

/// `Mixing` as `rust_anything_Mixing`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Mixing(
    which: std::ffi::c_int,
    nmix: std::ffi::c_int,
    hybrid: std::ffi::c_int,
    saved: f64,
    td: f64,
    hastd: std::ffi::c_int,
    hmint: std::ffi::c_int,
    mint: f64,
    hmaxt: std::ffi::c_int,
    maxt: f64,
    hmins: std::ffi::c_int,
    mins: f64,
    hmaxs: std::ffi::c_int,
    maxs: f64,
    hmino: std::ffi::c_int,
    mino: f64,
    hmaxo: std::ffi::c_int,
    maxo: f64,
    systs: std::ffi::c_int,
    mat1: f64,
    hum1: f64,
    xmpt1: f64,
    wprev1: f64,
    mat2: f64,
    hum2: f64,
    xmpt2: f64,
    wprev2: f64,
    outdb: f64,
    outbaro: f64,
) -> f64 {
    port::Mixing(
        which, nmix, hybrid, saved, td, hastd, hmint, mint, hmaxt, maxt, hmins, mins, hmaxs, maxs,
        hmino, mino, hmaxo, maxo, systs, mat1, hum1, xmpt1, wprev1, mat2, hum2, xmpt2, wprev2,
        outdb, outbaro,
    )
}
