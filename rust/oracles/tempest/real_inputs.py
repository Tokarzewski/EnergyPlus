"""Inputs for SetPointManager::SPMTempest::calculate as EnergyPlus meets it: warmest and coldest
managers on VAV and dual duct air loops of one to four zones (heating none, some or all of them),
zones at 15 to 30 C with heating and cooling loads of up to 30 kW (some idle, some below 1 W),
terminal maximum flows of 0 to 3 kg/s and the supply limits the input files give.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261213)


def pick(*xs):
    return rng.choice(xs)


for _ in range(13500):
    warmest = pick(1, 1, 0)
    nc = pick(1, 2, 3, 4, 4)
    nh = pick(0, nc, rng.randint(0, nc))
    mint = pick(12.8, 10.0, rng.uniform(8.0, 15.0))
    maxt = pick(18.0, 16.0, 50.0, rng.uniform(mint, 45.0))
    args = [warmest, nc, nh, mint, maxt]
    for _z in range(4):
        load = pick(rng.uniform(-30000.0, 0.0), rng.uniform(0.0, 30000.0), 0.0, rng.uniform(-0.5, 0.5))
        zt = pick(rng.uniform(21.0, 25.0), rng.uniform(15.0, 30.0))
        mmax = pick(rng.uniform(0.05, 3.0), 0.0, 0.0005)
        w = rng.uniform(0.003, 0.012)
        args += [load, zt, mmax, w]
    print(json.dumps({"fn": "Tempest", "args": args}))
