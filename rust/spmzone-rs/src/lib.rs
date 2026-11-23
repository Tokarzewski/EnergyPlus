//! C ABI for `spmzone.c`: drop-in replacements with the original names and signatures, generated
//! by rust-anything from `rust-anything.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `double SZTemp(int heating, double zonem, double inw, double toheat, double tocool, double zonet, double mint, double maxt)` (`spmzone.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn SZTemp(
    heating: std::ffi::c_int,
    zonem: f64,
    inw: f64,
    toheat: f64,
    tocool: f64,
    zonet: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::SZTemp(heating, zonem, inw, toheat, tocool, zonet, mint, maxt)
}

/// `SZTemp` as `rust_anything_SZTemp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_SZTemp(
    heating: std::ffi::c_int,
    zonem: f64,
    inw: f64,
    toheat: f64,
    tocool: f64,
    zonet: f64,
    mint: f64,
    maxt: f64,
) -> f64 {
    port::SZTemp(heating, zonem, inw, toheat, tocool, zonet, mint, maxt)
}

/// `double OAReset(int schedv, double oat, double low1, double high1, double lsp1, double hsp1, double low2, double high2, double lsp2, double hsp2)` (`spmzone.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn OAReset(
    schedv: std::ffi::c_int,
    oat: f64,
    low1: f64,
    high1: f64,
    lsp1: f64,
    hsp1: f64,
    low2: f64,
    high2: f64,
    lsp2: f64,
    hsp2: f64,
) -> f64 {
    port::OAReset(
        schedv, oat, low1, high1, lsp1, hsp1, low2, high2, lsp2, hsp2,
    )
}

/// `OAReset` as `rust_anything_OAReset`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn rust_anything_OAReset(
    schedv: std::ffi::c_int,
    oat: f64,
    low1: f64,
    high1: f64,
    lsp1: f64,
    hsp1: f64,
    low2: f64,
    high2: f64,
    lsp2: f64,
    hsp2: f64,
) -> f64 {
    port::OAReset(
        schedv, oat, low1, high1, lsp1, hsp1, low2, high2, lsp2, hsp2,
    )
}
