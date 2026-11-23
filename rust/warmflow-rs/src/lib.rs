//! C ABI for `warmflow.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double WarmFlow(int which, int flowfirst, int nc, double mint, double maxt, double minturn, double load1, double zt1, double mmax1, double w1, double load2, double zt2, double mmax2, double w2, double load3, double zt3, double mmax3, double w3, double load4, double zt4, double mmax4, double w4)` (`warmflow.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn WarmFlow(
    which: std::ffi::c_int,
    flowfirst: std::ffi::c_int,
    nc: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    minturn: f64,
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
    port::WarmFlow(
        which, flowfirst, nc, mint, maxt, minturn, load1, zt1, mmax1, w1, load2, zt2, mmax2, w2,
        load3, zt3, mmax3, w3, load4, zt4, mmax4, w4,
    )
}

/// `WarmFlow` as `rust_anything_WarmFlow`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_WarmFlow(
    which: std::ffi::c_int,
    flowfirst: std::ffi::c_int,
    nc: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    minturn: f64,
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
    port::WarmFlow(
        which, flowfirst, nc, mint, maxt, minturn, load1, zt1, mmax1, w1, load2, zt2, mmax2, w2,
        load3, zt3, mmax3, w3, load4, zt4, mmax4, w4,
    )
}

/// `double RABypass(double spt, double tloop, double tmix, double msup, double tsup, double mtot, double trab)` (`warmflow.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn RABypass(
    spt: f64,
    tloop: f64,
    tmix: f64,
    msup: f64,
    tsup: f64,
    mtot: f64,
    trab: f64,
) -> f64 {
    port::RABypass(spt, tloop, tmix, msup, tsup, mtot, trab)
}

/// `RABypass` as `rust_anything_RABypass`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_RABypass(
    spt: f64,
    tloop: f64,
    tmix: f64,
    msup: f64,
    tsup: f64,
    mtot: f64,
    trab: f64,
) -> f64 {
    port::RABypass(spt, tloop, tmix, msup, tsup, mtot, trab)
}
