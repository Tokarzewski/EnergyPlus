"""Inputs for SetPointManager's warmest temperature and flow manager and return air bypass flow
manager as EnergyPlus meets them: VAV air loops serving 1 to 4 zones at 20 to 28 C with maximum
flows of 0 to 3 kg/s (some below the small-flow cutoff), cooling loads of up to 15 kW, heating or
none, supply limits of 10 to 20 C and turndowns of 0.1 to 1, both strategies; and return air bypass
mixers with 0 to 5 kg/s, supply at 8 to 14 C, return air at 0 to 30 C (some below the 1 C floor)
and fan heat of 0 to 2 K. Both managers' cases are interleaved, all three warmest results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261222)


def pick(*xs):
    return rng.choice(xs)


def out(fn, args):
    print(json.dumps({"fn": fn, "args": args}))


for _ in range(3375):
    mint = rng.uniform(10.0, 14.0)
    maxt = pick(rng.uniform(mint, 20.0), mint)
    zones = []
    for _ in range(4):
        load = pick(-rng.uniform(0.0, 1.5e4), -rng.uniform(0.0, 2.0e3), rng.uniform(0.0, 3.0e3), 0.0, -rng.uniform(0.0, 1.0))
        zones += [load, rng.uniform(20.0, 28.0), pick(rng.uniform(0.0, 3.0), rng.uniform(0.0, 0.002)), rng.uniform(0.004, 0.014)]
    head = [rng.randint(0, 1), rng.randint(1, 4), mint, maxt, pick(rng.uniform(0.1, 1.0), 0.3, 1.0)]
    for which in range(3):
        out("WarmFlow", [which] + head + zones)
    tmix = rng.uniform(8.0, 16.0)
    out("RABypass", [rng.uniform(10.0, 16.0), tmix + pick(rng.uniform(0.0, 2.0), 0.0), tmix,
                     rng.uniform(0.0, 5.0), rng.uniform(8.0, 14.0), rng.uniform(0.0, 5.0), pick(rng.uniform(15.0, 30.0), rng.uniform(0.0, 1.0))])
