# Port `ep_kernels.PsyTwbFnTdbWPbBatch` to Rust

You're in a Rust crate fast-anything generated for this: `/home/bart/github/EnergyPlus-fast/fast/.fast/ports/ep_kernels.PsyTwbFnTdbWPbBatch`.
Write `PsyTwbFnTdbWPbBatch` in `src/port.rs` so it behaves **exactly** like the C++ below, and runs fast.
`src/lib.rs` is generated glue (JSON in, JSON out through a C entry point): don't edit it.
Only edit files in this folder. Don't touch the C++ project.

## The original (ep_kernels.cpp:170)

```cpp
{
    std::vector<double> out(tdb.size());
    for (size_t i = 0; i < tdb.size(); ++i) {
        out[i] = PsyTwbFnTdbWPb(tdb[i], w[i], pb[i]);
    }
    return out;
}
```

## Helpers it calls

Port these as private Rust functions in src/port.rs (they stay in C++ for everyone else).

```cpp
{
    constexpr int itmax(100);
    double newW;
    double W;
    double ResultX = 0.0;
    double WBT;
    double error;
    double X1 = 0.0;
    double Y1 = 0.0;
    double Wstar;
    double PSatstar;
    int icvg = 0;

    W = dW;
    if (W < 0.0) {
        W = 1.0e-5;
    }
    double const tBoil = PsyTsatFnPb(Patm);
    WBT = TDB;
    for (int iter = 1; iter <= itmax; ++iter) {
        if (WBT >= (tBoil - 0.09)) {
            WBT = tBoil - 0.1;
        }
        PSatstar = PsyPsatFnTemp(WBT);
        Wstar = 0.62198 * PSatstar / (Patm - PSatstar);
        if (WBT >= 0.0) {
            newW = ((2501.0 - 2.326 * WBT) * Wstar - 1.006 * (TDB - WBT)) / (2501.0 + 1.86 * TDB - 4.186 * WBT);
        } else {
            newW = ((2830.0 - 0.24 * WBT) * Wstar - 1.006 * (TDB - WBT)) / (2830.0 + 1.86 * TDB - 2.1 * WBT);
        }
        error = W - newW;
        Iterate(ResultX, iconvTol, WBT, error, X1, Y1, iter, icvg);
        WBT = ResultX;
        if (icvg == 1) {
            break;
        }
    }
    double TWB = WBT;
    if (TWB > TDB) {
        TWB = TDB;
    }
    return TWB;
}
```

## Real calls

fast-anything recorded 365 real calls (out of 365 while the app ran). Arguments arrive as JSON. A few:

```
PsyTwbFnTdbWPbBatch([-12.116666666666664,20.294166666666666,21.264749999999996,20.294166666666666,21.26474999…  ->  [-12.972971392612193,14.282001540881504,14.647064696838596,…
PsyTwbFnTdbWPbBatch([-3.7166666666666672,18.391958044905625,19.552762240415067,18.391958044905625,19.55276224…  ->  [-4.6040352238530895,7.764659712526733,8.30597054082297,7.7…
PsyTwbFnTdbWPbBatch([0.0,18.78492784620032,19.90643506158029,18.78492784620032,19.90643506158029,18.784927846…  ->  [-0.2799196922932033,9.40812036091842,9.90111538060072,9.40…
PsyTwbFnTdbWPbBatch([-2.8,18.48958488331686,19.640626394985173,18.48958488331686,19.640626394985173,18.489584…  ->  [-3.6584748860098495,8.045768059748022,8.578875219528165,8.…
PsyTwbFnTdbWPbBatch([-7.116666666666667,18.02964548869146,19.226680939822316,18.02964548869146,19.22668093982…  ->  [-8.246587214255294,6.9831027892962565,7.555752877559688,6.…
PsyTwbFnTdbWPbBatch([-11.1,17.618725314792407,18.85685278331316,17.618725314792407,18.85685278331316,17.61872…  ->  [-11.914546338890824,6.045267256884309,6.654032203226729,6.…
PsyTwbFnTdbWPbBatch([-18.98333333333333,16.787200516558254,18.108480464902428,16.787200516558254,18.108480464…  ->  [-19.875349354763536,4.631412823458884,5.314840756133378,4.…
PsyTwbFnTdbWPbBatch([-21.1,16.563927093999443,17.907534384599497,16.563927093999443,17.907534384599497,16.563…  ->  [-21.642348801695185,4.552847818177887,5.252880611061267,4.…
```

## Rules

- Same result for every input, edge cases included: empty input, zero, negatives, non-ASCII text, very long input. verify tests these too.
- `int` is `i32`, `long long`/`int64_t` is `i64`, `size_t` is `u64`; unsigned types wrap. Use `wrapping_*` where the original can overflow.
- `std::string` arrives as a UTF-8 String: `size()` and `s[i]` are bytes, use `as_bytes()` for the same behavior.
- `std::vector` is Vec, `std::array<T, N>` is `[T; N]`, `std::map<std::string, T>` is BTreeMap (same key order), `std::pair` is a tuple.
- Return `Err(message)` where the original throws.
- Return `Err(message)` where the original would fail.
- Make it fast: no needless allocation, borrow, `with_capacity`, fast paths for common cases. Threads are fine if the result is the same.
- Keep the function name.

## Check

Run `cargo check` here. Then fast-anything builds the crate in release mode and replays all recorded calls against it
(`fast-anything verify ep_kernels.PsyTwbFnTdbWPbBatch`).
