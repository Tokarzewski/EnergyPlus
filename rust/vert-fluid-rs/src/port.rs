//! The Rust port of `vertfluid.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The vertical ground heat exchanger's fluid-dependent code from EnergyPlus
//! `GroundHeatExchangers/Vertical.cc` (`GLHEVert`), on a plant loop of water: the pipe convection
//! resistance (Gnielinski with the laminar blend), the pipe resistance, the multipole borehole
//! resistances, and the short-time-step g-functions (the 535-cell radial model solved with
//! `TDMA` from `Base.cc`). Water properties come from `fluidproperties-rs` (unit 2), the friction
//! factor and pipe conduction from `vert-rs` (unit 14). Nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

use fluidproperties_rs::port::{
    WaterConductivity, WaterDensity, WaterSpecificHeat, WaterViscosity,
};
use vert_rs::port::{VertPipeConductionResistance, friction_factor};

const PI: f64 = std::f64::consts::PI; // `Constant::Pi`
/// The most short time steps a case may take, as the oracle caps them.
const MAX_SHORT_STEPS: i32 = 20000;
const TIME_STEP: f64 = 120.0; // `time_step`
const HEAT_FLUX: f64 = 1.0; // `heat_flux`

fn sq(x: f64) -> f64 {
    x * x // `pow_2`
}

fn pow_4(x: f64) -> f64 {
    let t = x * x;
    t * t
}

/// The pipe as the `GLHEVert` constructor sets it up.
#[derive(Clone, Copy)]
struct Pipe {
    out_dia: f64,
    inner_dia: f64,
    out_radius: f64,
    inner_radius: f64,
    thickness: f64,
    k: f64,
}

impl Pipe {
    fn new(out_dia: f64, thickness: f64, k: f64) -> Self {
        let inner_dia = out_dia - 2.0 * thickness;
        Pipe {
            out_dia,
            inner_dia,
            out_radius: out_dia / 2.0,
            inner_radius: inner_dia / 2.0,
            thickness,
            k,
        }
    }

    /// `calcPipeConvectionResistance` (`Vertical.cc:1386`).
    fn convection(&self, mass_flow: f64, num_bh: i32, t: f64) -> f64 {
        let cp = WaterSpecificHeat(t);
        let k = WaterConductivity(t);
        let mu = WaterViscosity(t);
        let re = 4.0 * (mass_flow / f64::from(num_bh)) / (mu * PI * self.inner_dia);
        let turbulent = || {
            let f = friction_factor(re);
            let pr = (cp * mu) / k;
            (f / 8.0) * (re - 1000.0) * pr
                / (1.0 + 12.7 * (f / 8.0).sqrt() * (pr.powf(2.0 / 3.0) - 1.0))
        };
        let nu = if re < 2000.0 {
            4.01
        } else if re < 4000.0 {
            let nu_high = turbulent();
            let sf = 1.0 / (1.0 + (-(re - 3000.0) / 150.0).exp());
            (1.0 - sf) * 4.01 + sf * nu_high
        } else {
            turbulent()
        };
        let h = nu * k / self.inner_dia;
        1.0 / (h * PI * self.inner_dia)
    }

    /// `calcPipeResistance` (`Vertical.cc:1454`).
    fn resistance(&self, mass_flow: f64, num_bh: i32, t: f64) -> f64 {
        VertPipeConductionResistance(self.out_dia, self.inner_dia, self.k)
            + self.convection(mass_flow, num_bh, t)
    }
}

/// A borehole with its multipole constants, as the `GLHEVert` constructor sets them up.
struct Bore {
    pipe: Pipe,
    mass_flow: f64,
    num_bh: i32,
    t: f64,
    bh_length: f64,
    bh_radius: f64,
    grout_k: f64,
    theta_1: f64,
    theta_2: f64,
    theta_3: f64,
    sigma: f64,
}

