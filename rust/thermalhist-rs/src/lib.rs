//! C ABI for `thermalhist.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double ThermalHist(int which, int seed, int n, int m, int simple, int overrides, int anysrc, int src, int algo, int curnum, int numhist, double tsz, double ctfts, double area, double tout, double tin, double ctfin, double ctfout, double qsrc, double tsrcconst, double tuserconst, double cross0, double inside0, double outside0, double srcin0, double srcout0, double tsrcout0, double tsrcin0, double tsrcq0, double tuserout0, double tuserin0, double tusersrc0)` (`thermalhist.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn ThermalHist(
    which: std::ffi::c_int,
    seed: std::ffi::c_int,
    n: std::ffi::c_int,
    m: std::ffi::c_int,
    simple: std::ffi::c_int,
    overrides: std::ffi::c_int,
    anysrc: std::ffi::c_int,
    src: std::ffi::c_int,
    algo: std::ffi::c_int,
    curnum: std::ffi::c_int,
    numhist: std::ffi::c_int,
    tsz: f64,
    ctfts: f64,
    area: f64,
    tout: f64,
    tin: f64,
    ctfin: f64,
    ctfout: f64,
    qsrc: f64,
    tsrcconst: f64,
    tuserconst: f64,
    cross0: f64,
    inside0: f64,
    outside0: f64,
    srcin0: f64,
    srcout0: f64,
    tsrcout0: f64,
    tsrcin0: f64,
    tsrcq0: f64,
    tuserout0: f64,
    tuserin0: f64,
    tusersrc0: f64,
) -> f64 {
    port::ThermalHist(
        which, seed, n, m, simple, overrides, anysrc, src, algo, curnum, numhist, tsz, ctfts, area,
        tout, tin, ctfin, ctfout, qsrc, tsrcconst, tuserconst, cross0, inside0, outside0, srcin0,
        srcout0, tsrcout0, tsrcin0, tsrcq0, tuserout0, tuserin0, tusersrc0,
    )
}

/// `ThermalHist` as `rust_anything_ThermalHist`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_ThermalHist(
    which: std::ffi::c_int,
    seed: std::ffi::c_int,
    n: std::ffi::c_int,
    m: std::ffi::c_int,
    simple: std::ffi::c_int,
    overrides: std::ffi::c_int,
    anysrc: std::ffi::c_int,
    src: std::ffi::c_int,
    algo: std::ffi::c_int,
    curnum: std::ffi::c_int,
    numhist: std::ffi::c_int,
    tsz: f64,
    ctfts: f64,
    area: f64,
    tout: f64,
    tin: f64,
    ctfin: f64,
    ctfout: f64,
    qsrc: f64,
    tsrcconst: f64,
    tuserconst: f64,
    cross0: f64,
    inside0: f64,
    outside0: f64,
    srcin0: f64,
    srcout0: f64,
    tsrcout0: f64,
    tsrcin0: f64,
    tsrcq0: f64,
    tuserout0: f64,
    tuserin0: f64,
    tusersrc0: f64,
) -> f64 {
    port::ThermalHist(
        which, seed, n, m, simple, overrides, anysrc, src, algo, curnum, numhist, tsz, ctfts, area,
        tout, tin, ctfin, ctfout, qsrc, tsrcconst, tuserconst, cross0, inside0, outside0, srcin0,
        srcout0, tsrcout0, tsrcin0, tsrcq0, tuserout0, tuserin0, tusersrc0,
    )
}
