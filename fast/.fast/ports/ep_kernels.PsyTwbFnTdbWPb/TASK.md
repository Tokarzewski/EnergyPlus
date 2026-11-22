# Port `ep_kernels.PsyTwbFnTdbWPb` to Rust

You're in a Rust crate fast-anything generated for this: `/home/bart/github/EnergyPlus-fast/fast/.fast/ports/ep_kernels.PsyTwbFnTdbWPb`.
Write `PsyTwbFnTdbWPb` in `src/port.rs` so it behaves **exactly** like the C++ below, and runs fast.
`src/lib.rs` is generated glue (JSON in, JSON out through a C entry point): don't edit it.
Only edit files in this folder. Don't touch the C++ project.

## The original (ep_kernels.cpp:120)

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

## Helpers it calls

Port these as private Rust functions in src/port.rs (they stay in C++ for everyone else).

```cpp
{
    constexpr double small(1.e-9);
    constexpr double Perturb(0.1);
    if (Iter != 1) {
        if (std::abs(X0 - X1) < Tol || Y0 == 0.0) {
            ResultX = X0;
            Cnvg = 1;
            return;
        }
    }
    Cnvg = 0;
    if (Iter == 1) {
        if (std::abs(X0) > small) {
            ResultX = X0 * (1.0 + Perturb);
        } else {
            ResultX = Perturb;
        }
    } else {
        double DY = Y0 - Y1;
        if (std::abs(DY) < small) {
            DY = small;
        }
        ResultX = (Y0 * X1 - Y1 * X0) / DY;
    }
    X1 = X0;
    Y1 = Y0;
}
```

```cpp
{
    double Pascal;
    double const Tkel(T + Kelvin);
    if (Tkel < 173.15) {
        Pascal = 0.001405102123874164;
    } else if (Tkel < TriplePointOfWaterTempKelvin) {
        constexpr double C1(-5674.5359);
        constexpr double C2(6.3925247);
        constexpr double C3(-0.9677843e-2);
        constexpr double C4(0.62215701e-6);
        constexpr double C5(0.20747825e-8);
        constexpr double C6(-0.9484024e-12);
        constexpr double C7(4.1635019);
        Pascal = std::exp(C1 / Tkel + C2 + Tkel * (C3 + Tkel * (C4 + Tkel * (C5 + C6 * Tkel))) + C7 * std::log(Tkel));
    } else if (Tkel <= 473.15) {
        constexpr double C8(-5800.2206);
        constexpr double C9(1.3914993);
        constexpr double C10(-0.048640239);
        constexpr double C11(0.41764768e-4);
        constexpr double C12(-0.14452093e-7);
        constexpr double C13(6.5459673);
        Pascal = std::exp(C8 / Tkel + C9 + Tkel * (C10 + Tkel * (C11 + Tkel * C12)) + C13 * std::log(Tkel));
    } else {
        Pascal = 1555073.745636215;
    }
    return Pascal;
}
```

```cpp
{
    constexpr int itmax(50);
    constexpr double convTol(0.0001);
    double tSat = 100.0;
    if (Press >= 1555000.0) {
        tSat = 200.0;
    } else if (Press <= 0.0017) {
        tSat = -100.0;
    } else if ((Press > 611.000) && (Press < 611.25)) {
        tSat = 0.0;
    } else {
        double pSat;
        double error;
        double X1 = 0.0;
        double Y1 = 0.0;
        double ResultX = 0.0;
        int icvg = 0;
        for (int iter = 1; iter <= itmax; ++iter) {
            pSat = PsyPsatFnTemp(tSat);
            error = Press - pSat;
            Iterate(ResultX, convTol, tSat, error, X1, Y1, iter, icvg);
            tSat = ResultX;
            if (icvg == 1) {
                break;
            }
        }
    }
    return tSat;
}
```

## Real calls

fast-anything recorded 500 real calls (out of 1,629,360 while the app ran). Arguments arrive as JSON. A few:

```
PsyTwbFnTdbWPb(19.14160879730096, 0.0018754816263399344, 99500.0)  ->  7.567427660399698
PsyTwbFnTdbWPb(19.776375672716267, 0.008100545106094544, 98183.33333333333)  ->  14.17811758535158
PsyTwbFnTdbWPb(22.43706856269472, 0.013626918641554598, 98600.0)  ->  19.663232193865312
PsyTwbFnTdbWPb(19.811058401887603, 0.001928277796579312, 100083.33333333331)  ->  7.991087557640216
PsyTwbFnTdbWPb(22.67655287687071, 0.007479515107155125, 98300.0)  ->  14.708913625435112
PsyTwbFnTdbWPb(18.90101660362111, 0.003915334653362226, 100500.0)  ->  9.850318020949747
PsyTwbFnTdbWPb(21.869974378124063, 0.014034985352789865, 99083.33333333333)  ->  19.842925626502435
PsyTwbFnTdbWPb(19.63873760545111, 0.0025849912472454698, 98900.0)  ->  8.571228786754427
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
(`fast-anything verify ep_kernels.PsyTwbFnTdbWPb`).
