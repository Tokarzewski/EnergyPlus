//! The Rust port of `localenv.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The local environment averages of `HeatBalanceSurfaceManager` for one exterior surface:
//! `GetGroundSurfacesTemperatureAverage`, `GetGroundSurfacesReflectanceAverage`,
//! `ReSetGroundSurfacesViewFactor` and `GetSurroundingSurfacesTemperatureAverage`. Every
//! expression keeps the C++ order. The oracle's guards (see its shim) return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

/// `Constant::Kelvin`.
const KELVIN: f64 = 273.15;

/// ObjexxFCL `pow_4`.
fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

/// ObjexxFCL `root_4`.
fn root_4(x: f64) -> f64 {
    x.sqrt().sqrt()
}

/// `DataSurfaces::GroundSurfacesData`: a schedule is its current value, `None` for `nullptr`.
#[derive(Clone, Copy)]
struct GroundSurface {
    view_factor: f64,
    temp_sched: Option<f64>,
    refl_sched: Option<f64>,
}

/// `DataSurfaces::GroundSurfacesProperty`.
struct GroundSurfacesProperty {
    num_gnd_surfs: usize,
    gnd_surfs: [GroundSurface; 4],
    surfs_temp_avg: f64,
    surfs_refl_avg: f64,
    surfs_view_factor_sum: f64,
}

/// `DataSurfaces::SurroundingSurfProperty`: the view factor and the temperature schedule's
/// current value.
#[derive(Clone, Copy)]
struct SurroundingSurface {
    view_factor: f64,
    temp_sched: f64,
}

/// The surface's fields these functions read and set, with its two properties.
struct Surface {
    is_surf_property_gnd_surfaces_defined: bool,
    use_surf_property_gnd_surf_temp: bool,
    use_surf_property_gnd_surf_refl: bool,
    view_factor_ground_ir: f64,
    surf_has_surrounding_surf_property: bool,
    view_factor_srd_surfs: f64,
    srd_surf_temp: f64,
    gnd: GroundSurfacesProperty,
    total_surrounding_surface: usize,
    surrounding_surfs: [SurroundingSurface; 4],
}

/// `GetGroundSurfacesTemperatureAverage` for the one surface.
fn get_ground_surfaces_temperature_average(any_local: bool, s: &mut Surface) {
    if !any_local {
        return;
    }
    if !s.is_surf_property_gnd_surfaces_defined {
        return;
    }
    let g = &mut s.gnd;
    if g.surfs_view_factor_sum == 0.0 {
        s.use_surf_property_gnd_surf_temp = false;
        return;
    }
    let mut gnd_surface_temp_sum = 0.0;
    for gs in &g.gnd_surfs[..g.num_gnd_surfs] {
        let gnd_surf_view_factor = gs.view_factor;
        if gnd_surf_view_factor == 0.0 {
            continue;
        }
        let Some(gnd_surface_temp) = gs.temp_sched else {
            continue;
        };
        gnd_surface_temp_sum += gnd_surf_view_factor * pow_4(gnd_surface_temp + KELVIN);
    }
    if gnd_surface_temp_sum == 0.0 {
        g.surfs_temp_avg = 0.0;
        s.use_surf_property_gnd_surf_temp = false;
        return;
    }
    g.surfs_temp_avg = root_4(gnd_surface_temp_sum / g.surfs_view_factor_sum) - KELVIN;
}

/// `GetGroundSurfacesReflectanceAverage` for the one surface.
fn get_ground_surfaces_reflectance_average(any_local: bool, s: &mut Surface) {
    if !any_local {
        return;
    }
    if !s.is_surf_property_gnd_surfaces_defined {
        return;
    }
    let g = &mut s.gnd;
    if g.surfs_view_factor_sum == 0.0 {
        s.use_surf_property_gnd_surf_refl = false;
        return;
    }
    let mut gnd_surfs_refl_sum = 0.0;
    for gs in &g.gnd_surfs[..g.num_gnd_surfs] {
        let Some(gnd_surf_refl) = gs.refl_sched else {
            continue;
        };
        gnd_surfs_refl_sum += gs.view_factor * gnd_surf_refl;
    }
    if gnd_surfs_refl_sum == 0.0 {
        g.surfs_refl_avg = 0.0;
        s.use_surf_property_gnd_surf_refl = false;
        return;
    }
    g.surfs_refl_avg = gnd_surfs_refl_sum / g.surfs_view_factor_sum;
}

