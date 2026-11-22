//! C ABI for `clipalgo.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double ClipOverlap(int which, int algo, int nv1, double ax1, double ay1, double ax2, double ay2, double ax3, double ay3, double ax4, double ay4, int nv2, double bx1, double by1, double bx2, double by2, double bx3, double by3, double bx4, double by4, double t1, double t2)` (`clipalgo.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ClipOverlap(
    which: std::ffi::c_int,
    algo: std::ffi::c_int,
    nv1: std::ffi::c_int,
    ax1: f64,
    ay1: f64,
    ax2: f64,
    ay2: f64,
    ax3: f64,
    ay3: f64,
    ax4: f64,
    ay4: f64,
    nv2: std::ffi::c_int,
    bx1: f64,
    by1: f64,
    bx2: f64,
    by2: f64,
    bx3: f64,
    by3: f64,
    bx4: f64,
    by4: f64,
    t1: f64,
    t2: f64,
) -> f64 {
    port::ClipOverlap(
        which, algo, nv1, ax1, ay1, ax2, ay2, ax3, ay3, ax4, ay4, nv2, bx1, by1, bx2, by2, bx3,
        by3, bx4, by4, t1, t2,
    )
}

/// `ClipOverlap` as `rust_anything_ClipOverlap`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ClipOverlap(
    which: std::ffi::c_int,
    algo: std::ffi::c_int,
    nv1: std::ffi::c_int,
    ax1: f64,
    ay1: f64,
    ax2: f64,
    ay2: f64,
    ax3: f64,
    ay3: f64,
    ax4: f64,
    ay4: f64,
    nv2: std::ffi::c_int,
    bx1: f64,
    by1: f64,
    bx2: f64,
    by2: f64,
    bx3: f64,
    by3: f64,
    bx4: f64,
    by4: f64,
    t1: f64,
    t2: f64,
) -> f64 {
    port::ClipOverlap(
        which, algo, nv1, ax1, ay1, ax2, ay2, ax3, ay3, ax4, ay4, nv2, bx1, by1, bx2, by2, bx3,
        by3, bx4, by4, t1, t2,
    )
}
