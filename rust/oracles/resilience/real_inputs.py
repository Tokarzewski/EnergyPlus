"""Inputs for HeatBalanceSurfaceManager::CalcThermalResilience: one zone's heat index (simplified
and extended methods) and humidex as EnergyPlus meets them, in SI units. Zone air temperatures and
humidity ratios span cold, comfortable, hot and humid, hot and dry, and overheated zones, through
the simplified method's branches (below 80 F, the regression, the dry and humid adjustments), at
sea level and altitude. Each state is asked for both results under both methods.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261203)


def pick(*xs):
    return rng.choice(xs)


for _ in range(3100):
    t = pick(rng.uniform(10.0, 26.0), rng.uniform(26.0, 32.0), rng.uniform(30.0, 45.0), rng.uniform(26.6, 30.6),
             rng.uniform(33.0, 44.0))
    w = pick(rng.uniform(0.001, 0.004), rng.uniform(0.004, 0.012), rng.uniform(0.012, 0.03))
    outbaro = pick(101325.0, rng.uniform(80000.0, 103000.0))
    for method in (0, 1):
        for which in (0, 1):
            print(json.dumps({"fn": "ThermalResilience", "args": [which, method, t, w, outbaro]}))
