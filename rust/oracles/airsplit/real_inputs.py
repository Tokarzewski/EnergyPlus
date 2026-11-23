"""Inputs for SplitterComponent's SimAirLoopSplitter (init, calc and update) as EnergyPlus meets it:
supply zone splitters with one to four outlets on air loops of 0 to 30 kg/s (shut off now and
then, with no flow available), supply air at 10 to 40 C, on the first call of the first HVAC
iteration or later ones, and on the second call after the terminal units have set the outlet
flows (often equal to the last inlet flow, often changed). Each case is asked for all 32 results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261209)


def pick(*xs):
    return rng.choice(xs)


def h(t, w):
    return 1.00484e3 * t + max(w, 1.0e-5) * (2.50094e6 + 1.85895e3 * t)


for _ in range(420):
    n = pick(1, 2, 2, 3, 4)
    fhvac = pick(0, 1)
    first = pick(0, 1)
    p = pick(101325.0, rng.uniform(70000.0, 105000.0))
    m0 = pick(rng.uniform(0.0, 5.0), rng.uniform(5.0, 30.0), 0.0)
    maxav0 = pick(m0, m0 * rng.uniform(1.0, 1.5), 0.0, m0)
    minav0 = pick(0.0, m0 * rng.uniform(0.0, 0.5))
    t0 = pick(rng.uniform(12.0, 16.0), rng.uniform(10.0, 40.0))
    w0 = rng.uniform(0.003, 0.012)
    args = [n, fhvac, first, m0, maxav0, minav0, t0, w0, h(t0, w0), p]
    share = [rng.random() for _ in range(4)]
    tot = sum(share[:n])
    for i in range(4):
        m = pick(m0 * share[i] / tot, m0 / n, rng.uniform(0.0, 5.0), 0.0)
        if i == 0 and rng.random() < 0.3:
            m = m0 * share[0] / tot + pick(0.005, -0.005, 0.02)
        m = max(m, 0.0)
        maxav = pick(m, maxav0 / n, m * rng.uniform(1.0, 2.0))
        minav = pick(0.0, m * rng.uniform(0.0, 0.5))
        t = rng.uniform(15.0, 30.0)
        w = rng.uniform(0.003, 0.012)
        args += [m, maxav, minav, t, w, h(t, w), p]
    for which in range(32):
        print(json.dumps({"fn": "AirSplit", "args": [which] + args}))
