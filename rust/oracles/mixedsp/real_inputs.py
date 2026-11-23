"""Inputs for SetPointManager::SPMMixedAir::calculate as EnergyPlus meets it: mixed air set points
from supply set points of 10 to 18 C less fan rises of 0 to 2 K, with and without the cooling coil
freeze check (coil outlet minimums of 2 to 10 C, outdoor air from -30 to 40 C, the coil cooling or
not), in blow through (the reference node at the coil outlet temperature), draw through and coil
outlet reference layouts. Each case is asked for both results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261211)


def pick(*xs):
    return rng.choice(xs)


for _ in range(6750):
    layout = pick(0, 1, 1, 2, 2, 3)
    refsp = pick(12.8, 13.0, rng.uniform(10.0, 18.0), rng.uniform(2.0, 10.0))
    fanin = rng.uniform(8.0, 30.0)
    fanout = fanin + pick(0.0, rng.uniform(0.0, 2.0))
    coilin = rng.uniform(5.0, 35.0)
    coilout = pick(coilin - rng.uniform(0.0, 15.0), coilin, coilin + rng.uniform(0.0, 3.0))
    reft = pick(coilout, coilout, rng.uniform(5.0, 30.0)) if layout == 1 else rng.uniform(5.0, 30.0)
    mincoil = pick(7.2, rng.uniform(2.0, 10.0))
    oat = pick(rng.uniform(-30.0, 10.0), rng.uniform(-10.0, 40.0), mincoil)
    args = [layout, refsp, reft, fanin, fanout, coilin, coilout, mincoil, oat]
    for which in range(2):
        print(json.dumps({"fn": "MixedSP", "args": [which] + args}))
