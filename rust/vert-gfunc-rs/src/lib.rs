//! C ABI for `vertgfunc.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `int VGCombinedLen(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)` (`vertgfunc.c:12`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VGCombinedLen(
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> std::ffi::c_int {
    port::VGCombinedLen(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}

/// `VGCombinedLen` as `rust_anything_VGCombinedLen`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VGCombinedLen(
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> std::ffi::c_int {
    port::VGCombinedLen(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}

/// `double VGCombinedLntts(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)` (`vertgfunc.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VGCombinedLntts(
    k: std::ffi::c_int,
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    port::VGCombinedLntts(
        k,
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}

/// `VGCombinedLntts` as `rust_anything_VGCombinedLntts`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VGCombinedLntts(
    k: std::ffi::c_int,
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    port::VGCombinedLntts(
        k,
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}

/// `double VGCombinedGFunction(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)` (`vertgfunc.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VGCombinedGFunction(
    k: std::ffi::c_int,
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    port::VGCombinedGFunction(
        k,
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}

/// `VGCombinedGFunction` as `rust_anything_VGCombinedGFunction`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VGCombinedGFunction(
    k: std::ffi::c_int,
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    port::VGCombinedGFunction(
        k,
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}

/// `double VGGFunc(double time, double gRefRatio, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)` (`vertgfunc.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn VGGFunc(
    time: f64,
    gRefRatio: f64,
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    port::VGGFunc(
        time,
        gRefRatio,
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}

/// `VGGFunc` as `rust_anything_VGGFunc`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_VGGFunc(
    time: f64,
    gRefRatio: f64,
    nx: std::ffi::c_int,
    ny: std::ffi::c_int,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
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
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    port::VGGFunc(
        time,
        gRefRatio,
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    )
}
