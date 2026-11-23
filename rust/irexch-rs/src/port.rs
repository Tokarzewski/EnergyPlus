//! The Rust port of `irexch.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! `HeatBalanceIntRadExchange::CalcInteriorRadExchange` for one radiant enclosure of opaque
//! surfaces and at most one bare window: the inside emissivities and Hottel's ScriptF (or the
//! Carroll method's F') at the start of an environment, then the net long-wave radiation to each
//! surface, and for the window the long-wave radiation from the zone incident on it. The enclosure,
//! its view factors (`CalcApproximateViewFactors`, `FixViewFactors`), `CalcScriptF` and `CalcFMRT`
//! come from `intrad-rs`. Every expression keeps the C++ order. The oracle's guards (see its shim)
//! return NaN here too.

#![forbid(unsafe_code)]

use intrad_rs::port::{
    MAX, Mat, STEFAN_BOLTZMANN, Zone, approximate_view_factors, fix_view_factors, fmrt, script_f,
    zone,
};

/// `Constant::Kelvin`.
const KELVIN: f64 = 273.15;

/// ObjexxFCL `pow_4`.
fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

/// The enclosure state `CalcInteriorRadExchange` keeps between calls.
struct Enclosure<'a> {
    z: &'a Zone,
    f: Mat,
    fmrt: [f64; MAX],
    carroll: bool,
    /// The window's index in the enclosure (from 0), if any.
    win: Option<usize>,
    /// The window's inside glass face temperature at the end of the last time step (C).
    twin: f64,
    script_f: Mat,
    fp: [f64; MAX],
}

impl Enclosure<'_> {
    /// One `CalcInteriorRadExchange` call: the net long-wave radiation to each surface and the
    /// long-wave radiation from the zone incident on each window (W/m2).
    fn calc(
        &mut self,
        surface_temp: &[f64],
        surf_iterations: i32,
        begin_envrn: bool,
    ) -> ([f64; MAX], [f64; MAX]) {
        let n = self.z.n;
        let area = &self.z.area;
        let mut net = [0.0; MAX];
        let mut ir = [0.0; MAX];
        if surf_iterations == 0 && begin_envrn {
            // The inside surface emissivities (SurfAbsThermalInt) for this time step.
            let mut emissivity = self.z.emiss;
            if self.carroll {
                for i in 0..n {
                    let e = emissivity[i];
                    self.fp[i] = STEFAN_BOLTZMANN * e / (e / self.fmrt[i] + 1.0 - e); // `CalcFp`
                }
            } else {
                self.script_f = script_f(&area[..n], &self.f, &mut emissivity[..n]);
                for sf in &mut self.script_f.v[..n * n] {
                    *sf *= STEFAN_BOLTZMANN;
                }
            }
        }

        let mut temp_rad = [0.0; MAX];
        let mut emiss = [0.0; MAX];
        let mut t4 = [0.0; MAX];
        let mut carroll_num = 0.0;
        let mut carroll_den = 0.0;
        for k in 0..n {
            temp_rad[k] = if self.win == Some(k) && surf_iterations == 0 {
                // A bare window at the first iteration: last time step's glass temperature.
                (self.twin + KELVIN) - KELVIN
            } else {
                surface_temp[k]
            };
            emiss[k] = self.z.emiss[k];
            t4[k] = pow_4(temp_rad[k] + KELVIN);
            if self.carroll {
                carroll_num += t4[k] * self.fp[k] * area[k];
                carroll_den += self.fp[k] * area[k];
            }
        }

        if self.carroll {
            let mrt4 = if carroll_den > 0.0 {
                carroll_num / carroll_den
            } else {
                293.15
            };
            for rec in 0..n {
                if self.win == Some(rec) {
                    let mut num_win = 0.0;
                    let mut den_win = 0.0;
                    for send in (0..n).filter(|&s| s != rec) {
                        num_win += pow_4(temp_rad[send] + KELVIN) * self.fp[send] * area[send];
                        den_win += self.fp[send] * area[send];
                    }
                    let mrt4_win = if den_win > 0.0 {
                        num_win / den_win
                    } else {
                        mrt4
                    };
                    ir[rec] += (self.fp[rec] * mrt4_win) / emiss[rec];
                }
                net[rec] += self.fp[rec] * (mrt4 - t4[rec]);
            }
        } else {
            let sf = &mut self.script_f.v;
            for rec in 0..n {
                if self.win == Some(rec) {
                    let mut script_f_acc = 0.0;
                    let mut net_cor = 0.0;
                    let mut ir_acc = 0.0;
                    for send in 0..n {
                        let script_f_temp = sf[rec * n + send] * t4[send];
                        ir_acc += script_f_temp;
                        if rec != send {
                            script_f_acc += sf[rec * n + send];
                        } else {
                            net_cor = script_f_temp;
                        }
                    }
                    net[rec] += ir_acc - net_cor - (script_f_acc * t4[rec]);
                    ir[rec] += ir_acc / emiss[rec];
                } else {
                    let mut acc = 0.0;
                    sf[rec * n + rec] = 0.0;
                    for send in 0..n {
                        acc += sf[rec * n + send] * (t4[send] - t4[rec]);
                    }
                    net[rec] += acc;
                }
            }
        }
        (net, ir)
    }
}

