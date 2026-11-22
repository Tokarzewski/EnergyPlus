"""Inputs for the ground heat exchanger's time stepping: runs from the first step through the
first hours (sub-hourly history only), past 207 hours (hourly history), and past 937 hours
(monthly aggregation), with 1 to 60 time steps an hour, flows that stop now and then, and
heating and cooling loads around the ground temperature. Every result is read after each run.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261125)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def borehole():
    d = rng.choice([0.1, 0.114, 0.15, rng.uniform(0.09, 0.18)])
    return [rng.choice([60.0, 76.2, 100.0, rng.uniform(40, 150)]), d, rng.uniform(0.3, 0.8) * d / 2 + 0.02,
            rng.choice([0.7, 1.0, 1.5]), rng.choice([1.5, 2.0, 2.5, rng.uniform(1.0, 3.5)]),
            rng.choice([2.0e6, 2.4e6, rng.uniform(1.6e6, 3.2e6)]), rng.choice([0.0267, 0.0334, 0.042]),
            rng.choice([0.0024, 0.003]), 0.39, rng.choice([1, 1, 4, 9, rng.randint(1, 40)])]


RUNS = [(1, 4), (2, 4), (3, 1), (5, 6), (24, 4), (60, 60), (61, 60), (100, 1), (250, 1), (230, 4), (300, 6),
        (900, 1), (960, 1), (1200, 1), (1500, 1), (2000, 2), (3000, 4), (4000, 4), (5000, 1), (6000, 6)]

for steps, ts in RUNS:
    for _ in range(2):
        tg = rng.choice([8.0, 10.0, 13.0, rng.uniform(5, 18)])
        run = [steps, ts, tg, tg + rng.choice([-6.0, -3.0, 4.0, 8.0, rng.uniform(-8, 12)]), rng.choice([0.0, 1.0, 3.0]),
               rng.choice([0.2, 0.5, 1.0, rng.uniform(0.05, 3.0)]), rng.choice([0, 0, 5, 17, 100]),
               rng.choice([0.6, 0.4, rng.uniform(0.2, 0.8)]), rng.choice([0.0, 0.002, -0.004]),
               rng.choice([0.0, 0.0, 0.0005])]
        bh = borehole()
        for which in range(5):
            out("GSRun", which, *run, *bh)

# Inlet swings so small that the glycol specific heat cache returns an earlier, nearby
# temperature's value (the key keeps 24 bits of the mantissa of T + 1000).
for amp in [1e-9, 3e-8, 1e-7, 1e-6]:
    run = [500, 4, 10.0, 20.0, amp, 0.5, 0, 0.6, 0.0, 0.0]
    for which in range(5):
        out("GSRun", which, *run, 100.0, 0.114, 0.05, 1.0, 2.0, 2.4e6, 0.0334, 0.003, 0.39, 1)
