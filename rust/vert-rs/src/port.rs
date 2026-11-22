//! The Rust port of `vert.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The vertical ground heat exchanger's long-time-step g-functions from EnergyPlus
//! `GroundHeatExchangers/Vertical.cc` (`GLHEVert`): point distances and the finite line source
//! response, the Simpson integrals over borehole points (placed as
//! `GLHEResponseFactors::SetupBHPointsForResponseFactorsObject` places them), the time vector and
//! the uniform heat flux g-functions, plus the friction factor, the pipe conduction resistance and
//! the annual time constant. `erfc` comes from the `libm` crate (ADR-006). Nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

const PI: f64 = std::f64::consts::PI; // `Constant::Pi`
const SECS_IN_HOUR: f64 = 3600.0; // `Constant::rSecsInHour`
const HOURS_IN_DAY: f64 = 24.0; // `Constant::rHoursInDay`

/// The Simpson panels per borehole (`numPanels_i`, `numPanels_ii`, `numPanels_j`).
const PANELS_I: i32 = 50;
const PANELS_II: i32 = 50;
const PANELS_J: i32 = 560;

/// A point (`MyCartesian`).
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// The borehole properties every borehole of a field shares (`GLHEVertProps`).
#[derive(Clone, Copy, Debug)]
pub struct Props {
    pub bh_length: f64,
    pub bh_top_depth: f64,
    pub bh_diameter: f64,
}

/// Port of `GLHEVert::distances` (`Vertical.cc:541`): to the point and to its mirror above
/// ground.
pub fn distances(pi: Point, pj: Point) -> [f64; 2] {
    let dx2 = (pi.x - pj.x) * (pi.x - pj.x);
    let dy2 = (pi.y - pj.y) * (pi.y - pj.y);
    let dz = pi.z - pj.z;
    let dzm = pi.z - (-pj.z);
    [
        (0.0 + dx2 + dy2 + dz * dz).sqrt(),
        (0.0 + dx2 + dy2 + dzm * dzm).sqrt(),
    ]
}

/// Port of `GLHEVert::calcResponse` (`Vertical.cc:566`).
pub fn calc_response(dists: [f64; 2], diffusivity: f64, curr_time: f64) -> f64 {
    let at = |d: f64| libm::erfc(d / (2.0 * (diffusivity * curr_time).sqrt())) / d;
    at(dists[0]) - at(dists[1])
}

/// Simpson's rule over `n + 1` values `f(0..=n)`, as the C++ accumulates it.
fn simpson(n: i32, dl: f64, mut f: impl FnMut(i32) -> f64) -> f64 {
    let mut sum = 0.0;
    for i in 0..=n {
        let v = f(i);
        if i == 0 || i == n {
            sum += v;
        } else if i % 2 == 0 {
            sum += 2.0 * v;
        } else {
            sum += 4.0 * v;
        }
    }
    (dl / 3.0) * sum
}

/// Port of `GLHEVert::integral` (`Vertical.cc:573`): the response at `pi` to borehole `(xj, yj)`.
pub fn integral(
    pi: Point,
    xj: f64,
    yj: f64,
    props: Props,
    diffusivity: f64,
    curr_time: f64,
) -> f64 {
    let dl = props.bh_length / f64::from(PANELS_J);
    simpson(PANELS_J, dl, |i| {
        let pj = Point {
            x: xj,
            y: yj,
            z: props.bh_top_depth + (f64::from(i) * dl),
        };
        calc_response(distances(pi, pj), diffusivity, curr_time)
    })
}

/// Port of `GLHEVert::doubleIntegral` (`Vertical.cc:604`). For a borehole on itself (`same`), the
/// points sit one radius off its axis.
pub fn double_integral(
    bh_i: (f64, f64),
    bh_j: (f64, f64),
    same: bool,
    props: Props,
    diffusivity: f64,
    curr_time: f64,
) -> f64 {
    let (x, y, n) = if same {
        let r = props.bh_diameter / 2.0;
        (
            bh_i.0 + r / 2f64.sqrt(),
            bh_i.1 + r / (-2f64.sqrt()),
            PANELS_II,
        )
    } else {
        (bh_i.0, bh_i.1, PANELS_I)
    };
    let dl = props.bh_length / f64::from(n);
    simpson(n, dl, |i| {
        let pi = Point {
            x,
            y,
            z: props.bh_top_depth + (f64::from(i) * dl),
        };
        integral(pi, bh_j.0, bh_j.1, props, diffusivity, curr_time)
    })
}

