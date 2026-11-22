"""Inputs for the polygon overlap: shadow-like polygon pairs in a surface's own plane, in metres -
rectangles, triangles and convex quadrilaterals of 1 cm to 30 m, in both vertex orders, placed
disjoint, touching, overlapping in part, one inside the other and identical, plus slivers whose
overlap is too small to count, near-duplicate vertices within the 0.02 mm grid, and polygons with
collinear or repeated vertices. Each pair is asked for the status, vertex count, areas,
transmittance and several of the overlap's vertices.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261206)


def out(*args):
    print(json.dumps({"fn": "PolyOverlap", "args": list(args)}))


def rect(x, y, w, h):
    return [(x, y), (x, y + h), (x + w, y + h), (x + w, y)]


def convex(cx, cy, r, n):
    angs = sorted(rng.uniform(0, 2 * math.pi) for _ in range(n))
    return [(cx + r * math.cos(a), cy + r * math.sin(a)) for a in angs]


def shape(cx, cy, size):
    k = rng.randrange(5)
    if k == 0:
        w, h = size * rng.uniform(0.3, 1.5), size * rng.uniform(0.3, 1.5)
        return rect(cx - w / 2, cy - h / 2, w, h)
    if k == 1:
        return convex(cx, cy, size, 3)
    if k == 2:
        return convex(cx, cy, size, 4)
    if k == 3:  # axis-aligned square on the grid
        s = round(size, 2)
        return rect(round(cx, 2), round(cy, 2), s, s)
    pts = convex(cx, cy, size, 3)  # repeated or collinear vertex
    return pts + [rng.choice([pts[0], ((pts[1][0] + pts[2][0]) / 2, (pts[1][1] + pts[2][1]) / 2)])]


def maybe_rev(p):
    return p[::-1] if rng.random() < 0.5 else p


def pad(p):
    q = list(p) + [(0.0, 0.0)] * (4 - len(p))
    return [c for v in q for c in v]


for _ in range(1100):
    size = rng.choice([0.01, 0.1, 0.5, 1.0, 2.0, 5.0, 30.0, rng.uniform(0.05, 10.0)])
    cx, cy = rng.uniform(-50, 50), rng.uniform(-50, 50)
    p1 = shape(cx, cy, size)
    rel = rng.randrange(7)
    if rel == 0:
        p2 = list(p1)  # identical
    elif rel == 1:  # inside
        p2 = shape(cx, cy, size * rng.uniform(0.05, 0.4))
    elif rel == 2:  # around
        p2 = shape(cx, cy, size * rng.uniform(2.5, 6))
    elif rel == 3:  # far away
        p2 = shape(cx + size * 10, cy - size * 7, size)
    elif rel == 4:  # sliver overlap or near-duplicate shift
        d = rng.choice([1e-7, 2e-5, 1e-5, size * 1e-4])
        p2 = [(x + d, y - d) for x, y in p1]
    elif rel == 5:  # shared edge: shifted by its own width
        xs = [v[0] for v in p1]
        p2 = [(x + (max(xs) - min(xs)), y) for x, y in p1]
    else:  # partial overlap
        p2 = shape(cx + size * rng.uniform(-1, 1), cy + size * rng.uniform(-1, 1), size * rng.uniform(0.5, 1.5))
    p1, p2 = maybe_rev(p1), maybe_rev(p2)
    t1 = rng.choice([1.0, 0.0, 0.5, rng.random()])
    t2 = rng.choice([1.0, 0.0, 0.3, rng.random()])
    whiches = list(range(6)) + rng.sample(range(6, 36), 6)
    for w in whiches:
        out(w, len(p1), *pad(p1), len(p2), *pad(p2), t1, t2)
