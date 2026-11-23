"""Inputs for HVACInterfaceManager's UpdateHalfLoopInletTemp as EnergyPlus meets it: chilled,
condenser and hot water loops of 0 to 50000 kg (autocalculated from their volume, or none) with
0 to 100 kg/s through the half loop just simulated (some off), outlet temperatures of 4 to 80 C,
pump heat of 0 to 50 kW, tank temperatures from this and the last time step nearby or far, the
first call of a time step or a repeat, and system time steps of 1 to 60 minutes (so the exponent
covers both sides of the 700 cutoff). All five results are asked for each call.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261223)


def pick(*xs):
    return rng.choice(xs)


for _ in range(2700):
    tin = pick(rng.uniform(4.0, 14.0), rng.uniform(20.0, 40.0), rng.uniform(40.0, 80.0))
    last = min(max(tin + rng.uniform(-8.0, 8.0), 1.0), 99.0)
    prev = pick(last, min(max(last + rng.uniform(-2.0, 2.0), 1.0), 99.0))
    mdot = pick(rng.uniform(0.0, 100.0), rng.uniform(0.0, 1.0), 0.0)
    mass = pick(rng.uniform(100.0, 50000.0), rng.uniform(0.0, 50.0), 0.0)
    args = [rng.randint(0, 1), rng.randint(0, 1), tin, mdot, pick(0.0, rng.uniform(0.0, 5.0e4)), mass, last, prev,
            pick(60.0, 600.0, 900.0, 3600.0, rng.uniform(1.0, 3600.0))]
    for which in range(5):
        print(json.dumps({"fn": "HalfLoop", "args": [which] + args}))