/// The g-function time scale `t_s`.
fn time_scale(bh_length: f64, diffusivity: f64) -> f64 {
    (bh_length * bh_length) / (9.0 * diffusivity)
}

/// Port of `GLHEVert::setupTimeVectors` (`Vertical.cc:993`): the LNTTS values from -8.5 in steps
/// of 0.5 until the time reaches `max_sim_years`. `None` past 1000 values (the C++ would not stop
/// for a zero or negative time scale; the oracle's domain, ADR-009). Every value is a multiple of
/// 0.5, so `-8.5 + 0.5 k` is exactly the C++'s running sum.
pub fn lntts(bh_length: f64, diffusivity: f64, max_sim_years: f64) -> Option<Vec<f64>> {
    let n = lntts_len(bh_length, diffusivity, max_sim_years)?;
    Some((0..n).map(lntts_value).collect())
}

fn lntts_value(k: i32) -> f64 {
    -8.5 + 0.5 * f64::from(k)
}

/// The number of LNTTS values, counted as `setupTimeVectors` counts them.
fn lntts_len(bh_length: f64, diffusivity: f64, max_sim_years: f64) -> Option<i32> {
    let t_s = time_scale(bh_length, diffusivity);
    let end = max_sim_years * 365.0 * HOURS_IN_DAY * SECS_IN_HOUR;
    let mut n = 1;
    while lntts_value(n - 1).exp() * t_s < end {
        if n >= 1000 {
            return None;
        }
        n += 1;
    }
    Some(n)
}

/// Port of `GLHEVert::calcUniformHeatFluxGFunctions` (`Vertical.cc:1026`) for one time: the
/// g-function of the boreholes at `bhs`.
pub fn uniform_heat_flux_gfunction(
    bhs: &[(f64, f64)],
    props: Props,
    diffusivity: f64,
    time: f64,
) -> f64 {
    let mut g = 0.0;
    for (i, &bh_i) in bhs.iter().enumerate() {
        let mut sum_t_ji = 0.0;
        for (j, &bh_j) in bhs.iter().enumerate() {
            sum_t_ji += double_integral(bh_i, bh_j, i == j, props, diffusivity, time);
        }
        g += sum_t_ji;
    }
    let total_tube_length = bhs.len() as f64 * props.bh_length;
    g / (2.0 * total_tube_length)
}

/// Port of `GLHEVert::frictionFactor` (`Vertical.cc:1430`): laminar, turbulent (Petukhov) and a
/// smooth blend between Reynolds numbers 1500 and 5000.
pub fn friction_factor(reynolds_num: f64) -> f64 {
    let turbulent = || (0.79 * reynolds_num.ln() - 1.64).powf(-2.0);
    if reynolds_num < 1500.0 {
        return 64.0 / reynolds_num;
    }
    if reynolds_num < 5000.0 {
        let f_low = 64.0 / reynolds_num;
        let sf = 1.0 / (1.0 + (-(reynolds_num - 3000.0) / 450.0).exp());
        return (1.0 - sf) * f_low + sf * turbulent();
    }
    turbulent()
}

// ---- the exports

pub fn VertDistance(xi: f64, yi: f64, zi: f64, xj: f64, yj: f64, zj: f64, which: i32) -> f64 {
    let d = distances(
        Point {
            x: xi,
            y: yi,
            z: zi,
        },
        Point {
            x: xj,
            y: yj,
            z: zj,
        },
    );
    usize::try_from(which)
        .ok()
        .and_then(|w| d.get(w).copied())
        .unwrap_or(f64::NAN)
}

pub fn VertResponse(d0: f64, d1: f64, diffusivity: f64, currTime: f64) -> f64 {
    calc_response([d0, d1], diffusivity, currTime)
}

