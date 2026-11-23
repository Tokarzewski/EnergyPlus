//! C ABI for `updtemp.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double UpdateTemperatures(int which, int shorten, int usezonehist, double prior, double tsys, int nsys, int nsyslast, int hasnode, double mat, double w, double x0, double x1, double x2, double x3, double ds0, double ds1, double ds2, double ds3, double wp0, double wp1, double wp2, double wp3, double dsw0, double dsw1, double dsw2, double dsw3)` (`updtemp.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn UpdateTemperatures(
    which: std::ffi::c_int,
    shorten: std::ffi::c_int,
    usezonehist: std::ffi::c_int,
    prior: f64,
    tsys: f64,
    nsys: std::ffi::c_int,
    nsyslast: std::ffi::c_int,
    hasnode: std::ffi::c_int,
    mat: f64,
    w: f64,
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    ds0: f64,
    ds1: f64,
    ds2: f64,
    ds3: f64,
    wp0: f64,
    wp1: f64,
    wp2: f64,
    wp3: f64,
    dsw0: f64,
    dsw1: f64,
    dsw2: f64,
    dsw3: f64,
) -> f64 {
    port::UpdateTemperatures(
        which,
        shorten,
        usezonehist,
        prior,
        tsys,
        nsys,
        nsyslast,
        hasnode,
        mat,
        w,
        x0,
        x1,
        x2,
        x3,
        ds0,
        ds1,
        ds2,
        ds3,
        wp0,
        wp1,
        wp2,
        wp3,
        dsw0,
        dsw1,
        dsw2,
        dsw3,
    )
}

/// `UpdateTemperatures` as `rust_anything_UpdateTemperatures`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_UpdateTemperatures(
    which: std::ffi::c_int,
    shorten: std::ffi::c_int,
    usezonehist: std::ffi::c_int,
    prior: f64,
    tsys: f64,
    nsys: std::ffi::c_int,
    nsyslast: std::ffi::c_int,
    hasnode: std::ffi::c_int,
    mat: f64,
    w: f64,
    x0: f64,
    x1: f64,
    x2: f64,
    x3: f64,
    ds0: f64,
    ds1: f64,
    ds2: f64,
    ds3: f64,
    wp0: f64,
    wp1: f64,
    wp2: f64,
    wp3: f64,
    dsw0: f64,
    dsw1: f64,
    dsw2: f64,
    dsw3: f64,
) -> f64 {
    port::UpdateTemperatures(
        which,
        shorten,
        usezonehist,
        prior,
        tsys,
        nsys,
        nsyslast,
        hasnode,
        mat,
        w,
        x0,
        x1,
        x2,
        x3,
        ds0,
        ds1,
        ds2,
        ds3,
        wp0,
        wp1,
        wp2,
        wp3,
        dsw0,
        dsw1,
        dsw2,
        dsw3,
    )
}

/// `double DownInterpolate5(int which, double old, double nw, double v0, double v1, double v2)` (`updtemp.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn DownInterpolate5(
    which: std::ffi::c_int,
    old: f64,
    nw: f64,
    v0: f64,
    v1: f64,
    v2: f64,
) -> f64 {
    port::DownInterpolate5(which, old, nw, v0, v1, v2)
}

/// `DownInterpolate5` as `rust_anything_DownInterpolate5`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_DownInterpolate5(
    which: std::ffi::c_int,
    old: f64,
    nw: f64,
    v0: f64,
    v1: f64,
    v2: f64,
) -> f64 {
    port::DownInterpolate5(which, old, nw, v0, v1, v2)
}
