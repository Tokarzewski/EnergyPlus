//! The Rust port of `solveroot.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `General::SolveRoot` (`General.cc:155`) with all six `RootAlgo` methods, and the
//! algorithm-picking wrapper `General::SolveRoot2` (`General.cc:322`). `solve_root` and
//! `solve_root2` take any closure, as the C++ takes a `std::function`. The exports run them on
//! the oracle's test functions (`test_fn`). Nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports take the C signatures: an algorithm, a test function and its bracket"
)]

/// `RootAlgo` (`HVACSystemRootFindingAlgorithm.hh`); any other value takes the C++'s `default`
/// branch, regula falsi.
pub mod algo {
    pub const INVALID: i32 = -1;
    pub const REGULA_FALSI: i32 = 0;
    pub const BISECTION: i32 = 1;
    pub const REGULA_FALSI_THEN_BISECTION: i32 = 2;
    pub const BISECTION_THEN_REGULA_FALSI: i32 = 3;
    pub const ALTERNATION: i32 = 4;
    pub const SHORT_BISECTION_THEN_REGULA_FALSI: i32 = 5;
    pub const NUM: i32 = 6;
}

/// `SOLVEROOT_ERROR_INIT` and `SOLVEROOT_ERROR_ITER` (`General.hh`).
pub const SOLVEROOT_ERROR_INIT: i32 = -2;
pub const SOLVEROOT_ERROR_ITER: i32 = -1;

/// `RootFindingData::NumOfIter`'s default, which `SolveRoot2` runs under.
const DEFAULT_NUM_OF_ITER: i32 = 5;

/// Port of `General::SolveRoot`: a root of `f` between `x_0` and `x_1`, as `(Flag, XRes)`.
/// `Flag` is the number of iterations, `SOLVEROOT_ERROR_INIT` when `f` has the same sign at both
/// bounds, or `SOLVEROOT_ERROR_ITER` without convergence. `root_algo` and `num_of_iter` are the
/// `state.dataRootFinder` settings the C++ reads.
pub fn solve_root(
    root_algo: i32,
    num_of_iter: i32,
    eps: f64,
    max_ite: i32,
    mut f: impl FnMut(f64) -> f64,
    x_0: f64,
    x_1: f64,
) -> (i32, f64) {
    const SMALL: f64 = 1e-10;
    let (mut x0, mut x1) = (x_0, x_1);
    let mut x_temp = x0;
    let (mut n_ite, mut alt_ite) = (0i32, 0i32);
    let mut y0 = f(x0);
    let mut y1 = f(x1);
    if y0 * y1 > 0.0 {
        return (SOLVEROOT_ERROR_INIT, x0);
    }
    loop {
        let mut dy = y0 - y1;
        if dy.abs() < SMALL {
            dy = SMALL;
        }
        if (x1 - x0).abs() < SMALL {
            break;
        }
        let falsi = (y0 * x1 - y1 * x0) / dy;
        let halve = (x1 + x0) / 2.0;
        x_temp = match root_algo {
            algo::BISECTION => halve,
            algo::REGULA_FALSI_THEN_BISECTION if n_ite > num_of_iter => halve,
            algo::BISECTION_THEN_REGULA_FALSI if n_ite <= num_of_iter => halve,
            algo::ALTERNATION if alt_ite > num_of_iter => {
                if alt_ite >= num_of_iter.wrapping_mul(2) {
                    alt_ite = 0;
                }
                halve
            }
            algo::SHORT_BISECTION_THEN_REGULA_FALSI if n_ite < 3 => halve,
            _ => falsi,
        };
        let y_temp = f(x_temp);
        n_ite = n_ite.wrapping_add(1);
        alt_ite = alt_ite.wrapping_add(1);
        if y_temp.abs() < eps {
            return (n_ite, x_temp);
        }
        if n_ite > max_ite {
            break;
        }
        // The bound whose value has the sign of `y_temp` moves to `x_temp`.
        if (y0 < 0.0) == (y_temp < 0.0) {
            x0 = x_temp;
            y0 = y_temp;
        } else {
            x1 = x_temp;
            y1 = y_temp;
        }
    }
    (SOLVEROOT_ERROR_ITER, x_temp)
}

/// Port of `General::SolveRootStats`.
#[derive(Clone, Debug)]
pub struct SolveRootStats {
    pub algo: i32,
    pub counts: i32,
    pub algo_counts: [i32; algo::NUM as usize],
    pub algo_iters: [i32; algo::NUM as usize],
}

impl Default for SolveRootStats {
    fn default() -> Self {
        Self {
            algo: algo::REGULA_FALSI,
            counts: 0,
            algo_counts: [0; 6],
            algo_iters: [0; 6],
        }
    }
}

