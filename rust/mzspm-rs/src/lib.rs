//! C ABI for `mzspm.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double MZTemp(int heating, int nc, double mint, double maxt, double m1, double load1, double zt1, double zw1, double iw1, double m2, double load2, double zt2, double zw2, double iw2, double m3, double load3, double zt3, double zw3, double iw3, double m4, double load4, double zt4, double zw4, double iw4)` (`mzspm.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn MZTemp(
    heating: std::ffi::c_int,
    nc: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    m1: f64,
    load1: f64,
    zt1: f64,
    zw1: f64,
    iw1: f64,
    m2: f64,
    load2: f64,
    zt2: f64,
    zw2: f64,
    iw2: f64,
    m3: f64,
    load3: f64,
    zt3: f64,
    zw3: f64,
    iw3: f64,
    m4: f64,
    load4: f64,
    zt4: f64,
    zw4: f64,
    iw4: f64,
) -> f64 {
    port::MZTemp(
        heating, nc, mint, maxt, m1, load1, zt1, zw1, iw1, m2, load2, zt2, zw2, iw2, m3, load3,
        zt3, zw3, iw3, m4, load4, zt4, zw4, iw4,
    )
}

/// `MZTemp` as `rust_anything_MZTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_MZTemp(
    heating: std::ffi::c_int,
    nc: std::ffi::c_int,
    mint: f64,
    maxt: f64,
    m1: f64,
    load1: f64,
    zt1: f64,
    zw1: f64,
    iw1: f64,
    m2: f64,
    load2: f64,
    zt2: f64,
    zw2: f64,
    iw2: f64,
    m3: f64,
    load3: f64,
    zt3: f64,
    zw3: f64,
    iw3: f64,
    m4: f64,
    load4: f64,
    zt4: f64,
    zw4: f64,
    iw4: f64,
) -> f64 {
    port::MZTemp(
        heating, nc, mint, maxt, m1, load1, zt1, zw1, iw1, m2, load2, zt2, zw2, iw2, m3, load3,
        zt3, zw3, iw3, m4, load4, zt4, zw4, iw4,
    )
}

/// `double MZHum(int kind, int nc, double minh, double maxh, double m1, double hum1, double dehum1, double zw1, double m2, double hum2, double dehum2, double zw2, double m3, double hum3, double dehum3, double zw3, double m4, double hum4, double dehum4, double zw4)` (`mzspm.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn MZHum(
    kind: std::ffi::c_int,
    nc: std::ffi::c_int,
    minh: f64,
    maxh: f64,
    m1: f64,
    hum1: f64,
    dehum1: f64,
    zw1: f64,
    m2: f64,
    hum2: f64,
    dehum2: f64,
    zw2: f64,
    m3: f64,
    hum3: f64,
    dehum3: f64,
    zw3: f64,
    m4: f64,
    hum4: f64,
    dehum4: f64,
    zw4: f64,
) -> f64 {
    port::MZHum(
        kind, nc, minh, maxh, m1, hum1, dehum1, zw1, m2, hum2, dehum2, zw2, m3, hum3, dehum3, zw3,
        m4, hum4, dehum4, zw4,
    )
}

/// `MZHum` as `rust_anything_MZHum`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_MZHum(
    kind: std::ffi::c_int,
    nc: std::ffi::c_int,
    minh: f64,
    maxh: f64,
    m1: f64,
    hum1: f64,
    dehum1: f64,
    zw1: f64,
    m2: f64,
    hum2: f64,
    dehum2: f64,
    zw2: f64,
    m3: f64,
    hum3: f64,
    dehum3: f64,
    zw3: f64,
    m4: f64,
    hum4: f64,
    dehum4: f64,
    zw4: f64,
) -> f64 {
    port::MZHum(
        kind, nc, minh, maxh, m1, hum1, dehum1, zw1, m2, hum2, dehum2, zw2, m3, hum3, dehum3, zw3,
        m4, hum4, dehum4, zw4,
    )
}
