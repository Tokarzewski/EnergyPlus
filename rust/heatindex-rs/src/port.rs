//! The Rust port of `heatindex.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The EnergyPlus extended heat index (namespace `ExtendedHI`, `ExtendedHeatIndex.cc`; Lu and
//! Romps 2022), with the one algorithm of `General::SolveRoot` it runs. `heatindex` selects
//! `RootAlgo::ShortBisectionThenRegulaFalsi`, and the steps are exported as they run inside it,
//! so no other algorithm is needed. Every formula keeps the C++ operation order with no FMA, and
//! `powf` and `exp` call the same C library functions as `pow` and `exp`. C has no error
//! channel: nothing panics, and a solver that does not converge returns what the C++ returns.

#![forbid(unsafe_code)]

// Thermodynamic parameters (`pvstar`, `Le`).
const TTRIP: f64 = 273.16; // K
const PTRIP: f64 = 611.65; // Pa
const E0V: f64 = 2.3740e6; // J/kg
const E0S: f64 = 0.3337e6; // J/kg
const RGASA: f64 = 287.04; // J/kg/K
const RGASV: f64 = 461.; // J/kg/K
const CVV: f64 = 1418.; // J/kg/K
const CVL: f64 = 4119.; // J/kg/K
const CVS: f64 = 1861.; // J/kg/K
const CPA: f64 = 719. + RGASA; // cva + rgasa
const CPV: f64 = CVV + RGASV;

// The body (Steadman 1979, Lu and Romps 2022).
const Q: f64 = 180.; // W/m2, metabolic rate per skin area
const PHI_SALT: f64 = 0.9; // vapour saturation pressure level of saline solution
const TC: f64 = 310.; // K, core temperature
const R: f64 = 124.; // Pa/K, Zf/Rf
const ZA: f64 = 60.6 / 17.4; // Pa m2/W, mass transfer resistance through air, exposed skin
const ZA_BAR: f64 = 60.6 / 11.6; // clothed skin
const ZA_UN: f64 = 60.6 / 12.3; // naked
const RS0: f64 = 0.0387; // m2 K/W, heat transfer resistance through skin
const PHI0: f64 = 0.84; // covering fraction
const PA0: f64 = 1.6e3; // Pa, reference air vapour pressure in regions III to VI

// `SolveRoot` settings as `ExtendedHeatIndex.cc` passes them.
const TOL: f64 = 1e-5;
const MAX_ITER: i32 = 100;

/// The `ExtendedHI::EqvarName` values, as the C++ enum numbers them.
const PHI: i32 = 0;
const RF: i32 = 1;
const RS: i32 = 2;
const DTCDT: i32 = 3;

/// `std::max(a, b)`, `a < b ? b : a`: a NaN `a` is returned, a NaN `b` is ignored.
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// `std::min(a, b)`, `b < a ? b : a`: a NaN `a` is returned, a NaN `b` is ignored.
fn cpp_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}

/// `General::SolveRoot` with `RootAlgo::ShortBisectionThenRegulaFalsi`: three bisections, then
/// regula falsi, until `|f| < TOL`. Without a sign change it returns `x_0`; without convergence,
/// the last estimate (the C++ also sets an error flag, which `ExtendedHeatIndex.cc` ignores).
fn solve_root(mut f: impl FnMut(f64) -> f64, x_0: f64, x_1: f64) -> f64 {
    const SMALL: f64 = 1e-10;
    let (mut x0, mut x1, mut x_temp) = (x_0, x_1, x_0);
    let mut n_ite = 0;
    let (mut y0, mut y1) = (f(x0), f(x1));
    if y0 * y1 > 0.0 {
        return x0;
    }
    loop {
        let dy = if (y0 - y1).abs() < SMALL {
            SMALL
        } else {
            y0 - y1
        };
        if (x1 - x0).abs() < SMALL {
            break;
        }
        x_temp = if n_ite < 3 {
            (x1 + x0) / 2.0
        } else {
            (y0 * x1 - y1 * x0) / dy
        };
        let y_temp = f(x_temp);
        n_ite += 1;
        if y_temp.abs() < TOL {
            return x_temp;
        }
        if n_ite > MAX_ITER {
            break;
        }
        // Keep the bound whose value has the other sign.
        if (y0 < 0.0) == (y_temp < 0.0) {
            (x0, y0) = (x_temp, y_temp);
        } else {
            (x1, y1) = (x_temp, y_temp);
        }
    }
    x_temp
}

