//! C ABI for `setflow.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SetFlow(int which, int ncomp, int k, int lock, int autosize, int series, int demand, int ems, double flow, double mdot1, double min1, double max1, double minav1, double maxav1, double req1, double mdot2, double min2, double max2, double minav2, double maxav2, double req2, double mdot3, double min3, double max3, double minav3, double maxav3, double req3, double mdot4, double min4, double max4, double minav4, double maxav4, double req4)` (`setflow.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SetFlow(
    which: std::ffi::c_int,
    ncomp: std::ffi::c_int,
    k: std::ffi::c_int,
    lock: std::ffi::c_int,
    autosize: std::ffi::c_int,
    series: std::ffi::c_int,
    demand: std::ffi::c_int,
    ems: std::ffi::c_int,
    flow: f64,
    mdot1: f64,
    min1: f64,
    max1: f64,
    minav1: f64,
    maxav1: f64,
    req1: f64,
    mdot2: f64,
    min2: f64,
    max2: f64,
    minav2: f64,
    maxav2: f64,
    req2: f64,
    mdot3: f64,
    min3: f64,
    max3: f64,
    minav3: f64,
    maxav3: f64,
    req3: f64,
    mdot4: f64,
    min4: f64,
    max4: f64,
    minav4: f64,
    maxav4: f64,
    req4: f64,
) -> f64 {
    port::SetFlow(
        which, ncomp, k, lock, autosize, series, demand, ems, flow, mdot1, min1, max1, minav1,
        maxav1, req1, mdot2, min2, max2, minav2, maxav2, req2, mdot3, min3, max3, minav3, maxav3,
        req3, mdot4, min4, max4, minav4, maxav4, req4,
    )
}

/// `SetFlow` as `rust_anything_SetFlow`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SetFlow(
    which: std::ffi::c_int,
    ncomp: std::ffi::c_int,
    k: std::ffi::c_int,
    lock: std::ffi::c_int,
    autosize: std::ffi::c_int,
    series: std::ffi::c_int,
    demand: std::ffi::c_int,
    ems: std::ffi::c_int,
    flow: f64,
    mdot1: f64,
    min1: f64,
    max1: f64,
    minav1: f64,
    maxav1: f64,
    req1: f64,
    mdot2: f64,
    min2: f64,
    max2: f64,
    minav2: f64,
    maxav2: f64,
    req2: f64,
    mdot3: f64,
    min3: f64,
    max3: f64,
    minav3: f64,
    maxav3: f64,
    req3: f64,
    mdot4: f64,
    min4: f64,
    max4: f64,
    minav4: f64,
    maxav4: f64,
    req4: f64,
) -> f64 {
    port::SetFlow(
        which, ncomp, k, lock, autosize, series, demand, ems, flow, mdot1, min1, max1, minav1,
        maxav1, req1, mdot2, min2, max2, minav2, maxav2, req2, mdot3, min3, max3, minav3, maxav3,
        req3, mdot4, min4, max4, minav4, maxav4, req4,
    )
}
