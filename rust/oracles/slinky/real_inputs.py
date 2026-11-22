"""Inputs for the Slinky unit, one {"fn": name, "args": [...]} per line, for
`rust-anything record --inputs`. They are the unit's whole recording (it is recorded with
`--count 0`): generated integers could ask for fields of millions of rings, which the C++ would
take hours over, so the edge cases are listed here instead.

Geometry is in the range of real slinky fields (the GSHP-Slinky example has 1 m coils at 0.2 m
pitch, 15 trenches 2 m apart, soil diffusivity 4.4e-7 m2/s, a 26.67 mm pipe), times run from the
first g-function (36 s) to ten years, and both configurations are covered. Edge cases: rings
outside the field, no trenches or coils, one or two integration points, zero, negative and NaN
times, zero diffusivity and coil diameter, a single trench, odd and even counts, and g-function
indices outside the range. Deterministic. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261122)
TWO_PI = 2 * math.pi
NAN = {"$num": "NaN"}


def emit(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def geom(max_trenches=15, max_coils=60):
    return [rng.choice([0, 1]), rng.choice([1.0, rng.uniform(0.6, 1.2)]), rng.choice([0.2, rng.uniform(0.2, 0.6)]),
            rng.uniform(1.0, 2.5), rng.choice([2.0, rng.uniform(1.0, 4.0)]), rng.randint(1, max_trenches),
            rng.randint(1, max_coils), rng.uniform(4e-7, 1e-6), rng.choice([0.013335, rng.uniform(0.0125, 0.02)])]


def rings(g):
    nt, nc = g[5], g[6]
    m, m1 = rng.randint(1, nt), rng.randint(1, nt)
    n = rng.randint(1, nc)
    n1 = n if rng.random() < 0.3 else rng.randint(1, nc)  # often the ring itself
    return [m, n, m1, n1]


def time():
    return 10 ** rng.uniform(math.log10(36.0), math.log10(3.2e8))


def angle():
    return rng.choice([0.0, math.pi, TWO_PI, rng.uniform(0.0, TWO_PI)])


# Realistic cases.
for _ in range(80):
    g = geom()
    r = rings(g)
    emit("SlinkyDistToCenter", *g, *r)
    emit("SlinkyDistance", *g, *r, angle(), angle())
    emit("SlinkyDistanceToFictRing", *g, *r, angle(), angle())
    emit("SlinkyNearFieldResponse", *g, *r, angle(), angle(), time())
    emit("SlinkyMidFieldResponse", *g, *r, time())
for _ in range(40):
    g = geom()
    emit("SlinkyIntegral", *g, *rings(g), time(), angle(), rng.choice([561, 1089, rng.randint(2, 200)]))
for _ in range(30):
    g = geom()
    emit("SlinkyDoubleIntegral", *g, *rings(g), time(), 33, rng.choice([561, 1089]))
for _ in range(10):
    g = geom()
    emit("SlinkyDoubleIntegral", *g, *rings(g), time(), rng.randint(2, 40), rng.randint(2, 120))
# g-functions of small fields (all near or mid field), each index once in a while.
for _ in range(36):
    g = geom(max_trenches=3, max_coils=8)
    years = rng.choice([1.0, 10.0, rng.uniform(0.001, 20.0)])
    n = int((math.log10(years * 356.0 * 24 * 3600 / 3600.0) + 2.0) / 0.25 + 1)
    emit("SlinkyGFunction", *g, years, rng.randint(0, n - 1))

# Edge cases, on one realistic geometry and its variants.
base = [0, 1.0, 0.2, 1.5, 2.0, 3, 10, 4.4e-7, 0.013335]
vert = [1] + base[1:]
for g in (base, vert):
    for r in ([1, 1, 1, 1], [3, 10, 1, 1], [0, 1, 1, 1], [4, 1, 1, 1], [1, 0, 1, 1], [1, 11, 1, 1],
              [1, 1, 0, 1], [1, 1, 4, 1], [1, 1, 1, 0], [1, 1, 1, 11], [-1, 1, 1, 1]):
        emit("SlinkyDistToCenter", *g, *r)
        emit("SlinkyDistance", *g, *r, 0.5, 1.5)
        emit("SlinkyNearFieldResponse", *g, *r, 0.5, 1.5, 3600.0)
        emit("SlinkyMidFieldResponse", *g, *r, 3600.0)
    for t in (0.0, -1.0, NAN, 1e30):
        emit("SlinkyNearFieldResponse", *g, 1, 1, 1, 1, 0.5, 1.5, t)
        emit("SlinkyMidFieldResponse", *g, 1, 1, 1, 2, t)
        emit("SlinkyIntegral", *g, 1, 1, 1, 1, t, 0.5, 9)
    for J0 in (-1, 0, 1, 2, 3):
        emit("SlinkyIntegral", *g, 1, 1, 1, 2, 3600.0, 0.5, J0)
    for I0 in (-1, 0, 1, 2, 3, 4, 5):
        emit("SlinkyDoubleIntegral", *g, 1, 1, 1, 2, 3600.0, I0, 9)
    for k in (-1, 0, 5, 27, 28, 100):
        emit("SlinkyGFunction", *g[:5], 1, 3, *g[7:], 10.0, k)
    for years in (0.0, -1.0, NAN, 1e-6, 1e-4, 1e9):
        emit("SlinkyGFunction", *g[:5], 1, 2, *g[7:], years, 0)
for g in ([0, 0.0, 0.2, 1.5, 2.0, 2, 4, 4.4e-7, 0.013335], [0, 1.0, 0.2, 1.5, 2.0, 2, 4, 0.0, 0.013335],
          [0, 1.0, 0.2, 1.5, 2.0, 0, 4, 4.4e-7, 0.013335], [0, 1.0, 0.2, 1.5, 2.0, 2, 0, 4.4e-7, 0.013335],
          [1, 1.0, 0.2, 0.0, 2.0, 1, 5, 4.4e-7, 0.013335], [0, 1.0, 0.2, 1.5, 2.0, 1, 1, 4.4e-7, 0.013335],
          [1, 1.0, 0.2, 1.5, 2.0, 2, 6, 4.4e-7, 0.013335], [0, 1.0, 0.2, 1.5, 2.0, 3, 7, 4.4e-7, 0.013335]):
    emit("SlinkyDistance", *g, 1, 1, 1, 1, 0.0, 0.0)
    emit("SlinkyNearFieldResponse", *g, 1, 1, 1, 1, 0.0, 0.0, 3600.0)
    emit("SlinkyDoubleIntegral", *g, 1, 1, 1, 1, 3600.0, 5, 9)
    emit("SlinkyGFunction", *g, 0.5, 3)