/// Port of `pvstar` (`ExtendedHeatIndex.cc:76`): saturation vapour pressure over ice below the
/// triple point and over liquid above it [Pa].
pub fn pvstar(T: f64) -> f64 {
    if T == 0.0 {
        return 0.0;
    }
    // The C++ writes the two branches out; their constants fold to the same doubles here.
    let (cv, e0) = if T < TTRIP {
        (CVS, E0V + E0S)
    } else {
        (CVL, E0V)
    };
    PTRIP
        * (T / TTRIP).powf((CPV - cv) / RGASV)
        * ((e0 - (CVV - cv) * TTRIP) / RGASV * (1. / TTRIP - 1. / T)).exp()
}

/// Port of `Le` (`ExtendedHeatIndex.cc:102`): latent heat of vaporisation [J/kg].
pub fn Le(T: f64) -> f64 {
    E0V + (CVV - CVL) * (T - TTRIP) + RGASV * T
}

/// Port of `Qv` (`ExtendedHeatIndex.cc:114`): respiratory heat loss [W/m2]. 1.43e-6 kg/J is
/// the inhaled mass over the metabolic rate, 1.013e5 Pa the atmospheric pressure.
pub fn Qv(Ta: f64, Pa: f64) -> f64 {
    1.43e-6
        * Q
        * (CPA * (TC - Ta) + Le(310.) * RGASA / (1.013e5 * RGASV) * (PHI_SALT * pvstar(TC) - Pa))
}

/// Port of `Zs` (`ExtendedHeatIndex.cc:135`): mass transfer resistance through skin.
pub fn Zs(Rs: f64) -> f64 {
    if Rs == 0.0387 {
        52.1
    } else {
        6.0e8 * Rs.powf(5.0)
    }
}

/// The radiative and convective air resistance shared by `Ra`, `Ra_bar` and `Ra_un`: emissivity
/// 0.97, the Stefan-Boltzmann constant 5.67e-8 W/m2/K4.
fn air_resistance(hc: f64, phi_rad: f64, ts: f64, ta: f64) -> f64 {
    1.0 / (hc + 0.97 * phi_rad * 5.67e-8 * (ts.powf(2.0) + ta.powf(2.0)) * (ts + ta))
}

/// Port of `Ra` (`ExtendedHeatIndex.cc:141`): exposed skin [K m2/W].
pub fn Ra(Ts: f64, Ta: f64) -> f64 {
    air_resistance(17.4, 0.85, Ts, Ta)
}

/// Port of `Ra_bar` (`ExtendedHeatIndex.cc:152`): clothed skin [K m2/W].
pub fn Ra_bar(Tf: f64, Ta: f64) -> f64 {
    air_resistance(11.6, 0.79, Tf, Ta)
}

/// Port of `Ra_un` (`ExtendedHeatIndex.cc:163`): naked [K m2/W].
pub fn Ra_un(Ts: f64, Ta: f64) -> f64 {
    air_resistance(12.3, 0.80, Ts, Ta)
}

/// The heat capacity of the core per skin area, `M cpc / A`: an 83.6 kg, 1.69 m adult with the
/// DuBois area and a core of 3492 J/kg/K.
fn core_heat_capacity() -> f64 {
    83.6 * 3492. / (0.202 * 83.6_f64.powf(0.425) * 1.69_f64.powf(0.725))
}

/// What every `find_eqvar_*` step computes first: the vapour pressure, and the temperature of
/// skin covered at `PHI0` (`ts`) with its heat balance (`flux1`).
struct Body {
    ta: f64,
    pa: f64,
    pc: f64,
    ts: f64,
    flux1: f64,
}

