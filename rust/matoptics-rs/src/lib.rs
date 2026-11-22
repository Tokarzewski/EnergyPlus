//! C ABI for `matoptics.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double ScreenTrans(int which, int model, double gamma, double refl, double reflVis, double phi, double theta)` (`matoptics.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ScreenTrans(
    which: std::ffi::c_int,
    model: std::ffi::c_int,
    gamma: f64,
    refl: f64,
    reflVis: f64,
    phi: f64,
    theta: f64,
) -> f64 {
    port::ScreenTrans(which, model, gamma, refl, reflVis, phi, theta)
}

/// `ScreenTrans` as `rust_anything_ScreenTrans`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ScreenTrans(
    which: std::ffi::c_int,
    model: std::ffi::c_int,
    gamma: f64,
    refl: f64,
    reflVis: f64,
    phi: f64,
    theta: f64,
) -> f64 {
    port::ScreenTrans(which, model, gamma, refl, reflVis, phi, theta)
}

/// `double RelPhiTheta(int which, double phiWin, double thetaWin, double sx, double sy, double sz)` (`matoptics.c:19`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn RelPhiTheta(
    which: std::ffi::c_int,
    phiWin: f64,
    thetaWin: f64,
    sx: f64,
    sy: f64,
    sz: f64,
) -> f64 {
    port::RelPhiTheta(which, phiWin, thetaWin, sx, sy, sz)
}

/// `RelPhiTheta` as `rust_anything_RelPhiTheta`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_RelPhiTheta(
    which: std::ffi::c_int,
    phiWin: f64,
    thetaWin: f64,
    sx: f64,
    sy: f64,
    sz: f64,
) -> f64 {
    port::RelPhiTheta(which, phiWin, thetaWin, sx, sy, sz)
}

/// `double NormPhiTheta(int which, double phi, double theta)` (`matoptics.c:24`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn NormPhiTheta(which: std::ffi::c_int, phi: f64, theta: f64) -> f64 {
    port::NormPhiTheta(which, phi, theta)
}

/// `NormPhiTheta` as `rust_anything_NormPhiTheta`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_NormPhiTheta(which: std::ffi::c_int, phi: f64, theta: f64) -> f64 {
    port::NormPhiTheta(which, phi, theta)
}

/// `double PhiThetaIdx(int which, double phi, double theta, double dPhi, double dTheta)` (`matoptics.c:29`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PhiThetaIdx(
    which: std::ffi::c_int,
    phi: f64,
    theta: f64,
    dPhi: f64,
    dTheta: f64,
) -> f64 {
    port::PhiThetaIdx(which, phi, theta, dPhi, dTheta)
}

/// `PhiThetaIdx` as `rust_anything_PhiThetaIdx`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PhiThetaIdx(
    which: std::ffi::c_int,
    phi: f64,
    theta: f64,
    dPhi: f64,
    dTheta: f64,
) -> f64 {
    port::PhiThetaIdx(which, phi, theta, dPhi, dTheta)
}

/// `double ProfIdx(int which, double profAng)` (`matoptics.c:34`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ProfIdx(which: std::ffi::c_int, profAng: f64) -> f64 {
    port::ProfIdx(which, profAng)
}

/// `ProfIdx` as `rust_anything_ProfIdx`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ProfIdx(which: std::ffi::c_int, profAng: f64) -> f64 {
    port::ProfIdx(which, profAng)
}

/// `double SlatIdx(int which, double slatAng)` (`matoptics.c:39`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SlatIdx(which: std::ffi::c_int, slatAng: f64) -> f64 {
    port::SlatIdx(which, slatAng)
}

/// `SlatIdx` as `rust_anything_SlatIdx`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SlatIdx(which: std::ffi::c_int, slatAng: f64) -> f64 {
    port::SlatIdx(which, slatAng)
}

/// `double SimpleGlazing(int which, double u, double shgc, int vtByUser, double vt)` (`matoptics.c:44`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SimpleGlazing(
    which: std::ffi::c_int,
    u: f64,
    shgc: f64,
    vtByUser: std::ffi::c_int,
    vt: f64,
) -> f64 {
    port::SimpleGlazing(which, u, shgc, vtByUser, vt)
}

/// `SimpleGlazing` as `rust_anything_SimpleGlazing`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SimpleGlazing(
    which: std::ffi::c_int,
    u: f64,
    shgc: f64,
    vtByUser: std::ffi::c_int,
    vt: f64,
) -> f64 {
    port::SimpleGlazing(which, u, shgc, vtByUser, vt)
}
