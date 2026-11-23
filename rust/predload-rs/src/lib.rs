//! C ABI for `predload.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double PredictedSystemLoad(int which, int ctrl, int algo, double rafnfrac, double tempdep, double tempind, double aircap, double t1, double setpt, double setptlo, double setpthi, int hasite, double itetemp, int beginsim, int hasnode, double nodetemp, int staged, int stagenum, double setptlast, double lcf, int mult, int listmult)` (`predload.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn PredictedSystemLoad(
    which: std::ffi::c_int,
    ctrl: std::ffi::c_int,
    algo: std::ffi::c_int,
    rafnfrac: f64,
    tempdep: f64,
    tempind: f64,
    aircap: f64,
    t1: f64,
    setpt: f64,
    setptlo: f64,
    setpthi: f64,
    hasite: std::ffi::c_int,
    itetemp: f64,
    beginsim: std::ffi::c_int,
    hasnode: std::ffi::c_int,
    nodetemp: f64,
    staged: std::ffi::c_int,
    stagenum: std::ffi::c_int,
    setptlast: f64,
    lcf: f64,
    mult: std::ffi::c_int,
    listmult: std::ffi::c_int,
) -> f64 {
    port::PredictedSystemLoad(
        which, ctrl, algo, rafnfrac, tempdep, tempind, aircap, t1, setpt, setptlo, setpthi, hasite,
        itetemp, beginsim, hasnode, nodetemp, staged, stagenum, setptlast, lcf, mult, listmult,
    )
}

/// `PredictedSystemLoad` as `rust_anything_PredictedSystemLoad`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_PredictedSystemLoad(
    which: std::ffi::c_int,
    ctrl: std::ffi::c_int,
    algo: std::ffi::c_int,
    rafnfrac: f64,
    tempdep: f64,
    tempind: f64,
    aircap: f64,
    t1: f64,
    setpt: f64,
    setptlo: f64,
    setpthi: f64,
    hasite: std::ffi::c_int,
    itetemp: f64,
    beginsim: std::ffi::c_int,
    hasnode: std::ffi::c_int,
    nodetemp: f64,
    staged: std::ffi::c_int,
    stagenum: std::ffi::c_int,
    setptlast: f64,
    lcf: f64,
    mult: std::ffi::c_int,
    listmult: std::ffi::c_int,
) -> f64 {
    port::PredictedSystemLoad(
        which, ctrl, algo, rafnfrac, tempdep, tempind, aircap, t1, setpt, setptlo, setpthi, hasite,
        itetemp, beginsim, hasnode, nodetemp, staged, stagenum, setptlast, lcf, mult, listmult,
    )
}