/// The skin (`ra`, `za` for exposed skin) or clothing (`Ra_bar`, `ZA_BAR`) temperature that
/// balances the heat through it, as the first two solves of each step find it.
fn surface_temperature(ra: fn(f64, f64) -> f64, za: f64, ta: f64, pa: f64, pc: f64) -> f64 {
    let zs_rs = Zs(RS0);
    let m = (pc - pa) / (zs_rs + za);
    solve_root(
        |t| (t - ta) / ra(t, ta) + (pc - pa) / (zs_rs + za) - (TC - t) / RS0,
        cpp_max(0.0, cpp_min(TC, ta) - RS0 * m.abs()),
        cpp_max(TC, ta) + RS0 * m.abs(),
    )
}

impl Body {
    fn new(ta: f64, rh: f64) -> Body {
        let (pa, pc) = (rh * pvstar(ta), PHI_SALT * pvstar(TC));
        let ts = surface_temperature(Ra, ZA, ta, pa, pc);
        let flux1 = Q - Qv(ta, pa) - (1.0 - PHI0) * (TC - ts) / RS0;
        Body {
            ta,
            pa,
            pc,
            ts,
            flux1,
        }
    }

    /// `Q - Qv(Ta, Pa)`, the heat the skin must lose.
    fn q_net(&self) -> f64 {
        Q - Qv(self.ta, self.pa)
    }

    /// The heat balance with the clothing temperature (`flux2`).
    fn flux2(&self) -> f64 {
        let tf = surface_temperature(Ra_bar, ZA_BAR, self.ta, self.pa, self.pc);
        self.flux1 - PHI0 * (TC - tf) / RS0
    }

    /// The heat balance of a naked body (`flux3`).
    fn flux3(&self) -> f64 {
        self.q_net() - (TC - self.ta) / Ra_un(TC, self.ta) - (self.pc - self.pa) / ZA_UN
    }

    /// Region I: the covering fraction that balances the heat.
    fn phi(&self) -> f64 {
        1.0 - self.q_net() * RS0 / (TC - self.ts)
    }

    /// Region II: the clothing resistance that balances the heat.
    fn rf(&self) -> f64 {
        let (ta, pa, pc) = (self.ta, self.pa, self.pc);
        let zs_rs = Zs(RS0);
        let ts_bar = TC - self.q_net() * RS0 / PHI0 + (1.0 / PHI0 - 1.0) * (TC - self.ts);
        let tf = solve_root(
            |tf| {
                (tf - ta) / Ra_bar(tf, ta)
                    + (pc - pa) * (tf - ta)
                        / ((zs_rs + ZA_BAR) * (tf - ta) + R * Ra_bar(tf, ta) * (ts_bar - tf))
                    - (TC - ts_bar) / RS0
            },
            ta,
            ts_bar,
        );
        Ra_bar(tf, ta) * (ts_bar - tf) / (tf - ta)
    }

    /// Region III and IV: the skin resistance of a naked, sweating body.
    fn rs(&self) -> f64 {
        let (ta, pa, pc) = (self.ta, self.pa, self.pc);
        let q = self.q_net();
        let mut ts = solve_root(
            |ts| (ts - ta) / Ra_un(ts, ta) + (pc - pa) / (Zs((TC - ts) / q) + ZA_UN) - q,
            0.0,
            TC,
        );
        let mut rs = (TC - ts) / q;
        let zs_rs = Zs(rs);
        let ps = pc - (pc - pa) * zs_rs / (zs_rs + ZA_UN);
        if ps > PHI_SALT * pvstar(ts) {
            ts = solve_root(
                |ts| (ts - ta) / Ra_un(ts, ta) + (PHI_SALT * pvstar(ts) - pa) / ZA_UN - q,
                0.0,
                TC,
            );
            rs = (TC - ts) / q;
        }
        rs
    }
}

/// Port of `find_eqvar_phi` (`ExtendedHeatIndex.cc:175`).
pub fn find_eqvar_phi(Ta: f64, RH: f64) -> f64 {
    let b = Body::new(Ta, RH);
    if b.flux1 <= 0.0 { b.phi() } else { PHI0 }
}

/// Port of `find_eqvar_Rf` (`ExtendedHeatIndex.cc:206`).
pub fn find_eqvar_Rf(Ta: f64, RH: f64) -> f64 {
    let b = Body::new(Ta, RH);
    if b.flux1 <= 0.0 {
        f64::INFINITY
    } else if b.flux2() <= 0.0 {
        b.rf()
    } else {
        0.0
    }
}