/// `ReSetGroundSurfacesViewFactor` for the surface.
fn re_set_ground_surfaces_view_factor(s: &mut Surface) {
    if !s.is_surf_property_gnd_surfaces_defined {
        return;
    }
    s.gnd.surfs_view_factor_sum = s.view_factor_ground_ir;
    if s.gnd.surfs_view_factor_sum == 0.0 {
        s.use_surf_property_gnd_surf_refl = false;
        s.use_surf_property_gnd_surf_temp = false;
        return;
    }
    s.gnd.gnd_surfs[0].view_factor = s.gnd.surfs_view_factor_sum;
}

/// `GetSurroundingSurfacesTemperatureAverage` for the one surface.
fn get_surrounding_surfaces_temperature_average(any_local: bool, s: &mut Surface) {
    if !any_local {
        return;
    }
    if !s.surf_has_surrounding_surf_property {
        return;
    }
    let mut srd_surface_temp_sum = 0.0;
    for ss in &s.surrounding_surfs[..s.total_surrounding_surface] {
        let srd_surface_temp = ss.temp_sched + KELVIN;
        srd_surface_temp_sum += ss.view_factor * pow_4(srd_surface_temp);
    }
    s.srd_surf_temp = root_4(srd_surface_temp_sum / s.view_factor_srd_surfs) - KELVIN;
}

/// A 0 or 1 flag as a bool, `None` for anything else.
fn flag(v: i32) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// A count of 0 to 4, `None` for anything else.
fn count(v: i32) -> Option<usize> {
    usize::try_from(v).ok().filter(|&n| n <= 4)
}

/// A ground surface with view factor `vf` and, when `ht` or `hr` is 1, schedules at `t` and `r`.
fn ground(vf: f64, ht: i32, t: f64, hr: i32, r: f64) -> Option<GroundSurface> {
    Some(GroundSurface {
        view_factor: vf,
        temp_sched: flag(ht)?.then_some(t),
        refl_sched: flag(hr)?.then_some(r),
    })
}

