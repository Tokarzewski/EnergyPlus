//! The Rust port of `groundtemp.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The undisturbed ground temperature models from EnergyPlus `GroundTemperatureModeling/`: the
//! Kusuda-Achenbach and Xing correlations, and the monthly site tables (`Site:GroundTemperature:
//! Deep`, `:Shallow`, `:BuildingSurface`, `:FCfactorMethod`, which share their code), each at a
//! depth and a time in seconds or months. `remainder` (IEEE, exact) comes from the `libm` crate.
//! Nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

const PI: f64 = std::f64::consts::PI; // `Constant::Pi`
const SECS_IN_DAY: f64 = 86400.0; // `Constant::rSecsInDay`

/// `KusudaGroundTempsModel::getGroundTemp` (`KusudaAchenbachGroundTemperatureModel.cc:185`).
fn kusuda(depth: f64, t: f64, ndy: i32, ka: [f64; 4]) -> f64 {
    let [ave, amp, phase, diff] = ka;
    let secs_in_year = SECS_IN_DAY * f64::from(ndy);
    let term1 = -depth * (PI / (secs_in_year * diff)).sqrt();
    let term2 = (2.0 * PI / secs_in_year)
        * (t - phase - (depth / 2.0) * (secs_in_year / (PI * diff)).sqrt());
    ave - amp * term1.exp() * term2.cos()
}

/// Folds a time past the year back into it, as both correlations do.
fn fold(t: f64, year: f64) -> f64 {
    if t > year {
        libm::remainder(t, year)
    } else {
        t
    }
}

/// `XingGroundTempsModel::getGroundTemp` (`XingGroundTemperatureModel.cc:128`).
fn xing(depth: f64, days: f64, ndy: i32, x: [f64; 6]) -> f64 {
    let [ave, ts_1, pl_1, ts_2, pl_2, diff] = x;
    let tp = f64::from(ndy);
    let term = |n: f64, ts: f64, pl: f64| {
        let gamma = ((n * PI) / (diff * tp)).sqrt();
        let cos = 2.0 * PI * n / tp * (days - pl) - depth * gamma;
        (-depth * gamma).exp() * ts * cos.cos()
    };
    ave - (term(1.0, ts_1, pl_1) + term(2.0, ts_2, pl_2))
}

/// The table index a site model reads for `month`, after its `month % 12` fold.
fn site_index(month: i32) -> Option<i32> {
    let m = if (1..=12).contains(&month) {
        month
    } else {
        month % 12
    };
    (1..=12).contains(&m).then_some(m - 1)
}

/// A month number as the C++ converts a double to `int`, or `None` where that is undefined.
fn to_int(m: f64) -> Option<i32> {
    (m >= f64::from(i32::MIN) && m <= f64::from(i32::MAX)).then_some(m as i32)
}

pub fn GTKusudaSeconds(
    depth: f64,
    seconds: f64,
    numDaysInYear: i32,
    ave: f64,
    amp: f64,
    phaseSecs: f64,
    diffusivity: f64,
) -> f64 {
    let t = fold(seconds, f64::from(numDaysInYear) * SECS_IN_DAY);
    kusuda(depth, t, numDaysInYear, [ave, amp, phaseSecs, diffusivity])
}

pub fn GTKusudaMonth(
    depth: f64,
    month: i32,
    numDaysInYear: i32,
    ave: f64,
    amp: f64,
    phaseSecs: f64,
    diffusivity: f64,
) -> f64 {
    let Some(m1) = month.checked_sub(1) else {
        return f64::NAN;
    };
    let ave_secs_in_month = f64::from(numDaysInYear / 12) * SECS_IN_DAY;
    let t = fold(
        ave_secs_in_month * (f64::from(m1) + 0.5),
        f64::from(numDaysInYear) * SECS_IN_DAY,
    );
    kusuda(depth, t, numDaysInYear, [ave, amp, phaseSecs, diffusivity])
}

pub fn GTXingSeconds(
    depth: f64,
    seconds: f64,
    numDaysInYear: i32,
    ave: f64,
    amp1: f64,
    phase1: f64,
    amp2: f64,
    phase2: f64,
    diffusivity: f64,
) -> f64 {
    let days = fold(seconds / SECS_IN_DAY, f64::from(numDaysInYear));
    xing(
        depth,
        days,
        numDaysInYear,
        [ave, amp1, phase1, amp2, phase2, diffusivity],
    )
}

pub fn GTXingMonth(
    depth: f64,
    month: i32,
    numDaysInYear: i32,
    ave: f64,
    amp1: f64,
    phase1: f64,
    amp2: f64,
    phase2: f64,
    diffusivity: f64,
) -> f64 {
    let ave_days_in_month = f64::from(numDaysInYear / 12); // integer division, as in the C++
    let m = if (1..=12).contains(&month) {
        month
    } else {
        month % 12
    };
    let days = ave_days_in_month * (f64::from(m - 1) + 0.5);
    xing(
        depth,
        days,
        numDaysInYear,
        [ave, amp1, phase1, amp2, phase2, diffusivity],
    )
}

/// The site tables' `getGroundTempAtTimeInMonths` and `getGroundTempAtTimeInSeconds`
/// (`SiteDeepGroundTemperatures.cc:161` and the same code in the other three) on the table
/// `t0 + dt i`; NaN where the C++ indexes outside the table (ADR-009).
pub fn GTSite(kind: i32, byMonth: i32, when: f64, numDaysInYear: i32, t0: f64, dt: f64) -> f64 {
    if !(0..=3).contains(&kind) {
        return f64::NAN;
    }
    let month = if byMonth != 0 {
        to_int(when).filter(|&m| f64::from(m) == when)
    } else {
        let sec_per_month = f64::from(numDaysInYear) * SECS_IN_DAY / 12.0;
        Some(to_int((when / sec_per_month).ceil()).unwrap_or(0))
    };
    match month.and_then(site_index) {
        Some(i) => t0 + dt * f64::from(i),
        None => f64::NAN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_months_fold_into_the_year() {
        assert_eq!(site_index(1), Some(0));
        assert_eq!(site_index(13), Some(0));
        assert_eq!(site_index(12), Some(11));
        assert_eq!(site_index(24), None);
        assert_eq!(site_index(0), None);
        assert_eq!(site_index(-1), None);
    }

    #[test]
    fn kusuda_at_the_surface_follows_the_annual_wave() {
        // At depth 0 and the phase shift the cosine is 1: the minimum, ave - amp.
        assert_eq!(
            GTKusudaSeconds(0.0, 1.0e6, 365, 12.0, 5.0, 1.0e6, 1e-6),
            7.0
        );
    }
}
