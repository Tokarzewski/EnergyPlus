//! The Rust port of `fluidproperties.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! Each function forwards, in the original, to EnergyPlus's `FluidProperties.cc` through one
//! `EnergyPlusData` in which only `Fluid::InitConstantFluidPropertiesData` has run: the built-in
//! `WATER` glycol and `STEAM` refrigerant tables. The ports below keep the C++ operation order
//! exactly (bit-exact) and take the non-`PERFORMANCE_OPT` branches. `ObjexxFCL` arrays are 1-based,
//! so every index here is 1-based too and goes through [`at`].
//!
//! Warnings are never issued in the original (`WarmupFlag` is set) and never change a result, so
//! they are not ported. The fatal errors (a steam quality outside `[0, 1]` for density and specific
//! heat) abort the original; the ports carry on and return the interpolated value instead.
//!
//! `WaterSpecificHeat` is `GlycolProps::getSpecificHeat` with `EP_cache_GlycolSpecificHeat` on: a
//! cache keyed on the top 28 bits of `Temperature + 1000 * Num`. In a fresh process the first call
//! always misses (the initial key, 1000, is the bit pattern of a subnormal that `T + 1000.0` cannot
//! produce), so it returns the raw value for that exact temperature, which is what is ported. In a
//! long-running EnergyPlus the cache can instead return a value computed for a nearby temperature
//! that shares the truncated key.

#![forbid(unsafe_code)]
#![expect(
    non_snake_case,
    reason = "the names and parameters keep the C++ spelling (`WaterSpecificHeat`, `Temperature`)"
)]

/// `DefaultGlycolTemps` from `FluidProperties.cc` (33 values).
const GLYCOL_TEMPS: [f64; 33] = [
    -35.0, -30.0, -25.0, -20.0, -15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0,
    40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0, 75.0, 80.0, 85.0, 90.0, 95.0, 100.0, 105.0, 110.0,
    115.0, 120.0, 125.0,
];

/// `DefaultWaterCpData` from `FluidProperties.cc` (33 values).
const WATER_CP: [f64; 33] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4217.0, 4198.0, 4191.0, 4185.0, 4181.0, 4179.0, 4180.0,
    4180.0, 4180.0, 4180.0, 4181.0, 4183.0, 4185.0, 4188.0, 4192.0, 4196.0, 4200.0, 4203.0, 4208.0,
    4213.0, 4218.0, 4223.0, 4228.0, 4233.0, 4238.0, 4243.0,
];

/// `DefaultWaterViscData` from `FluidProperties.cc` (33 values).
const WATER_VISC: [f64; 33] = [
    0.0e-3, 0.0e-3, 0.0e-3, 0.0e-3, 0.0e-3, 0.0e-3, 0.0e-3, 1.7912e-3, 1.5183e-3, 1.306e-3,
    1.1376e-3, 1.0016e-3, 0.8901e-3, 0.7974e-3, 0.7193e-3, 0.653e-3, 0.5961e-3, 0.5468e-3,
    0.504e-3, 0.4664e-3, 0.4332e-3, 0.4039e-3, 0.3777e-3, 0.3543e-3, 0.3333e-3, 0.3144e-3,
    0.2973e-3, 0.2817e-3, 0.0e-3, 0.0e-3, 0.0e-3, 0.0e-3, 0.0e-3,
];

/// `DefaultWaterRhoData` from `FluidProperties.cc` (33 values).
const WATER_RHO: [f64; 33] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 999.8, 999.9, 999.7, 999.1, 998.2, 997.0, 995.6, 994.0,
    992.2, 990.2, 988.0, 985.7, 983.2, 980.5, 977.7, 974.8, 971.8, 968.6, 965.3, 961.9, 958.3, 0.0,
    0.0, 0.0, 0.0, 0.0,
];

/// `DefaultWaterCondData` from `FluidProperties.cc` (33 values).
const WATER_COND: [f64; 33] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.561, 0.5705, 0.58, 0.5893, 0.5984, 0.6072, 0.6155, 0.6233,
    0.6306, 0.6373, 0.6436, 0.6492, 0.6543, 0.659, 0.6631, 0.6668, 0.67, 0.6728, 0.6753, 0.6773,
    0.6791, 0.0, 0.0, 0.0, 0.0, 0.0,
];

