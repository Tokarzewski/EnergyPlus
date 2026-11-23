"""Inputs for SetPointManager's multizone average temperature and humidity managers as EnergyPlus
meets them: air loops serving 1 to 4 zones at 18 to 30 C with 0 to 3 kg/s of supply each (some
off), heating and cooling loads of up to 20 kW (some below the 1 W cutoff), humidity ratios of
0.002 to 0.02 and moisture loads either side of zero (some below the small-load cutoff), with
supply limits that bind or not. Both managers' cases are interleaved, all six kinds.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261220)


def pick(*xs):
    return rng.choice(xs)


def out(fn, args):
    print(json.dumps({"fn": fn, "args": args}))


for _ in range(6750):
    nc = rng.randint(1, 4)
    mint = pick(rng.uniform(10.0, 14.0), rng.uniform(10.0, 30.0))
    maxt = pick(rng.uniform(mint, 45.0), mint, 50.0)
    zones = []
    for _ in range(4):
        m = pick(rng.uniform(0.0, 3.0), 0.0, rng.uniform(0.0, 0.2))
        load = pick(rng.uniform(-2.0e4, 2.0e4), rng.uniform(-1.0, 1.0), 0.0, rng.uniform(-3.0e3, 3.0e3))
        zw = rng.uniform(0.002, 0.02)
        zones += [m, load, rng.uniform(18.0, 30.0), zw, pick(zw, rng.uniform(0.002, 0.02))]
    out("MZTemp", [rng.randint(0, 1), nc, mint, maxt] + zones)
    minh = pick(rng.uniform(0.0, 0.008), 0.00001, 0.0)
    maxh = pick(rng.uniform(minh, 0.02), minh, 0.03)
    zones = []
    for _ in range(4):
        hum = pick(rng.uniform(0.0, 0.003), rng.uniform(-1e-5, 1e-5), 0.0, -rng.uniform(0.0, 0.002))
        dehum = pick(-rng.uniform(0.0, 0.003), rng.uniform(-1e-5, 1e-5), 0.0, rng.uniform(0.0, 0.002))
        zones += [pick(rng.uniform(0.0, 3.0), 0.0, rng.uniform(0.0, 0.002)), hum, dehum, rng.uniform(0.002, 0.02)]
    out("MZHum", [rng.randint(0, 3), nc, minh, maxh] + zones)
