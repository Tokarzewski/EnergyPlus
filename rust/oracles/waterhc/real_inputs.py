"""Inputs for WaterCoils::CalcSimpleHeatingCoil as EnergyPlus meets it: reheat, preheat and air handler
hot water coils with UA of 50 to 20000 W/K on 0.05 to 15 kg/s of air at -20 to 25 C, fed 0 to
the maximum water flow at 40 to 90 C (and now and then cooler), as the controller searches the
water flow, with the fan continuous or cycling, available or not, in design and simulation
calculations. Each coil is asked for all six results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261203)


def pick(*xs):
    return rng.choice(xs)


for _ in range(2250):
    ua = pick(rng.uniform(50.0, 1000.0), rng.uniform(1000.0, 20000.0))
    mdota = pick(rng.uniform(0.05, 2.0), rng.uniform(2.0, 15.0), 0.0) if rng.random() < 0.97 else 0.0
    tain = pick(rng.uniform(10.0, 20.0), rng.uniform(-20.0, 25.0))
    win = pick(rng.uniform(0.002, 0.01), rng.uniform(0.0005, 0.015))
    mdotwmax = pick(rng.uniform(0.01, 1.0), rng.uniform(1.0, 10.0))
    mdotw = mdotwmax * pick(1.0, rng.uniform(0.0, 1.0), rng.uniform(0.0, 0.05), 0.0)
    twin = pick(82.0, 60.0, rng.uniform(40.0, 90.0), rng.uniform(5.0, 40.0))
    hwin = twin * 4180.0 + rng.uniform(-500.0, 500.0)
    cycling = pick(0, 0, 1)
    plr = pick(1.0, rng.uniform(0.0, 1.0), 0.0) if cycling else 1.0
    design = pick(0, 0, 0, 1)
    avail = pick(1.0, 1.0, 1.0, 0.0)
    for which in range(6):
        print(json.dumps({"fn": "WaterHeatCoil", "args": [which, cycling, design, ua, avail, mdota, tain, win, mdotw, mdotwmax, twin, hwin, plr]}))
