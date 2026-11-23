//! The Rust port of `thermalhist.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatBalanceSurfaceManager::UpdateThermalHistories` for one opaque surface: the current inside
//! and outside conduction fluxes (and, with a source or sink, the source and user location
//! temperatures) from the first CTF terms, then the history shift. Simple CTF models shift every
//! history by one term; otherwise the master histories move on at the end of each construction
//! time step and the surface's histories are interpolated between them, the source histories
//! likewise. Each expression keeps the C++ order of terms. The history values come from the same
//! splitmix64 draw as the oracle's shim; its guards return NaN here too.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]

/// History terms kept per array, as the oracle dimensions them.
const TERMS: usize = 8;

/// Indexes into `State::hist`: the surface histories, then their masters (`+ MASTER`).
const IN_T: usize = 0;
const OUT_T: usize = 1;
const IN_F: usize = 2;
const OUT_F: usize = 3;
const MASTER: usize = 4;
/// Indexes into `State::srcs`: the source, source flux and user location histories, then their
/// masters (`+ SRC_MASTER`).
const TSRC: usize = 0;
const QSRC: usize = 1;
const TUSER: usize = 2;
const SRC_MASTER: usize = 3;

/// splitmix64, as the oracle's shim draws the history values.
struct SplitMix(u64);

impl SplitMix {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        (z >> 11) as f64 * (1.0 / 9_007_199_254_740_992.0) * 200.0 - 100.0
    }
}

/// One surface's histories and what the update writes. Terms are 0-based here (term 1 is `[0]`).
struct State {
    /// `SurfInsideTempHist`, `SurfOutsideTempHist`, `SurfInsideFluxHist`, `SurfOutsideFluxHist`
    /// and their masters.
    hist: [[f64; TERMS]; 8],
    /// `SurfTsrcHist`, `SurfQsrcHist`, `SurfTuserHist` and their masters.
    srcs: [[f64; TERMS]; 6],
    /// `TempExt1`, `TempInt1`, `QExt1`, `QInt1`, `Tsrc1`, `Tuser1`, `Qsrc1`.
    ones: [f64; 7],
    /// `SurfOpaqInsFaceCond`, `SurfOpaqInsFaceCondFlux`, `SurfOpaqOutFaceCond`,
    /// `SurfOpaqOutFaceCondFlux`, `SurfTempSource`, `SurfTempUserLoc`.
    outs: [f64; 6],
    curr_num_hist: i32,
    sum_time: f64,
}

/// The model and construction as `UpdateThermalHistories` reads them.
struct Model {
    ctf: bool,
    n: usize,
    max_ctf_terms: usize,
    simple: bool,
    overrides: bool,
    any_src: bool,
    src: bool,
    num_histories: i32,
    time_step_zone: f64,
    ctf_time_step: f64,
    area: f64,
    temp_in: f64,
    const_in: f64,
    const_out: f64,
    tsrc_const: f64,
    tuser_const: f64,
    cross0: f64,
    inside0: f64,
    outside0: f64,
    src_in0: f64,
    src_out0: f64,
    tsrc_out0: f64,
    tsrc_in0: f64,
    tsrc_q0: f64,
    tuser_out0: f64,
    tuser_in0: f64,
    tuser_src0: f64,
}

/// `TH(k) = THM(k) - (THM(k) - prev) * sum_steps`, the interpolation between master terms.
fn interp(master: f64, prev: f64, sum_steps: f64) -> f64 {
    master - (master - prev) * sum_steps
}

