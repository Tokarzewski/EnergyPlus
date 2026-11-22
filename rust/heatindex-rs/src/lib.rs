//! C ABI for `heatindex.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double pvstar(double T)` (`heatindex.c:22`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn pvstar(T: f64) -> f64 {
    port::pvstar(T)
}

/// `pvstar` as `rust_anything_pvstar`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_pvstar(T: f64) -> f64 {
    port::pvstar(T)
}

/// `double Le(double T)` (`heatindex.c:27`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Le(T: f64) -> f64 {
    port::Le(T)
}

/// `Le` as `rust_anything_Le`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Le(T: f64) -> f64 {
    port::Le(T)
}

/// `double Qv(double Ta, double Pa)` (`heatindex.c:32`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Qv(Ta: f64, Pa: f64) -> f64 {
    port::Qv(Ta, Pa)
}

/// `Qv` as `rust_anything_Qv`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Qv(Ta: f64, Pa: f64) -> f64 {
    port::Qv(Ta, Pa)
}

/// `double Zs(double Rs)` (`heatindex.c:37`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Zs(Rs: f64) -> f64 {
    port::Zs(Rs)
}

/// `Zs` as `rust_anything_Zs`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Zs(Rs: f64) -> f64 {
    port::Zs(Rs)
}

/// `double Ra(double Ts, double Ta)` (`heatindex.c:42`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Ra(Ts: f64, Ta: f64) -> f64 {
    port::Ra(Ts, Ta)
}

/// `Ra` as `rust_anything_Ra`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Ra(Ts: f64, Ta: f64) -> f64 {
    port::Ra(Ts, Ta)
}

/// `double Ra_bar(double Tf, double Ta)` (`heatindex.c:47`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Ra_bar(Tf: f64, Ta: f64) -> f64 {
    port::Ra_bar(Tf, Ta)
}

/// `Ra_bar` as `rust_anything_Ra_bar`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Ra_bar(Tf: f64, Ta: f64) -> f64 {
    port::Ra_bar(Tf, Ta)
}

/// `double Ra_un(double Ts, double Ta)` (`heatindex.c:52`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn Ra_un(Ts: f64, Ta: f64) -> f64 {
    port::Ra_un(Ts, Ta)
}

/// `Ra_un` as `rust_anything_Ra_un`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_Ra_un(Ts: f64, Ta: f64) -> f64 {
    port::Ra_un(Ts, Ta)
}

/// `double find_eqvar_phi(double Ta, double RH)` (`heatindex.c:57`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn find_eqvar_phi(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_phi(Ta, RH)
}

/// `find_eqvar_phi` as `rust_anything_find_eqvar_phi`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_find_eqvar_phi(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_phi(Ta, RH)
}

/// `double find_eqvar_Rf(double Ta, double RH)` (`heatindex.c:62`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn find_eqvar_Rf(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_Rf(Ta, RH)
}

/// `find_eqvar_Rf` as `rust_anything_find_eqvar_Rf`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_find_eqvar_Rf(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_Rf(Ta, RH)
}

/// `double find_eqvar_rs(double Ta, double RH)` (`heatindex.c:67`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn find_eqvar_rs(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_rs(Ta, RH)
}

/// `find_eqvar_rs` as `rust_anything_find_eqvar_rs`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_find_eqvar_rs(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_rs(Ta, RH)
}

/// `double find_eqvar_dTcdt(double Ta, double RH)` (`heatindex.c:72`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn find_eqvar_dTcdt(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_dTcdt(Ta, RH)
}

/// `find_eqvar_dTcdt` as `rust_anything_find_eqvar_dTcdt`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_find_eqvar_dTcdt(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_dTcdt(Ta, RH)
}

/// `double find_eqvar_value(double Ta, double RH)` (`heatindex.c:77`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn find_eqvar_value(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_value(Ta, RH)
}

/// `find_eqvar_value` as `rust_anything_find_eqvar_value`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_find_eqvar_value(Ta: f64, RH: f64) -> f64 {
    port::find_eqvar_value(Ta, RH)
}

/// `int find_eqvar_name(double Ta, double RH)` (`heatindex.c:82`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn find_eqvar_name(Ta: f64, RH: f64) -> std::ffi::c_int {
    port::find_eqvar_name(Ta, RH)
}

/// `find_eqvar_name` as `rust_anything_find_eqvar_name`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_find_eqvar_name(Ta: f64, RH: f64) -> std::ffi::c_int {
    port::find_eqvar_name(Ta, RH)
}

/// `double find_T(int eqvar_name, double eqvar)` (`heatindex.c:87`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn find_T(eqvar_name: std::ffi::c_int, eqvar: f64) -> f64 {
    port::find_T(eqvar_name, eqvar)
}

/// `find_T` as `rust_anything_find_T`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_find_T(eqvar_name: std::ffi::c_int, eqvar: f64) -> f64 {
    port::find_T(eqvar_name, eqvar)
}

/// `double heatindex(double Ta, double RH)` (`heatindex.c:92`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn heatindex(Ta: f64, RH: f64) -> f64 {
    port::heatindex(Ta, RH)
}

/// `heatindex` as `rust_anything_heatindex`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_heatindex(Ta: f64, RH: f64) -> f64 {
    port::heatindex(Ta, RH)
}
