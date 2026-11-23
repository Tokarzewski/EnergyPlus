"""Inputs for ThermalComfort::CalcFangerPMV and CalcStandardEffectiveTemp as EnergyPlus meets them
from People objects, in SI units: air and mean radiant temperatures of heated, cooled and free
running zones, relative humidities of 10 to 90 %, air velocities from still air to fans, activity
levels from seated (0.8 met) to heavy work (4 met), clothing from none to winter wear (2 clo),
with and without work efficiency, at sea level and altitude. PMV takes the activity in W/m2 and SET
in met. Each PMV case is asked for all four results and each SET case for all six.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261127)


def pick(*xs):
    return rng.choice(xs)


def person():
    at = pick(rng.uniform(19.0, 27.0), rng.uniform(5.0, 38.0))
    rt = at + pick(0.0, rng.uniform(-2.0, 2.0), rng.uniform(-8.0, 10.0))
    rh = pick(rng.uniform(0.3, 0.6), rng.uniform(0.1, 0.9))
    vel = pick(0.1, 0.137, 0.2, rng.uniform(0.0, 0.3), rng.uniform(0.3, 1.5))
    met = pick(1.0, 1.1, 1.2, 0.8, rng.uniform(0.8, 2.0), rng.uniform(2.0, 4.0))
    clo = pick(0.5, 1.0, 0.36, 0.0, rng.uniform(0.3, 1.2), rng.uniform(1.2, 2.0))
    work = pick(0.0, 0.0, 0.0, rng.uniform(0.0, 0.2) * met * 58.2)
    return at, rt, rh, vel, met, clo, work


for _ in range(1750):
    at, rt, rh, vel, met, clo, work = person()
    for which in range(4):
        print(json.dumps({"fn": "FangerPMV", "args": [which, at, rt, rh, vel, met * 58.2, clo, work]}))

for _ in range(1100):
    at, rt, rh, vel, met, clo, work = person()
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    for which in range(6):
        print(json.dumps({"fn": "StandardEffectiveTemp", "args": [which, outbaro, at, rt, rh, vel, met, clo, work]}))
