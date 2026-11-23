//! C ABI for `tstat.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double TstatSetPoints(int which, int ctype, int heatused, double heatsp, double coolsp, double hcsp, double dhsp, double dcsp, double setpt0, double hi0, double lo0, int anyop, int opctrl, double frac, double mrt, int anyhum, int ovctrl, double ovrange, double ovratio, double relhum, double dehum, int emsheat, double emsheatval, int emscool, double emscoolval)` (`tstat.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn TstatSetPoints(
    which: std::ffi::c_int,
    ctype: std::ffi::c_int,
    heatused: std::ffi::c_int,
    heatsp: f64,
    coolsp: f64,
    hcsp: f64,
    dhsp: f64,
    dcsp: f64,
    setpt0: f64,
    hi0: f64,
    lo0: f64,
    anyop: std::ffi::c_int,
    opctrl: std::ffi::c_int,
    frac: f64,
    mrt: f64,
    anyhum: std::ffi::c_int,
    ovctrl: std::ffi::c_int,
    ovrange: f64,
    ovratio: f64,
    relhum: f64,
    dehum: f64,
    emsheat: std::ffi::c_int,
    emsheatval: f64,
    emscool: std::ffi::c_int,
    emscoolval: f64,
) -> f64 {
    port::TstatSetPoints(
        which, ctype, heatused, heatsp, coolsp, hcsp, dhsp, dcsp, setpt0, hi0, lo0, anyop, opctrl,
        frac, mrt, anyhum, ovctrl, ovrange, ovratio, relhum, dehum, emsheat, emsheatval, emscool,
        emscoolval,
    )
}

/// `TstatSetPoints` as `rust_anything_TstatSetPoints`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_TstatSetPoints(
    which: std::ffi::c_int,
    ctype: std::ffi::c_int,
    heatused: std::ffi::c_int,
    heatsp: f64,
    coolsp: f64,
    hcsp: f64,
    dhsp: f64,
    dcsp: f64,
    setpt0: f64,
    hi0: f64,
    lo0: f64,
    anyop: std::ffi::c_int,
    opctrl: std::ffi::c_int,
    frac: f64,
    mrt: f64,
    anyhum: std::ffi::c_int,
    ovctrl: std::ffi::c_int,
    ovrange: f64,
    ovratio: f64,
    relhum: f64,
    dehum: f64,
    emsheat: std::ffi::c_int,
    emsheatval: f64,
    emscool: std::ffi::c_int,
    emscoolval: f64,
) -> f64 {
    port::TstatSetPoints(
        which, ctype, heatused, heatsp, coolsp, hcsp, dhsp, dcsp, setpt0, hi0, lo0, anyop, opctrl,
        frac, mrt, anyhum, ovctrl, ovrange, ovratio, relhum, dehum, emsheat, emsheatval, emscool,
        emscoolval,
    )
}
