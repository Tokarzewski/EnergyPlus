"""Inputs for the polygon overlap under the three clipping algorithms: shadow-like polygon pairs in
a surface's own plane, in metres. SutherlandHodgman (algo 0) gets a sample of the pairs unit 28
covers. SlaterBarskyandSutherlandHodgman (algo 1) mostly gets an axis-aligned rectangle as the
clipping polygon, in every starting vertex and both orders, so CLIPRECT runs: subject polygons
inside it, around it, across one or two of its corners, sharing its edges or corners on the
0.01 m grid, touching it, disjoint and in slivers; the rest are rotated rectangles and triangles,
which fall back to Sutherland-Hodgman. ConvexWeilerAtherton (algo 2) gets the same relations
with coordinates within its 10 m guard. Each pair is asked for the status, vertex count, areas,
transmittance and several of the overlap's vertices.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261207)


def out(*args):
    print(json.dumps({"fn": "ClipOverlap", "args": list(args)}))


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


def shift(p):
    k = rng.randrange(len(p))
    p = p[k:] + p[:k]
    return p[::-1] if rng.random() < 0.5 else p


def pad(p):
    q = list(p) + [(0.0, 0.0)] * (4 - len(p))
    return [c for v in q for c in v]


def related(p1, cx, cy, size):
    rel = rng.randrange(7)
    if rel == 0:
        return list(p1)  # identical
    if rel == 1:  # inside
        return shape(cx, cy, size * rng.uniform(0.05, 0.4))
    if rel == 2:  # around
        return shape(cx, cy, size * rng.uniform(2.5, 6))
    if rel == 3:  # far away
        return shape(cx + size * 10, cy - size * 7, size)
    if rel == 4:  # sliver overlap or near-duplicate shift
        d = rng.choice([1e-7, 2e-5, 1e-5, size * 1e-4])
        return [(x + d, y - d) for x, y in p1]
    if rel == 5:  # shared edge: shifted by its own width
        xs = [v[0] for v in p1]
        return [(x + (max(xs) - min(xs)), y) for x, y in p1]
    return shape(cx + size * rng.uniform(-1, 1), cy + size * rng.uniform(-1, 1), size * rng.uniform(0.5, 1.5))


def subject_for_rect(x, y, w, h):
    """A subject polygon placed against the rectangle (x, y, w, h)."""
    k = rng.randrange(9)
    cx, cy = x + w / 2, y + h / 2
    if k == 0:  # inside
        return shape(cx, cy, min(w, h) * rng.uniform(0.05, 0.4))
    if k == 1:  # around: the clip is all corners
        return shape(cx, cy, max(w, h) * rng.uniform(1.5, 4))
    if k == 2:  # across a corner
        c = rng.choice([(x, y), (x, y + h), (x + w, y + h), (x + w, y)])
        return shape(c[0], c[1], min(w, h) * rng.uniform(0.2, 0.8))
    if k == 3:  # across two corners: a band through the rectangle
        if rng.random() < 0.5:
            return rect(x - w * 0.3, y + h * rng.uniform(0.1, 0.5), w * 1.6, h * rng.uniform(0.1, 0.4))
        return rect(x + w * rng.uniform(0.1, 0.5), y - h * 0.3, w * rng.uniform(0.1, 0.4), h * 1.6)
    if k == 4:  # on the grid, sharing edges or corners
        g = 0.01 * rng.randrange(1, 4)
        xs = [x, x + w / 2, x + w, x - g, x + w + g]
        ys = [y, y + h / 2, y + h, y - g, y + h + g]
        x0, x1 = sorted(rng.sample(xs, 2))
        y0, y1 = sorted(rng.sample(ys, 2))
        return rect(x0, y0, x1 - x0, y1 - y0)
    if k == 5:  # touching from outside
        return rect(x + w, y + h * rng.uniform(-0.5, 0.5), w * 0.5, h * 0.5)
    if k == 6:  # disjoint
        return shape(x + w * 3, y - h * 2, min(w, h))
    if k == 7:  # triangle with a vertex inside and two outside
        return [(cx, cy), (x - w * rng.uniform(0.1, 1), y + h * rng.uniform(-1, 2)),
                (x + w * rng.uniform(-1, 2), y - h * rng.uniform(0.1, 1))]
    return shape(cx + w * rng.uniform(-1, 1), cy + h * rng.uniform(-1, 1), min(w, h) * rng.uniform(0.3, 1.2))


def emit(algo, p1, p2):
    p1, p2 = shift(p1), shift(p2)
    t1 = rng.choice([1.0, 0.0, 0.5, rng.random()])
    t2 = rng.choice([1.0, 0.0, 0.3, rng.random()])
    whiches = list(range(6)) + rng.sample(range(6, 36), 6)
    for w in whiches:
        out(w, algo, len(p1), *pad(p1), len(p2), *pad(p2), t1, t2)


def sizes(big):
    if big:
        return rng.choice([0.01, 0.1, 0.5, 1.0, 2.0, 5.0, 30.0, rng.uniform(0.05, 10.0)])
    return rng.choice([0.01, 0.1, 0.5, 1.0, 2.0, 3.0, rng.uniform(0.05, 3.0)])


for _ in range(200):
    size = sizes(True)
    cx, cy = rng.uniform(-50, 50), rng.uniform(-50, 50)
    p1 = shape(cx, cy, size)
    emit(0, p1, related(p1, cx, cy, size))

for _ in range(900):
    size = sizes(True)
    if rng.random() < 0.5:
        x, y = round(rng.uniform(-50, 50), 2), round(rng.uniform(-50, 50), 2)
        w, h = round(size * rng.uniform(0.3, 1.5), 2) or 0.01, round(size * rng.uniform(0.3, 1.5), 2) or 0.01
    else:
        x, y = rng.uniform(-50, 50), rng.uniform(-50, 50)
        w, h = size * rng.uniform(0.3, 1.5), size * rng.uniform(0.3, 1.5)
    if rng.random() < 0.85:
        emit(1, subject_for_rect(x, y, w, h), rect(x, y, w, h))
    else:  # not an axis-aligned rectangle: Sutherland-Hodgman
        p1 = shape(x, y, size)
        emit(1, p1, related(p1, x, y, size))

for _ in range(900):
    size = sizes(False)
    lim = 9.9 - 6 * size
    cx, cy = rng.uniform(-max(lim, 0.5), max(lim, 0.5)), rng.uniform(-max(lim, 0.5), max(lim, 0.5))
    if rng.random() < 0.3:
        w, h = size, size * rng.uniform(0.5, 1.5)
        p1, p2 = subject_for_rect(cx - w / 2, cy - h / 2, w, h), rect(cx - w / 2, cy - h / 2, w, h)
    else:
        p1 = shape(cx, cy, size)
        p2 = related(p1, cx, cy, size)
    clamp = [[(max(-10.0, min(10.0, a)), max(-10.0, min(10.0, b))) for a, b in p] for p in (p1, p2)]
    emit(2, clamp[0], clamp[1])
