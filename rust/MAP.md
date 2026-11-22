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

## Next (plain-number leaf code, ADR-005)

Units 8 to 11 were generated with the shared plain-number oracle tools in `rust/oracles/`
(`gen_plain.py`, `plain_inputs.py`, `build_plain_shim.sh`): a unit folder holds `gen.args`,
`real_inputs.py` and a one-line `build_shim.sh`.

- The rest of `General::SolveRoot` (the other solver methods).
- Ground heat exchanger response factors.
- Units that read simulation state through a shim-built `EnergyPlusData`.

## Not started

- **Front door** (whole-app step 4): EnergyPlus is a CLI (`energyplus -w <epw> -d <out> <idf>`).
  The `energyplus` executable is not built in the WSL clone yet (only `energyplusapi`), and no
  app-level cases are recorded. Wiring a unit means the C++ calls its Rust through the C ABI,
  switched at build time.
- Fortran tools (ExpandObjects, ReadVars and the others) and the Python API are out of scope for now.

## Open questions

- How app-level output is compared (ESO/CSV/SQL bytes, or values only): decide when the front
  door is built.
