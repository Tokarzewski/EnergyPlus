//! C ABI for `retplenum.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double RetPlenum(int which, int nin, int nadu, int nind, double m1, double x1, double n1, double p1, double m2, double x2, double n2, double p2, double m3, double x3, double n3, double p3, double m4, double x4, double n4, double p4, int u1, int d1, double lu1, double ld1, double lp1, double dx1, double dn1, int u2, int d2, double lu2, double ld2, double lp2, double dx2, double dn2, double i1, double i2, double tz, double wz, double hz)` (`retplenum.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn RetPlenum(
    which: std::ffi::c_int,
    nin: std::ffi::c_int,
    nadu: std::ffi::c_int,
    nind: std::ffi::c_int,
    m1: f64,
    x1: f64,
    n1: f64,
    p1: f64,
    m2: f64,
    x2: f64,
    n2: f64,
    p2: f64,
    m3: f64,
    x3: f64,
    n3: f64,
    p3: f64,
    m4: f64,
    x4: f64,
    n4: f64,
    p4: f64,
    u1: std::ffi::c_int,
    d1: std::ffi::c_int,
    lu1: f64,
    ld1: f64,
    lp1: f64,
    dx1: f64,
    dn1: f64,
    u2: std::ffi::c_int,
    d2: std::ffi::c_int,
    lu2: f64,
    ld2: f64,
    lp2: f64,
    dx2: f64,
    dn2: f64,
    i1: f64,
    i2: f64,
    tz: f64,
    wz: f64,
    hz: f64,
) -> f64 {
    port::RetPlenum(
        which, nin, nadu, nind, m1, x1, n1, p1, m2, x2, n2, p2, m3, x3, n3, p3, m4, x4, n4, p4, u1,
        d1, lu1, ld1, lp1, dx1, dn1, u2, d2, lu2, ld2, lp2, dx2, dn2, i1, i2, tz, wz, hz,
    )
}

/// `RetPlenum` as `rust_anything_RetPlenum`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_RetPlenum(
    which: std::ffi::c_int,
    nin: std::ffi::c_int,
    nadu: std::ffi::c_int,
    nind: std::ffi::c_int,
    m1: f64,
    x1: f64,
    n1: f64,
    p1: f64,
    m2: f64,
    x2: f64,
    n2: f64,
    p2: f64,
    m3: f64,
    x3: f64,
    n3: f64,
    p3: f64,
    m4: f64,
    x4: f64,
    n4: f64,
    p4: f64,
    u1: std::ffi::c_int,
    d1: std::ffi::c_int,
    lu1: f64,
    ld1: f64,
    lp1: f64,
    dx1: f64,
    dn1: f64,
    u2: std::ffi::c_int,
    d2: std::ffi::c_int,
    lu2: f64,
    ld2: f64,
    lp2: f64,
    dx2: f64,
    dn2: f64,
    i1: f64,
    i2: f64,
    tz: f64,
    wz: f64,
    hz: f64,
) -> f64 {
    port::RetPlenum(
        which, nin, nadu, nind, m1, x1, n1, p1, m2, x2, n2, p2, m3, x3, n3, p3, m4, x4, n4, p4, u1,
        d1, lu1, ld1, lp1, dx1, dn1, u2, d2, lu2, ld2, lp2, dx2, dn2, i1, i2, tz, wz, hz,
    )
}
