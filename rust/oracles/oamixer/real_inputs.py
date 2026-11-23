"""Inputs for MixedAir::OAMixerProps::CalcOAMixer as EnergyPlus meets it: air loops of 0.1 to 50 kg/s
with return air at 18 to 30 C and outdoor air from -40 to 45 C (dry winters and humid summers,
some at saturation and some foggy), at site pressures of 70 to 105 kPa, the outdoor and relief
flows the economiser and minimum outdoor air controls set (relief now and then above the return
flow, and no flow at all). The enthalpies are the nodes', from the temperature and humidity ratio.
Each mixer is asked for all six results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261207)


def pick(*xs):
    return rng.choice(xs)


def h(t, w):
    return 1.00484e3 * t + max(w, 1.0e-5) * (2.50094e6 + 1.85895e3 * t)


def wsat(t, p):
    # Tetens over water or ice, good enough to place cases near saturation.
    import math
    if t >= 0.0:
        pw = 610.78 * math.exp(17.27 * t / (t + 237.3))
    else:
        pw = 610.78 * math.exp(21.875 * t / (t + 265.5))
    return 0.62198 * pw / (p - pw)


for _ in range(2250):
    p = pick(101325.0, rng.uniform(70000.0, 105000.0))
    retm = pick(rng.uniform(0.1, 5.0), rng.uniform(5.0, 50.0), 0.0)
    rett = rng.uniform(18.0, 30.0)
    retw = rng.uniform(0.003, 0.9 * wsat(rett, p))
    oat = pick(rng.uniform(-40.0, 0.0), rng.uniform(0.0, 25.0), rng.uniform(25.0, 45.0))
    oaw = wsat(oat, p) * pick(rng.uniform(0.1, 1.0), 1.0, rng.uniform(1.0, 1.3))
    oaw = min(oaw, 0.05)
    frac = pick(rng.uniform(0.1, 0.4), 1.0, 0.0, rng.uniform(0.0, 1.0))
    oam = retm * frac if retm > 0 else pick(0.0, rng.uniform(0.0, 2.0))
    relm = pick(oam, oam * rng.uniform(0.0, 1.0), retm * rng.uniform(1.0, 1.5), 0.0)
    oap = p
    retp = p + pick(0.0, rng.uniform(-200.0, 200.0))
    args = [rett, retw, h(rett, retw), retp, retm, oat, oaw, h(oat, oaw), oap, oam, relm]
    for which in range(6):
        print(json.dumps({"fn": "OAMixer", "args": [which] + args}))
