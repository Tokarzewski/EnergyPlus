//! C ABI for `outsidetemp.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double OutsideSurfTemp(int which, double cross0, double inside0, double outside0, double srcout0, double srcin0, int src, double hconvint, double hconvext, double hairext, double hskyext, double hgrdext, double hsrdext, double ctfout, double ctfin, double qswout, double qswin, double qsrc, double tempin, double qaddout, double qintgains, double qnetlwin, double mat, double qswmvins, double hmov, double text, double skytemp, double odb, int hassrd, double srdtemp, int usegnd, double gndtemp, int osc, double osctrad, double oschrad)` (`outsidetemp.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn OutsideSurfTemp(
    which: std::ffi::c_int,
    cross0: f64,
    inside0: f64,
    outside0: f64,
    srcout0: f64,
    srcin0: f64,
    src: std::ffi::c_int,
    hconvint: f64,
    hconvext: f64,
    hairext: f64,
    hskyext: f64,
    hgrdext: f64,
    hsrdext: f64,
    ctfout: f64,
    ctfin: f64,
    qswout: f64,
    qswin: f64,
    qsrc: f64,
    tempin: f64,
    qaddout: f64,
    qintgains: f64,
    qnetlwin: f64,
    mat: f64,
    qswmvins: f64,
    hmov: f64,
    text: f64,
    skytemp: f64,
    odb: f64,
    hassrd: std::ffi::c_int,
    srdtemp: f64,
    usegnd: std::ffi::c_int,
    gndtemp: f64,
    osc: std::ffi::c_int,
    osctrad: f64,
    oschrad: f64,
) -> f64 {
    port::OutsideSurfTemp(
        which, cross0, inside0, outside0, srcout0, srcin0, src, hconvint, hconvext, hairext,
        hskyext, hgrdext, hsrdext, ctfout, ctfin, qswout, qswin, qsrc, tempin, qaddout, qintgains,
        qnetlwin, mat, qswmvins, hmov, text, skytemp, odb, hassrd, srdtemp, usegnd, gndtemp, osc,
        osctrad, oschrad,
    )
}

/// `OutsideSurfTemp` as `rust_anything_OutsideSurfTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_OutsideSurfTemp(
    which: std::ffi::c_int,
    cross0: f64,
    inside0: f64,
    outside0: f64,
    srcout0: f64,
    srcin0: f64,
    src: std::ffi::c_int,
    hconvint: f64,
    hconvext: f64,
    hairext: f64,
    hskyext: f64,
    hgrdext: f64,
    hsrdext: f64,
    ctfout: f64,
    ctfin: f64,
    qswout: f64,
    qswin: f64,
    qsrc: f64,
    tempin: f64,
    qaddout: f64,
    qintgains: f64,
    qnetlwin: f64,
    mat: f64,
    qswmvins: f64,
    hmov: f64,
    text: f64,
    skytemp: f64,
    odb: f64,
    hassrd: std::ffi::c_int,
    srdtemp: f64,
    usegnd: std::ffi::c_int,
    gndtemp: f64,
    osc: std::ffi::c_int,
    osctrad: f64,
    oschrad: f64,
) -> f64 {
    port::OutsideSurfTemp(
        which, cross0, inside0, outside0, srcout0, srcin0, src, hconvint, hconvext, hairext,
        hskyext, hgrdext, hsrdext, ctfout, ctfin, qswout, qswin, qsrc, tempin, qaddout, qintgains,
        qnetlwin, mat, qswmvins, hmov, text, skytemp, odb, hassrd, srdtemp, usegnd, gndtemp, osc,
        osctrad, oschrad,
    )
}
