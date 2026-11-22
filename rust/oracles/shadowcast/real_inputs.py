"""Inputs for SHADOW with one receiving surface and one shadowing surface: building-like scenes in
world coordinates, in metres. The receiving surface is a wall, roof or tilted rectangle (or a
triangle) with its local coordinate system, lower-left vertex as origin and EnergyPlus vertex
order; the caster is an overhang above it, a side fin, a wall in front parallel to it (whose
shadow is an axis-aligned rectangle, so SlaterBarsky's CLIPRECT runs), a tilted panel, a
triangle, a panel crossing the receiving plane (CLIP cuts it), one wholly behind the plane, or one
attached at the receiving surface's edges and corners (the 6-unit vertex snap). Some vertices
are on the 0.01 m grid. The sun comes from all over the hemisphere in front, from behind, and
in the receiving plane (no projection); a few suns are too low for the 4x projection guard. Each
scene is asked for the sunlit area, status, clipped and overlapped vertex counts, areas,
transmittance, projections and several of the overlap's vertices.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261208)


def out(*args):
    print(json.dumps({"fn": "ShadowCast", "args": list(args)}))


def add(*vs):
    return tuple(sum(c) for c in zip(*vs))


def mul(k, v):
    return tuple(k * c for c in v)


def frame(az, tilt):
    """lcsx, lcsy, lcsz of a surface with outward normal at azimuth az and tilt (degrees)."""
    a, t = math.radians(az), math.radians(tilt)
    n = (math.sin(a) * math.sin(t), math.cos(a) * math.sin(t), math.cos(t))
    lx = (math.cos(a), -math.sin(a), 0.0)
    ly = (n[1] * lx[2] - n[2] * lx[1], n[2] * lx[0] - n[0] * lx[2], n[0] * lx[1] - n[1] * lx[0])
    return lx, ly, n


def maybe_grid(v):
    return tuple(round(c, 2) for c in v) if rng.random() < 0.3 else v


def pad(vs):
    q = list(vs) + [(0.0, 0.0, 0.0)] * (4 - len(vs))
    return [c for v in q for c in v]


def scene():
    az = rng.choice([0.0, 90.0, 180.0, 270.0, rng.uniform(0, 360)])
    tilt = rng.choice([90.0, 90.0, 0.0, 30.0, rng.uniform(5, 175)])
    lx, ly, lz = frame(az, tilt)
    w = rng.choice([1.0, 2.5, 4.0, rng.uniform(0.3, 8.0)])
    h = rng.choice([1.0, 2.7, 3.0, rng.uniform(0.3, 6.0)])
    p = maybe_grid((rng.uniform(-10, 10), rng.uniform(-10, 10), rng.uniform(0, 10)))
    at = lambda x, y, d=0.0: add(p, mul(x, lx), mul(y, ly), mul(d, lz))  # noqa: E731
    if rng.random() < 0.15:  # triangular receiving surface
        recv = [at(0, h), at(0, 0), at(w, 0)]
    else:
        recv = [at(0, h), at(0, 0), at(w, 0), at(w, h)]
    k = rng.randrange(9)
    if k == 0:  # overhang along the top edge
        x0, x1 = rng.uniform(-0.5, 0.3) * w, rng.uniform(0.7, 1.5) * w
        e, dep = rng.choice([0.0, 0.0, rng.uniform(0, 0.5)]), rng.uniform(0.2, 2.0)
        cast = [at(x0, h + e, dep), at(x0, h + e, 0), at(x1, h + e, 0), at(x1, h + e, dep)]
    elif k == 1:  # side fin
        x = rng.choice([0.0, w, rng.uniform(-0.5, 1.5) * w])
        dep = rng.uniform(0.2, 2.0)
        y0, y1 = rng.uniform(-0.5, 0.2) * h, rng.uniform(0.8, 1.5) * h
        cast = [at(x, y1, 0), at(x, y0, 0), at(x, y0, dep), at(x, y1, dep)]
    elif k == 2:  # wall in front, parallel
        d = rng.uniform(0.5, 6.0)
        x0, y0 = rng.uniform(-1, 0.8) * w, rng.uniform(-1, 0.8) * h
        cw, ch = rng.uniform(0.2, 1.5) * w, rng.uniform(0.2, 1.5) * h
        cast = [at(x0, y0 + ch, d), at(x0 + cw, y0 + ch, d), at(x0 + cw, y0, d), at(x0, y0, d)]
    elif k == 3:  # tilted panel in front
        d0, d1 = rng.uniform(0.2, 3), rng.uniform(0.2, 3)
        x0, x1 = rng.uniform(-1, 0.5) * w, rng.uniform(0.5, 2) * w
        y0, y1 = rng.uniform(-1, 0.5) * h, rng.uniform(0.5, 2) * h
        cast = [at(x0, y1, d1), at(x0, y0, d0), at(x1, y0, d0), at(x1, y1, d1)]
    elif k == 4:  # triangle in front
        cast = [at(rng.uniform(-1, 2) * w, rng.uniform(-1, 2) * h, rng.uniform(0.1, 4)) for _ in range(3)]
    elif k == 5:  # crossing the receiving plane
        x0, x1 = rng.uniform(-1, 0.5) * w, rng.uniform(0.5, 2) * w
        y0, y1 = rng.uniform(-1, 0.5) * h, rng.uniform(0.5, 2) * h
        ds = [rng.uniform(-2, 2) for _ in range(4)]
        if rng.random() < 0.3:
            ds[rng.randrange(4)] = 0.0  # one vertex on the plane
        cast = [at(x0, y1, ds[0]), at(x0, y0, ds[1]), at(x1, y0, ds[2]), at(x1, y1, ds[3])]
    elif k == 6:  # wholly behind the plane, or lying in it
        d = rng.choice([-rng.uniform(0.1, 3), 0.0])
        cast = [at(0.2 * w, 0.8 * h, d), at(0.2 * w, 0.2 * h, d), at(0.8 * w, 0.2 * h, d), at(0.8 * w, 0.8 * h, d)]
    elif k == 7:  # attached at the corners: shadow vertices on the receiving ones
        dep = rng.uniform(0.1, 1.5)
        cast = [at(0, h, 0), at(w, h, 0), at(w, h, dep), at(0, h, dep)]
        if rng.random() < 0.5:
            cast = [at(0, 0, 0), at(0, h, 0), at(0, h, dep), at(0, 0, dep)]
    else:  # far away: no overlap
        d = rng.uniform(0.5, 3)
        cast = [at(3 * w, 3 * h, d), at(3 * w, 2 * h, d), at(4 * w, 2 * h, d), at(4 * w, 3 * h, d)]
    if rng.random() < 0.5:
        cast = cast[::-1]
    cast = [maybe_grid(v) for v in cast]
    return recv, cast, p, (lx, ly, lz), w * h if len(recv) == 4 else w * h / 2


def sun(frm):
    lx, ly, lz = frm
    k = rng.randrange(10)
    if k == 0:  # in the receiving plane: no projection
        a = rng.uniform(0, 2 * math.pi)
        return add(mul(math.cos(a), lx), mul(math.sin(a), ly))
    if k == 1:  # from behind
        a, b = rng.uniform(0, 2 * math.pi), rng.uniform(0.3, 1.0)
        s = add(mul(-b, lz), mul(math.sqrt(1 - b * b) * math.cos(a), lx), mul(math.sqrt(1 - b * b) * math.sin(a), ly))
        return s
    if k == 2:  # normal to the surface
        return lz
    # in front, at least 15 degrees above the plane (so the projection stays under 4)
    a, b = rng.uniform(0, 2 * math.pi), rng.uniform(math.sin(math.radians(15)), 1.0)
    if k == 3:
        b = rng.uniform(0.01, 0.2)  # low: often past the guard
    c = math.sqrt(max(0.0, 1 - b * b))
    return add(mul(b, lz), mul(c * math.cos(a), lx), mul(c * math.sin(a), ly))


for _ in range(1400):
    recv, cast, p, frm, area = scene()
    s = sun(frm)
    algo = 0 if rng.random() < 0.6 else 1
    sched = rng.choice([0.0, 0.0, 0.0, 0.5, 1.0, rng.random()])
    if rng.random() < 0.2:
        area *= rng.uniform(0.3, 0.9)  # a subsurface taken out of the net area
    whiches = list(range(12)) + rng.sample(range(12, 42), 4)
    lx, ly, lz = frm
    for wch in whiches:
        out(wch, algo, len(recv), *pad(recv), len(cast), *pad(cast), *p, *lx, *ly, *lz, *s, sched, area)