fn update_thermal_histories(s: &mut State, c: &Model) {
    // Current fluxes (and source temperatures) from the first CTF terms.
    if c.ctf && c.n != 0 {
        let tout = s.hist[OUT_T][0];
        let qsrc = s.srcs[QSRC][0];
        let mut inside_flux = tout * c.cross0 - c.temp_in * c.inside0 + c.const_in;
        if c.src {
            inside_flux += qsrc * c.src_in0;
        }
        s.outs[0] = c.area * inside_flux;
        s.outs[1] = inside_flux;
        s.hist[IN_F][0] = inside_flux;
        if c.src {
            let tsrc =
                tout * c.tsrc_out0 + c.temp_in * c.tsrc_in0 + qsrc * c.tsrc_q0 + c.tsrc_const;
            s.srcs[TSRC][0] = tsrc;
            s.outs[4] = tsrc;
            let tuser =
                tout * c.tuser_out0 + c.temp_in * c.tuser_in0 + qsrc * c.tuser_src0 + c.tuser_const;
            s.srcs[TUSER][0] = tuser;
            s.outs[5] = tuser;
            s.hist[OUT_F][0] =
                tout * c.outside0 - c.temp_in * c.cross0 + qsrc * c.src_out0 + c.const_out;
        } else {
            s.hist[OUT_F][0] = tout * c.outside0 - c.temp_in * c.cross0 + c.const_out;
        }
        s.outs[3] = -s.hist[OUT_F][0];
        s.outs[2] = c.area * s.outs[3];
    }

    if c.simple && !c.overrides {
        // The C++ moves the term arrays along; by value every term takes the one before it.
        for h in &mut s.hist[..MASTER] {
            for k in (2..=c.max_ctf_terms).rev() {
                h[k] = h[k - 1];
            }
            h[1] = h[0];
        }
        return;
    }

    if c.ctf && s.curr_num_hist == 0 {
        s.ones[0] = s.hist[OUT_T][0];
        s.ones[1] = s.hist[IN_T][0];
        s.ones[2] = s.hist[OUT_F][0];
        s.ones[3] = s.hist[IN_F][0];
    }
    if c.any_src && c.ctf && s.curr_num_hist == 0 {
        s.ones[4] = s.srcs[TSRC][0];
        s.ones[5] = s.srcs[TUSER][0];
        s.ones[6] = s.srcs[QSRC][0];
    }

    // The first values of the series, by history: TempExt1, TempInt1, QExt1, QInt1.
    let first = [(OUT_T, 0), (IN_T, 1), (OUT_F, 2), (IN_F, 3)];
    if c.ctf {
        s.curr_num_hist += 1;
        s.sum_time = f64::from(s.curr_num_hist) * c.time_step_zone;
        if s.curr_num_hist == c.num_histories {
            s.curr_num_hist = 0;
            if c.n > 1 {
                for k in (2..=c.n).rev() {
                    for h in [IN_T, OUT_T, IN_F, OUT_F] {
                        s.hist[h + MASTER][k] = s.hist[h + MASTER][k - 1];
                    }
                    for h in [OUT_T, IN_T, OUT_F, IN_F] {
                        s.hist[h][k] = s.hist[h + MASTER][k - 1];
                    }
                }
            }
            for (h, one) in first {
                s.hist[h + MASTER][1] = s.ones[one];
            }
            for (h, _) in first {
                s.hist[h][1] = s.hist[h + MASTER][1];
            }
        } else {
            let sum_steps = s.sum_time / c.ctf_time_step;
            if c.n > 1 {
                for k in (2..=c.n).rev() {
                    for h in [OUT_T, IN_T, OUT_F, IN_F] {
                        s.hist[h][k] =
                            interp(s.hist[h + MASTER][k], s.hist[h + MASTER][k - 1], sum_steps);
                    }
                }
            }
            for (h, one) in first {
                s.hist[h][1] = interp(s.hist[h + MASTER][1], s.ones[one], sum_steps);
            }
        }
    }

    // The source, source flux and user location histories, with Tsrc1, Qsrc1 and Tuser1.
    if c.any_src && c.src && c.ctf {
        let src_first = [(TSRC, 4), (QSRC, 6), (TUSER, 5)];
        if s.curr_num_hist == 0 {
            if c.n > 1 {
                for k in (2..=c.n).rev() {
                    for (h, _) in src_first {
                        let v = s.srcs[h + SRC_MASTER][k - 1];
                        s.srcs[h + SRC_MASTER][k] = v;
                        s.srcs[h][k] = v;
                    }
                }
            }
            for (h, one) in [(TSRC, 4), (TUSER, 5), (QSRC, 6)] {
                s.srcs[h + SRC_MASTER][1] = s.ones[one];
            }
            for (h, _) in [(TSRC, 4), (TUSER, 5), (QSRC, 6)] {
                s.srcs[h][1] = s.srcs[h + SRC_MASTER][1];
            }
        } else {
            let sum_steps = s.sum_time / c.ctf_time_step;
            if c.n > 1 {
                for k in (2..=c.n).rev() {
                    for (h, _) in src_first {
                        s.srcs[h][k] = interp(
                            s.srcs[h + SRC_MASTER][k],
                            s.srcs[h + SRC_MASTER][k - 1],
                            sum_steps,
                        );
                    }
                }
            }
            for (h, one) in src_first {
                s.srcs[h][1] = interp(s.srcs[h + SRC_MASTER][1], s.ones[one], sum_steps);
            }
        }
    }
}