/// `DefaultSteamTemps` from `FluidProperties.cc` (111 values).
const STEAM_TEMPS: [f64; 111] = [
    1.00e-002, 1.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0, 45.0, 50.0, 55.0, 60.0, 65.0,
    70.0, 72.0, 74.0, 76.0, 78.0, 80.0, 82.0, 84.0, 86.0, 88.0, 90.0, 92.0, 94.0, 96.0, 98.0, 99.0,
    100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0, 108.0, 109.0, 110.0, 111.0, 112.0,
    113.0, 114.0, 115.0, 116.0, 117.0, 118.0, 119.0, 120.0, 121.0, 122.0, 123.0, 124.0, 125.0,
    126.0, 127.0, 128.0, 129.0, 130.0, 132.0, 134.0, 136.0, 138.0, 140.0, 142.0, 144.0, 146.0,
    148.0, 150.0, 152.0, 154.0, 156.0, 158.0, 160.0, 162.0, 164.0, 166.0, 168.0, 170.0, 172.0,
    174.0, 176.0, 178.0, 180.0, 185.0, 190.0, 195.0, 200.0, 205.0, 210.0, 215.0, 220.0, 225.0,
    230.0, 240.0, 250.0, 260.0, 270.0, 280.0, 290.0, 300.0, 310.0, 320.0, 330.0, 340.0, 350.0,
    360.0, 370.0,
];

/// `DefaultSteamPressData` from `FluidProperties.cc` (111 values).
const STEAM_PRESS: [f64; 111] = [
    611.7, 657.1, 872.6, 1228.0, 1706.0, 2339.0, 3170.0, 4247.0, 5629.0, 7385.0, 9595.0, 12350.0,
    15760.0, 19950.0, 25040.0, 31200.0, 34000.0, 37010.0, 40240.0, 43700.0, 47410.0, 51390.0,
    55640.0, 60170.0, 65020.0, 70180.0, 75680.0, 81540.0, 87770.0, 94390.0, 97850.0, 101400.0,
    105100.0, 108900.0, 112800.0, 116800.0, 120900.0, 125100.0, 129500.0, 134000.0, 138600.0,
    143400.0, 148300.0, 153300.0, 158400.0, 163700.0, 169200.0, 174800.0, 180500.0, 186400.0,
    192500.0, 198700.0, 205000.0, 211600.0, 218300.0, 225200.0, 232200.0, 239500.0, 246900.0,
    254500.0, 262300.0, 270300.0, 286800.0, 304200.0, 322400.0, 341500.0, 361500.0, 382500.0,
    404400.0, 427300.0, 451200.0, 476200.0, 502200.0, 529500.0, 557800.0, 587400.0, 618200.0,
    650300.0, 683700.0, 718500.0, 754600.0, 792200.0, 831200.0, 871800.0, 913800.0, 957500.0,
    1003000.0, 1123000.0, 1255000.0, 1399000.0, 1555000.0, 1724000.0, 1908000.0, 2106000.0,
    2320000.0, 2550000.0, 2797000.0, 3347000.0, 3976000.0, 4692000.0, 5503000.0, 6417000.0,
    7442000.0, 8588000.0, 9865000.0, 11280000.0, 12860000.0, 14600000.0, 16530000.0, 18670000.0,
    21040000.0,
];

/// `DefaultSteamEnthalpyFluidData` from `FluidProperties.cc` (111 values).
const STEAM_HF: [f64; 111] = [
    0.59, 4177.0, 21020.0, 42020.0, 62980.0, 83910.0, 104800.0, 125700.0, 146600.0, 167500.0,
    188400.0, 209300.0, 230300.0, 251200.0, 272100.0, 293100.0, 301400.0, 309800.0, 318200.0,
    326600.0, 335000.0, 343400.0, 351800.0, 360200.0, 368600.0, 377000.0, 385500.0, 393900.0,
    402300.0, 410700.0, 414900.0, 419200.0, 423400.0, 427600.0, 431800.0, 436000.0, 440300.0,
    444500.0, 448700.0, 453000.0, 457200.0, 461400.0, 465600.0, 469900.0, 474100.0, 478400.0,
    482600.0, 486800.0, 491100.0, 495300.0, 499600.0, 503800.0, 508100.0, 512300.0, 516600.0,
    520800.0, 525100.0, 529300.0, 533600.0, 537900.0, 542100.0, 546400.0, 554900.0, 563500.0,
    572000.0, 580600.0, 589200.0, 597700.0, 606300.0, 614900.0, 623600.0, 632200.0, 640800.0,
    649500.0, 658100.0, 666800.0, 675500.0, 684200.0, 692900.0, 701600.0, 710300.0, 719100.0,
    727800.0, 736600.0, 745400.0, 754200.0, 763100.0, 785200.0, 807400.0, 829800.0, 852300.0,
    874900.0, 897600.0, 920500.0, 943600.0, 966800.0, 990200.0, 1038000.0, 1086000.0, 1135000.0,
    1185000.0, 1237000.0, 1290000.0, 1345000.0, 1402000.0, 1462000.0, 1526000.0, 1595000.0,
    1671000.0, 1762000.0, 1891000.0,
];

