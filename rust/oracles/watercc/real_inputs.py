"""Inputs for WaterCoils::CoolingCoil with the simple analysis as EnergyPlus meets it: chilled
water coils in air handlers, fan coils and DOAS units with UAs of 200 to 40000 W/K on 0.05 to 15
kg/s of mixed or outdoor air at 15 to 35 C and humidity ratios of 0.003 to 0.02 (dry and wet
coils), fed 0 to the maximum water flow at 4 to 14 C as the controller searches the water flow,
counter or cross flow, with the fan continuous or cycling, available or not, in design and
simulation calculations, at sea level and up to about 1500 m. Each coil is asked for all eight
results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261227)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1687):
    mdota = pick(rng.uniform(0.05, 2.0), rng.uniform(2.0, 15.0)) if rng.random() < 0.96 else pick(0.0, 0.0005)
    desvol = pick(mdota / 1.2, rng.uniform(0.05, 12.0), 0.0) if rng.random() < 0.97 else 0.0
    uatot = pick(rng.uniform(200.0, 3000.0), rng.uniform(3000.0, 40000.0))
    uaint = uatot * rng.uniform(1.5, 4.0)
    uaext = uatot * rng.uniform(1.2, 3.0)
    uatot, uaint, uaext = pick((uatot, uaint, uaext), (uatot, uaint, uaext), (uatot, 0.0, uaext), (0.0, uaint, 0.0))
    tain = pick(rng.uniform(22.0, 28.0), rng.uniform(15.0, 35.0))
    win = pick(rng.uniform(0.008, 0.013), rng.uniform(0.003, 0.02), rng.uniform(0.003, 0.006))
    mdotwmax = pick(rng.uniform(0.05, 2.0), rng.uniform(2.0, 20.0))
    mdotw = mdotwmax * pick(1.0, rng.uniform(0.0, 1.0), rng.uniform(0.0, 0.05), 0.0)
    twin = pick(6.67, 7.0, rng.uniform(4.0, 8.0), rng.uniform(8.0, 14.0))
    hwin = twin * 4190.0 + rng.uniform(-200.0, 200.0)
    cycling = pick(0, 0, 1)
    plr = pick(1.0, rng.uniform(0.0, 1.0), 0.0) if cycling else 1.0
    design = pick(0, 0, 0, 1)
    avail = pick(1.0, 1.0, 1.0, 0.0)
    hexk = pick(2, 2, 1)
    pb = pick(101325.0, rng.uniform(84000.0, 102000.0))
    for which in range(8):
        print(json.dumps({"fn": "WaterCoolCoil", "args": [which, cycling, design, hexk, avail, uatot, uaint, uaext, desvol, mdota, tain,
                                                           win, mdotw, mdotwmax, twin, hwin, plr, pb]}))
for k in range(4):
    print(json.dumps({"fn": "WaterCoolCoil", "args": [k, 0, 0, 2, 1.0, 5000.0, 12000.0, 9000.0, 2.0, 2.4, 26.0, 0.011, 0.0, 3.0,
                                                       7.0, 29330.0, 1.0, 101325.0]}))
