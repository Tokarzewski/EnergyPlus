# EnergyPlus to Rust: the map

**Goal:** the EnergyPlus simulation engine in Rust, bit-exact with the C++ (ADR-001, ADR-002).
The C++ in `src/EnergyPlus` stays as the reference and the oracle.

Read first: [DECISIONS.md](DECISIONS.md) (ADR-001 to ADR-010), then [HANDOFF.md](HANDOFF.md).
`rust-anything map --write` generates the measured state in `PORT_MAP.md`.

## Units

| # | Unit | Crate | C++ source | State | Cases | Notes |
|---|---|---|---|---|---|---|
| 1 | Psychrometrics | `psychrometrics-rs` | `Psychrometrics.cc` | done | 7844/7844 | |
| 2 | Water and steam properties | `fluidproperties-rs` | `FluidProperties.cc` | done | 1622/1622 | size waived (ADR-007); not ported: superheated steam (its tables are compiled out, `#ifdef UNUSED_FLUID_PROPS`) and `SteamQuality` (`RefrigProps::getQuality` is declared but never defined) |
| 3 | Curve evaluation (formula types) | `curves-rs` | `CurveManager.cc` | done | 7587/7587, 1 known diff | `erf` from libm (ADR-006); table curves not ported |
| 4 | Convection correlations | `convection-rs` | `ConvectionCoefficients.cc` | done | 7884/7884 | the overloads that read state stay in C++ |
| 5 | Sandia PV model | `sandia-pv-rs` | `Photovoltaics.cc` | done | 4107/4107 | |
| 6 | Extended heat index | `heatindex-rs` | `ExtendedHeatIndex.cc`, `General::SolveRoot` | done | 4125/4125 | only the `ShortBisectionThenRegulaFalsi` solver |
| 7 | Slinky ground heat exchanger g-functions | `slinky-rs` | `GroundHeatExchangers/Slinky.cc` | done | 708/708, 23 known diffs | `erfc` from libm (ADR-006); size waived (ADR-007) |
| 8 | General helpers | `general-rs` | `General.cc`, `General.hh` | done | 1986/1986 | dates, `SafeDivide`, `rotAzmDiffDeg`, `epexp`, `Interp` |
| 9 | Weather helpers | `weather-rs` | `WeatherManager.cc` | done | 1658/1658 | `GetSTM` folds longitudes past 180 by subtracting 180, kept as in the C++ |
| 10 | Thermal comfort helpers | `comfort-rs` | `ThermalComfort.cc` | done | 531/531 | PPD, relative air velocity, two saturation pressure fits |
| 11 | Equivalent-layer window helpers | `window-eql-rs` | `WindowEquivalentLayer.cc` | done | 2334/2334 | `FRA` uses `BCP` and `BVISC` for the quadratic terms, kept as in the C++ |
| 12 | Root finder | `solveroot-rs` | `General.cc` (`SolveRoot`, `SolveRoot2`) | done | 3441/3441 | all six `RootAlgo` methods; size waived (ADR-007); units 6 and later can call `solve_root` |
| 13 | Leaf helpers from eleven modules | `leaf-rs` | `WindowManager`, `WaterThermalTanks`, `TARCOGCommon`, `OutputReportTabular`, `IceThermalStorage`, `PlantUtilities`, `PhotovoltaicThermalCollectors`, `TranspiredCollector`, `UtilityRoutines`, `SwimmingPool`, `MicroCHPElectricGenerator` | done | 7386/7386 | 26 functions; `overallUfactorFromFilmsAndCond` left out (its class needs constructor arguments) |
| 14 | Vertical ground heat exchanger g-functions | `vert-rs` | `GroundHeatExchangers/Vertical.cc`, `ResponseFactors.cc` | done | 3118/3118, 21 known diffs | uniform heat flux g-functions, friction factor, pipe conduction; `erfc` from libm (ADR-006); size waived (ADR-007); the short-time-step g-functions and fluid-dependent resistances are unit 15 |
| 15 | Vertical ground heat exchanger resistances and short-time-step g-functions | `vert-fluid-rs` | `GroundHeatExchangers/Vertical.cc`, `Base.cc` (`TDMA`) | done | 2524/2524 | first unit on shim-built plant state (a water loop and an inlet node); uses `fluidproperties-rs` and `vert-rs`; size waived (ADR-007); exposes `short_series` for unit 16; not yet: the GHEDesigner path |
| 16 | Vertical ground heat exchanger g-function curve | `vert-gfunc-rs` | `GroundHeatExchangers/Vertical.cc` (`calcGFunctions`, `combineShortAndLongTimestepGFunctions`, `getGFunc`), `Base.cc` (`interpGFunc`) | done | 1841/1841 | fields up to 2 by 2; uses `vert-rs` and `vert-fluid-rs`; size waived (ADR-007) |
| 17 | Ground heat exchanger time stepping and load aggregation | `glhe-sim-rs` | `GroundHeatExchangers/Base.cc` (`calcGroundHeatExchanger`, `calcAggregateLoad`) | done | 601/601 | first unit that keeps state between time steps; copies the glycol specific heat cache; runs up to 6000 steps with a given g-function curve; size waived (ADR-007); speed a tie |
| 18 | Ground temperature models | `groundtemp-rs` | `GroundTemperatureModeling/` (Kusuda-Achenbach, Xing, four site tables) | done | 2252/2252 | `remainder` from libm (exact); size waived (ADR-007); not yet: the finite difference model and the input factories |
| 19 | Surface geometry helpers | `vectors-rs` | `Vectors.cc`, `Geometry/Plane.cc`, `Geometry/Vector3D.hh` | done | 9525/9525 | polygons of 3 to 5 vertices; size waived (ADR-007); not yet: `CalcPolyhedronVolume` |
| 20 | Interior radiation exchange | `intrad-rs` | `HeatBalanceIntRadExchange.cc` (view factors, `FixViewFactors`, `CalcScriptF`, `CalcMatrixInverse`, `CalcFMRT`, `CalcFp`) | done | 3331/3331 | box zones of up to 7 surfaces; size waived (ADR-007); not yet: `InitInteriorRadExchange` and `CalcInteriorRadExchange` (enclosure state) |
| 21 | Perez sky model and line clipper | `perezsky-rs` | `SolarShading.cc` (`AnisoSkyViewFactors`, `CLIPLINE`) | done | 6618/6618 | size waived (ADR-007); not yet: the polygon shadow engine and the rest of `SolarShading.cc` |
| 22 | Weather sun position and sky helpers | `solarpos-rs` | `WeatherManager.cc` (`CalcSkyEmissivity`, `ASHRAETauModel`, `CalculateDailySolarCoeffs`, `CalculateSunDirectionCosines`, `DetermineSunUpDown`, `WaterMainsTempFromCorrelation`, `DayltgLuminousEfficacy`) | done | 10812/10812 | uses `psychrometrics-rs` and `weather-rs`; size waived (ADR-007); speed a tie (sky emissivity 0.5x: the C++ caches saturation pressure) |
| 23 | Window glazing optics and solvers | `glazing-rs` | `WindowManager.cc` (`TransAndReflAtPhi`, `W5LsqFit`, `DiffuseAverage`, `ViewFac`, `InvertMatrix`, `LUdecomposition`, `LUsolution`, `Interpolate`) | done | 8305/8305 | uses `leaf-rs`; size waived (ADR-007); not yet: the window heat balance (`SolveForWindowTemperatures`, gas and shade flow) |
| 24 | TARCOG gas properties and solver | `tarcoggas-rs` | `TARCOGGasses90.cc` (`GASSES90`, `GassesLow`), `TARCOGCommon.cc` (`EquationsSolver`, `ludcmp`, `lubksb`) | done | 4685/4685 | size waived (ADR-007); not yet: the TARCOG layer heat balance (`TARCOGMain.cc`), deflection and output |
| 25 | Window gap gas routines | `wingas-rs` | `WindowManager.cc` (`WindowGasConductance`, `WindowGasPropertiesAtTemp`, `NusseltNumber`) | done | 4481/4481 | built-in gas table; size waived (ADR-007) |
| 26 | Venetian blind optics | `blindoptics-rs` | `WindowManager.cc` (`BlindOpticsDiffuse`, `BlindOpticsBeam`), `Material.cc` (`MaterialBlind::BeamBeamTrans`) | done | 7108/7108 | uses `glazing-rs`; size waived (ADR-007); not yet: `CalcWindowBlindProperties` (the slat and profile angle tables) |
| 27 | Screen and simple glazing optics | `matoptics-rs` | `Material.cc` (`CalcScreenTransmittance`, `GetRelativePhiTheta`, `NormalizePhiTheta`, `GetPhiThetaIndices`, `GetProfIndices`, `GetSlatIndicesInterpFac`, `MaterialGlass::SetupSimpleWindowGlazingSystem`) | done | 11750/11750 | size waived (ADR-007); not yet: the material input routines (`GetMaterialData`) |
| 28 | Shadow polygon overlap | `polyclip-rs` | `SolarShading.cc` (`HTRANS1`, `HTRANS0`, `CLIPPOLY`, `DeterminePolygonOverlap` with Sutherland-Hodgman) | done | 13585/13585 | size waived (ADR-007); not yet: `CLIPRECT` (SlaterBarsky), `INCLOS`/`INTCPT`/`ORDER` (the older clipper), the shadow projection |