/// `DefaultSteamEnthalpyGasFluidData` from `FluidProperties.cc` (111 values).
const STEAM_HFG: [f64; 111] = [
    2501000.0, 2503000.0, 2510000.0, 2519000.0, 2528000.0, 2537000.0, 2547000.0, 2556000.0,
    2565000.0, 2574000.0, 2582000.0, 2591000.0, 2600000.0, 2609000.0, 2618000.0, 2626000.0,
    2630000.0, 2633000.0, 2636000.0, 2640000.0, 2643000.0, 2646000.0, 2650000.0, 2653000.0,
    2656000.0, 2660000.0, 2663000.0, 2666000.0, 2669000.0, 2672000.0, 2674000.0, 2676000.0,
    2677000.0, 2679000.0, 2680000.0, 2682000.0, 2683000.0, 2685000.0, 2686000.0, 2688000.0,
    2690000.0, 2691000.0, 2693000.0, 2694000.0, 2696000.0, 2697000.0, 2699000.0, 2700000.0,
    2702000.0, 2703000.0, 2704000.0, 2706000.0, 2707000.0, 2709000.0, 2710000.0, 2712000.0,
    2713000.0, 2715000.0, 2716000.0, 2717000.0, 2719000.0, 2720000.0, 2723000.0, 2726000.0,
    2728000.0, 2731000.0, 2733000.0, 2736000.0, 2739000.0, 2741000.0, 2744000.0, 2746000.0,
    2748000.0, 2751000.0, 2753000.0, 2755000.0, 2757000.0, 2760000.0, 2762000.0, 2764000.0,
    2766000.0, 2768000.0, 2770000.0, 2772000.0, 2774000.0, 2775000.0, 2777000.0, 2781000.0,
    2785000.0, 2789000.0, 2792000.0, 2795000.0, 2797000.0, 2799000.0, 2801000.0, 2802000.0,
    2803000.0, 2803000.0, 2801000.0, 2797000.0, 2790000.0, 2780000.0, 2767000.0, 2750000.0,
    2728000.0, 2701000.0, 2666000.0, 2622000.0, 2564000.0, 2481000.0, 2335000.0,
];

/// `DefaultSteamCpFluidData` from `FluidProperties.cc` (111 values).
const STEAM_CPF: [f64; 111] = [
    4220.0, 4217.0, 4205.0, 4196.0, 4189.0, 4184.0, 4182.0, 4180.0, 4180.0, 4180.0, 4180.0, 4182.0,
    4183.0, 4185.0, 4187.0, 4190.0, 4191.0, 4193.0, 4194.0, 4195.0, 4197.0, 4198.0, 4200.0, 4202.0,
    4203.0, 4205.0, 4207.0, 4209.0, 4211.0, 4213.0, 4215.0, 4216.0, 4217.0, 4218.0, 4219.0, 4220.0,
    4222.0, 4223.0, 4224.0, 4226.0, 4227.0, 4228.0, 4230.0, 4231.0, 4233.0, 4234.0, 4236.0, 4237.0,
    4239.0, 4240.0, 4242.0, 4244.0, 4245.0, 4247.0, 4249.0, 4250.0, 4252.0, 4254.0, 4256.0, 4258.0,
    4260.0, 4261.0, 4265.0, 4270.0, 4274.0, 4278.0, 4283.0, 4287.0, 4292.0, 4297.0, 4302.0, 4307.0,
    4312.0, 4318.0, 4324.0, 4329.0, 4335.0, 4341.0, 4348.0, 4354.0, 4361.0, 4368.0, 4375.0, 4382.0,
    4390.0, 4397.0, 4405.0, 4425.0, 4447.0, 4471.0, 4496.0, 4523.0, 4551.0, 4582.0, 4615.0, 4650.0,
    4688.0, 4772.0, 4870.0, 4986.0, 5123.0, 5289.0, 5493.0, 5750.0, 6085.0, 6537.0, 7186.0, 8208.0,
    10120.0, 15000.0, 45160.0,
];