/// Port of `double LocalEnvironment(...)` (`localenv.c:8`): see the oracle's shim for the
/// arguments and what `which` and `op` pick.
pub fn LocalEnvironment(
    which: std::ffi::c_int,
    op: std::ffi::c_int,
    anylocal: std::ffi::c_int,
    gnddefined: std::ffi::c_int,
    vfsum: f64,
    ngnd: std::ffi::c_int,
    gvf1: f64,
    ht1: std::ffi::c_int,
    gt1: f64,
    hr1: std::ffi::c_int,
    gr1: f64,
    gvf2: f64,
    ht2: std::ffi::c_int,
    gt2: f64,
    hr2: std::ffi::c_int,
    gr2: f64,
    gvf3: f64,
    ht3: std::ffi::c_int,
    gt3: f64,
    hr3: std::ffi::c_int,
    gr3: f64,
    gvf4: f64,
    ht4: std::ffi::c_int,
    gt4: f64,
    hr4: std::ffi::c_int,
    gr4: f64,
    hassrd: std::ffi::c_int,
    nsrd: std::ffi::c_int,
    svf1: f64,
    st1: f64,
    svf2: f64,
    st2: f64,
    svf3: f64,
    st3: f64,
    svf4: f64,
    st4: f64,
    vfsrd: f64,
    vfgir: f64,
    tavg0: f64,
    ravg0: f64,
    srdt0: f64,
    usegndtemp0: std::ffi::c_int,
    usegndrefl0: std::ffi::c_int,
) -> f64 {
    let built = (|| {
        let any_local = flag(anylocal)?;
        let gnd_surfs = [
            ground(gvf1, ht1, gt1, hr1, gr1)?,
            ground(gvf2, ht2, gt2, hr2, gr2)?,
            ground(gvf3, ht3, gt3, hr3, gr3)?,
            ground(gvf4, ht4, gt4, hr4, gr4)?,
        ];
        let srd = |view_factor, temp_sched| SurroundingSurface {
            view_factor,
            temp_sched,
        };
        let s = Surface {
            is_surf_property_gnd_surfaces_defined: flag(gnddefined)?,
            use_surf_property_gnd_surf_temp: flag(usegndtemp0)?,
            use_surf_property_gnd_surf_refl: flag(usegndrefl0)?,
            view_factor_ground_ir: vfgir,
            surf_has_surrounding_surf_property: flag(hassrd)?,
            view_factor_srd_surfs: vfsrd,
            srd_surf_temp: srdt0,
            gnd: GroundSurfacesProperty {
                num_gnd_surfs: count(ngnd)?,
                gnd_surfs,
                surfs_temp_avg: tavg0,
                surfs_refl_avg: ravg0,
                surfs_view_factor_sum: vfsum,
            },
            total_surrounding_surface: count(nsrd)?,
            surrounding_surfs: [
                srd(svf1, st1),
                srd(svf2, st2),
                srd(svf3, st3),
                srd(svf4, st4),
            ],
        };
        Some((any_local, s))
    })();
    let Some((any_local, mut s)) = built else {
        return f64::NAN;
    };
    if !(0..=6).contains(&which) {
        return f64::NAN;
    }
    match op {
        0 => get_ground_surfaces_temperature_average(any_local, &mut s),
        1 => get_ground_surfaces_reflectance_average(any_local, &mut s),
        2 => re_set_ground_surfaces_view_factor(&mut s),
        3 => get_surrounding_surfaces_temperature_average(any_local, &mut s),
        _ => return f64::NAN,
    }
    let b = |v: bool| if v { 1.0 } else { 0.0 };
    match which {
        0 => s.gnd.surfs_temp_avg,
        1 => b(s.use_surf_property_gnd_surf_temp),
        2 => s.gnd.surfs_refl_avg,
        3 => b(s.use_surf_property_gnd_surf_refl),
        4 => s.srd_surf_temp,
        5 => s.gnd.surfs_view_factor_sum,
        _ => s.gnd.gnd_surfs[0].view_factor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One call with two ground surfaces (view factors 0.2 and 0.3, at 10 C and 30 C,
    /// reflectances 0.2 and 0.4) and two surrounding surfaces (0.1 and 0.3, at 15 C and 25 C).
    fn call(which: i32, op: i32, anylocal: i32, vfsum: f64, vfgir: f64) -> f64 {
        LocalEnvironment(
            which, op, anylocal, 1, vfsum, 2, 0.2, 1, 10.0, 1, 0.2, 0.3, 1, 30.0, 1, 0.4, 0.0, 0,
            0.0, 0, 0.0, 0.0, 0, 0.0, 0, 0.0, 1, 2, 0.1, 15.0, 0.3, 25.0, 0.0, 0.0, 0.0, 0.0, 0.4,
            vfgir, -99.0, -99.0, -99.0, 1, 1,
        )
    }

    #[test]
    fn ground_temperature_is_radiative_average() {
        let t = call(0, 0, 1, 0.5, 0.5);
        let k = (0.2 * pow_4(283.15) + 0.3 * pow_4(303.15)) / 0.5;
        assert_eq!(t, root_4(k) - KELVIN);
        assert!(t > 20.0 && t < 30.0);
    }

    #[test]
    fn ground_reflectance_is_weighted_average() {
        assert_eq!(call(2, 1, 1, 0.5, 0.5), (0.2 * 0.2 + 0.3 * 0.4) / 0.5);
    }

    #[test]
    fn zero_view_factor_sum_turns_ground_off() {
        assert_eq!(call(1, 0, 1, 0.0, 0.5), 0.0);
        assert_eq!(call(0, 0, 1, 0.0, 0.5), -99.0);
        assert_eq!(call(3, 2, 1, 0.5, 0.0), 0.0);
        assert_eq!(call(6, 2, 1, 0.5, 0.35), 0.35);
    }

    #[test]
    fn surrounding_temperature_and_no_local_environment() {
        let t = call(4, 3, 1, 0.5, 0.5);
        assert_eq!(
            t,
            root_4((0.1 * pow_4(288.15) + 0.3 * pow_4(298.15)) / 0.4) - KELVIN
        );
        assert_eq!(call(4, 3, 0, 0.5, 0.5), -99.0);
    }

    #[test]
    fn guards_return_nan() {
        assert!(call(7, 0, 1, 0.5, 0.5).is_nan());
        assert!(call(0, 4, 1, 0.5, 0.5).is_nan());
        assert!(call(0, 0, 2, 0.5, 0.5).is_nan());
    }
}
