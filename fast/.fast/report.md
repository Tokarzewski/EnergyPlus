# ⚡ fast: 0.3× faster

`g++ -O2 -std=c++17 main.cpp ep_kernels.cpp` went from **2.78s** to **8.56s** (median of 5 runs each, original and Rust interleaved).

![fast-anything report](report.svg)

| function | per call | checked against |
|---|---|---|
| `ep_kernels.PsyTwbFnTdbWPb` | **1.1×** (946ns → 900ns) | 547/547 recorded real calls |

The original code stays in place: set `FAST_ANYTHING_DISABLE=1` to run it, or `fast-anything disable <id>` to remove the switch.

<sub>Made with [fast-anything](https://github.com/EL4CTEO/fast-anything)</sub>
