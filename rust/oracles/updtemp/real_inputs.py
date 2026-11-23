"""Inputs for ZoneSpaceHeatBalanceData::updateTemperatures for one fully mixed zone as EnergyPlus
meets it, in SI units and hours: zone time steps of 10 to 60 minutes shortened to a half, a third,
a quarter or less (and not shortened), with the system time step count changed or kept since the
last zone time step, zones with and without a system node, air temperature and humidity ratio
histories of steady, warming, cooling, drying and humidifying zones, and both history choices.
Each case is asked for all 22 results. Then the five-value DownInterpolate4HistoryValues the
contaminant predictor uses, over the same time step ratios and CO2 (ppm) and generic contaminant
histories, each asked for all five values.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261124)


def pick(*xs):
    return rng.choice(xs)


def hist(v, spread):
    d = rng.uniform(-spread, spread)
    return [v + k * d + rng.uniform(-spread / 10.0, spread / 10.0) for k in range(4)]


for _ in range(560):
    zone_ts = pick(1.0 / 6.0, 0.25, 1.0 / 3.0, 0.5, 1.0)
    ratio = pick(2, 3, 4, 5, 6, 10, rng.uniform(1.5, 12.0))
    prior = pick(zone_ts, zone_ts / pick(1, 2))
    tsys = prior / ratio
    shorten = 1 if rng.random() < 0.7 else 0
    nsys = pick(2, 3, 4, 6, 10)
    nsyslast = nsys if rng.random() < 0.4 else pick(1, 2, 3, 4)
    hasnode = 1 if rng.random() < 0.75 else 0
    usezonehist = 1 if rng.random() < 0.5 else 0
    t = pick(rng.uniform(18.0, 26.0), rng.uniform(-5.0, 40.0))
    wv = pick(rng.uniform(0.004, 0.012), rng.uniform(0.001, 0.02))
    mat = t + rng.uniform(-0.5, 0.5)
    w = wv + rng.uniform(-0.0003, 0.0003)
    x = hist(t, pick(0.05, 0.5, 2.0))
    ds = hist(t, pick(0.02, 0.3, 1.0))
    wp = hist(wv, pick(0.00005, 0.0005))
    dsw = hist(wv, pick(0.00003, 0.0003))
    args = [shorten, usezonehist, prior, tsys, nsys, nsyslast, hasnode, mat, w] + x + ds + wp + dsw
    for which in range(22):
        print(json.dumps({"fn": "UpdateTemperatures", "args": [which] + args}))

for _ in range(300):
    old = pick(1.0 / 6.0, 0.25, 0.5, 1.0)
    ratio = pick(2, 3, 4, 5, 6, 2.005, 2.995, rng.uniform(1.5, 12.0))
    nw = old / ratio
    base = pick(rng.uniform(400.0, 1200.0), rng.uniform(0.0, 50.0))
    d = base * pick(0.001, 0.02, 0.1) * rng.uniform(-1.0, 1.0)
    v = [base, base + d, base + 2.0 * d + rng.uniform(-abs(d), abs(d))]
    for which in range(5):
        print(json.dumps({"fn": "DownInterpolate5", "args": [which, old, nw] + v}))
