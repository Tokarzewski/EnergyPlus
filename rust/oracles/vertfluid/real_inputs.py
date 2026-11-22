"""Inputs for the vertical ground heat exchanger's fluid-dependent code: pipes, grouts, soils and
flows as designers use them (laminar, the 2000-4000 blend and turbulent flow), water from just
above freezing to hot, one to many boreholes, and every short time step of a few fields.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261123)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def pipe():
    od = rng.choice([0.0267, 0.0334, 0.042, rng.uniform(0.02, 0.05)])
    return od, rng.choice([0.0024, 0.003, 0.0038, rng.uniform(0.002, 0.005)])


def flow():
    return rng.choice([0.0, 0.05, 0.1, 0.3, 0.5, 1.0, 2.0, rng.uniform(0.01, 3.0), rng.uniform(0.1, 30.0)])


def nbh():
    return rng.choice([1, 1, 2, 4, 9, 16, rng.randint(1, 100)])


def temp():
    return rng.choice([2.0, 10.0, 13.0, 20.0, 35.0, rng.uniform(-5, 80), rng.uniform(0, 30)])


def bore():
    d = rng.choice([0.1, 0.114, 0.15, rng.uniform(0.08, 0.2)])
    return [rng.choice([50.0, 76.2, 100.0, 150.0, rng.uniform(30, 200)]), d, rng.uniform(0.3, 0.8) * d / 2 + 0.02,
            rng.choice([0.7, 1.0, 1.5, rng.uniform(0.4, 3.0)])]


def soil():
    return rng.choice([1.5, 2.0, 2.5, rng.uniform(0.8, 4.0)]), rng.choice([2.0e6, 2.4e6, rng.uniform(1.5e6, 3.5e6)])


for _ in range(200):
    od, th = pipe()
    out("VFConvectionResistance", od, th, flow(), nbh(), temp())
for _ in range(150):
    od, th = pipe()
    out("VFPipeResistance", od, th, rng.choice([0.39, 0.4, rng.uniform(0.2, 0.6)]), flow(), nbh(), temp())
for _ in range(300):
    od, th = pipe()
    out("VFBoreholeResistance", rng.choice([0, 1, 2, 3, 3]), *bore(), soil()[0], od, th, rng.choice([0.39, 0.4]), flow(), nbh(), temp())
for _ in range(40):
    k, rc = soil()
    out("VFShortSteps", rng.choice([50.0, 100.0, 150.0, rng.uniform(30, 200)]), k, rc)

# Every step of a few short fields, then single steps of longer ones.
for i in range(8):
    od, th = pipe()
    b = bore()
    b[0] = rng.uniform(8, 25)
    k, rc = soil()
    rest = [rng.choice([3.9e6, rng.uniform(1e6, 4e6)]), k, rc, od, th, 0.39, rng.choice([1.8e6, 1.77e6]), flow(), nbh(), temp()]
    for step in [0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]:
        out("VFShortGFunction", step, *b, *rest)
        if i < 3:
            out("VFShortLntts", step, *b, *rest)
for _ in range(25):
    od, th = pipe()
    k, rc = soil()
    rest = [rng.uniform(1e6, 4e6), k, rc, od, th, 0.39, 1.8e6, flow(), nbh(), temp()]
    out("VFShortGFunction", rng.choice([0, 10, 100, 500, 1000]), *bore(), *rest)
