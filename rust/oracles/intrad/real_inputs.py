"""Inputs for interior radiation exchange: box zones from closets to halls, with and without a
roof, walls left out (three-surface and two-surface enclosures), internal masses from small to
larger than the rest of the zone (the self-view branch), rotations near the 10 degree same-facing
limit, and emissivities up to and past the 0.99999 limit.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261128)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


MASKS = [63, 127, 63, 127, 7, 3, 61, 31, 15, 64 + 3, 1 + 4 + 16, 64 + 1, 126]


def zone():
    mask = rng.choice(MASKS)
    L, W, H = rng.choice([3.0, 5.0, 10.0, 30.0, rng.uniform(1, 50)]), rng.choice([3.0, 4.0, rng.uniform(1, 40)]), rng.choice([2.7, 3.0, rng.uniform(2, 10)])
    mass = rng.choice([0.0, 10.0, 50.0, 500.0, 5000.0, rng.uniform(0, 300)])
    rot = rng.choice([0.0, 0.0, 30.0, 5.0, 9.99, 10.01, 355.0, rng.uniform(0, 360)])
    return mask, L, W, H, mass, rot


def n_of(mask):
    return bin(mask).count("1")


for _ in range(120):
    mask, L, W, H, mass, rot = zone()
    n = n_of(mask)
    for fixed in (0, 1):
        for _ in range(4):
            out("IRView", fixed, rng.randint(1, n), rng.randint(1, n), mask, L, W, H, mass, rot)
    for j in range(1, 6):
        out("IRView", 1, 0, j, mask, L, W, H, mass, rot)
    e0 = rng.choice([0.9, 0.85, 0.1, 0.99999, rng.uniform(0.05, 0.95)])
    de = rng.choice([0.0, 0.01, -0.05, 0.002])
    for _ in range(4):
        out("IRScriptF", rng.randint(1, n), rng.randint(1, n), mask, L, W, H, mass, rot, e0, de)
    for fp in (0, 1):
        out("IRFmrt", fp, rng.randint(1, n), mask, L, W, H, mass, e0, de)
# A whole matrix of each kind for one ordinary and one massive zone.
for args in [(127, 5.0, 4.0, 3.0, 40.0, 0.0), (127, 5.0, 4.0, 3.0, 4000.0, 20.0), (7, 5.0, 4.0, 3.0, 0.0, 0.0)]:
    n = n_of(args[0])
    for i in range(1, n + 1):
        for j in range(1, n + 1):
            out("IRView", 1, i, j, *args)
            out("IRScriptF", i, j, *args, 0.9, 0.0)
