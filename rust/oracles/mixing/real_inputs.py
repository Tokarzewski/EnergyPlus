"""Inputs for the zone mixing part of ZoneEquipmentManager::CalcAirFlowSimple, one ZoneMixing
object bringing air from a source zone into a receiving zone as EnergyPlus meets it, in SI units:
design flow rates of small to large openings and transfer fans, delta temperatures positive,
negative and zero (as values and schedules), minimum and maximum zone, source zone and outdoor
temperature schedules, hybrid ventilation closing, source zones warmer and cooler than the
receiving zone (sunspaces, attics, cores, corridors), the first and later system time steps,
winter and summer, sea level to high altitude. Each case is asked for all seven results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261208)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1900):
    nmix = 1 if rng.random() < 0.95 else 0
    hybrid = 1 if rng.random() < 0.06 else 0
    saved = pick(rng.uniform(0.01, 1.5), rng.uniform(0.0, 0.2), 0.0)
    td = pick(0.0, 0.0, 1.0, 2.0, -1.0, -2.0, rng.uniform(-5.0, 5.0))
    hastd = 1 if rng.random() < 0.2 else 0
    lims = []
    for lo, hi in ((15.0, 22.0), (24.0, 30.0), (10.0, 20.0), (25.0, 40.0), (-10.0, 15.0), (20.0, 35.0)):
        lims += [1 if rng.random() < 0.25 else 0, rng.uniform(lo, hi)]
    systs = pick(1, 1, 2, 3)
    mat1 = rng.uniform(16.0, 28.0)
    mat2 = mat1 + rng.uniform(-8.0, 12.0)
    hum1 = rng.uniform(0.003, 0.014)
    hum2 = hum1 * rng.uniform(0.8, 1.2)
    z = [mat1, hum1, mat1 + rng.uniform(-1.0, 1.0), hum1 * rng.uniform(0.95, 1.05),
         mat2, hum2, mat2 + rng.uniform(-1.0, 1.0), hum2 * rng.uniform(0.95, 1.05)]
    outdb = rng.uniform(-15.0, 36.0)
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    args = [nmix, hybrid, saved, td, hastd] + lims + [systs] + z + [outdb, outbaro]
    for which in range(7):
        print(json.dumps({"fn": "Mixing", "args": [which] + args}))