impl Bore {
    fn new(pipe: Pipe, mass_flow: f64, num_bh: i32, t: f64, bh: [f64; 4], soil_k: f64) -> Self {
        let [bh_length, bh_diameter, u_tube_dist, grout_k] = bh;
        let bh_radius = bh_diameter / 2.0;
        let theta_1 = u_tube_dist / (2.0 * bh_radius);
        let theta_2 = bh_radius / pipe.out_radius;
        Bore {
            pipe,
            mass_flow,
            num_bh,
            t,
            bh_length,
            bh_radius,
            grout_k,
            theta_1,
            theta_2,
            theta_3: 1.0 / (2.0 * theta_1 * theta_2),
            sigma: (grout_k - soil_k) / (grout_k + soil_k),
        }
    }

    fn pipe_resistance(&self) -> f64 {
        self.pipe.resistance(self.mass_flow, self.num_bh, self.t)
    }

    fn beta(&self) -> f64 {
        2.0 * PI * self.grout_k * self.pipe_resistance()
    }

    /// `calcBHAverageResistance` (`Vertical.cc:1313`).
    fn average(&self) -> f64 {
        let (t1, t3, s) = (self.theta_1, self.theta_3, self.sigma);
        let beta = self.beta();
        let p4 = pow_4(t1);
        let final_1 = (self.theta_2 / (2.0 * t1 * (1.0 - p4).powf(s))).ln();
        let num_2 = sq(t3) * sq(1.0 - (4.0 * s * p4) / (1.0 - p4));
        let den_2 = (1.0 + beta) / (1.0 - beta) + sq(t3) * (1.0 + (16.0 * s * p4) / sq(1.0 - p4));
        1.0 / (4.0 * PI * self.grout_k) * (beta + final_1 - num_2 / den_2)
    }

    /// `calcBHTotalInternalResistance` (`Vertical.cc:1332`).
    fn total_internal(&self) -> f64 {
        let (t1, t3, s) = (self.theta_1, self.theta_3, self.sigma);
        let beta = self.beta();
        let p4 = pow_4(t1);
        let final_1 = ((1.0 + sq(t1)).powf(s) / (t3 * (1.0 - sq(t1)).powf(s))).ln();
        let num_2 = sq(t3) * sq(1.0 - p4 + 4.0 * s * sq(t1));
        let den_1 = (1.0 + beta) / (1.0 - beta) * sq(1.0 - p4);
        let den_2 = sq(t3) * sq(1.0 - p4);
        let den_3 = 8.0 * s * sq(t1) * sq(t3) * (1.0 + p4);
        1.0 / (PI * self.grout_k) * (beta + final_1 - num_2 / (den_1 - den_2 + den_3))
    }

    /// `calcHXResistance` (`Vertical.cc:1362`).
    fn effective(&self) -> f64 {
        if self.mass_flow <= 0.0 {
            return 0.0;
        }
        let cp = WaterSpecificHeat(self.t);
        self.average()
            + 1.0 / (3.0 * self.total_internal()) * sq(self.bh_length / (self.mass_flow * cp))
    }
}

/// The steps `calcShortTimestepGFunctions` takes; 0 past `MAX_SHORT_STEPS` (or never stopping).
fn short_steps(bh_length: f64, diffusivity: f64) -> i32 {
    let t_max = (-8.6f64).exp() * (sq(bh_length) / (9.0 * diffusivity));
    let mut total = 0.0;
    let mut n = 0;
    while total < t_max {
        total += TIME_STEP;
        n += 1;
        if n > MAX_SHORT_STEPS {
            return 0;
        }
    }
    n
}

/// One radial cell of the short-time-step model (`Cell`).
#[derive(Clone, Copy)]
struct Cell {
    center: f64,
    outer: f64,
    inner: f64,
    k: f64,
    rho_cp: f64,
}

/// `n` cells `th` thick from `r0` outwards.
fn layer(cells: &mut Vec<Cell>, n: i32, r0: f64, th: f64, k: f64, rho_cp: f64) {
    for i in 0..n {
        let inner = r0 + f64::from(i) * th;
        cells.push(Cell {
            center: inner + th / 2.0,
            outer: inner + th,
            inner,
            k,
            rho_cp,
        });
    }
}

