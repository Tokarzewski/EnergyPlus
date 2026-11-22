//! C ABI for `blindoptics.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double BlindDiffuse(int which, int isolvis, double width, double sep, double thick, double b_el, double c6, double c7, double c8, double c9, double c10, double c11, double c12, double c13, double c14)` (`blindoptics.c:10`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BlindDiffuse(
    which: std::ffi::c_int,
    isolvis: std::ffi::c_int,
    width: f64,
    sep: f64,
    thick: f64,
    b_el: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
    c12: f64,
    c13: f64,
    c14: f64,
) -> f64 {
    port::BlindDiffuse(
        which, isolvis, width, sep, thick, b_el, c6, c7, c8, c9, c10, c11, c12, c13, c14,
    )
}

/// `BlindDiffuse` as `rust_anything_BlindDiffuse`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BlindDiffuse(
    which: std::ffi::c_int,
    isolvis: std::ffi::c_int,
    width: f64,
    sep: f64,
    thick: f64,
    b_el: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
    c12: f64,
    c13: f64,
    c14: f64,
) -> f64 {
    port::BlindDiffuse(
        which, isolvis, width, sep, thick, b_el, c6, c7, c8, c9, c10, c11, c12, c13, c14,
    )
}

/// `double BlindBeam(int which, double width, double sep, double thick, double b_el, double s_el, double c3, double c4, double c5, double c6, double c7, double c8, double c9, double c10, double c11)` (`blindoptics.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BlindBeam(
    which: std::ffi::c_int,
    width: f64,
    sep: f64,
    thick: f64,
    b_el: f64,
    s_el: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
) -> f64 {
    port::BlindBeam(
        which, width, sep, thick, b_el, s_el, c3, c4, c5, c6, c7, c8, c9, c10, c11,
    )
}

/// `BlindBeam` as `rust_anything_BlindBeam`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BlindBeam(
    which: std::ffi::c_int,
    width: f64,
    sep: f64,
    thick: f64,
    b_el: f64,
    s_el: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
    c8: f64,
    c9: f64,
    c10: f64,
    c11: f64,
) -> f64 {
    port::BlindBeam(
        which, width, sep, thick, b_el, s_el, c3, c4, c5, c6, c7, c8, c9, c10, c11,
    )
}

/// `double BlindBeamBeam(double width, double sep, double thick, double profAng, double slatAng)` (`blindoptics.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn BlindBeamBeam(
    width: f64,
    sep: f64,
    thick: f64,
    profAng: f64,
    slatAng: f64,
) -> f64 {
    port::BlindBeamBeam(width, sep, thick, profAng, slatAng)
}

/// `BlindBeamBeam` as `rust_anything_BlindBeamBeam`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_BlindBeamBeam(
    width: f64,
    sep: f64,
    thick: f64,
    profAng: f64,
    slatAng: f64,
) -> f64 {
    port::BlindBeamBeam(width, sep, thick, profAng, slatAng)
}
