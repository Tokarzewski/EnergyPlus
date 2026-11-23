"""Inputs for HVACInterfaceManager's UpdateCommonPipe with a single common pipe as EnergyPlus meets
it: primary-secondary chilled and hot water loops of 0 to 50000 kg, with primary flows of 0 to 80
kg/s against secondary flows above, below or equal to them (some differing by less than the mass
flow tolerance, some off), side outlets at 4 to 80 C, pump heat, tank temperatures from this and
the last time step, the first call of a time step or a repeat, and system time steps of 1 to 60
minutes. All eight results are asked for each call.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261225)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1687):
    tin = pick(rng.uniform(4.0, 16.0), rng.uniform(40.0, 80.0))
    last = min(max(tin + rng.uniform(-6.0, 6.0), 1.0), 99.0)
    prev = pick(last, min(max(last + rng.uniform(-2.0, 2.0), 1.0), 99.0))
    mpri = pick(rng.uniform(0.0, 80.0), 0.0)
    msec = pick(mpri, mpri + 1.0e-10, max(mpri - 1.0e-10, 0.0), rng.uniform(0.0, 80.0), mpri * rng.uniform(0.5, 1.5), 0.0)
    args = [rng.randint(0, 1), rng.randint(0, 1), tin, msec, mpri, pick(0.0, rng.uniform(0.0, 5.0e4)),
            pick(rng.uniform(100.0, 50000.0), 0.0), last, prev, min(max(tin + rng.uniform(-8.0, 8.0), -20.0), 150.0),
            pick(60.0, 600.0, 900.0, rng.uniform(1.0, 3600.0))]
    for which in range(8):
        print(json.dumps({"fn": "CommonPipe", "args": [which] + args}))
for k in range(4):
    print(json.dumps({"fn": "CommonPipe", "args": [k, 0, 0, 12.0, 0.0, 0.0, 0.0, 0.0, 7.0, 7.0, 7.0, 600.0]}))