/// `DefaultSteamCpGasFluidData` from `FluidProperties.cc` (111 values).
const STEAM_CPFG: [f64; 111] = [
    1884.0, 1885.0, 1889.0, 1895.0, 1900.0, 1906.0, 1912.0, 1918.0, 1925.0, 1931.0, 1939.0, 1947.0,
    1955.0, 1965.0, 1975.0, 1986.0, 1991.0, 1996.0, 2001.0, 2006.0, 2012.0, 2018.0, 2024.0, 2030.0,
    2036.0, 2043.0, 2050.0, 2057.0, 2064.0, 2072.0, 2076.0, 2080.0, 2084.0, 2088.0, 2093.0, 2097.0,
    2101.0, 2106.0, 2110.0, 2115.0, 2120.0, 2124.0, 2129.0, 2134.0, 2139.0, 2144.0, 2150.0, 2155.0,
    2160.0, 2166.0, 2171.0, 2177.0, 2183.0, 2189.0, 2195.0, 2201.0, 2207.0, 2213.0, 2219.0, 2226.0,
    2232.0, 2239.0, 2252.0, 2266.0, 2281.0, 2296.0, 2311.0, 2327.0, 2343.0, 2359.0, 2376.0, 2394.0,
    2412.0, 2430.0, 2449.0, 2468.0, 2488.0, 2509.0, 2529.0, 2551.0, 2572.0, 2594.0, 2617.0, 2640.0,
    2664.0, 2688.0, 2713.0, 2777.0, 2844.0, 2915.0, 2990.0, 3068.0, 3150.0, 3237.0, 3329.0, 3426.0,
    3528.0, 3754.0, 4011.0, 4308.0, 4656.0, 5073.0, 5582.0, 6220.0, 7045.0, 8159.0, 9753.0,
    12240.0, 16690.0, 27360.0, 96600.0,
];

/// `DefaultSteamDensityFluidData` from `FluidProperties.cc` (111 values).
const STEAM_RHOF: [f64; 111] = [
    999.8, 999.9, 999.9, 999.7, 999.1, 998.2, 997.0, 995.6, 994.0, 992.2, 990.2, 988.0, 985.7,
    983.2, 980.5, 977.7, 976.6, 975.4, 974.2, 973.0, 971.8, 970.5, 969.2, 967.9, 966.6, 965.3,
    963.9, 962.6, 961.2, 959.8, 959.1, 958.3, 957.6, 956.9, 956.2, 955.4, 954.7, 954.0, 953.2,
    952.5, 951.7, 950.9, 950.2, 949.4, 948.6, 947.9, 947.1, 946.3, 945.5, 944.7, 943.9, 943.1,
    942.3, 941.5, 940.7, 939.8, 939.0, 938.2, 937.4, 936.5, 935.7, 934.8, 933.1, 931.4, 929.7,
    927.9, 926.1, 924.3, 922.5, 920.7, 918.9, 917.0, 915.1, 913.2, 911.3, 909.4, 907.4, 905.5,
    903.5, 901.5, 899.5, 897.5, 895.4, 893.3, 891.2, 889.1, 887.0, 881.6, 876.1, 870.4, 864.7,
    858.8, 852.7, 846.5, 840.2, 833.7, 827.1, 813.4, 798.9, 783.6, 767.5, 750.3, 731.9, 712.1,
    690.7, 667.1, 640.8, 610.7, 574.7, 527.6, 451.4,
];

