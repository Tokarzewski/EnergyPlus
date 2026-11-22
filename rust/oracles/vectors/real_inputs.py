"""Inputs for the surface geometry helpers: walls facing every way, roofs, floors, sloped and
nearly flat surfaces, triangles to pentagons, surfaces slightly out of plane, collinear and
repeated vertices, and the rounding EnergyPlus applies to coordinates.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261127)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def rect(w, h, az_deg, tilt_deg, origin):
    """A w by h rectangle, outward normal at azimuth az (clockwise from north) and tilt from up,
    listed counter-clockwise from upper left as EnergyPlus surfaces are."""
    az, tl = math.radians(az_deg), math.radians(tilt_deg)
    nrm = (math.sin(tl) * math.sin(az), math.sin(tl) * math.cos(az), math.cos(tl))
    u = (math.cos(az), -math.sin(az), 0.0)  # along the bottom edge, to the right seen from outside
    v = (nrm[1] * u[2] - nrm[2] * u[1], nrm[2] * u[0] - nrm[0] * u[2], nrm[0] * u[1] - nrm[1] * u[0])
    o = origin
    pt = lambda a, b: [o[i] + a * u[i] + b * v[i] for i in range(3)]
    return [pt(0, h), pt(0, 0), pt(w, 0), pt(w, h)]


def flat(pts):
    pts = pts + [[0.0, 0.0, 0.0]] * (5 - len(pts))
    return [c for p in pts for c in p]


def shapes():
    for _ in range(60):
        az = rng.choice([0.0, 90.0, 180.0, 270.0, 45.0, rng.uniform(0, 360)])
        tilt = rng.choice([90.0, 90.0, 0.0, 180.0, 30.0, 1e-7, 179.9999999, rng.uniform(0, 180)])
        o = [rng.uniform(-20, 20) for _ in range(3)]
        r = rect(rng.uniform(0.5, 20), rng.uniform(0.5, 6), az, tilt, o)
        yield 4, r
        yield 3, r[:3]
        p5 = r[:2] + [[(r[1][i] + r[2][i]) / 2 for i in range(3)]] + r[2:]
        yield 5, p5
        bent = [list(q) for q in r]
        bent[3][2] += rng.choice([1e-5, 2e-4, 0.01])
        yield 4, bent
    yield 3, [[0, 0, 0], [1, 1, 1], [2, 2, 2]]
    yield 4, [[0, 0, 0], [0, 0, 0], [1, 0, 0], [1, 1, 0]]
    yield 4, [[0, 0, 3], [0, 0, 0], [5, 0, 0], [5, 0, 3]]


for n, pts in shapes():
    f = flat(pts)
    out("VAreaPolygon", n, *f)
    for w in range(3):
        out("VNewellArea", w, n, *f)
        out("VNewellNormal", w, n, *f)
    for w in range(11):
        out("VAzimuthTilt", w, n, *f)
    for w in range(3):
        out("VCoPlanar", w, n, *f)
    for w in [-2, -1, 0, 1, 4]:
        out("VPointsInPlane", w, n, *f)

for _ in range(80):
    xyz = [rng.uniform(-100, 100) for _ in range(3)]
    for w in range(3):
        out("VRound", w, *xyz, rng.choice([1e4, 1e6, 100.0, 1.0]))
    for w in range(5):
        out("VNormalize", w, *xyz)
out("VRound", 0, 2.5, -2.5, 0.5, 1.0)
out("VRound", 1, 2.5, -2.5, 0.5, 1.0)
