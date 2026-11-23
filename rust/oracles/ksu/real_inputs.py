"""Inputs for ThermalComfort::CalcThermalComfortKSU as EnergyPlus meets it from People objects with
the KSU two-node model, in SI units: cold to hot zones (mostly 18 to 30 C) at humidity ratios of
2 to 20 g/kg, mean radiant temperatures near the air temperature, most without and some with
radiant heaters, activity levels of 70 to 400 W per person with and without work efficiency,
clothing of 0.3 to 1.5 clo, still air to 2 m/s, at sea level and altitude. Each case is asked
for all three results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261129)


def pick(*xs):
    return rng.choice(xs)


for _ in range(4500):
    ztav = pick(rng.uniform(18.0, 30.0), rng.uniform(0.0, 45.0))
    w = pick(rng.uniform(0.005, 0.012), rng.uniform(0.002, 0.020))
    mrt = ztav + pick(0.0, rng.uniform(-2.0, 2.0), rng.uniform(-6.0, 8.0))
    mrt = min(max(mrt, -10.0), 60.0)
    qrad = pick(0.0, 0.0, 0.0, rng.uniform(0.0, 300.0))
    act = pick(108.0, 120.0, 126.0, rng.uniform(70.0, 200.0), rng.uniform(200.0, 400.0))
    eff = pick(0.0, 0.0, 0.0, rng.uniform(0.0, 0.2))
    clo = pick(0.5, 1.0, 0.36, rng.uniform(0.3, 1.5))
    vel = pick(0.1, 0.137, 0.2, rng.uniform(0.0, 0.3), rng.uniform(0.3, 2.0))
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    for which in range(3):
        print(json.dumps({"fn": "KsuTsv", "args": [which, outbaro, ztav, w, mrt, qrad, act, eff, clo, vel]}))
