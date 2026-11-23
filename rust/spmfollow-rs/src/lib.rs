//! C ABI for `spmfollow.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double OneStage(int heating, int stage, double on, double off)` (`spmfollow.c:12`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn OneStage(
    heating: std::ffi::c_int,
    stage: std::ffi::c_int,
    on: f64,
    off: f64,
) -> f64 {
    port::OneStage(heating, stage, on, off)
}

/// `OneStage` as `rust_anything_OneStage`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_OneStage(
    heating: std::ffi::c_int,
    stage: std::ffi::c_int,
    on: f64,
    off: f64,
) -> f64 {
    port::OneStage(heating, stage, on, off)
}

/// `double SZHum(int maxhum, double zonem, double zonew, double hum, double dehum)` (`spmfollow.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SZHum(
    maxhum: std::ffi::c_int,
    zonem: f64,
    zonew: f64,
    hum: f64,
    dehum: f64,
) -> f64 {
    port::SZHum(maxhum, zonem, zonew, hum, dehum)
}

/// `SZHum` as `rust_anything_SZHum`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SZHum(
    maxhum: std::ffi::c_int,
    zonem: f64,
    zonew: f64,
    hum: f64,
    dehum: f64,
) -> f64 {
    port::SZHum(maxhum, zonem, zonew, hum, dehum)
}

/// `double Pretreat(int ctrl, double refsp, double retv, double mixm, double oam, double minsp, double maxsp)` (`spmfollow.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Pretreat(
    ctrl: std::ffi::c_int,
    refsp: f64,
    retv: f64,
    mixm: f64,
    oam: f64,
    minsp: f64,
    maxsp: f64,
) -> f64 {
    port::Pretreat(ctrl, refsp, retv, mixm, oam, minsp, maxsp)
}

/// `Pretreat` as `rust_anything_Pretreat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Pretreat(
    ctrl: std::ffi::c_int,
    refsp: f64,
    retv: f64,
    mixm: f64,
    oam: f64,
    minsp: f64,
    maxsp: f64,
) -> f64 {
    port::Pretreat(ctrl, refsp, retv, mixm, oam, minsp, maxsp)
}

/// `double FollowOA(int wet, double oadb, double oawb, double offset, double mint, double maxt)` (`spmfollow.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FollowOA(
    wet: std::ffi::c_int,
    oadb: f64,
    oawb: f64,
    offset: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::FollowOA(wet, oadb, oawb, offset, mint, maxt)
}

/// `FollowOA` as `rust_anything_FollowOA`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FollowOA(
    wet: std::ffi::c_int,
    oadb: f64,
    oawb: f64,
    offset: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::FollowOA(wet, oadb, oawb, offset, mint, maxt)
}

/// `double FollowNode(int wet, int more, double nodet, double nodewb, double offset, double mint, double maxt)` (`spmfollow.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn FollowNode(
    wet: std::ffi::c_int,
    more: std::ffi::c_int,
    nodet: f64,
    nodewb: f64,
    offset: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::FollowNode(wet, more, nodet, nodewb, offset, mint, maxt)
}

/// `FollowNode` as `rust_anything_FollowNode`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_FollowNode(
    wet: std::ffi::c_int,
    more: std::ffi::c_int,
    nodet: f64,
    nodewb: f64,
    offset: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::FollowNode(wet, more, nodet, nodewb, offset, mint, maxt)
}
