//! C ABI for `supplenum.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SupPlenum(int which, int first, int nout, double inm, double inmx, double inmn, double m1, double x1, double n1, double m2, double x2, double n2, double m3, double x3, double n3, double m4, double x4, double n4, double tz, double wz, double hz)` (`supplenum.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SupPlenum(
    which: std::ffi::c_int,
    first: std::ffi::c_int,
    nout: std::ffi::c_int,
    inm: f64,
    inmx: f64,
    inmn: f64,
    m1: f64,
    x1: f64,
    n1: f64,
    m2: f64,
    x2: f64,
    n2: f64,
    m3: f64,
    x3: f64,
    n3: f64,
    m4: f64,
    x4: f64,
    n4: f64,
    tz: f64,
    wz: f64,
    hz: f64,
) -> f64 {
    port::SupPlenum(
        which, first, nout, inm, inmx, inmn, m1, x1, n1, m2, x2, n2, m3, x3, n3, m4, x4, n4, tz,
        wz, hz,
    )
}

/// `SupPlenum` as `rust_anything_SupPlenum`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SupPlenum(
    which: std::ffi::c_int,
    first: std::ffi::c_int,
    nout: std::ffi::c_int,
    inm: f64,
    inmx: f64,
    inmn: f64,
    m1: f64,
    x1: f64,
    n1: f64,
    m2: f64,
    x2: f64,
    n2: f64,
    m3: f64,
    x3: f64,
    n3: f64,
    m4: f64,
    x4: f64,
    n4: f64,
    tz: f64,
    wz: f64,
    hz: f64,
) -> f64 {
    port::SupPlenum(
        which, first, nout, inm, inmx, inmn, m1, x1, n1, m2, x2, n2, m3, x3, n3, m4, x4, n4, tz,
        wz, hz,
    )
}
