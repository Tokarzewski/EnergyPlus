"""Inputs for SetPointManager's SPMSingleZoneTemp::calculate and SPMOutsideAir::calculate as
EnergyPlus meets them: single zone heating and cooling managers for zones at 15 to 30 C served by 0
to 5 kg/s (some off) with loads to their set points of up to 50 kW either way and the supply limits
the input files give; and outdoor air reset managers for hot water and supply air (the input
files' typical lines, falling and rising, a line of zero width now and then) at outdoor
temperatures from -30 to 40 C, with no schedule or one at 1 or 2.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

SZ = []
OA = []

rng = random.Random(20261212)


def pick(*xs):
    return rng.choice(xs)


for _ in range(6750):
    heating = pick(0, 1)
    zonem = pick(rng.uniform(0.05, 1.0), rng.uniform(1.0, 5.0), 0.0, 0.0008)
    inw = rng.uniform(0.003, 0.012)
    toheat = pick(rng.uniform(-5000.0, 50000.0), rng.uniform(-2000.0, 2000.0))
    tocool = toheat - rng.uniform(0.0, 5000.0)
    zonet = pick(rng.uniform(20.0, 24.0), rng.uniform(15.0, 30.0))
    mint = pick(10.0, 12.8, rng.uniform(5.0, 15.0))
    maxt = pick(50.0, 40.0, rng.uniform(mint, 60.0))
    SZ.append(json.dumps({"fn": "SZTemp", "args": [heating, zonem, inw, toheat, tocool, zonet, mint, maxt]}))

LINES = [(-6.7, 10.0, 82.2, 65.6), (0.0, 15.6, 16.7, 12.8), (-17.8, 10.0, 60.0, 40.0), (10.0, 10.0, 15.0, 13.0)]
for _ in range(6750):
    schedv = pick(0, 0, 1, 2)
    oat = pick(rng.uniform(-30.0, 40.0), rng.uniform(-10.0, 20.0))
    ls = []
    for _k in range(2):
        if rng.random() < 0.5:
            low, high, lsp, hsp = rng.choice(LINES)
        else:
            low = rng.uniform(-30.0, 20.0)
            high = low + pick(rng.uniform(0.0, 30.0), 0.0, -rng.uniform(0.0, 5.0))
            lsp = rng.uniform(10.0, 80.0)
            hsp = rng.uniform(10.0, 80.0)
        ls += [low, high, lsp, hsp]
    OA.append(json.dumps({"fn": "OAReset", "args": [schedv, oat] + ls}))

# Interleaved, so any leading slice of the cases holds both managers.
for a, b in zip(SZ, OA):
    print(a)
    print(b)
