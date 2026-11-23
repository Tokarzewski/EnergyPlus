"""Inputs for MixerComponent::CalcAirMixer as EnergyPlus meets it: return and supply zone mixers with
one to four inlets carrying 0 to 20 kg/s each (some shut off, all shut off now and then) of air at
15 to 35 C from the zones or the terminal units, humidity ratios of 0.002 to 0.02, at site
pressures of 70 to 105 kPa, with the available flows the loop set and the flow fractions an
earlier call left. The enthalpies are the nodes', from the temperature and humidity ratio. Each
mixer is asked for all eleven results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261208)


def pick(*xs):
    return rng.choice(xs)


def h(t, w):
    return 1.00484e3 * t + max(w, 1.0e-5) * (2.50094e6 + 1.85895e3 * t)


for _ in range(1227):
    n = pick(1, 2, 2, 3, 4)
    p = pick(101325.0, rng.uniform(70000.0, 105000.0))
    off = rng.random() < 0.1
    fr = [rng.random() for _ in range(4)]
    s = sum(fr[:n])
    fr = [x / s if i < n else pick(0.0, x) for i, x in enumerate(fr)]
    args = [n]
    for i in range(4):
        m = 0.0 if off else pick(rng.uniform(0.0, 2.0), rng.uniform(0.0, 20.0), 0.0)
        maxav = pick(m, m * rng.uniform(1.0, 2.0), 0.0)
        minav = pick(0.0, 0.0, m * rng.uniform(0.0, 1.0))
        t = pick(rng.uniform(20.0, 26.0), rng.uniform(15.0, 35.0))
        w = rng.uniform(0.002, 0.02)
        args += [m, maxav, minav, t, w, h(t, w), p + pick(0.0, rng.uniform(-300.0, 300.0)), fr[i]]
    for which in range(11):
        print(json.dumps({"fn": "AirMixer", "args": [which] + args}))