/// `DefaultSteamDensityGasFluidData` from `FluidProperties.cc` (111 values).
const STEAM_RHOFG: [f64; 111] = [
    4.86e-003, 5.20e-003, 6.80e-003, 9.41e-003, 1.28e-002, 1.73e-002, 2.31e-002, 3.04e-002,
    3.97e-002, 5.12e-002, 6.56e-002, 8.32e-002, 0.10, 0.13, 0.16, 0.20, 0.22, 0.23, 0.25, 0.27,
    0.29, 0.32, 0.34, 0.37, 0.39, 0.42, 0.45, 0.49, 0.52, 0.56, 0.58, 0.60, 0.62, 0.64, 0.66, 0.68,
    0.71, 0.73, 0.75, 0.78, 0.80, 0.83, 0.85, 0.88, 0.91, 0.94, 0.97, 1.00, 1.03, 1.06, 1.09, 1.12,
    1.16, 1.19, 1.23, 1.26, 1.30, 1.34, 1.38, 1.42, 1.46, 1.50, 1.58, 1.67, 1.77, 1.86, 1.97, 2.07,
    2.19, 2.30, 2.42, 2.55, 2.68, 2.82, 2.96, 3.11, 3.26, 3.42, 3.59, 3.76, 3.94, 4.12, 4.32, 4.52,
    4.72, 4.94, 5.16, 5.75, 6.40, 7.10, 7.86, 8.69, 9.59, 10.56, 11.62, 12.75, 13.99, 16.75, 19.97,
    23.71, 28.07, 33.16, 39.13, 46.17, 54.54, 64.64, 77.05, 92.76, 113.60, 143.90, 201.80,
];

/// The 1-based element `i` of an `ObjexxFCL` `Array1D`.
fn at(a: &[f64], i: usize) -> f64 {
    a[i - 1]
}

/// The 1-based index of the first value above zero, or 0 if there is none (`setTempLimits`).
const fn low_index(values: &[f64]) -> usize {
    let mut i = 0;
    while i < values.len() {
        if values[i] > 0.0 {
            return i + 1;
        }
        i += 1;
    }
    0
}

/// The 1-based index of the last value above zero, or 0 if there is none (`setTempLimits`).
const fn high_index(values: &[f64]) -> usize {
    let mut i = values.len();
    while i >= 1 {
        if values[i - 1] > 0.0 {
            return i;
        }
        i -= 1;
    }
    0
}

/// One property of the `WATER` glycol after `GlycolProps::setTempLimits`.
struct GlycolProp {
    values: &'static [f64; 33],
    /// `*LowTempIndex` and `*LowTempValue`.
    low_index: usize,
    low_temp: f64,
    /// `*HighTempIndex` and `*HighTempValue`.
    high_index: usize,
    high_temp: f64,
}

impl GlycolProp {
    const fn new(values: &'static [f64; 33]) -> Self {
        let low_index = low_index(values);
        let high_index = high_index(values);
        Self {
            values,
            low_index,
            low_temp: GLYCOL_TEMPS[low_index - 1],
            high_index,
            high_temp: GLYCOL_TEMPS[high_index - 1],
        }
    }

    /// The `getDensity` / `getConductivity` / `getViscosity` body (non-`PERFORMANCE_OPT`).
    fn get(&self, Temp: f64) -> f64 {
        let v = self.values;
        if Temp < self.low_temp {
            at(v, self.low_index)
        } else if Temp > self.high_temp {
            at(v, self.high_index)
        } else {
            let t = &GLYCOL_TEMPS;
            let lo = FindArrayIndex(Temp, t, 1, t.len());
            let TempInterpRatio = (Temp - at(t, lo)) / (at(t, lo + 1) - at(t, lo));
            at(v, lo) + TempInterpRatio * (at(v, lo + 1) - at(v, lo))
        }
    }
}

const WATER_CP_PROP: GlycolProp = GlycolProp::new(&WATER_CP);
const WATER_RHO_PROP: GlycolProp = GlycolProp::new(&WATER_RHO);
const WATER_COND_PROP: GlycolProp = GlycolProp::new(&WATER_COND);
const WATER_VISC_PROP: GlycolProp = GlycolProp::new(&WATER_VISC);