/// Port of `find_eqvar_rs` (`ExtendedHeatIndex.cc:269`).
pub fn find_eqvar_rs(Ta: f64, RH: f64) -> f64 {
    let b = Body::new(Ta, RH);
    if b.flux1 > 0.0 && b.flux2() > 0.0 {
        if b.flux3() < 0.0 { b.rs() } else { 0.0 }
    } else {
        RS0
    }
}

/// Port of `find_eqvar_dTcdt` (`ExtendedHeatIndex.cc:347`).
pub fn find_eqvar_dTcdt(Ta: f64, RH: f64) -> f64 {
    let b = Body::new(Ta, RH);
    let flux3 = b.flux3();
    if b.flux1 > 0.0 && b.flux2() > 0.0 && flux3 >= 0.0 {
        (1.0 / core_heat_capacity()) * flux3
    } else {
        0.0
    }
}

/// `find_eqvar_name_and_value` (`ExtendedHeatIndex.cc:400`): the region and its variable.
fn eqvar_name_and_value(ta: f64, rh: f64) -> (i32, f64) {
    let b = Body::new(ta, rh);
    if b.flux1 <= 0.0 {
        return (PHI, b.phi());
    }
    if b.flux2() <= 0.0 {
        return (RF, b.rf());
    }
    let flux3 = b.flux3();
    if flux3 < 0.0 {
        return (RS, b.rs());
    }
    (DTCDT, (1.0 / core_heat_capacity()) * flux3)
}

/// The value `find_eqvar_name_and_value` returns.
pub fn find_eqvar_value(Ta: f64, RH: f64) -> f64 {
    eqvar_name_and_value(Ta, RH).1
}

/// The `EqvarName` `find_eqvar_name_and_value` sets, as its C++ enum number.
pub fn find_eqvar_name(Ta: f64, RH: f64) -> i32 {
    eqvar_name_and_value(Ta, RH).0
}

/// Port of `find_T` (`ExtendedHeatIndex.cc:509`): the air temperature at which the reference
/// person has the same equivalent variable. Any name other than `Phi`, `Rf` or `Rs` is solved
/// as `DTcdt`, as the C++ `else` does.
pub fn find_T(eqvar_name: i32, eqvar: f64) -> f64 {
    let rh = |t: f64| PA0 / pvstar(t); // the humidity of the reference vapour pressure
    match eqvar_name {
        PHI => solve_root(|t| find_eqvar_phi(t, 1.0) - eqvar, 0.0, 240.0),
        RF => solve_root(
            |t| find_eqvar_Rf(t, cpp_min(1.0, rh(t))) - eqvar,
            230.0,
            300.0,
        ),
        RS => solve_root(|t| find_eqvar_rs(t, rh(t)) - eqvar, 295.0, 350.0),
        _ => solve_root(|t| find_eqvar_dTcdt(t, rh(t)) - eqvar, 340.0, 1000.0),
    }
}

/// Port of `heatindex` (`ExtendedHeatIndex.cc:538`): the extended heat index [K] of air at `Ta`
/// [K] and relative humidity `RH` (0 to 1). 0 K gives 0; the C++ solves first and discards the
/// result, which cannot change it.
pub fn heatindex(Ta: f64, RH: f64) -> f64 {
    if Ta == 0.0 {
        return 0.0;
    }
    let (name, value) = eqvar_name_and_value(Ta, RH);
    find_T(name, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_and_special_cases() {
        // A root at 2 between 0 and 3; without a sign change, the first bound.
        assert!((solve_root(|x| x - 2.0, 0.0, 3.0) - 2.0).abs() < TOL);
        assert_eq!(solve_root(|x| x * x + 1.0, -1.0, 1.0), -1.0);
        assert_eq!((pvstar(0.0), Zs(0.0387)), (0.0, 52.1));
        assert_eq!(heatindex(0.0, 0.5), 0.0);
        assert_eq!((cpp_max(1.0, f64::NAN), cpp_min(1.0, f64::NAN)), (1.0, 1.0));
    }
}
