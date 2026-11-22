//! C ABI for `glhesim.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double GSRun(int which, int steps, int tsPerHour, double tGround, double tIn0, double tInAmp, double flow0, int offEvery, double gA, double gB, double gRefRatio, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, int numBH)` (`glhesim.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GSRun(
    which: std::ffi::c_int,
    steps: std::ffi::c_int,
    tsPerHour: std::ffi::c_int,
    tGround: f64,
    tIn0: f64,
    tInAmp: f64,
    flow0: f64,
    offEvery: std::ffi::c_int,
    gA: f64,
    gB: f64,
    gRefRatio: f64,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    numBH: std::ffi::c_int,
) -> f64 {
    port::GSRun(
        which, steps, tsPerHour, tGround, tIn0, tInAmp, flow0, offEvery, gA, gB, gRefRatio,
        bhLength, bhDiameter, uTubeDist, groutK, soilK, soilRhoCp, outDia, thickness, kPipe, numBH,
    )
}

/// `GSRun` as `rust_anything_GSRun`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GSRun(
    which: std::ffi::c_int,
    steps: std::ffi::c_int,
    tsPerHour: std::ffi::c_int,
    tGround: f64,
    tIn0: f64,
    tInAmp: f64,
    flow0: f64,
    offEvery: std::ffi::c_int,
    gA: f64,
    gB: f64,
    gRefRatio: f64,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    numBH: std::ffi::c_int,
) -> f64 {
    port::GSRun(
        which, steps, tsPerHour, tGround, tIn0, tInAmp, flow0, offEvery, gA, gB, gRefRatio,
        bhLength, bhDiameter, uTubeDist, groutK, soilK, soilRhoCp, outDia, thickness, kPipe, numBH,
    )
}
