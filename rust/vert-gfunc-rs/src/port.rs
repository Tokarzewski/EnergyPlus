//! The Rust port of `vertgfunc.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The vertical ground heat exchanger's g-function curve from EnergyPlus
//! `GroundHeatExchangers/Vertical.cc` (`GLHEVert`): `calcGFunctions` with the uniform heat flux
//! method (the short time steps from `vert-fluid-rs`, the long ones from `vert-rs`), then
//! `combineShortAndLongTimestepGFunctions`, and `getGFunc` with `interpGFunc` from `Base.cc`.
//! Only the long-time-step values the combined curve keeps are worked out; the C++ computes them
//! all. Nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

use vert_fluid_rs::port::{ShortInputs, short_series};
use vert_rs::port::{Props, lntts, uniform_heat_flux_gfunction};

/// The most long-time-step values a case may make, as the oracle caps them.
const MAX_LONG: usize = 60;

/// A field of `nx` by `ny` boreholes on a water loop, as the oracle builds it.
#[derive(Clone, Copy)]
struct Field {
    nx: i32,
    ny: i32,
    spacing: f64,
    props: Props,
    short: ShortInputs,
    max_sim_years: f64,
}

impl Field {
    /// The combined curve: LNTTS values and g-functions, or `None` outside the oracle's domain
    /// (ADR-009): 1 or 2 boreholes each way, at most 20000 short and 60 long steps, and an LNTTS
    /// vector of at least two values with no NaN.
    fn combined(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        if !(1..=2).contains(&self.nx) || !(1..=2).contains(&self.ny) {
            return None;
        }
        let s = &self.short;
        let diffusivity = s.soil_k / s.soil_rho_cp;
        let long = lntts(self.props.bh_length, diffusivity, self.max_sim_years)
            .filter(|l| l.len() <= MAX_LONG)?;
        let (mut x, mut y): (Vec<f64>, Vec<f64>) =
            short_series(s)?.into_iter().map(|(g, l)| (l, g)).unzip();
        let highest = *x.last()?;
        let mut bhs = Vec::new();
        for iy in 0..self.ny {
            for ix in 0..self.nx {
                bhs.push((f64::from(ix) * self.spacing, f64::from(iy) * self.spacing));
            }
        }
        let t_s = (self.props.bh_length * self.props.bh_length) / (9.0 * diffusivity);
        for l in long.into_iter().filter(|&l| l > highest) {
            x.push(l);
            y.push(uniform_heat_flux_gfunction(
                &bhs,
                self.props,
                diffusivity,
                l.exp() * t_s,
            ));
        }
        (x.len() >= 2 && !x.iter().any(|v| v.is_nan())).then_some((x, y))
    }
}

/// Port of `GLHEBase::interpGFunc` (`Base.cc:548`): linear, and extrapolated past both ends.
/// `x` is sorted, has no NaN and at least two values.
fn interp(x: &[f64], y: &[f64], x_val: f64) -> f64 {
    // std::upper_bound. For a NaN x_val it finds 0 where the C++ finds the end; the result is NaN
    // either way.
    let upper = x.partition_point(|&e| e <= x_val);
    let (l, u) = if upper == 0 {
        (0, 1)
    } else if upper == x.len() {
        (x.len() - 2, x.len() - 1)
    } else {
        (upper - 1, upper)
    };
    (x_val - x[l]) / (x[u] - x[l]) * (y[u] - y[l]) + y[l]
}

fn field(
    nx: i32,
    ny: i32,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    inletTemp: f64,
    maxSimYears: f64,
) -> Field {
    Field {
        nx,
        ny,
        spacing,
        props: Props {
            bh_length: bhLength,
            bh_top_depth: bhTopDepth,
            bh_diameter: bhDiameter,
        },
        short: ShortInputs {
            bh: [bhLength, bhDiameter, uTubeDist, groutK],
            grout_rho_cp: groutRhoCp,
            soil_k: soilK,
            soil_rho_cp: soilRhoCp,
            pipe: [outDia, thickness, kPipe],
            pipe_rho_cp: pipeRhoCp,
            mass_flow: massFlow,
            num_bh: nx.saturating_mul(ny),
            inlet_temp: inletTemp,
        },
        max_sim_years: maxSimYears,
    }
}

/// The k-th value of the combined LNTTS (`which` 0) or g-function (1) vector.
fn combined_at(k: i32, f: Field, which: usize) -> f64 {
    let Some((x, y)) = f.combined() else {
        return f64::NAN;
    };
    let v = if which == 0 { x } else { y };
    usize::try_from(k)
        .ok()
        .and_then(|k| v.get(k).copied())
        .unwrap_or(f64::NAN)
}

pub fn VGCombinedLen(
    nx: i32,
    ny: i32,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    inletTemp: f64,
    maxSimYears: f64,
) -> i32 {
    let f = field(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    );
    f.combined()
        .and_then(|(x, _)| i32::try_from(x.len()).ok())
        .unwrap_or(0)
}

pub fn VGCombinedLntts(
    k: i32,
    nx: i32,
    ny: i32,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    let f = field(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    );
    combined_at(k, f, 0)
}

pub fn VGCombinedGFunction(
    k: i32,
    nx: i32,
    ny: i32,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    let f = field(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    );
    combined_at(k, f, 1)
}

/// Port of `GLHEVert::getGFunc` (`Vertical.cc:1464`) on the combined curve; a positive
/// `gRefRatio` replaces the field's own reference ratio (`bhRadius / bhLength`).
pub fn VGGFunc(
    time: f64,
    gRefRatio: f64,
    nx: i32,
    ny: i32,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    groutRhoCp: f64,
    soilK: f64,
    soilRhoCp: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    pipeRhoCp: f64,
    massFlow: f64,
    inletTemp: f64,
    maxSimYears: f64,
) -> f64 {
    let f = field(
        nx,
        ny,
        spacing,
        bhLength,
        bhTopDepth,
        bhDiameter,
        uTubeDist,
        groutK,
        groutRhoCp,
        soilK,
        soilRhoCp,
        outDia,
        thickness,
        kPipe,
        pipeRhoCp,
        massFlow,
        inletTemp,
        maxSimYears,
    );
    let Some((x, y)) = f.combined() else {
        return f64::NAN;
    };
    let bh_radius = bhDiameter / 2.0;
    let g_ref = if gRefRatio > 0.0 {
        gRefRatio
    } else {
        bh_radius / bhLength
    };
    let mut g = interp(&x, &y, time.ln());
    if bh_radius / bhLength != g_ref {
        g -= (bh_radius / (bhLength * g_ref)).ln();
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interp_extrapolates_past_both_ends() {
        let (x, y) = ([0.0, 1.0, 2.0], [0.0, 10.0, 30.0]);
        assert_eq!(interp(&x, &y, -1.0), -10.0);
        assert_eq!(interp(&x, &y, 0.5), 5.0);
        assert_eq!(interp(&x, &y, 1.0), 10.0);
        assert_eq!(interp(&x, &y, 3.0), 50.0);
    }

    #[test]
    fn fields_past_two_by_two_are_outside_the_domain() {
        let g = VGCombinedLen(
            3, 1, 6.0, 60.0, 1.0, 0.114, 0.05, 1.5, 3.9e6, 2.0, 2.0e6, 0.0334, 0.003, 0.39, 1.8e6,
            0.5, 20.0, 1.0,
        );
        assert_eq!(g, 0);
    }
}
