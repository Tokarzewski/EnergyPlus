//! C ABI for `oscill.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double Oscillate(int which, int nzones, double tsys, int comfort, double ha1, double hb1, double hc1, double zt1, int occ1, int db1, double ha2, double hb2, double hc2, double zt2, int occ2, int db2, double ha3, double hb3, double hc3, double zt3, int occ3, int db3, double sum1, double sum2, double sum3)` (`oscill.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Oscillate(
    which: std::ffi::c_int,
    nzones: std::ffi::c_int,
    tsys: f64,
    comfort: std::ffi::c_int,
    ha1: f64,
    hb1: f64,
    hc1: f64,
    zt1: f64,
    occ1: std::ffi::c_int,
    db1: std::ffi::c_int,
    ha2: f64,
    hb2: f64,
    hc2: f64,
    zt2: f64,
    occ2: std::ffi::c_int,
    db2: std::ffi::c_int,
    ha3: f64,
    hb3: f64,
    hc3: f64,
    zt3: f64,
    occ3: std::ffi::c_int,
    db3: std::ffi::c_int,
    sum1: f64,
    sum2: f64,
    sum3: f64,
) -> f64 {
    port::Oscillate(
        which, nzones, tsys, comfort, ha1, hb1, hc1, zt1, occ1, db1, ha2, hb2, hc2, zt2, occ2, db2,
        ha3, hb3, hc3, zt3, occ3, db3, sum1, sum2, sum3,
    )
}

/// `Oscillate` as `rust_anything_Oscillate`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Oscillate(
    which: std::ffi::c_int,
    nzones: std::ffi::c_int,
    tsys: f64,
    comfort: std::ffi::c_int,
    ha1: f64,
    hb1: f64,
    hc1: f64,
    zt1: f64,
    occ1: std::ffi::c_int,
    db1: std::ffi::c_int,
    ha2: f64,
    hb2: f64,
    hc2: f64,
    zt2: f64,
    occ2: std::ffi::c_int,
    db2: std::ffi::c_int,
    ha3: f64,
    hb3: f64,
    hc3: f64,
    zt3: f64,
    occ3: std::ffi::c_int,
    db3: std::ffi::c_int,
    sum1: f64,
    sum2: f64,
    sum3: f64,
) -> f64 {
    port::Oscillate(
        which, nzones, tsys, comfort, ha1, hb1, hc1, zt1, occ1, db1, ha2, hb2, hc2, zt2, occ2, db2,
        ha3, hb3, hc3, zt3, occ3, db3, sum1, sum2, sum3,
    )
}