// `RefrigProps::setTempLimits` for `STEAM`. Every steam value is above zero, so each low index
// is 1 and each high index 111; they are still derived the way the original does.
const PS_LOW: usize = low_index(&STEAM_PRESS);
const PS_HIGH: usize = high_index(&STEAM_PRESS);
const HF_LOW: usize = low_index(&STEAM_HF);
const HF_HIGH: usize = high_index(&STEAM_HF);
const CPF_LOW: usize = low_index(&STEAM_CPF);
const CPF_HIGH: usize = high_index(&STEAM_CPF);
const RHOF_LOW: usize = low_index(&STEAM_RHOF);
const RHOF_HIGH: usize = high_index(&STEAM_RHOF);

/// `GetInterpValue` (`FluidProperties.hh`).
fn GetInterpValue(Tact: f64, Tlo: f64, Thi: f64, Xlo: f64, Xhi: f64) -> f64 {
    Xhi - (((Thi - Tact) / (Thi - Tlo)) * (Xhi - Xlo))
}

/// `FindArrayIndex(Value, Array, LowBound, UpperBound)` for a 1-based array (`l() == 1`).
fn FindArrayIndex(Value: f64, Array: &[f64], LowBound: usize, UpperBound: usize) -> usize {
    let mut beg = LowBound - 1;
    if Value < Array[beg] {
        return 0;
    }
    let mut end = UpperBound - 1;
    if Value > Array[end] {
        return UpperBound;
    }
    while beg + 1 < end {
        let mid = (beg + end) >> 1;
        if Value > Array[mid] {
            beg = mid;
        } else {
            end = mid;
        }
    }
    1 + beg
}

/// `GetInterpolatedSatProp`.
fn GetInterpolatedSatProp(
    Temperature: f64,
    PropTemps: &[f64],
    LiqProp: &[f64],
    VapProp: &[f64],
    Quality: f64,
    LowBound: usize,
    UpperBound: usize,
) -> f64 {
    let LoTempIndex = FindArrayIndex(Temperature, PropTemps, LowBound, UpperBound);
    if LoTempIndex == 0 {
        let liq = at(LiqProp, LowBound);
        liq + Quality * (at(VapProp, LowBound) - liq)
    } else if LoTempIndex >= UpperBound {
        let liq = at(LiqProp, UpperBound);
        liq + Quality * (at(VapProp, UpperBound) - liq)
    } else {
        let HiTempIndex = LoTempIndex + 1;
        let LiqProp_Lo = at(LiqProp, LoTempIndex);
        let LoSatProp = LiqProp_Lo + Quality * (at(VapProp, LoTempIndex) - LiqProp_Lo);
        let LiqProp_Hi = at(LiqProp, HiTempIndex);
        let HiSatProp = LiqProp_Hi + Quality * (at(VapProp, HiTempIndex) - LiqProp_Hi);
        let PropTemps_Lo = at(PropTemps, LoTempIndex);
        let TempInterpRatio =
            (Temperature - PropTemps_Lo) / (at(PropTemps, HiTempIndex) - PropTemps_Lo);
        LoSatProp + TempInterpRatio * (HiSatProp - LoSatProp)
    }
}

/// Port of `double WaterSpecificHeat(double Temperature)` (`fluidproperties.c:17`):
/// `GlycolProps::getSpecificHeat`, whose cache always misses in a fresh process (see the module
/// documentation).
pub fn WaterSpecificHeat(Temperature: f64) -> f64 {
    WaterSpecificHeat_raw(Temperature)
}

/// Port of `double WaterSpecificHeat_raw(double Temperature)` (`fluidproperties.c:22`):
/// `GlycolProps::getSpecificHeat_raw` (non-`PERFORMANCE_OPT`).
pub fn WaterSpecificHeat_raw(Temperature: f64) -> f64 {
    let (p, t) = (&WATER_CP_PROP, &GLYCOL_TEMPS);
    if Temperature < p.low_temp {
        return at(p.values, p.low_index);
    }
    if Temperature > p.high_temp {
        return at(p.values, p.high_index);
    }
    let (mut beg, mut end) = (1, t.len());
    while beg + 1 < end {
        let mid = (beg + end) >> 1;
        if Temperature > at(t, mid) {
            beg = mid;
        } else {
            end = mid;
        }
    }
    GetInterpValue(
        Temperature,
        at(t, beg),
        at(t, end),
        at(p.values, beg),
        at(p.values, end),
    )
}

