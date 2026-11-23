"""Inputs for ThermalComfort::CalcCoolingEffectAdjustedPMV as EnergyPlus meets it from People
objects with the ASHRAE 55 cooling effect, in SI units: warm and hot zones (and some cool ones)
with ceiling fans or open windows from still air to 2 m/s, relative humidities of 20 to 80 %,
activity levels of 0.8 to 2.5 met, clothing of 0.3 to 1.2 clo, with and without work efficiency,
at sea level and altitude. Each case is asked for all three results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261128)


def pick(*xs):
    return rng.choice(xs)


for _ in range(4500):
    at = pick(rng.uniform(25.0, 32.0), rng.uniform(15.0, 38.0))
    rt = at + pick(0.0, rng.uniform(-2.0, 2.0), rng.uniform(-5.0, 6.0))
    rt = min(max(rt, 10.0), 40.0)
    rh = pick(rng.uniform(0.3, 0.6), rng.uniform(0.2, 0.8))
    vel = pick(0.1, 0.2, 0.5, 0.8, 1.2, rng.uniform(0.0, 0.3), rng.uniform(0.3, 2.0))
    met = pick(1.0, 1.1, 1.2, rng.uniform(0.8, 1.6), rng.uniform(1.6, 2.5))
    clo = pick(0.5, 0.36, 0.6, rng.uniform(0.3, 1.2))
    work = pick(0.0, 0.0, 0.0, rng.uniform(0.0, 0.2) * met * 58.2)
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    for which in range(3):
        print(json.dumps({"fn": "CoolingEffect", "args": [which, outbaro, at, rt, rh, vel, met * 58.2, clo, work]}))
