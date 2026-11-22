"""Inputs for the window gap gas routines: air, argon, krypton and xenon fills and their mixtures
(90/10 argon-air, equal shares, three and four gases), gaps from 6 mm to 25 mm, surface
temperatures from -20 C to 60 C with small and large differences, and the gap Nusselt number over
Rayleigh numbers from below 1e4 to 1e6, tilts from flat to vertical on both sides of 60 degrees,
windows heated from above and from below, and real and nominal window heights.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261203)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def mix():
    n = rng.choice([1, 1, 2, 2, 2, 3, 4])
    gases = rng.sample([1, 2, 3, 4], n) + [1] * (4 - n)
    if n == 2 and rng.random() < 0.5:
        fr = [0.9, 0.1]
    else:
        fr = [rng.random() + 0.01 for _ in range(n)]
        s = sum(fr)
        fr = [f / s for f in fr]
    return n, gases, fr + [0.0] * (4 - n)


for _ in range(500):
    n, gases, fr = mix()
    t1 = rng.choice([253.15, 273.15, 283.15, 293.15, 333.15, rng.uniform(250, 340)])
    t2 = t1 + rng.choice([0.0, 0.5, 5.0, 15.0, -10.0, rng.uniform(-30, 30)])
    w = rng.choice([0.006, 0.0127, 0.016, 0.025, rng.uniform(0.004, 0.03)])
    quad = rng.choice([0.0, 0.0, 1.0, -2.5])
    for k in range(3):
        out("GapCon", k, t1, t2, w, n, *gases, *fr, quad)

for _ in range(300):
    n, gases, fr = mix()
    t = rng.uniform(250, 340)
    quad = rng.choice([0.0, 0.0, 1.0])
    for k in range(2):
        out("GapProps", k, t, n, *gases, *fr, quad)

for _ in range(1500):
    tso = rng.uniform(250, 320)
    tsi = tso + rng.choice([5.0, -5.0, 20.0, -20.0, rng.uniform(-30, 30)])
    w = rng.choice([0.006, 0.0127, 0.016, 0.025, rng.uniform(0.004, 0.03)])
    h = rng.choice([0.0, 1.0, 1.524, 2.5, rng.uniform(0.3, 3.0)])
    tilt = rng.choice([90.0, 90.0, 60.0, 59.9, 45.0, 30.0, 0.0, 120.0, rng.uniform(0, 180)])
    ra = 10 ** rng.uniform(-4, 6.5)
    pr = rng.choice([0.71, 0.68, 0.7])
    out("GapNusselt", tso, tsi, w, h, tilt, ra / pr, pr)
