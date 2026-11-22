"""Inputs for the Perez sky model and CLIPLINE: suns from overhead to below 85 degrees zenith and
just under the horizon, walls, roofs and floors facing every way, clear to overcast skies (the
eight clearness bins), sites from sea level to 3000 m, incidence cosines just past 1 (clamped) and
past the tolerance (fatal), and line segments inside, across, along and outside a clip window.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261129)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def direction(alt_deg, az_deg):
    a, z = math.radians(alt_deg), math.radians(az_deg)
    return math.cos(a) * math.sin(z), math.cos(a) * math.cos(z), math.sin(a)


def surface():
    tilt = rng.choice([0.0, 1.0, 1.99, 2.0, 30.0, 45.0, 90.0, 90.0, 120.0, 180.0, rng.uniform(0, 180)])
    az = rng.choice([0.0, 90.0, 180.0, 270.0, rng.uniform(0, 360)])
    t, z = math.radians(tilt), math.radians(az)
    n = (math.sin(t) * math.sin(z), math.sin(t) * math.cos(z), math.cos(t))
    return tilt, n, 0.5 * (1.0 + math.cos(t))


for _ in range(700):
    alt = rng.choice([90.0, 60.0, 30.0, 15.0, 5.0, 4.9, 2.0, 0.5, rng.uniform(0, 90), rng.uniform(-3, 10)])
    sun = direction(alt, rng.uniform(0, 360))
    tilt, n, vf = surface()
    if rng.random() < 0.05:  # the sun along the normal, a hair past 1 (clamped) or past the tolerance
        f = rng.choice([1.00000001, 1.00005, 1.001])
        sun = tuple(f * c for c in n)
    dif = rng.choice([0.0, 1.0, 20.0, 80.0, 150.0, 300.0, rng.uniform(1, 500)])
    beam = rng.choice([0.0, 50.0, 300.0, 700.0, 900.0, rng.uniform(0, 1000)])
    elev = rng.choice([0.0, 100.0, 1600.0, 3000.0, rng.uniform(-50, 4000)])
    iso, sunlit, horiz = rng.choice([1.0, 0.8, 0.0, rng.random()]), rng.choice([1.0, 0.0, rng.random()]), rng.choice([1.0, 0.5, rng.random()])
    detailed = rng.choice([0, 0, 1])
    for which in range(5):
        out("PerezSky", which, detailed, *sun, elev, beam, dif, *n, vf, tilt, iso, sunlit, horiz)

for _ in range(500):
    minX, minY = rng.uniform(-5, 5), rng.uniform(-5, 5)
    maxX, maxY = minX + rng.choice([rng.uniform(0.1, 10), 0.0]), minY + rng.uniform(0.1, 10)
    pts = [rng.uniform(-10, 15) for _ in range(4)]
    kind = rng.randrange(6)
    if kind == 1:
        pts[1] = pts[0]  # vertical
    elif kind == 2:
        pts[3] = pts[2]  # horizontal
    elif kind == 3:
        pts = [minX, maxX, minY, maxY]  # the diagonal
    elif kind == 4:
        pts[1], pts[3] = pts[0], pts[2]  # a point
    for which in range(5):
        out("ClipLine", which, *pts, maxX, minX, maxY, minY)
