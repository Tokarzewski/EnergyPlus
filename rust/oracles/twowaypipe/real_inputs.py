"""Inputs for HVACInterfaceManager's ManageTwoWayCommonPipe with a constant speed primary pump as
EnergyPlus meets it: primary-secondary chilled and hot water loops with primary flows of 0 to 80
kg/s and secondary flows around them, supply or demand inlet set points (or both, or neither) of
6 to 8 C or 45 to 60 C, tank temperatures near, at or far from them (including equal ones, below
the 0.0001 K tolerance), the common pipe's flows from the last call, either side just simulated.
All six results are asked for each call.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261226)


def pick(*xs):
    return rng.choice(xs)


for _ in range(2250):
    hot = rng.random() < 0.3
    sp = rng.uniform(45.0, 60.0) if hot else rng.uniform(6.0, 8.0)
    tpri = sp + pick(0.0, rng.uniform(-2.0, 2.0), 0.00005)
    tsec = sp + (pick(rng.uniform(-15.0, 0.0), 0.0) if hot else pick(rng.uniform(0.0, 8.0), 0.0))
    tsec = pick(tsec, tpri, tpri + 0.00005)
    mpri = pick(rng.uniform(0.0, 80.0), 0.0)
    msec = pick(mpri, mpri * rng.uniform(0.3, 1.5), rng.uniform(0.0, 80.0), 0.0)
    pts = pick(0.0, rng.uniform(0.0, msec + 1.0))
    prc = pick(0.0, rng.uniform(0.0, mpri + 1.0))
    src = pick(0.0, rng.uniform(0.0, msec + 1.0))
    side = rng.randint(0, 1)
    tank, other = (tsec, tpri) if side == 1 else (tpri, tsec)
    args = [side, rng.randint(0, 1), rng.randint(0, 1), tank, other, sp, sp + pick(0.0, rng.uniform(-1.0, 1.0)),
            sp + rng.uniform(-1.0, 1.0), sp + rng.uniform(-1.0, 3.0), msec, mpri, pts, prc, src]
    for which in range(6):
        print(json.dumps({"fn": "TwoWayPipe", "args": [which] + args}))
