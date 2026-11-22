"""Inputs for the vertical ground heat exchanger's combined g-functions: one to four boreholes
of designers' sizes on a water loop, every part of the combined curve (short steps, the seam
where the long steps take over, the long steps) and getGFunc from a minute to the end of the
simulation, with the field's own reference ratio and another one.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261124)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def counts(f):
    """The short steps and long LNTTS values the field makes (as the shim counts them)."""
    L, k, rc, years = f[3], f[9], f[10], f[17]
    t_s = (L * L) / (9 * (k / rc))
    t_max, total, n = math.exp(-8.6) * t_s, 0.0, 0
    while total < t_max:
        total += 120
        n += 1
    lntts, m = -8.5, 1
    while math.exp(lntts) * t_s < years * 365 * 24.0 * 3600.0:
        lntts += 0.5
        m += 1
    return n, m, t_s


def field():
    nx, ny = rng.choice([(1, 1), (1, 1), (1, 1), (2, 1), (1, 2), (2, 2)])
    d = rng.choice([0.1, 0.114, 0.15, rng.uniform(0.09, 0.18)])
    od = rng.choice([0.0267, 0.0334, 0.042])
    return [nx, ny, rng.choice([4.0, 6.0, rng.uniform(3, 8)]), rng.choice([40.0, 60.0, 76.2, 100.0, rng.uniform(30, 120)]),
            rng.choice([1.0, 2.0]), d, rng.uniform(0.3, 0.8) * d / 2 + 0.02, rng.choice([0.7, 1.0, 1.5, rng.uniform(0.5, 2.5)]),
            rng.choice([3.9e6, rng.uniform(1e6, 4e6)]), rng.choice([1.5, 2.0, 2.5, rng.uniform(1.0, 3.5)]),
            rng.choice([2.0e6, 2.4e6, rng.uniform(1.6e6, 3.2e6)]), od, rng.choice([0.0024, 0.003]), 0.39, 1.8e6,
            rng.choice([0.1, 0.3, 0.5, 1.0, rng.uniform(0.05, 2.0)]), rng.choice([10.0, 13.0, 20.0, rng.uniform(2, 30)]),
            rng.choice([1.0, 5.0, 10.0, 20.0, 25.0])]


for _ in range(14):
    f = field()
    n, m, t_s = counts(f)
    out("VGCombinedLen", *f)
    ks = sorted({0, 1, 7, n // 2, n - 1, n, n + 1, n + m // 2, n + m - 2, n + m - 1, n + m})
    for k in ks:
        out("VGCombinedGFunction", k, *f)
    for k in rng.sample(ks, 4):
        out("VGCombinedLntts", k, *f)
    end = f[17] * 365 * 24 * 3600.0
    for t in [60.0, 120.0, 3600.0, math.exp(-8.6) * t_s, math.exp(-8.5) * t_s, 8.64e4, 2.6e6, 3.15e7, end, 2 * end,
              rng.uniform(120, end)]:
        out("VGGFunc", t, rng.choice([0.0, 0.0, 0.0005, f[5] / 2 / f[3]]), *f)
