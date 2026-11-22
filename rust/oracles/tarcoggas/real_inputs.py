"""Inputs for TARCOG's gas properties and solver: air, argon, krypton and xenon fills and their
two- to four-gas mixtures (typical 90/10 fills, equal shares, a zero fraction), mean gap
temperatures from -30 C to 80 C, atmospheric and low pressures, all three standards, low-pressure
conduction with gamma at and near 1, and gap heat balance sized systems (1 to 20 unknowns, with
and without pivoting, and singular).
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261202)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


for _ in range(450):
    n = rng.choice([1, 1, 2, 2, 2, 3, 4])
    gases = rng.sample([1, 2, 3, 4], n) + [1] * (4 - n)
    if n == 1:
        fr = [rng.choice([1.0, 1.0, 0.9])]
    else:
        kind = rng.randrange(4)
        if kind == 0:
            fr = [0.9, 0.1] if n == 2 else [round(1.0 / n, 6)] * n
        elif kind == 1:
            fr = [rng.random() for _ in range(n)]
            s = sum(fr)
            fr = [f / s for f in fr]
        elif kind == 2:
            fr = [1.0] + [rng.random() for _ in range(n - 1)]  # first gas at 1.0: single-gas path
        else:
            fr = [rng.random() for _ in range(n)]
            fr[rng.randrange(1, n)] = 0.0  # error 2011
    fr = fr + [0.0] * (4 - len(fr))
    t = rng.choice([243.15, 273.15, 283.15, 293.15, 300.0, 353.15, rng.uniform(230, 360)])
    p = rng.choice([101325.0, 101325.0, 90000.0, 10.0, rng.uniform(1, 110000)])
    std = rng.randrange(3)
    quad = rng.choice([0.0, 0.0, 1.0, -2.5])
    for w in range(6):
        out("Gasses90", w, std, t, p, n, *gases, *fr, quad)

for _ in range(150):
    t = rng.uniform(230, 360)
    m = rng.choice([28.97, 39.948, 83.8, 131.3])
    p = rng.choice([0.1, 1.0, 10.0, rng.uniform(0, 100)])
    g = rng.choice([1.0, 1.4, 1.67, 1.0000001, rng.uniform(1.1, 1.7)])
    for w in range(2):
        out("GassesLow", w, t, m, p, g)

for _ in range(150):
    n = rng.randint(1, 20)
    d = rng.choice([0.0, 0.5, 5.0, 50.0, -20.0, rng.uniform(-10, 60)])
    k = rng.choice([0.0, 1.0, -0.3, rng.uniform(-5, 5)])
    o = rng.choice([0.0, 1.0, 10.0, rng.uniform(-10, 10)])
    p = rng.uniform(0, 6.3)
    for w in sorted(set(rng.sample(range(n + 1), min(n + 1, 6)) + [n])):
        out("TarcogSolve", w, n, d, k, o, p)
out("TarcogSolve", 1, 1, 0.0, 0.0, 0.0, 0.0)
out("TarcogSolve", 0, 3, 0.0, 0.0, 0.0, 1.0)
