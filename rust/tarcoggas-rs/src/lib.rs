//! C ABI for `tarcoggas.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double Gasses90(int which, int standard, double tmean, double pres, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad)` (`tarcoggas.c:10`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Gasses90(
    which: std::ffi::c_int,
    standard: std::ffi::c_int,
    tmean: f64,
    pres: f64,
    nmix: std::ffi::c_int,
    g1: std::ffi::c_int,
    g2: std::ffi::c_int,
    g3: std::ffi::c_int,
    g4: std::ffi::c_int,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    quad: f64,
) -> f64 {
    port::Gasses90(
        which, standard, tmean, pres, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad,
    )
}

/// `Gasses90` as `rust_anything_Gasses90`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Gasses90(
    which: std::ffi::c_int,
    standard: std::ffi::c_int,
    tmean: f64,
    pres: f64,
    nmix: std::ffi::c_int,
    g1: std::ffi::c_int,
    g2: std::ffi::c_int,
    g3: std::ffi::c_int,
    g4: std::ffi::c_int,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    quad: f64,
) -> f64 {
    port::Gasses90(
        which, standard, tmean, pres, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad,
    )
}

/// `double GassesLow(int which, double tmean, double mwght, double pressure, double gama)` (`tarcoggas.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GassesLow(
    which: std::ffi::c_int,
    tmean: f64,
    mwght: f64,
    pressure: f64,
    gama: f64,
) -> f64 {
    port::GassesLow(which, tmean, mwght, pressure, gama)
}

/// `GassesLow` as `rust_anything_GassesLow`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GassesLow(
    which: std::ffi::c_int,
    tmean: f64,
    mwght: f64,
    pressure: f64,
    gama: f64,
) -> f64 {
    port::GassesLow(which, tmean, mwght, pressure, gama)
}

/// `double TarcogSolve(int which, int n, double d, double k, double o, double p)` (`tarcoggas.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn TarcogSolve(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    d: f64,
    k: f64,
    o: f64,
    p: f64,
) -> f64 {
    port::TarcogSolve(which, n, d, k, o, p)
}

/// `TarcogSolve` as `rust_anything_TarcogSolve`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_TarcogSolve(
    which: std::ffi::c_int,
    n: std::ffi::c_int,
    d: f64,
    k: f64,
    o: f64,
    p: f64,
) -> f64 {
    port::TarcogSolve(which, n, d, k, o, p)
}
