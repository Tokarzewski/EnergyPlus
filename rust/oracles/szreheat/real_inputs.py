"""Inputs for SetPointManager::SPMSingleZoneReheat::calculate as EnergyPlus meets it: single zone
systems of 0 to 5 kg/s (some off) serving zones at 15 to 30 C, with heating and cooling loads of up
to 50 kW, the loads to the heating and cooling set points around them, in the dead band or not,
mixers at 0 to 100 % outdoor air with outdoor air from -30 to 40 C (or no mixer, the loop inlet
temperature instead), with or without a fan of up to 2 K temperature rise, and the supply limits
the input files give.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261210)


def pick(*xs):
    return rng.choice(xs)


def h(t, w):
    return 1.00484e3 * t + max(w, 1.0e-5) * (2.50094e6 + 1.85895e3 * t)


for _ in range(13500):
    hasoa = pick(1, 1, 0)
    hasfan = pick(1, 0)
    deadband = pick(0, 0, 1)
    oafrac = pick(rng.uniform(0.1, 0.4), 1.0, 0.0, rng.uniform(0.0, 1.0))
    zonem = pick(rng.uniform(0.05, 1.0), rng.uniform(1.0, 5.0), 0.0, 0.0005)
    zonet = pick(rng.uniform(20.0, 24.0), rng.uniform(15.0, 30.0))
    kind = rng.random()
    if kind < 0.4:
        load = rng.uniform(100.0, 50000.0)
        toheat = load
        tocool = load - rng.uniform(500.0, 5000.0)
    elif kind < 0.8:
        load = -rng.uniform(100.0, 50000.0)
        tocool = load
        toheat = load + rng.uniform(500.0, 5000.0)
    else:
        load = pick(0.0, rng.uniform(-0.9, 0.9))
        toheat = rng.uniform(-3000.0, 3000.0)
        tocool = toheat - rng.uniform(500.0, 5000.0)
    inw = rng.uniform(0.003, 0.012)
    rett = zonet + rng.uniform(-0.5, 1.5)
    retw = rng.uniform(0.004, 0.012)
    oat = rng.uniform(-30.0, 40.0)
    oaw = rng.uniform(0.0005, 0.02)
    loopt = rng.uniform(15.0, 28.0)
    fanin = rng.uniform(10.0, 30.0)
    fanout = fanin + rng.uniform(0.0, 2.0)
    mint = pick(10.0, 12.8, rng.uniform(5.0, 15.0))
    maxt = pick(50.0, 40.0, rng.uniform(mint, 60.0))
    args = [hasoa, hasfan, deadband, oafrac, zonem, inw, load, tocool, toheat, zonet, retw, h(rett, retw), oaw,
            h(oat, oaw), loopt, fanin, fanout, mint, maxt]
    print(json.dumps({"fn": "SZReheat", "args": args}))
