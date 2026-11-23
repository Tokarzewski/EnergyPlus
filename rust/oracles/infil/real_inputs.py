"""Inputs for the infiltration part of ZoneEquipmentManager::CalcAirFlowSimple for one zone as
EnergyPlus meets it, in SI units: one or two ZoneInfiltration objects of each model (design flow
rate with the DOE-2, BLAST and constant coefficient sets, effective leakage area with the
Sherman-Grimsrud stack and wind coefficients of one to three storey houses, flow coefficient with
AIM-2 coefficients and shelter factors), all three density bases, schedules on and off, EMS
overrides, zones with and without an outdoor air node, the first and later system time steps,
winter and summer, sea level to high altitude. Each case is asked for all nine results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261206)


def pick(*xs):
    return rng.choice(xs)


def obj():
    mod = rng.choices([0, 1, 2], weights=[60, 20, 20])[0]
    den = rng.choices([0, 1, 2], weights=[70, 15, 15])[0]
    frac = pick(1.0, 1.0, 0.0, 0.5, rng.uniform(0.0, 1.0), -0.1)
    coef = pick((1.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.224, 0.0), (0.606, 0.03636, 0.1177, 0.0),
                (rng.uniform(-0.2, 1.0), rng.uniform(0.0, 0.05), rng.uniform(0.0, 0.3), rng.uniform(0.0, 0.02)))
    dl = pick(0.0, rng.uniform(0.001, 0.3), rng.uniform(0.0, 0.05))
    la = rng.uniform(50.0, 1500.0)
    cs = pick(0.000145, 0.000290, 0.000435)
    cw = pick(0.000104, 0.000174, 0.000246, 0.000319, 0.000420)
    fc = rng.uniform(0.01, 0.1)
    acs = pick(0.069, 0.089, 0.107)
    acw = pick(0.128, 0.142, 0.156, 0.170)
    n = pick(0.67, 0.65, rng.uniform(0.5, 1.0))
    sf = pick(1.0, 0.9, 0.7, 0.5, 0.3)
    ems = 1 if rng.random() < 0.08 else 0
    emsv = rng.uniform(-0.01, 0.2)
    return [mod, den, frac, dl] + list(coef) + [la, cs, cw, fc, acs, acw, n, sf, ems, emsv]


for _ in range(1500):
    ninf = rng.choices([0, 1, 2], weights=[5, 70, 25])[0]
    systs = pick(1, 1, 2, 3)
    outdb = rng.uniform(-25.0, 38.0)
    mat = rng.uniform(15.0, 30.0)
    hum = rng.uniform(0.002, 0.016)
    xmpt = mat + rng.uniform(-1.0, 1.0)
    wprev = hum * rng.uniform(0.95, 1.05)
    wind = pick(0.0, rng.uniform(0.0, 12.0))
    metwind = wind * rng.uniform(0.8, 1.4)
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    outhum = rng.uniform(0.0005, 0.02)
    linked = 1 if rng.random() < 0.1 else 0
    linkedhum = outhum * rng.uniform(0.9, 1.1)
    stdrho = pick(1.2041, rng.uniform(1.0, 1.21))
    args = ([ninf] + obj() + obj() +
            [systs, mat, hum, xmpt, wprev, outdb, wind, metwind, outbaro, outhum, linked, linkedhum, stdrho])
    for which in range(9):
        print(json.dumps({"fn": "Infiltration", "args": [which] + args}))
