//! C ABI for `solveroot.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SolveRootX(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1)` (`solveroot.c:13`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SolveRootX(
    algo: std::ffi::c_int,
    numOfIter: std::ffi::c_int,
    Eps: f64,
    MaxIte: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> f64 {
    port::SolveRootX(algo, numOfIter, Eps, MaxIte, func, a, b, c, X_0, X_1)
}

/// `SolveRootX` as `rust_anything_SolveRootX`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SolveRootX(
    algo: std::ffi::c_int,
    numOfIter: std::ffi::c_int,
    Eps: f64,
    MaxIte: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> f64 {
    port::SolveRootX(algo, numOfIter, Eps, MaxIte, func, a, b, c, X_0, X_1)
}

/// `int SolveRootFlag(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1)` (`solveroot.c:18`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SolveRootFlag(
    algo: std::ffi::c_int,
    numOfIter: std::ffi::c_int,
    Eps: f64,
    MaxIte: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> std::ffi::c_int {
    port::SolveRootFlag(algo, numOfIter, Eps, MaxIte, func, a, b, c, X_0, X_1)
}

/// `SolveRootFlag` as `rust_anything_SolveRootFlag`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SolveRootFlag(
    algo: std::ffi::c_int,
    numOfIter: std::ffi::c_int,
    Eps: f64,
    MaxIte: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> std::ffi::c_int {
    port::SolveRootFlag(algo, numOfIter, Eps, MaxIte, func, a, b, c, X_0, X_1)
}

/// `int SolveRoot2Algo(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1)` (`solveroot.c:23`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SolveRoot2Algo(
    start: std::ffi::c_int,
    calls: std::ffi::c_int,
    Eps: f64,
    maxIters: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> std::ffi::c_int {
    port::SolveRoot2Algo(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1)
}

/// `SolveRoot2Algo` as `rust_anything_SolveRoot2Algo`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SolveRoot2Algo(
    start: std::ffi::c_int,
    calls: std::ffi::c_int,
    Eps: f64,
    maxIters: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> std::ffi::c_int {
    port::SolveRoot2Algo(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1)
}

/// `double SolveRoot2X(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1)` (`solveroot.c:28`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SolveRoot2X(
    start: std::ffi::c_int,
    calls: std::ffi::c_int,
    Eps: f64,
    maxIters: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> f64 {
    port::SolveRoot2X(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1)
}

/// `SolveRoot2X` as `rust_anything_SolveRoot2X`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SolveRoot2X(
    start: std::ffi::c_int,
    calls: std::ffi::c_int,
    Eps: f64,
    maxIters: std::ffi::c_int,
    func: std::ffi::c_int,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> f64 {
    port::SolveRoot2X(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1)
}