/// `log(r_a / r_b) / (2 pi k)`: half a cell's radial resistance.
fn half(r_a: f64, r_b: f64, k: f64) -> f64 {
    (r_a / r_b).ln() / (2.0 * PI * k)
}

/// A borehole and its loop for the short-time-step model, as plain numbers: `bh` is
/// `[bhLength, bhDiameter, uTubeDist, groutK]`, `pipe` is `[outDia, thickness, k]`.
#[derive(Clone, Copy, Debug)]
pub struct ShortInputs {
    pub bh: [f64; 4],
    pub grout_rho_cp: f64,
    pub soil_k: f64,
    pub soil_rho_cp: f64,
    pub pipe: [f64; 3],
    pub pipe_rho_cp: f64,
    pub mass_flow: f64,
    pub num_bh: i32,
    pub inlet_temp: f64,
}

impl ShortInputs {
    fn diffusivity(&self) -> f64 {
        self.soil_k / self.soil_rho_cp
    }

    /// The steps the run takes, or `None` outside the oracle's domain (ADR-009).
    fn steps(&self) -> Option<i32> {
        let n = short_steps(self.bh[0], self.diffusivity());
        (self.num_bh >= 1 && n > 0).then_some(n)
    }
}

/// Port of `calcShortTimestepGFunctions` (`Vertical.cc:1048`): every step's g-function and LNTTS
/// (`GFNC_shortTimestep`, `LNTTS_shortTimestep`), or `None` outside the oracle's domain.
pub fn short_series(inp: &ShortInputs) -> Option<Vec<(f64, f64)>> {
    Some(short_run(inp, inp.steps()?))
}

