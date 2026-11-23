"""Inputs for CalculateZoneMRT: the mean radiant temperature of a zone and its radiant
enclosures as EnergyPlus meets it, in SI units. A zone of two spaces, each its own enclosure,
holds up to six heat transfer surfaces split between them: walls, floors, ceilings and windows of
realistic areas, inside thermal absorptances from 0.84 or 0.9 down to low-emissivity coatings,
and inside face temperatures from cold glazing to warm floors. Some cases sit on the edges the
code tests: a space with no surfaces, area-emissivity sums at or below 0.01 (tiny or zero areas
or absorptances) for the zone or one enclosure, so their MRT falls back to the air temperature,
and space volumes at or below 0.01. Each zone is asked for all eleven results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261126)


def pick(*xs):
    return rng.choice(xs)


def maybe_round(v, nd=2):
    return round(v, nd) if rng.random() < 0.3 else v


for _ in range(1100):
    n = rng.randint(0, 6) if rng.random() < 0.15 else pick(4, 5, 6, 6, 6)
    n1 = rng.randint(0, n)
    n2 = n - n1
    tiny = rng.random() < 0.1
    a, e, t = [], [], []
    mat = rng.uniform(15.0, 30.0)
    for _k in range(6):
        if tiny:
            a.append(pick(0.0, rng.uniform(0.0, 0.01)))
        else:
            a.append(pick(rng.uniform(1.0, 20.0), rng.uniform(5.0, 120.0), rng.uniform(0.2, 4.0)))
        e.append(pick(0.84, 0.9, rng.uniform(0.05, 0.95), 0.0 if rng.random() < 0.1 else 0.9))
        t.append(mat + pick(rng.uniform(-15.0, 5.0), rng.uniform(-2.0, 2.0), rng.uniform(-2.0, 15.0)))
    mat1 = mat + rng.uniform(-1.0, 1.0)
    mat2 = mat + rng.uniform(-1.0, 1.0)
    vol1 = pick(rng.uniform(10.0, 2000.0), 0.0, 0.005)
    vol2 = pick(rng.uniform(10.0, 2000.0), 0.0, 0.005)
    args = [n1, n2] + [maybe_round(v) for v in a] + e + [maybe_round(v) for v in t] + [mat, mat1, mat2,
                                                                                      maybe_round(vol1),
                                                                                      maybe_round(vol2)]
    for which in range(11):
        print(json.dumps({"fn": "ZoneMRT", "args": [which] + args}))
