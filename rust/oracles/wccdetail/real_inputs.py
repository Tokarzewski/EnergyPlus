"""Inputs for WaterCoils::CoolingCoil with the detailed analysis as EnergyPlus meets it: chilled
water coils in air handlers, fan coils and DOAS units with internal and external UAs of 300 to
80000 W/K over 5 to 800 m2 of outside surface, on 0.05 to 15 kg/s of mixed or outdoor air at 15 to
35 C and humidity ratios of 0.003 to 0.02 (dry, wet and part wet coils), fed 0 to the maximum water
flow at 4 to 14 C as the controller searches the water flow, counter or cross flow, with the fan
continuous or cycling, available or not, on the first HVAC iteration or a later one with the wet
fraction it saved, in design and simulation calculations, at sea level and up to about 1500 m.
Each coil is asked for all nine results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261228)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1500):
    mdota = pick(rng.uniform(0.05, 2.0), rng.uniform(2.0, 15.0)) if rng.random() < 0.96 else pick(0.0, 0.0005)
    desvol = pick(mdota / 1.2, rng.uniform(0.05, 12.0)) if rng.random() < 0.97 else 0.0
    uaext = pick(rng.uniform(300.0, 5000.0), rng.uniform(5000.0, 60000.0))
    uaint = uaext * pick(rng.uniform(1.0, 3.0), rng.uniform(0.3, 1.0))
    area = uaext / pick(rng.uniform(40.0, 120.0), rng.uniform(10.0, 200.0))
    area = min(max(area, 5.0), 800.0)
    tain = pick(rng.uniform(22.0, 28.0), rng.uniform(15.0, 35.0))
    win = pick(rng.uniform(0.008, 0.013), rng.uniform(0.003, 0.02), rng.uniform(0.006, 0.009))
    mdotwmax = pick(rng.uniform(0.05, 2.0), rng.uniform(2.0, 20.0))
    mdotw = mdotwmax * pick(1.0, rng.uniform(0.0, 1.0), rng.uniform(0.0, 0.05), 0.0)
    twin = pick(6.67, 7.0, rng.uniform(4.0, 8.0), rng.uniform(8.0, 14.0))
    hwin = twin * 4190.0 + rng.uniform(-200.0, 200.0)
    cycling = pick(0, 0, 1)
    plr = pick(1.0, rng.uniform(0.0, 1.0), 0.0) if cycling else 1.0
    design = pick(0, 0, 0, 1)
    avail = pick(1.0, 1.0, 1.0, 0.0)
    hexk = pick(2, 2, 1)
    first = pick(0, 1)
    saved = pick(rng.uniform(0.0, 1.0), rng.uniform(0.2, 0.8), 0.0, 1.0)
    pb = pick(101325.0, rng.uniform(84000.0, 102000.0))
    for which in range(9):
        print(json.dumps({"fn": "WaterCoolDetail", "args": [which, cycling, design, hexk, first, avail, uaint, uaext, area, saved, desvol,
                                                             mdota, tain, win, mdotw, mdotwmax, twin, hwin, plr, pb]}))