/// The first `steps` steps of `calcShortTimestepGFunctions`. The matrix and TDMA's forward sweep
/// do not change between steps, so they are worked out once; the values are the ones the C++
/// recomputes each step.
fn short_run(inp: &ShortInputs, steps: i32) -> Vec<(f64, f64)> {
    let (grout_rho_cp, pipe_rho_cp, soil_rho_cp) =
        (inp.grout_rho_cp, inp.pipe_rho_cp, inp.soil_rho_cp);
    let (soil_k, diffusivity) = (inp.soil_k, inp.diffusivity());
    let pipe = Pipe::new(inp.pipe[0], inp.pipe[1], inp.pipe[2]);
    let b = &Bore::new(
        pipe,
        inp.mass_flow,
        inp.num_bh,
        inp.inlet_temp,
        inp.bh,
        soil_k,
    );
    let pcf = b.pipe.thickness / 4.0;
    let r_pipe_out = 2.0f64.sqrt() * b.pipe.out_radius;
    let r_pipe_in = r_pipe_out - b.pipe.thickness;
    let r_conv = r_pipe_in - 1.0 * pcf;
    let r_fluid = r_conv - (3.0 - 0.5) * pcf;
    let r_grout = b.bh_radius;

    let bh_res = b.average();
    let conv_res = b.pipe.convection(b.mass_flow, b.num_bh, b.t);
    let eq_tube_grout = bh_res - conv_res / 2.0;
    let eq_conv = bh_res - eq_tube_grout;
    let t0 = b.t;
    let (cp, rho) = (WaterSpecificHeat(t0), WaterDensity(t0));

    let mut cells = Vec::with_capacity(535);
    let fluid_rho_cp = 2.0 * cp * rho * sq(b.pipe.inner_radius) / (sq(r_conv) - sq(r_fluid));
    for i in 0..3 {
        let center = r_fluid + f64::from(i) * pcf;
        let inner = if i == 0 { center } else { center - pcf / 2.0 };
        cells.push(Cell {
            center,
            outer: center + pcf / 2.0,
            inner,
            k: 200.0,
            rho_cp: fluid_rho_cp,
        });
    }
    layer(
        &mut cells,
        1,
        r_conv,
        pcf,
        (r_pipe_in / r_conv).ln() / (2.0 * PI * eq_conv),
        1.0,
    );
    let k_tube_grout = (r_grout / r_pipe_in).ln() / (2.0 * PI * eq_tube_grout);
    layer(&mut cells, 4, r_pipe_in, pcf, k_tube_grout, pipe_rho_cp);
    layer(
        &mut cells,
        27,
        r_pipe_out,
        (r_grout - r_pipe_out) / 27.0,
        k_tube_grout,
        grout_rho_cp,
    );
    layer(
        &mut cells,
        500,
        r_grout,
        (10.0 - r_grout) / 500.0,
        soil_k,
        soil_rho_cp,
    );

    // The matrix (a, b, c), the forward-swept c and each row's pivot, and the source term 1 / AD.
    let n = cells.len() - 1;
    let (mut a, mut bb, mut c, mut src) = (
        vec![0.0; n + 1],
        vec![0.0; n + 1],
        vec![0.0; n + 1],
        vec![0.0; n + 1],
    );
    for i in 0..=n {
        let me = cells[i];
        if i == n {
            bb[i] = 1.0;
            continue;
        }
        let east = cells[i + 1];
        let ae = 1.0 / (half(me.outer, me.center, me.k) + half(east.center, east.inner, east.k));
        let ad = me.rho_cp * (PI * (sq(me.outer) - sq(me.inner))) / TIME_STEP;
        src[i] = HEAT_FLUX / ad;
        c[i] = ae / ad;
        if i == 0 {
            bb[i] = -ae / ad - 1.0;
        } else {
            let west = cells[i - 1];
            let aw =
                -1.0 / (half(west.outer, west.center, west.k) + half(me.center, me.inner, me.k));
            a[i] = -aw / ad;
            bb[i] = aw / ad - ae / ad - 1.0;
        }
    }
    let mut piv = vec![bb[0]; n + 1];
    c[0] /= bb[0];
    for i in 1..=n {
        piv[i] = bb[i] - a[i] * c[i - 1];
        if i < n {
            c[i] /= piv[i];
        }
    }

    let t_s = sq(b.bh_length) / (9.0 * diffusivity);
    let mut temps = vec![t0; n + 1];
    let mut d = vec![0.0; n + 1];
    let mut total = 0.0;
    let mut out = Vec::with_capacity(usize::try_from(steps).unwrap_or(0));
    for _ in 0..steps {
        d[0] = (-temps[0] - src[0]) / piv[0];
        for i in 1..n {
            d[i] = (-temps[i] - a[i] * d[i - 1]) / piv[i];
        }
        d[n] = (temps[n] - a[n] * d[n - 1]) / piv[n];
        for i in (0..n).rev() {
            d[i] -= c[i] * d[i + 1];
        }
        std::mem::swap(&mut temps, &mut d);
        total += TIME_STEP;
        out.push((
            2.0 * PI * soil_k * ((temps[0] - t0) / HEAT_FLUX - bh_res),
            (total / t_s).ln(),
        ));
    }
    out
}

pub fn VFConvectionResistance(
    outDia: f64,
    thickness: f64,
    massFlow: f64,
    numBH: i32,
    inletTemp: f64,
) -> f64 {
    if numBH < 1 {
        return f64::NAN;
    }
    Pipe::new(outDia, thickness, 0.0).convection(massFlow, numBH, inletTemp)
}

pub fn VFPipeResistance(
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    massFlow: f64,
    numBH: i32,
    inletTemp: f64,
) -> f64 {
    if numBH < 1 {
        return f64::NAN;
    }
    Pipe::new(outDia, thickness, kPipe).resistance(massFlow, numBH, inletTemp)
}

