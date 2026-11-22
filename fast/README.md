# fast-anything harness for EnergyPlus

fast-anything's C/C++ mode only drives a compiler command it builds itself, so it cannot
profile the `energyplus` binary. This folder is a single-run harness made from verbatim copies of
EnergyPlus kernels (develop 7ab256b5a, `state` arguments dropped): General::Iterate, the Psy*
psychrometric functions and the ASHRAE/TARP, MoWITT, DOE-2 and Sparrow exterior convection
correlations. It runs a synthetic 5-zone year (6 timesteps per hour, 3 HVAC iterations, 12 exterior
surfaces per zone) on the Chicago O'Hare TMY3 file.

    g++ -O2 -std=c++17 main.cpp simulation.cpp ep_kernels.cpp -o ep_harness && ./ep_harness
    # 8760 hours, checksum 13654.679482061

## What fast-anything found hot

| function | share | calls |
|---|---|---|
| PsyTwbFnTdbWPb | 52% | 1.63M |
| CalcDOE2Forced | 9.4% | 2.37M |
| CalcDOE2Windward / Leeward | 7.3% / 7.1% | 1.19M / 1.18M |
| CalcMoWITTLeeward / Windward | 5.5% / 5.1% | 1.20M / 1.16M |

## What was ported

- `PsyTwbFnTdbWPb` (a scalar port in `.fast/ports/`): 547 of 547 recorded and edge cases are
  bit-identical and it is 1.1× faster per call. The run as a whole is 0.3× the speed (2.78 s to
  8.56 s), because fast-anything's C++ bridge passes JSON through dlopen on every call (about
  3–5 µs each). A batched form was 1.3× slower per call and was not enabled.
- `SimulateYear` in `rust/`: the whole annual loop, so the bridge is crossed once per run. The
  checksum is bit-identical to C++ (bits `40caab56f944a7d2`). It runs in about 190 ms against
  1.6–1.8 s for the C++ harness, about 9× faster on 24 cores.
  - **Why it is faster:** the zone states run in one sequential pass. The 60 surface histories
    and the 1.63M wet-bulb solves then run in parallel, and the boiling point is cached per
    pressure, as EnergyPlus also does.
  - **Why it stays bit-identical:** the checksum terms are added back in the C++ order.

  This is a harness result, not a full EnergyPlus run.

    cd rust && cargo run --release   # SIM_TIMING=1 prints each phase's time
