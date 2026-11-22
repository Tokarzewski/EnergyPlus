# Decisions for the EnergyPlus Rust port

Short ADRs, numbered in order and never renumbered. A changed decision gets a new ADR that
supersedes the old one.

## ADR-001: Port the C++ engine to Rust, inside this fork
Status: accepted (2026-11-21)
Context: The goal is EnergyPlus in Rust. The fork Tokarzewski/EnergyPlus tracks upstream
NatLabRockies/EnergyPlus.
Decision: Port the C++ simulation engine (`src/EnergyPlus`) unit by unit with rust-anything. Every
crate lives under `rust/`; work is pushed to `origin` only, never upstream.
Consequences: The C++ stays as the reference and the oracle until the lift.

## ADR-002: Results must be bit-exact
Status: accepted (2026-11-21)
Context: Simulation results feed compliance and research work; small float drift compounds over a
year-long run.
Decision: Every recorded case must match the C++ bit for bit: same operation order, no FMA, the
same C library maths (`exp`, `log`, `pow`, `sin`, `cos` are glibc's, as in the C++). A case may
differ only when the owner accepts it, and then it is listed in the crate's `known-diffs.jsonl`
with the reason.
Consequences: No `--float-tolerance`. Where Rust has no function the C++ uses, see ADR-006.

## ADR-003: Independent of EnergyPlusMojo
Status: accepted (2026-11-21)
Decision: The Rust port does not share code, oracles or plans with the Mojo port.

## ADR-004: Oracles are thin C++ shims over the real EnergyPlus build
Status: accepted (2026-11-21)
Context: rust-anything bridges C units; EnergyPlus is C++ with an `EnergyPlusData` state.
Decision: Each unit has `rust/oracles/<unit>/`: a generator script writing a C++ shim
(`extern "C"` plain-number wrappers over the real functions, linked against the static EnergyPlus
libraries with `-O3 -ffp-contract=off` and the real defines) and a C forwarder that
rust-anything bridges. The shim builds its `EnergyPlusData` when the library loads, so forked
recorder children start from the same untouched state. Recorded inputs are realistic
(`real_inputs.py`) plus rust-anything's generated edge cases.
Consequences: Units must be reachable through plain numbers; methods get their object built from
plain numbers in the shim (see unit 7).

## ADR-005: Bottom-up, leaf numeric code first
Status: accepted (2026-11-21)
Decision: Start with Psychrometrics, then self-contained numeric modules whose inputs are plain
numbers, before code that reads or writes simulation state.

## ADR-006: libm for erf and erfc
Status: accepted (2026-11-22)
Context: Stable Rust has no `erf` or `erfc`. Porting glibc's is LGPL code; calling glibc's needs
`unsafe`.
Decision: Use the `libm` crate (MIT, 0.2.16). Cases where it rounds differently from glibc are
listed in `known-diffs.jsonl` (curves: 1 case; slinky: 23 cases).

## ADR-007: The size check may be waived when its limit comes from the forwarder
Status: accepted (2026-11-21)
Context: gate's size limit is 3x the C unit's lines plus 50, and the C unit is a forwarder much
shorter than the real C++.
Decision: The owner accepts a size failure in that case; it is recorded in the crate's README
(fluid properties, slinky). Every other check must pass.

## ADR-008: No `unsafe` in `port.rs`; no copied LGPL code
Status: accepted (2026-11-21)
Decision: `port.rs` is `#![forbid(unsafe_code)]`. The generated exports in `lib.rs` carry their
own `// SAFETY:` comments. glibc sources are not copied into the port.

## ADR-009: A unit's domain is where the C++ is defined
Status: accepted (2026-11-22)
Decision: Where the C++ would index outside its arrays or hit undefined behaviour, the shim and
the port both return NaN, and the oracle's generator says so. Inputs the recorder marks `crash`
are undefined behaviour and any result passes.

## ADR-010: Work without stopping between units
Status: accepted (2026-11-22)
Decision: Pick the next unit and the recommended answer to each trade-off without asking; commit
and push each unit to `origin` (branch `rust/psychrometrics`) once its gate passes. Ask only for
what the rules reserve to the owner: new `unsafe`, licence questions, anything destructive or
upstream-facing.