pub fn VFBoreholeResistance(
    which: i32,
    bhLength: f64,
    bhDiameter: f64,
    uTubeDist: f64,
    groutK: f64,
    soilK: f64,
    outDia: f64,
    thickness: f64,
    kPipe: f64,
    massFlow: f64,
    numBH: i32,
    inletTemp: f64,
) -> f64 {
    if numBH < 1 || !(0..=3).contains(&which) {
        return f64::NAN;
    }
    let pipe = Pipe::new(outDia, thickness, kPipe);
    let b = Bore::new(
        pipe,
        massFlow,
        numBH,
        inletTemp,
        [bhLength, bhDiameter, uTubeDist, groutK],
        soilK,
    );
    match which {
        0 => b.average(),
        1 => b.total_internal(),
        2 => b.average() - b.pipe_resistance() / 2.0,
        _ => b.effective(),
    }
}

pub fn VFShortSteps(bhLength: f64, soilK: f64, soilRhoCp: f64) -> i32 {
    short_steps(bhLength, soilK / soilRhoCp)
}

/// The k-th short time step's g-function and LNTTS, or `None` where the oracle returns NaN.
fn short_at(k: i32, inp: &ShortInputs) -> Option<(f64, f64)> {
    if k < 0 || k >= inp.steps()? {
        return None;
    }
    short_run(inp, k + 1).last().copied()
}

pub fn VFShortGFunction(
    k: i32,
    bhLength: f64,
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
    numBH: i32,
    inletTemp: f64,
) -> f64 {
    short_at(
        k,
        &ShortInputs {
            bh: [bhLength, bhDiameter, uTubeDist, groutK],
            grout_rho_cp: groutRhoCp,
            soil_k: soilK,
            soil_rho_cp: soilRhoCp,
            pipe: [outDia, thickness, kPipe],
            pipe_rho_cp: pipeRhoCp,
            mass_flow: massFlow,
            num_bh: numBH,
            inlet_temp: inletTemp,
        },
    )
    .map_or(f64::NAN, |r| r.0)
}

pub fn VFShortLntts(
    k: i32,
    bhLength: f64,
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
    numBH: i32,
    inletTemp: f64,
) -> f64 {
    short_at(
        k,
        &ShortInputs {
            bh: [bhLength, bhDiameter, uTubeDist, groutK],
            grout_rho_cp: groutRhoCp,
            soil_k: soilK,
            soil_rho_cp: soilRhoCp,
            pipe: [outDia, thickness, kPipe],
            pipe_rho_cp: pipeRhoCp,
            mass_flow: massFlow,
            num_bh: numBH,
            inlet_temp: inletTemp,
        },
    )
    .map_or(f64::NAN, |r| r.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laminar_convection_uses_the_mean_nusselt_number() {
        // No flow: Nu = 4.01, so R = 1 / (4.01 k pi).
        let r = VFConvectionResistance(0.0334, 0.003, 0.0, 1, 20.0);
        assert!((r - 1.0 / (4.01 * WaterConductivity(20.0) * PI)).abs() < 1e-12);
    }

    #[test]
    fn short_steps_count_120_second_steps() {
        let t_max = (-8.6f64).exp() * (100.0 * 100.0 / (9.0 * 1e-6));
        let n = VFShortSteps(100.0, 2.0, 2.0e6);
        assert!(f64::from(n - 1) * TIME_STEP < t_max && t_max <= f64::from(n) * TIME_STEP);
        assert_eq!(VFShortSteps(f64::NAN, 2.0, 2.0e6), 0);
    }

    #[test]
    fn short_lntts_is_the_log_of_elapsed_time() {
        let args = (
            100.0, 0.114, 0.05, 1.5, 3.9e6, 2.0, 2.0e6, 0.0334, 0.003, 0.39, 1.8e6, 0.5, 1, 20.0,
        );
        let t_s = 100.0 * 100.0 / (9.0 * 1e-6);
        let l = VFShortLntts(
            4, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9,
            args.10, args.11, args.12, args.13,
        );
        assert!((l - (600.0f64 / t_s).ln()).abs() < 1e-12);
    }
}
