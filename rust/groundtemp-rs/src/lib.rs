//! C ABI for `groundtemp.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double GTKusudaSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity)` (`groundtemp.c:12`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GTKusudaSeconds(
    depth: f64,
    seconds: f64,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp: f64,
    phaseSecs: f64,
    diffusivity: f64,
) -> f64 {
    port::GTKusudaSeconds(
        depth,
        seconds,
        numDaysInYear,
        ave,
        amp,
        phaseSecs,
        diffusivity,
    )
}

/// `GTKusudaSeconds` as `rust_anything_GTKusudaSeconds`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GTKusudaSeconds(
    depth: f64,
    seconds: f64,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp: f64,
    phaseSecs: f64,
    diffusivity: f64,
) -> f64 {
    port::GTKusudaSeconds(
        depth,
        seconds,
        numDaysInYear,
        ave,
        amp,
        phaseSecs,
        diffusivity,
    )
}

/// `double GTKusudaMonth(double depth, int month, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity)` (`groundtemp.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GTKusudaMonth(
    depth: f64,
    month: std::ffi::c_int,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp: f64,
    phaseSecs: f64,
    diffusivity: f64,
) -> f64 {
    port::GTKusudaMonth(
        depth,
        month,
        numDaysInYear,
        ave,
        amp,
        phaseSecs,
        diffusivity,
    )
}

/// `GTKusudaMonth` as `rust_anything_GTKusudaMonth`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GTKusudaMonth(
    depth: f64,
    month: std::ffi::c_int,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp: f64,
    phaseSecs: f64,
    diffusivity: f64,
) -> f64 {
    port::GTKusudaMonth(
        depth,
        month,
        numDaysInYear,
        ave,
        amp,
        phaseSecs,
        diffusivity,
    )
}

/// `double GTXingSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity)` (`groundtemp.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GTXingSeconds(
    depth: f64,
    seconds: f64,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp1: f64,
    phase1: f64,
    amp2: f64,
    phase2: f64,
    diffusivity: f64,
) -> f64 {
    port::GTXingSeconds(
        depth,
        seconds,
        numDaysInYear,
        ave,
        amp1,
        phase1,
        amp2,
        phase2,
        diffusivity,
    )
}

/// `GTXingSeconds` as `rust_anything_GTXingSeconds`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GTXingSeconds(
    depth: f64,
    seconds: f64,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp1: f64,
    phase1: f64,
    amp2: f64,
    phase2: f64,
    diffusivity: f64,
) -> f64 {
    port::GTXingSeconds(
        depth,
        seconds,
        numDaysInYear,
        ave,
        amp1,
        phase1,
        amp2,
        phase2,
        diffusivity,
    )
}

/// `double GTXingMonth(double depth, int month, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity)` (`groundtemp.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GTXingMonth(
    depth: f64,
    month: std::ffi::c_int,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp1: f64,
    phase1: f64,
    amp2: f64,
    phase2: f64,
    diffusivity: f64,
) -> f64 {
    port::GTXingMonth(
        depth,
        month,
        numDaysInYear,
        ave,
        amp1,
        phase1,
        amp2,
        phase2,
        diffusivity,
    )
}

/// `GTXingMonth` as `rust_anything_GTXingMonth`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GTXingMonth(
    depth: f64,
    month: std::ffi::c_int,
    numDaysInYear: std::ffi::c_int,
    ave: f64,
    amp1: f64,
    phase1: f64,
    amp2: f64,
    phase2: f64,
    diffusivity: f64,
) -> f64 {
    port::GTXingMonth(
        depth,
        month,
        numDaysInYear,
        ave,
        amp1,
        phase1,
        amp2,
        phase2,
        diffusivity,
    )
}

/// `double GTSite(int kind, int byMonth, double when, int numDaysInYear, double t0, double dt)` (`groundtemp.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn GTSite(
    kind: std::ffi::c_int,
    byMonth: std::ffi::c_int,
    when: f64,
    numDaysInYear: std::ffi::c_int,
    t0: f64,
    dt: f64,
) -> f64 {
    port::GTSite(kind, byMonth, when, numDaysInYear, t0, dt)
}

/// `GTSite` as `rust_anything_GTSite`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_GTSite(
    kind: std::ffi::c_int,
    byMonth: std::ffi::c_int,
    when: f64,
    numDaysInYear: std::ffi::c_int,
    t0: f64,
    dt: f64,
) -> f64 {
    port::GTSite(kind, byMonth, when, numDaysInYear, t0, dt)
}