/// Port of `double WaterDensity(double Temperature)` (`fluidproperties.c:27`): `getDensity`.
pub fn WaterDensity(Temperature: f64) -> f64 {
    WATER_RHO_PROP.get(Temperature)
}

/// Port of `double WaterConductivity(double Temperature)` (`fluidproperties.c:32`): `getConductivity`.
pub fn WaterConductivity(Temperature: f64) -> f64 {
    WATER_COND_PROP.get(Temperature)
}

/// Port of `double WaterViscosity(double Temperature)` (`fluidproperties.c:37`): `getViscosity`.
pub fn WaterViscosity(Temperature: f64) -> f64 {
    WATER_VISC_PROP.get(Temperature)
}

/// The shared body of `getSatPressure` and `getSatTemperature`: interpolate `ys` at `x` in `xs`.
/// The `Ps*TempIndex` and `Ps*PresIndex` limits are the same indices, so one pair serves both.
fn ps_interp(x: f64, xs: &[f64], ys: &[f64]) -> f64 {
    let lo = FindArrayIndex(x, xs, PS_LOW, PS_HIGH);
    if lo == 0 {
        at(ys, PS_LOW)
    } else if lo + 1 > PS_HIGH {
        at(ys, PS_HIGH)
    } else {
        let InterpRatio = (x - at(xs, lo)) / (at(xs, lo + 1) - at(xs, lo));
        at(ys, lo) + InterpRatio * (at(ys, lo + 1) - at(ys, lo))
    }
}

/// Port of `double SteamSatPressure(double Temperature)` (`fluidproperties.c:42`): `getSatPressure`.
pub fn SteamSatPressure(Temperature: f64) -> f64 {
    ps_interp(Temperature, &STEAM_TEMPS, &STEAM_PRESS)
}

/// Port of `double SteamSatTemperature(double Pressure)` (`fluidproperties.c:47`): `getSatTemperature`.
pub fn SteamSatTemperature(Pressure: f64) -> f64 {
    ps_interp(Pressure, &STEAM_PRESS, &STEAM_TEMPS)
}

/// Port of `double SteamSatEnthalpy(double Temperature, double Quality)` (`fluidproperties.c:52`):
/// `getSatEnthalpy`, which, unlike density and specific heat, does not check the quality.
pub fn SteamSatEnthalpy(Temperature: f64, Quality: f64) -> f64 {
    GetInterpolatedSatProp(
        Temperature,
        &STEAM_TEMPS,
        &STEAM_HF,
        &STEAM_HFG,
        Quality,
        HF_LOW,
        HF_HIGH,
    )
}

/// The specific volume `1 / rho_f + Quality * (1 / rho_g - 1 / rho_f)` at the 1-based index `i`.
fn sat_volume(i: usize, Quality: f64) -> f64 {
    let (f, g) = (at(&STEAM_RHOF, i), at(&STEAM_RHOFG, i));
    1.0 / f + Quality * (1.0 / g - 1.0 / f)
}

/// Port of `double SteamSatDensity(double Temperature, double Quality)` (`fluidproperties.c:57`):
/// `getSatDensity`. A quality outside `[0, 1]` is a fatal error in the original; here it is
/// interpolated like any other.
pub fn SteamSatDensity(Temperature: f64, Quality: f64) -> f64 {
    let t = &STEAM_TEMPS;
    let LoTempIndex = FindArrayIndex(Temperature, t, RHOF_LOW, RHOF_HIGH);
    let HiTempIndex = LoTempIndex + 1;
    if LoTempIndex == 0 {
        1.0 / sat_volume(RHOF_LOW, Quality)
    } else if HiTempIndex > RHOF_HIGH {
        1.0 / sat_volume(RHOF_HIGH, Quality)
    } else {
        let LoSatProp = sat_volume(LoTempIndex, Quality);
        let HiSatProp = sat_volume(HiTempIndex, Quality);
        let (t_lo, t_hi) = (at(t, LoTempIndex), at(t, HiTempIndex));
        let TempInterpRatio = (Temperature - t_lo) / (t_hi - t_lo);
        1.0 / (LoSatProp + TempInterpRatio * (HiSatProp - LoSatProp))
    }
}

