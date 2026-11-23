//! C ABI for `intconv.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double IntConvCoeff(int model, int setbyzone, double tsurf, double mat, double humrat, double costilt, double tilt, double sintilt, double height, double area, int window, int extbound, double outwind, double outbaro, double volume, double hvalue, int emsover, double emsval, double adjratio, double lowlimit)` (`intconv.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn IntConvCoeff(
    model: std::ffi::c_int,
    setbyzone: std::ffi::c_int,
    tsurf: f64,
    mat: f64,
    humrat: f64,
    costilt: f64,
    tilt: f64,
    sintilt: f64,
    height: f64,
    area: f64,
    window: std::ffi::c_int,
    extbound: std::ffi::c_int,
    outwind: f64,
    outbaro: f64,
    volume: f64,
    hvalue: f64,
    emsover: std::ffi::c_int,
    emsval: f64,
    adjratio: f64,
    lowlimit: f64,
) -> f64 {
    port::IntConvCoeff(
        model, setbyzone, tsurf, mat, humrat, costilt, tilt, sintilt, height, area, window,
        extbound, outwind, outbaro, volume, hvalue, emsover, emsval, adjratio, lowlimit,
    )
}

/// `IntConvCoeff` as `rust_anything_IntConvCoeff`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_IntConvCoeff(
    model: std::ffi::c_int,
    setbyzone: std::ffi::c_int,
    tsurf: f64,
    mat: f64,
    humrat: f64,
    costilt: f64,
    tilt: f64,
    sintilt: f64,
    height: f64,
    area: f64,
    window: std::ffi::c_int,
    extbound: std::ffi::c_int,
    outwind: f64,
    outbaro: f64,
    volume: f64,
    hvalue: f64,
    emsover: std::ffi::c_int,
    emsval: f64,
    adjratio: f64,
    lowlimit: f64,
) -> f64 {
    port::IntConvCoeff(
        model, setbyzone, tsurf, mat, humrat, costilt, tilt, sintilt, height, area, window,
        extbound, outwind, outbaro, volume, hvalue, emsover, emsval, adjratio, lowlimit,
    )
}
