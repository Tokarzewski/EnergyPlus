//! C ABI for `vertfluid.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double VFConvectionResistance(double outDia, double thickness, double massFlow, int numBH, double inletTemp)` (`vertfluid.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VFConvectionResistance(
    outDia: f64,
    thickness: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFConvectionResistance(outDia, thickness, massFlow, numBH, inletTemp)
}

/// `VFConvectionResistance` as `rust_anything_VFConvectionResistance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VFConvectionResistance(
    outDia: f64,
    thickness: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFConvectionResistance(outDia, thickness, massFlow, numBH, inletTemp)
}

/// `double VFPipeResistance(double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp)` (`vertfluid.c:20`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VFPipeResistance(
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFPipeResistance(outDia, thickness, kPipe, massFlow, numBH, inletTemp)
}

/// `VFPipeResistance` as `rust_anything_VFPipeResistance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VFPipeResistance(
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFPipeResistance(outDia, thickness, kPipe, massFlow, numBH, inletTemp)
}

/// `double VFBoreholeResistance(int which, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp)` (`vertfluid.c:25`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VFBoreholeResistance(
    which: std::ffi::c_int,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    soilK: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFBoreholeResistance(
        which, bhLength, bhDiameter, uTubeDist, groutK, soilK, outDia, thickness, kPipe, massFlow,
        numBH, inletTemp,
    )
}

/// `VFBoreholeResistance` as `rust_anything_VFBoreholeResistance`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VFBoreholeResistance(
    which: std::ffi::c_int,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    soilK: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFBoreholeResistance(
        which, bhLength, bhDiameter, uTubeDist, groutK, soilK, outDia, thickness, kPipe, massFlow,
        numBH, inletTemp,
    )
}

/// `int VFShortSteps(double bhLength, double soilK, double soilRhoCp)` (`vertfluid.c:30`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VFShortSteps(bhLength: f64, soilK: f64, soilRhoCp: f64) -> std::ffi::c_int {
    port::VFShortSteps(bhLength, soilK, soilRhoCp)
}

/// `VFShortSteps` as `rust_anything_VFShortSteps`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VFShortSteps(
    bhLength: f64,
    soilK: f64,
    soilRhoCp: f64,
) -> std::ffi::c_int {
    port::VFShortSteps(bhLength, soilK, soilRhoCp)
}

/// `double VFShortGFunction(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp)` (`vertfluid.c:35`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VFShortGFunction(
    k: std::ffi::c_int,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFShortGFunction(
        k, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia,
        thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp,
    )
}

/// `VFShortGFunction` as `rust_anything_VFShortGFunction`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VFShortGFunction(
    k: std::ffi::c_int,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFShortGFunction(
        k, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia,
        thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp,
    )
}

/// `double VFShortLntts(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp)` (`vertfluid.c:40`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VFShortLntts(
    k: std::ffi::c_int,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFShortLntts(
        k, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia,
        thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp,
    )
}

/// `VFShortLntts` as `rust_anything_VFShortLntts`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VFShortLntts(
    k: std::ffi::c_int,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    numBH: std::ffi::c_int,
    inletTemp: f64,
) -> f64 {
    port::VFShortLntts(
        k, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia,
        thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp,
    )
}
