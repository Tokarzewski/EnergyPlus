"""Inputs for SolveRoot and SolveRoot2: each root algorithm and switch-over count on cubic,
exponential and sine test functions, with brackets that hold a root, brackets that do not, and
tolerances and iteration limits from none to tight. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261122)
NAN = {"$num": "NaN"}
EPS = [1e-2, 1e-4, 1e-6, 1e-9, 1e-12, 0.0]
MAXITE = [0, 1, 2, 5, 10, 30, 100, 500, -1, 2000]
ALGO = [-1, 0, 1, 2, 3, 4, 5, 6, 7]
NUMOFITER = [0, 1, 3, 5, 10, -1, 2000]


def problem():
    """(fn, a, b, c, X_0, X_1), mostly bracketing a root."""
    fn = rng.choice([0, 0, 1, 2, 3])
    if fn == 0:
        a, b, r = rng.choice([0.0, 0.5, 1.0, -1.0, 2.0]), rng.choice([1.0, 3.0, -2.0, 0.0]), rng.uniform(-5, 5)
        c = -(a * r * r * r + b * r)
    elif fn == 1:
        a, b, c, r = rng.uniform(0.1, 2), rng.uniform(1, 10), rng.uniform(0, 1), 1.0
    elif fn == 2:
        a, b, c, r = rng.uniform(0.5, 3), rng.choice([0.0, 0.1, 1.0]), rng.uniform(-0.5, 0.5), 0.0
    else:
        a, b, c, r = rng.choice([0.0, 1.0, -1.0]), 0.0, 0.0, 0.0
    lo, hi = r - rng.uniform(0.01, 5), r + rng.uniform(0.01, 5)
    if rng.random() < 0.5:
        lo, hi = hi, lo
    if rng.random() < 0.1:
        lo, hi = r + 1, r + 3  # no root inside
    return [fn, a, b, c, lo, hi]


def special(args):
    """Now and then a NaN, an infinity or a zero-width bracket."""
    k = rng.random()
    if k < 0.03:
        args[rng.randrange(5, 10)] = NAN
    elif k < 0.05:
        args[rng.randrange(8, 10)] = {"$num": "Infinity"}
    elif k < 0.07:
        args[9] = args[8]
    return args


for _ in range(900):
    args = special([rng.choice(ALGO), rng.choice(NUMOFITER), rng.choice(EPS), rng.choice(MAXITE), *problem()])
    for fn in ("SolveRootX", "SolveRootFlag"):
        print(json.dumps({"fn": fn, "args": args}))

for _ in range(250):
    args = special([rng.choice([0, 1, 2, 3, 4, 5, -1, 6]), rng.choice([0, 1, 5, 29, 30, 31, 40, 60, 101]),
                    rng.choice(EPS[:4]), rng.choice([5, 10, 30, 100, 1001]), *problem()])
    for fn in ("SolveRoot2Algo", "SolveRoot2X"):
        print(json.dumps({"fn": fn, "args": args}))
