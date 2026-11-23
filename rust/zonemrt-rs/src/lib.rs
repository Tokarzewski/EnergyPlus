//! C ABI for `zonemrt.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double ZoneMRT(int which, int n1, int n2, double a1, double a2, double a3, double a4, double a5, double a6, double e1, double e2, double e3, double e4, double e5, double e6, double t1, double t2, double t3, double t4, double t5, double t6, double mat, double mat1, double mat2, double vol1, double vol2)` (`zonemrt.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ZoneMRT(
    which: std::ffi::c_int,
    n1: std::ffi::c_int,
    n2: std::ffi::c_int,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    e1: f64,
    e2: f64,
    e3: f64,
    e4: f64,
    e5: f64,
    e6: f64,
    t1: f64,
    t2: f64,
    t3: f64,
    t4: f64,
    t5: f64,
    t6: f64,
    mat: f64,
    mat1: f64,
    mat2: f64,
    vol1: f64,
    vol2: f64,
) -> f64 {
    port::ZoneMRT(
        which, n1, n2, a1, a2, a3, a4, a5, a6, e1, e2, e3, e4, e5, e6, t1, t2, t3, t4, t5, t6, mat,
        mat1, mat2, vol1, vol2,
    )
}

/// `ZoneMRT` as `rust_anything_ZoneMRT`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ZoneMRT(
    which: std::ffi::c_int,
    n1: std::ffi::c_int,
    n2: std::ffi::c_int,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    e1: f64,
    e2: f64,
    e3: f64,
    e4: f64,
    e5: f64,
    e6: f64,
    t1: f64,
    t2: f64,
    t3: f64,
    t4: f64,
    t5: f64,
    t6: f64,
    mat: f64,
    mat1: f64,
    mat2: f64,
    vol1: f64,
    vol2: f64,
) -> f64 {
    port::ZoneMRT(
        which, n1, n2, a1, a2, a3, a4, a5, a6, e1, e2, e3, e4, e5, e6, t1, t2, t3, t4, t5, t6, mat,
        mat1, mat2, vol1, vol2,
    )
}
