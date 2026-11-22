# Handoff

Load the `rust-anything` skill, read [MAP.md](MAP.md) and [DECISIONS.md](DECISIONS.md).

## Where things stand

Units 1 to 7 are done (see MAP.md), each on branch `rust/psychrometrics` of
Tokarzewski/EnergyPlus. No unit is wired into the C++ yet.

## Environment

- Work in the WSL clone `~/github/EnergyPlus` (distro `Ubuntu-24.04`), not the CRLF Windows checkout.
- The EnergyPlus build is in `build/` (`ninja energyplusapi`, Release, no Python, no Fortran);
  the shims link its static libraries from `build/Products`.
- Each unit: `rust/oracles/<unit>/build_shim.sh` builds the shim and prints the `CFLAGS` that
  `rust-anything record`, `build`, `diff` and `gate` need.
- From Windows, run WSL commands as script files (`wsl -d Ubuntu-24.04 -- bash /mnt/c/.../x.sh`),
  not inline `bash -c`; never write into the clone through `\\wsl.localhost`.

## Traps already met

- `rust-anything bridge` detects Python if the oracle folder has `.py` files: move them aside
  while bridging.
- libstdc++ `std::reduce` sums four at a time; `std::clamp` with `lo > hi` returns `hi`;
  ObjexxFCL `max` and `min` are `a < b ? b : a` and `a < b ? a : b`.
- LLVM may turn `pow(10, x)` into `exp10`: keep the base opaque (`std::hint::black_box`).
- `diff --json` lists at most 20 mismatches: rerun after adding known diffs.
- Decls hidden from headers (`find_eqvar_dTcdt`) can be declared in the shim.
- Some declared functions are never defined (`RefrigProps::getQuality`): the shim fails at load.