/// `v` is finite and within `lo` to `hi`.
fn within(v: f64, lo: f64, hi: f64) -> bool {
    v.is_finite() && v >= lo && v <= hi
}

/// Port of `double IRExchange(int which, int i, int mask, double L, double W, double H, double
/// mass, double rot, double e0, double de, int carroll, int win, double t0, double dt, double twin,
/// int iter)` (`irexch.c:9`): see the oracle's shim.
#[expect(
    clippy::too_many_arguments,
    reason = "the exported function keeps the C signature of the forwarder"
)]
pub fn IRExchange(
    which: std::ffi::c_int,
    i: std::ffi::c_int,
    mask: std::ffi::c_int,
    L: f64,
    W: f64,
    H: f64,
    mass: f64,
    rot: f64,
    e0: f64,
    de: f64,
    carroll: std::ffi::c_int,
    win: std::ffi::c_int,
    t0: f64,
    dt: f64,
    twin: f64,
    iter: std::ffi::c_int,
) -> f64 {
    let ok = (0..=1).contains(&which)
        && (0..=1).contains(&carroll)
        && (0..=1).contains(&iter)
        && (1..=127).contains(&mask)
        && within(L, 0.5, 50.0)
        && within(W, 0.5, 50.0)
        && within(H, 0.5, 50.0)
        && within(mass, 0.0, 500.0)
        && !((mask >> 6) & 1 == 1 && mass < 0.5)
        && within(rot, 0.0, 360.0)
        && within(e0, 0.05, 0.99)
        && de.is_finite()
        && within(t0, -30.0, 80.0)
        && within(dt, -10.0, 10.0)
        && within(twin, -30.0, 80.0);
    if !ok {
        return f64::NAN;
    }
    let Some(z) = zone(mask, L, W, H, mass, rot, e0, de) else {
        return f64::NAN;
    };
    let n = z.n;
    let (Ok(i), Ok(win)) = (usize::try_from(i), usize::try_from(win)) else {
        return f64::NAN;
    };
    if !(1..=n).contains(&i) || win > n || !within(e0 + de * (n as f64 - 1.0), 0.05, 0.99) {
        return f64::NAN;
    }
    let mut f = approximate_view_factors(&z);
    if fix_view_factors(&z.area[..n], &mut f).is_none() {
        return f64::NAN;
    }
    let carroll = carroll == 1;
    let mut encl = Enclosure {
        z: &z,
        f,
        fmrt: if carroll {
            fmrt(&z.area[..n])
        } else {
            [0.0; MAX]
        },
        carroll,
        win: win.checked_sub(1),
        twin,
        script_f: Mat::new(n, 0.0),
        fp: [0.0; MAX],
    };
    let mut surface_temp = [0.0; MAX];
    for (k, t) in surface_temp.iter_mut().enumerate().take(n) {
        *t = t0 + dt * k as f64;
    }
    let mut out = encl.calc(&surface_temp, 0, true);
    if iter == 1 {
        for (k, t) in surface_temp.iter_mut().enumerate().take(n) {
            *t += 0.5 * (k + 1) as f64;
        }
        out = encl.calc(&surface_temp, 1, false);
    }
    if which == 0 {
        out.0[i - 1]
    } else {
        out.1[i - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_long_wave_balances() {
        let net = |i, carroll| {
            IRExchange(
                0, i, 63, 4.0, 3.0, 2.7, 0.0, 0.0, 0.9, 0.0, carroll, 0, 20.0, 1.0, 20.0, 1,
            )
        };
        for carroll in 0..=1 {
            let sum: f64 = (1..=6)
                .map(|i| {
                    net(i, carroll)
                        * if i <= 2 {
                            12.0
                        } else if i % 2 == 1 {
                            10.8
                        } else {
                            8.1
                        }
                })
                .sum();
            assert!(sum.abs() < 1.0);
            assert!(net(1, carroll) > 0.0 && net(6, carroll) < 0.0);
        }
        assert!(
            IRExchange(
                1, 3, 63, 4.0, 3.0, 2.7, 0.0, 0.0, 0.9, 0.0, 0, 3, 20.0, 0.0, 15.0, 0
            ) > 0.0
        );
        assert!(
            IRExchange(
                0, 7, 63, 4.0, 3.0, 2.7, 0.0, 0.0, 0.9, 0.0, 0, 0, 20.0, 0.0, 15.0, 0
            )
            .is_nan()
        );
    }
}