## Next (plain-number leaf code, ADR-005)

Units 8 to 11 and 13 were generated with the shared plain-number oracle tools in `rust/oracles/`
(`gen_plain.py`, `plain_inputs.py`, `build_plain_shim.sh`): a unit folder holds `gen.args`,
`real_inputs.py` and a one-line `build_shim.sh`; `gen.args` may list several headers and
namespaces, comma-separated. The headers' plain-number functions are now nearly all ported.

- Units that read simulation state through a shim-built `EnergyPlusData` (unit 15 shows the
  pattern: the shim allocates the plant loops and nodes the code reads).
- The finite difference ground temperature model (`FiniteDifferenceGroundTemperatureModel.cc`,
  about 1000 lines; it simulates a year of weather).

## Not started

- **Front door** (whole-app step 4): EnergyPlus is a CLI (`energyplus -w <epw> -d <out> <idf>`).
  The `energyplus` executable is not built in the WSL clone yet (only `energyplusapi`), and no
  app-level cases are recorded. Wiring a unit means the C++ calls its Rust through the C ABI,
  switched at build time.
- Fortran tools (ExpandObjects, ReadVars and the others) and the Python API are out of scope for now.

## Open questions

- How app-level output is compared (ESO/CSV/SQL bytes, or values only): decide when the front
  door is built.
