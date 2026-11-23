//! C ABI for `airsplit.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double AirSplit(int which, int n, int fhvac, int first, double m0, double maxav0, double minav0, double t0, double w0, double h0, double p0, double m1, double maxav1, double minav1, double t1, double w1, double h1, double p1, double m2, double maxav2, double minav2, double t2, double w2, double h2, double p2, double m3, double maxav3, double minav3, double t3, double w3, double h3, double p3, double m4, double maxav4, double minav4, double t4, double w4, double h4, double p4)` (`airsplit.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn AirSplit(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    fhvac: std::ffi::c_int,
    first: std::ffi::c_int,
    m0: f64,
    maxav0: f64,
    minav0: f64,
    t0: f64,
    w0: f64,
    h0: f64,
    p0: f64,
    m1: f64,
    maxav1: f64,
    minav1: f64,
    t1: f64,
    w1: f64,
    h1: f64,
    p1: f64,
    m2: f64,
    maxav2: f64,
    minav2: f64,
    t2: f64,
    w2: f64,
    h2: f64,
    p2: f64,
    m3: f64,
    maxav3: f64,
    minav3: f64,
    t3: f64,
    w3: f64,
    h3: f64,
    p3: f64,
    m4: f64,
    maxav4: f64,
    minav4: f64,
    t4: f64,
    w4: f64,
    h4: f64,
    p4: f64,
) -> f64 {
    port::AirSplit(
        which, n, fhvac, first, m0, maxav0, minav0, t0, w0, h0, p0, m1, maxav1, minav1, t1, w1, h1,
        p1, m2, maxav2, minav2, t2, w2, h2, p2, m3, maxav3, minav3, t3, w3, h3, p3, m4, maxav4,
        minav4, t4, w4, h4, p4,
    )
}

/// `AirSplit` as `rust_anything_AirSplit`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_AirSplit(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    fhvac: std::ffi::c_int,
    first: std::ffi::c_int,
    m0: f64,
    maxav0: f64,
    minav0: f64,
    t0: f64,
    w0: f64,
    h0: f64,
    p0: f64,
    m1: f64,
    maxav1: f64,
    minav1: f64,
    t1: f64,
    w1: f64,
    h1: f64,
    p1: f64,
    m2: f64,
    maxav2: f64,
    minav2: f64,
    t2: f64,
    w2: f64,
    h2: f64,
    p2: f64,
    m3: f64,
    maxav3: f64,
    minav3: f64,
    t3: f64,
    w3: f64,
    h3: f64,
    p3: f64,
    m4: f64,
    maxav4: f64,
    minav4: f64,
    t4: f64,
    w4: f64,
    h4: f64,
    p4: f64,
) -> f64 {
    port::AirSplit(
        which, n, fhvac, first, m0, maxav0, minav0, t0, w0, h0, p0, m1, maxav1, minav1, t1, w1, h1,
        p1, m2, maxav2, minav2, t2, w2, h2, p2, m3, maxav3, minav3, t3, w3, h3, p3, m4, maxav4,
        minav4, t4, w4, h4, p4,
    )
}
