"""Inputs for ZonePlenum's CalcAirZoneSupplyPlenum and UpdateAirZoneSupplyPlenum as EnergyPlus meets
them: supply plenums feeding 1 to 4 zone terminals that ask for 0 to 5 kg/s each, with available
flows around them, an inlet node that already carries the summed flow (converged), a little off it
(within and beyond the 0.01 kg/s tolerance) or anything, and plenum zones at 10 to 35 C. Both passes
and all eight results are asked for each plenum.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261216)


def pick(*xs):
    return rng.choice(xs)


for _ in range(843):
    nout = rng.randint(1, 4)
    outs = []
    for _ in range(4):
        m = pick(rng.uniform(0.0, 5.0), rng.uniform(0.0, 0.3), 0.0)
        outs += [m, pick(m, m * rng.uniform(1.0, 1.5), rng.uniform(0.0, 6.0)), pick(0.0, m * rng.uniform(0.0, 1.0))]
    total = sum(outs[3 * k] for k in range(nout))
    inm = pick(total, total + rng.uniform(-0.01, 0.01), total + rng.uniform(-0.5, 0.5), rng.uniform(0.0, 10.0), 0.0)
    inm = max(inm, 0.0)
    inmx = pick(inm, rng.uniform(0.0, 12.0))
    inmn = pick(0.0, inm * rng.uniform(0.0, 1.0))
    tz = rng.uniform(10.0, 35.0)
    wz = rng.uniform(0.002, 0.018)
    hz = 1.00484e3 * tz + wz * (2.50094e6 + 1.85895e3 * tz)
    for first in (0, 1):
        for which in range(8):
            print(json.dumps({"fn": "SupPlenum", "args": [which, first, nout, inm, inmx, inmn] + outs + [tz, wz, hz]}))
for _ in range(12):
    print(json.dumps({"fn": "SupPlenum", "args": [0, 0, 1, 1.0, 1.0, 0.0] + [1.0, 1.0, 0.0] * 4 + [22.0, 0.008, 42500.0]}))
