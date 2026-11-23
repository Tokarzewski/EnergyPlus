"""Inputs for HeatingCoils::CalcElectricHeatingCoil and CalcFuelHeatingCoil as EnergyPlus meets them:
reheat, preheat and furnace coils of 1 to 200 kW on 0.05 to 10 kg/s of air at -20 to 25 C,
asked for a load (from none to more than the capacity) by a unitary system or terminal, or for
their set point of 12 to 50 C by a set point manager, with the fan continuous or cycling at part
load ratios of 0 to 1, now and then unavailable or with no flow, efficiencies of 0.78 to 1 (fuel)
or 1 (electric), with and without parasitics and a part load fraction curve (the I/O reference's
0.8 + 0.2 x and others). Each coil is asked for all ten results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261202)

CURVES = [[0.8, 0.2, 0.0], [0.85, 0.15, 0.0], [0.75, 0.25, 0.0], [0.9, 0.05, 0.05], [1.0, 0.0, 0.0]]


def pick(*xs):
    return rng.choice(xs)


for _ in range(1350):
    fuel = pick(0, 1)
    cycling = pick(0, 0, 1)
    sensed = pick(0, 0, 1)
    cap = pick(rng.uniform(1000.0, 20000.0), rng.uniform(20000.0, 200000.0))
    mdot = pick(rng.uniform(0.05, 1.0), rng.uniform(1.0, 10.0), 0.0) if rng.random() < 0.95 else 0.0
    tin = pick(rng.uniform(10.0, 22.0), rng.uniform(-20.0, 25.0))
    win = pick(rng.uniform(0.002, 0.01), rng.uniform(0.0005, 0.015))
    tset = pick(rng.uniform(12.0, 35.0), rng.uniform(30.0, 50.0), tin + rng.uniform(-0.2, 0.2))
    qreq = pick(rng.uniform(0.0, cap), rng.uniform(cap, 2.0 * cap), 0.0, rng.uniform(-1000.0, 0.0))
    plr = pick(1.0, rng.uniform(0.0, 1.0), 0.0) if cycling else pick(1.0, rng.uniform(0.0, 1.0))
    eff = 1.0 if fuel == 0 else pick(0.8, 0.78, rng.uniform(0.78, 0.98))
    parelec = 0.0 if fuel == 0 else pick(0.0, rng.uniform(0.0, 500.0))
    parfuel = 0.0 if fuel == 0 else pick(0.0, rng.uniform(0.0, 300.0))
    plf = pick(0, 1) if fuel == 1 else pick(0, 0, 1)
    c = pick(*CURVES)
    avail = pick(1.0, 1.0, 1.0, 1.0, 0.0)
    for which in range(10):
        print(json.dumps({"fn": "HeatCoil", "args": [which, fuel, cycling, sensed, plf, avail, mdot, tin, win, tset, qreq, plr, eff, cap,
                                                    parelec, parfuel, *c]}))