/// One value after `UpdateThermalHistories` (see the oracle's shim for the arguments and `which`).
pub fn ThermalHist(
    which: i32,
    seed: i32,
    n: i32,
    m: i32,
    simple: i32,
    overrides: i32,
    anysrc: i32,
    src: i32,
    algo: i32,
    curnum: i32,
    numhist: i32,
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
    let flag = |v: i32| (0..=1).contains(&v);
    let (Ok(which), Ok(n), Ok(m)) = (
        usize::try_from(which),
        usize::try_from(n),
        usize::try_from(m),
    ) else {
        return f64::NAN;
    };
    if which > 127
        || !(flag(simple) && flag(overrides) && flag(anysrc) && flag(src))
        || !(0..=2).contains(&algo)
        || n > 7
        || !(1..=7).contains(&m)
        || m < n
        || numhist < 1
        || !(0..numhist).contains(&curnum)
        || (src == 1 && anysrc == 0)
    {
        return f64::NAN;
    }
    let mut g = SplitMix(i64::from(seed) as u64);
    let mut s = State {
        hist: [[0.0; TERMS]; 8],
        srcs: [[0.0; TERMS]; 6],
        ones: [0.0; 7],
        outs: [0.0; 6],
        curr_num_hist: curnum,
        sum_time: 0.0,
    };
    for v in s.hist.iter_mut().chain(s.srcs.iter_mut()).flatten() {
        *v = g.next();
    }
    for v in s.ones.iter_mut().chain(s.outs.iter_mut()) {
        *v = g.next();
    }
    s.hist[OUT_T][0] = tout;
    s.srcs[QSRC][0] = qsrc;
    let c = Model {
        ctf: algo < 2,
        n,
        max_ctf_terms: m,
        simple: simple == 1,
        overrides: overrides == 1,
        any_src: anysrc == 1,
        src: src == 1,
        num_histories: numhist,
        time_step_zone: tsz,
        ctf_time_step: ctfts,
        area,
        temp_in: tin,
        const_in: ctfin,
        const_out: ctfout,
        tsrc_const: tsrcconst,
        tuser_const: tuserconst,
        cross0,
        inside0,
        outside0,
        src_in0: srcin0,
        src_out0: srcout0,
        tsrc_out0: tsrcout0,
        tsrc_in0: tsrcin0,
        tsrc_q0: tsrcq0,
        tuser_out0: tuserout0,
        tuser_in0: tuserin0,
        tuser_src0: tusersrc0,
    };
    update_thermal_histories(&mut s, &c);
    let (a, k) = (which / 8, which % 8);
    match a {
        0..=7 => s.hist[a][k],
        8..=13 => s.srcs[a - 8][k],
        14 => match k {
            0..=5 => s.outs[k],
            6 => f64::from(s.curr_num_hist),
            _ => s.sum_time,
        },
        _ => s.ones.get(k).copied().unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-term wall (cross 1, inside 3, outside 4), outside face at `tout`, inside at 20 C,
    /// no constant parts, area 2, 15-minute steps, `numhist` steps per construction step.
    fn wall(which: i32, simple: i32, numhist: i32, curnum: i32, tout: f64) -> f64 {
        ThermalHist(
            which,
            7,
            1,
            2,
            simple,
            0,
            0,
            0,
            0,
            curnum,
            numhist,
            0.25,
            0.25 * f64::from(numhist),
            2.0,
            tout,
            20.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            3.0,
            4.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        )
    }

    #[test]
    fn current_fluxes_follow_the_first_ctf_terms() {
        // Inside: 30 * 1 - 20 * 3 = -30; outside: 30 * 4 - 20 * 1 = 100, reported as -100.
        assert_eq!(wall(113, 1, 1, 0, 30.0), -30.0);
        assert_eq!(wall(112, 1, 1, 0, 30.0), -60.0);
        assert_eq!(wall(115, 1, 1, 0, 30.0), -100.0);
        assert_eq!(wall(114, 1, 1, 0, 30.0), -200.0);
    }

    #[test]
    fn simple_ctf_shifts_each_history_by_one_term() {
        // Outside temperature term 2 takes term 1; term 3 takes the old term 2.
        assert_eq!(wall(9, 1, 1, 0, 30.0), 30.0);
        let old2 = wall(9, 0, 1, 0, 30.0);
        assert!(old2.is_finite());
        // Inside flux term 2 takes the new current flux.
        assert_eq!(wall(17, 1, 1, 0, 30.0), -30.0);
    }

    #[test]
    fn master_history_moves_on_at_the_end_of_a_series() {
        // One step per series: the step ends it, so the series restarts and term 2 is TempExt1,
        // which the first step of the series set to the current outside temperature.
        assert_eq!(wall(9, 0, 1, 0, 30.0), 30.0);
        assert_eq!(wall(118, 0, 1, 0, 30.0), 0.0);
        // Midway through a two-step series the position moves on and time accumulates.
        assert_eq!(wall(118, 0, 2, 0, 30.0), 1.0);
        assert_eq!(wall(119, 0, 2, 0, 30.0), 0.25);
    }

    #[test]
    fn guards_return_nan() {
        assert!(wall(128, 1, 1, 0, 30.0).is_nan());
        assert!(wall(0, 1, 2, 2, 30.0).is_nan());
        assert!(wall(0, 2, 1, 0, 30.0).is_nan());
    }
}
