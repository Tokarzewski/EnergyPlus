"""Inputs for the vertical ground heat exchanger: borehole fields of one to nine boreholes,
lengths, depths and soils as designers use them, the whole g-function time range, points on and
off the borehole wall, and Reynolds numbers through the laminar-turbulent blend.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261122)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def soil():
    return rng.choice([5e-7, 7.5e-7, 1e-6, 1.5e-6, rng.uniform(4e-7, 2e-6)])


def times():
    return rng.choice([3600.0, 8.6e4, 3.15e7, 3.15e8])


for _ in range(150):
    p = [rng.uniform(-10, 10) for _ in range(6)]
    if rng.random() < 0.2:
        p[3:5] = p[0:2]
    out("VertDistance", *p, rng.choice([0, 1, 1, 0, 2, -1]))

for _ in range(150):
    d0 = rng.choice([0.0, 0.05, rng.uniform(0.01, 50)])
    out("VertResponse", d0, d0 + rng.uniform(0, 300), soil(), rng.choice([1.0, 3600.0, 8.6e4, 3.15e7, rng.uniform(1, 3e9)]))

for _ in range(80):
    L, top = rng.choice([50.0, 100.0, 150.0, rng.uniform(20, 200)]), rng.choice([1.0, 2.0, rng.uniform(0, 5)])
    out("VertIntegral", rng.uniform(-10, 10), rng.uniform(-10, 10), rng.uniform(top, top + L), rng.uniform(-10, 10),
        rng.uniform(-10, 10), L, top, rng.choice([0.1, 0.15, 0.2]), soil(), times())

for _ in range(80):
    L, top = rng.choice([50.0, 100.0, 150.0, rng.uniform(20, 200)]), rng.choice([1.0, 2.0, rng.uniform(0, 5)])
    out("VertDoubleIntegral", 0.0, 0.0, rng.choice([3.0, 6.0, rng.uniform(2, 10)]), rng.choice([0.0, rng.uniform(0, 10)]),
        rng.choice([0, 1]), L, top, rng.choice([0.1, 0.15, 0.2]), soil(), times())

for _ in range(60):
    args = [rng.choice([50.0, 100.0, 150.0, 0.0, rng.uniform(20, 200)]), soil(), rng.choice([1.0, 10.0, 25.0, 0.0, -1.0])]
    out("VertNumLntts", *args)
    out("VertLntts", *args, rng.choice([0, 1, 5, 10, 18, 40]))

for _ in range(70):
    nx, ny = rng.choice([(1, 1), (1, 1), (2, 1), (2, 2), (3, 1), (3, 2), (3, 3), (4, 1), (0, 1)])
    out("VertGFunction", nx, ny, rng.choice([4.0, 6.0, rng.uniform(3, 10)]), rng.choice([60.0, 100.0, rng.uniform(30, 200)]),
        rng.choice([1.0, 2.0]), rng.choice([0.1, 0.15]), soil(), rng.choice([1.0, 10.0, 20.0]), rng.choice([0, 3, 7, 10, 14, 18, 30]))

for re in [0.0, 1.0, 100.0, 1499.0, 1500.0, 2000.0, 3000.0, 4999.0, 5000.0, 1e4, 1e5, 1e6, -1.0]:
    out("VertFrictionFactor", re)
for _ in range(60):
    out("VertFrictionFactor", rng.uniform(100, 2e4))
for _ in range(60):
    od = rng.choice([0.0267, 0.0334, 0.042, rng.uniform(0.02, 0.05)])
    out("VertPipeConductionResistance", od, od - 2 * rng.uniform(0.002, 0.005), rng.choice([0.39, 0.4, rng.uniform(0.2, 0.6)]))
for _ in range(60):
    out("VertTimeSS", rng.choice([50.0, 100.0, 150.0, rng.uniform(20, 200), 0.0]), soil())