pub fn VertIntegral(
    xi: f64,
    yi: f64,
    zi: f64,
    xj: f64,
    yj: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    currTime: f64,
) -> f64 {
    let props = Props {
        bh_length: bhLength,
        bh_top_depth: bhTopDepth,
        bh_diameter: bhDiameter,
    };
    integral(
        Point {
            x: xi,
            y: yi,
            z: zi,
        },
        xj,
        yj,
        props,
        diffusivity,
        currTime,
    )
}

pub fn VertDoubleIntegral(
    xi: f64,
    yi: f64,
    xj: f64,
    yj: f64,
    same: i32,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    currTime: f64,
) -> f64 {
    let props = Props {
        bh_length: bhLength,
        bh_top_depth: bhTopDepth,
        bh_diameter: bhDiameter,
    };
    let same = same != 0;
    let bh_j = if same { (xi, yi) } else { (xj, yj) };
    double_integral((xi, yi), bh_j, same, props, diffusivity, currTime)
}

pub fn VertNumLntts(bhLength: f64, diffusivity: f64, maxSimYears: f64) -> i32 {
    lntts_len(bhLength, diffusivity, maxSimYears).unwrap_or(0)
}

/// The `k`-th LNTTS value, `None` outside the vector.
fn lntts_at(bhLength: f64, diffusivity: f64, maxSimYears: f64, k: i32) -> Option<f64> {
    let n = lntts_len(bhLength, diffusivity, maxSimYears)?;
    (0..n).contains(&k).then(|| lntts_value(k))
}

pub fn VertLntts(bhLength: f64, diffusivity: f64, maxSimYears: f64, k: i32) -> f64 {
    lntts_at(bhLength, diffusivity, maxSimYears, k).unwrap_or(f64::NAN)
}

pub fn VertGFunction(
    nx: i32,
    ny: i32,
    spacing: f64,
    bhLength: f64,
    bhTopDepth: f64,
    bhDiameter: f64,
    diffusivity: f64,
    maxSimYears: f64,
    k: i32,
) -> f64 {
    if !(1..=3).contains(&nx) || !(1..=3).contains(&ny) {
        return f64::NAN;
    }
    let Some(l) = lntts_at(bhLength, diffusivity, maxSimYears, k) else {
        return f64::NAN;
    };
    let bhs: Vec<(f64, f64)> = (0..ny)
        .flat_map(|iy| (0..nx).map(move |ix| (f64::from(ix) * spacing, f64::from(iy) * spacing)))
        .collect();
    let props = Props {
        bh_length: bhLength,
        bh_top_depth: bhTopDepth,
        bh_diameter: bhDiameter,
    };
    let time = l.exp() * time_scale(bhLength, diffusivity);
    uniform_heat_flux_gfunction(&bhs, props, diffusivity, time)
}

pub fn VertFrictionFactor(reynoldsNum: f64) -> f64 {
    friction_factor(reynoldsNum)
}

/// Port of `GLHEVert::calcPipeConductionResistance` (`Vertical.cc:1377`).
pub fn VertPipeConductionResistance(outDia: f64, innerDia: f64, k: f64) -> f64 {
    (outDia / innerDia).ln() / (2.0 * PI * k)
}

/// Port of `GLHEVert::getAnnualTimeConstant` (`Vertical.cc:491`): `timeSS` [years].
pub fn VertTimeSS(bhLength: f64, diffusivity: f64) -> f64 {
    const HR_IN_YEAR: f64 = 8760.0;
    (bhLength * bhLength / (9.0 * diffusivity)) / SECS_IN_HOUR / HR_IN_YEAR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        assert_eq!(VertDistance(0.0, 0.0, 1.0, 3.0, 4.0, 1.0, 0), 5.0);
        assert!(VertDistance(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2).is_nan());
        assert_eq!(VertNumLntts(0.0, 1e-6, 1.0), 0);
        assert_eq!(VertLntts(100.0, 1e-6, 10.0, 0), -8.5);
        assert_eq!(friction_factor(1000.0), 0.064);
        assert!(VertGFunction(4, 1, 6.0, 100.0, 1.0, 0.15, 1e-6, 10.0, 0).is_nan());
    }
}
