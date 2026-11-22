"""Inputs for the window glazing helpers: clear, tinted and coated glass (equal and unequal front
and back reflectance, so both of TransAndReflAtPhi's branches), opaque glass, the simple glazing
model over its U-factor and SHGC cells and exactly on their edges, blind slats at many slat and
profile angles, window heat balance sized matrices (one to ten unknowns, with and without
pivoting, and singular), and short interpolation tables.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261201)

COS_PHIS = [math.cos(math.radians(10 * i)) for i in range(10)]
U_EDGES = [1.4195, 1.7034, 3.4068, 4.5424]
SHGC_EDGES = [0.25, 0.3, 0.35, 0.45, 0.5, 0.55, 0.6, 0.65]


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def glass():
    kind = rng.randrange(5)
    if kind == 0:  # clear
        tf = rng.uniform(0.75, 0.9)
        rf = rb = rng.uniform(0.07, 0.09)
    elif kind == 1:  # tinted
        tf = rng.uniform(0.2, 0.65)
        rf = rb = rng.uniform(0.04, 0.07)
    elif kind == 2:  # coated: front and back differ
        tf = rng.uniform(0.1, 0.8)
        rf, rb = rng.uniform(0.05, 0.4), rng.uniform(0.05, 0.4)
    elif kind == 3:
        tf, rf, rb = rng.choice([(0.0, 0.3, 0.3), (-0.0, 0.1, 0.2), (0.837, 0.075, 0.075), (0.645, 0.06, 0.06), (0.646, 0.06, 0.0601)])
    else:
        tf = rng.uniform(0.01, 0.95)
        rf = rng.uniform(0.0, 1.0 - tf)
        rb = rf if rng.random() < 0.5 else rng.uniform(0.0, 1.0 - tf)
    return tf, rf, rb


for _ in range(900):
    tf, rf, rb = glass()
    cs = rng.choice(COS_PHIS + [0.0, 0.0005, 0.9995, 1.0, rng.random()])
    simple = 1 if rng.random() < 0.4 else 0
    u = rng.choice(U_EDGES + [rng.uniform(0.5, 6.5), rng.uniform(0.5, 6.5), 0.9, 2.5, 5.8])
    shgc = rng.choice(SHGC_EDGES + [rng.uniform(0.1, 0.9), rng.uniform(0.1, 0.9), 0.2, 0.7])
    for w in range(3):
        out("GlassPhi", w, cs, tf, rf, rb, simple, shgc, u)

for _ in range(250):
    tf, rf, rb = glass()
    for w in range(8):
        out("GlassFit", w, tf, rf, rb)

for _ in range(80):
    s = rng.choice([0.025, 0.05, 0.0889, rng.uniform(0.01, 0.1)])
    h = rng.choice([0.01875, 0.04, 0.075, rng.uniform(0.005, 0.1)])
    phib = math.radians(rng.choice([0.0, 45.0, -45.0, 90.0, -89.9, rng.uniform(-90, 90)]))
    phis = math.radians(rng.choice([0.0, 30.0, 89.95, 90.0, -60.0, rng.uniform(-90, 90)]))
    for i in range(6):
        for j in range(6):
            if rng.random() < 0.4:
                out("SlatViewFac", i, j, s, h, phib, phis)

for _ in range(120):
    n = rng.randint(1, 10)
    d = rng.choice([0.0, 0.5, 5.0, 50.0, -20.0, rng.uniform(-10, 60)])
    k = rng.choice([0.0, 1.0, -0.3, rng.uniform(-5, 5)])
    o = rng.choice([0.0, 1.0, 10.0, rng.uniform(-10, 10)])
    p = rng.uniform(0, 6.3)
    for w in rng.sample(range(n * n), min(n * n, 8)):
        out("MatInverse", w, n, d, k, o, p)
out("MatInverse", 0, 1, 0.0, 0.0, 0.0, 0.0)
out("MatInverse", 3, 3, 0.0, 0.0, 0.0, 1.0)

for _ in range(300):
    n = rng.randint(1, 5)
    xs = sorted(rng.uniform(0, 10) for _ in range(5))
    if rng.random() < 0.2:
        xs[1] = xs[0]
    ys = [rng.uniform(-5, 5) for _ in range(5)]
    xin = rng.choice([rng.uniform(-1, 11), xs[rng.randrange(5)], xs[0], -1.0, 12.0])
    out("Interp", xin, n, *xs, *ys)