/// Port of `double SteamSatSpecificHeat(double Temperature, double Quality)` (`fluidproperties.c:62`):
/// `getSatSpecificHeat`. A quality outside `[0, 1]` is a fatal error in the original; here it is
/// interpolated like any other.
pub fn SteamSatSpecificHeat(Temperature: f64, Quality: f64) -> f64 {
    GetInterpolatedSatProp(
        Temperature,
        &STEAM_TEMPS,
        &STEAM_CPF,
        &STEAM_CPFG,
        Quality,
        CPF_LOW,
        CPF_HIGH,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_have_the_original_sizes() {
        assert_eq!(GLYCOL_TEMPS.len(), 33);
        assert_eq!(STEAM_TEMPS.len(), 111);
        assert_eq!(GLYCOL_TEMPS[7], 0.0);
        assert_eq!(STEAM_TEMPS[0], 0.01);
        assert_eq!(STEAM_TEMPS[110], 370.0);
    }

    #[test]
    fn temp_limits_match_set_temp_limits() {
        // Water: Cp has data from 0 C to 125 C; density, conductivity and viscosity to 100 C.
        let cp = &WATER_CP_PROP;
        assert_eq!(
            (cp.low_index, cp.low_temp, cp.high_index, cp.high_temp),
            (8, 0.0, 33, 125.0)
        );
        for p in [&WATER_RHO_PROP, &WATER_COND_PROP, &WATER_VISC_PROP] {
            assert_eq!(
                (p.low_index, p.low_temp, p.high_index, p.high_temp),
                (8, 0.0, 28, 100.0)
            );
        }
        for (lo, hi) in [
            (PS_LOW, PS_HIGH),
            (HF_LOW, HF_HIGH),
            (CPF_LOW, CPF_HIGH),
            (RHOF_LOW, RHOF_HIGH),
        ] {
            assert_eq!((lo, hi), (1, 111));
        }
    }

    #[test]
    fn table_points_are_returned_exactly() {
        assert_eq!(WaterSpecificHeat(20.0), 4181.0);
        assert_eq!(WaterDensity(0.0), 999.8);
        assert_eq!(WaterConductivity(0.0), 0.561);
        assert_eq!(WaterViscosity(0.0), 1.7912e-3);
        assert_eq!(SteamSatPressure(100.0), 101400.0);
        assert_eq!(SteamSatTemperature(101400.0), 100.0);
        assert_eq!(SteamSatEnthalpy(100.0, 0.0), 419200.0);
        assert_eq!(SteamSatEnthalpy(100.0, 1.0), 2676000.0);
        assert_eq!(SteamSatSpecificHeat(100.0, 0.0), 4216.0);
    }

    #[test]
    fn interpolates_between_points_and_pegs_the_ends() {
        assert_eq!(WaterSpecificHeat(2.5), 4217.0 - 0.5 * (4217.0 - 4198.0));
        assert_eq!(WaterSpecificHeat(-10.0), 4217.0);
        assert_eq!(WaterSpecificHeat(200.0), 4243.0);
        assert_eq!(WaterDensity(150.0), 958.3);
        assert_eq!(SteamSatPressure(-50.0), 611.7);
        assert_eq!(SteamSatPressure(1000.0), 21040000.0);
        let mid = SteamSatPressure(99.5);
        assert!(mid > 97850.0 && mid < 101400.0);
    }

    #[test]
    fn nan_follows_the_comparisons() {
        assert!(WaterSpecificHeat(f64::NAN).is_nan());
        assert!(WaterDensity(f64::NAN).is_nan());
        assert!(SteamSatPressure(f64::NAN).is_nan());
        assert!(SteamSatEnthalpy(10.0, f64::NAN).is_nan());
    }

    #[test]
    fn density_is_the_inverse_of_the_interpolated_volume() {
        // At the first table point the temperature ratio is 0, so only the quality blend remains.
        let (f, g) = (999.8, 4.86e-3);
        assert_eq!(
            SteamSatDensity(0.01, 0.0),
            1.0 / (1.0 / f + 0.0 * (1.0 / g - 1.0 / f))
        );
        assert_eq!(
            SteamSatDensity(0.01, 1.0),
            1.0 / (1.0 / f + 1.0 * (1.0 / g - 1.0 / f))
        );
        // Below the table the lowest point is used.
        assert_eq!(SteamSatDensity(-5.0, 0.5), SteamSatDensity(0.01, 0.5));
    }
}
