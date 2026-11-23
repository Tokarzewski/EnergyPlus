//! C ABI for `condupd.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double CondUpd(int which, int hr, int first, int sim0, int nconn, int dem1, int dem2, double q, double tmin, double tmout, double mmod, double inm, double intemp, double outm, double outt)` (`condupd.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn CondUpd(
    which: std::ffi::c_int,
    hr: std::ffi::c_int,
    first: std::ffi::c_int,
    sim0: std::ffi::c_int,
    nconn: std::ffi::c_int,
    dem1: std::ffi::c_int,
    dem2: std::ffi::c_int,
    q: f64,
    tmin: f64,
    tmout: f64,
    mmod: f64,
    inm: f64,
    intemp: f64,
    outm: f64,
    outt: f64,
) -> f64 {
    port::CondUpd(
        which, hr, first, sim0, nconn, dem1, dem2, q, tmin, tmout, mmod, inm, intemp, outm, outt,
    )
}

/// `CondUpd` as `rust_anything_CondUpd`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_CondUpd(
    which: std::ffi::c_int,
    hr: std::ffi::c_int,
    first: std::ffi::c_int,
    sim0: std::ffi::c_int,
    nconn: std::ffi::c_int,
    dem1: std::ffi::c_int,
    dem2: std::ffi::c_int,
    q: f64,
    tmin: f64,
    tmout: f64,
    mmod: f64,
    inm: f64,
    intemp: f64,
    outm: f64,
    outt: f64,
) -> f64 {
    port::CondUpd(
        which, hr, first, sim0, nconn, dem1, dem2, q, tmin, tmout, mmod, inm, intemp, outm, outt,
    )
}