/// Port of `General::SolveRoot2`: `solve_root` under `stats.algo`, as `(SolFla, XRes)`. Each
/// converged call counts towards its algorithm; the first 30 cycle through the six, and the 30th
/// picks the one with the fewest iterations (`INVALID` if none beats `5 * max_iters`).
pub fn solve_root2(
    eps: f64,
    max_iters: i32,
    f: impl FnMut(f64) -> f64,
    x_0: f64,
    x_1: f64,
    stats: &mut SolveRootStats,
) -> (i32, f64) {
    const TRIALS_PER_COUNT: i32 = 5;
    let (flag, x_res) = solve_root(stats.algo, DEFAULT_NUM_OF_ITER, eps, max_iters, f, x_0, x_1);
    if flag > 0 {
        stats.counts = stats.counts.wrapping_add(1);
        if let Ok(i) = usize::try_from(stats.algo)
            && let (Some(n), Some(it)) = (stats.algo_counts.get_mut(i), stats.algo_iters.get_mut(i))
        {
            *n = n.wrapping_add(1);
            *it = it.wrapping_add(flag);
        }
        if stats.counts < TRIALS_PER_COUNT * algo::NUM {
            stats.algo += 1;
            if stats.algo == algo::NUM {
                stats.algo = algo::REGULA_FALSI;
            }
        } else if stats.counts == TRIALS_PER_COUNT * algo::NUM {
            let mut min_iters = max_iters.wrapping_mul(TRIALS_PER_COUNT);
            stats.algo = algo::INVALID;
            for (i, &iters) in (0..).zip(stats.algo_iters.iter()) {
                if iters < min_iters {
                    stats.algo = i;
                    min_iters = iters;
                }
            }
        }
    }
    (flag, x_res)
}

/// The oracle's test functions (`fx` in `shim/ep_solveroot_shim.cc`).
pub fn test_fn(func: i32, a: f64, b: f64, c: f64, x: f64) -> f64 {
    match func {
        0 => a * x * x * x + b * x + c,
        1 => (a * x).exp() - b + c * x,
        2 => (a * x).sin() + b * x + c,
        _ => a,
    }
}

/// The oracle's domain: beyond it the C++'s counters overflow or it runs too long (ADR-009).
fn defined(num_of_iter: i32, max_ite: i32) -> bool {
    max_ite <= 1000 && (-1000..=1000).contains(&num_of_iter)
}

fn defined2(start: i32, calls: i32, max_iters: i32) -> bool {
    (0..algo::NUM).contains(&start)
        && (0..=100).contains(&calls)
        && (-1000..=1000).contains(&max_iters)
}

/// `calls` `solve_root2` calls from fresh stats starting at `start`, the k-th with `c + 0.01 k`,
/// stopping once the stats pick no algorithm; the final algorithm and the last root.
fn run2(
    start: i32,
    calls: i32,
    eps: f64,
    max_iters: i32,
    func: i32,
    a: f64,
    b: f64,
    c: f64,
    x_0: f64,
    x_1: f64,
) -> (i32, f64) {
    let mut stats = SolveRootStats {
        algo: start,
        ..SolveRootStats::default()
    };
    let mut x = 0.0;
    for k in 0..calls {
        if stats.algo == algo::INVALID {
            break;
        }
        let ck = c + 0.01 * f64::from(k);
        x = solve_root2(
            eps,
            max_iters,
            |x| test_fn(func, a, b, ck, x),
            x_0,
            x_1,
            &mut stats,
        )
        .1;
    }
    (stats.algo, x)
}

/// `SolveRoot`'s `XRes` for root algorithm `algo` on test function `func`.
pub fn SolveRootX(
    algo: i32,
    numOfIter: i32,
    Eps: f64,
    MaxIte: i32,
    func: i32,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> f64 {
    if !defined(numOfIter, MaxIte) {
        return f64::NAN;
    }
    solve_root(
        algo,
        numOfIter,
        Eps,
        MaxIte,
        |x| test_fn(func, a, b, c, x),
        X_0,
        X_1,
    )
    .1
}

/// `SolveRoot`'s `Flag` for root algorithm `algo` on test function `func`.
pub fn SolveRootFlag(
    algo: i32,
    numOfIter: i32,
    Eps: f64,
    MaxIte: i32,
    func: i32,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> i32 {
    if !defined(numOfIter, MaxIte) {
        return 0;
    }
    solve_root(
        algo,
        numOfIter,
        Eps,
        MaxIte,
        |x| test_fn(func, a, b, c, x),
        X_0,
        X_1,
    )
    .0
}

/// The algorithm `SolveRoot2`'s stats hold after `calls` calls.
pub fn SolveRoot2Algo(
    start: i32,
    calls: i32,
    Eps: f64,
    maxIters: i32,
    func: i32,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> i32 {
    if !defined2(start, calls, maxIters) {
        return 0;
    }
    run2(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1).0
}

/// The root of `SolveRoot2`'s last call.
pub fn SolveRoot2X(
    start: i32,
    calls: i32,
    Eps: f64,
    maxIters: i32,
    func: i32,
    a: f64,
    b: f64,
    c: f64,
    X_0: f64,
    X_1: f64,
) -> f64 {
    if !defined2(start, calls, maxIters) {
        return f64::NAN;
    }
    run2(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roots() {
        let (flag, x) = solve_root(algo::BISECTION, 5, 1e-8, 200, |x| x * x - 2.0, 0.0, 2.0);
        assert!(flag > 0 && (x - std::f64::consts::SQRT_2).abs() < 1e-8);
        assert_eq!(
            solve_root(algo::REGULA_FALSI, 5, 1e-6, 10, |x| x * x + 1.0, 0.0, 1.0),
            (SOLVEROOT_ERROR_INIT, 0.0)
        );
        assert_eq!(
            solve_root(algo::REGULA_FALSI, 5, 0.0, 0, |x| x, -1.0, 2.0).0,
            SOLVEROOT_ERROR_ITER
        );
    }

    #[test]
    fn stats_settle_after_thirty() {
        let mut stats = SolveRootStats::default();
        for _ in 0..30 {
            solve_root2(1e-9, 100, |x| x - 0.3, 0.0, 1.0, &mut stats);
        }
        assert_eq!(stats.counts, 30);
        assert!((0..algo::NUM).contains(&stats.algo));
    }
}
