//! C ABI for `tempest.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double Tempest(int warmest, int nc, int nh, double mint, double maxt, double load1, double zt1, double mmax1, double w1, double load2, double zt2, double mmax2, double w2, double load3, double zt3, double mmax3, double w3, double load4, double zt4, double mmax4, double w4)` (`tempest.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Tempest(
    warmest: std::ffi::c_int,
    nc: std::ffi::c_int,
    nh: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    load1: f64,
    zt1: f64,
    mmax1: f64,
    w1: f64,
    load2: f64,
    zt2: f64,
    mmax2: f64,
    w2: f64,
    load3: f64,
    zt3: f64,
    mmax3: f64,
    w3: f64,
    load4: f64,
    zt4: f64,
    mmax4: f64,
    w4: f64,
) -> f64 {
    port::Tempest(
        warmest, nc, nh, mint, maxt, load1, zt1, mmax1, w1, load2, zt2, mmax2, w2, load3, zt3,
        mmax3, w3, load4, zt4, mmax4, w4,
    )
}

/// `Tempest` as `rust_anything_Tempest`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Tempest(
    warmest: std::ffi::c_int,
    nc: std::ffi::c_int,
    nh: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    load1: f64,
    zt1: f64,
    mmax1: f64,
    w1: f64,
    load2: f64,
    zt2: f64,
    mmax2: f64,
    w2: f64,
    load3: f64,
    zt3: f64,
    mmax3: f64,
    w3: f64,
    load4: f64,
    zt4: f64,
    mmax4: f64,
    w4: f64,
) -> f64 {
    port::Tempest(
        warmest, nc, nh, mint, maxt, load1, zt1, mmax1, w1, load2, zt2, mmax2, w2, load3, zt3,
        mmax3, w3, load4, zt4, mmax4, w4,
    )
}
