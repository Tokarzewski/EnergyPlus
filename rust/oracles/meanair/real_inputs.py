"""Inputs for HeatBalanceAirManager::calcMeanAirTemps for one zone as EnergyPlus meets it, in SI
units: zone air temperatures and humidity ratios of heated, cooled and free running zones,
mean radiant temperatures near and away from the air temperature, sea level to high altitude, with
and without operative temperature control (constant and scheduled radiative fractions) and with
and without the wet bulb globe temperature reported. Each case is asked for all six results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261210)


def pick(*xs):
    return rng.choice(xs)


for _ in range(2100):
    ztav = pick(rng.uniform(18.0, 26.0), rng.uniform(-5.0, 45.0))
    w = pick(rng.uniform(0.003, 0.014), rng.uniform(0.0005, 0.025), 0.0)
    mrt = ztav + pick(rng.uniform(-2.0, 2.0), rng.uniform(-10.0, 15.0))
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    anyop = 1 if rng.random() < 0.4 else 0
    controlled = 1 if rng.random() < 0.7 else 0
    opctrl = rng.choices([0, 1, 2], weights=[40, 35, 25])[0]
    frac = pick(0.5, 0.3, 0.7, rng.uniform(0.0, 0.9))
    wbgt = 1 if rng.random() < 0.5 else 0
    args = [ztav, w, mrt, outbaro, anyop, controlled, opctrl, frac, wbgt]
    for which in range(6):
        print(json.dumps({"fn": "MeanAirTemps", "args": [which] + args}))
