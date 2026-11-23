"""Inputs for PlantUtilities' node helpers as EnergyPlus meets them: component design flows of 0
to 100 kg/s (and the negative autosize leftovers InitComponentNodes floors at zero); water and
steam inlet nodes at -20 to 200 C with flow limits that are ordered, equal, crossed or zero
(including signed zeros, which ObjexxFCL's min and max keep); flows bounded by node limits and by
two values, inside, below, above and on them; and available-flow ranges tightened by new limits
inside, outside and crossing them. The five routines' cases are interleaved.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261218)


def pick(*xs):
    return rng.choice(xs)


def fl(hi=100.0):
    return pick(rng.uniform(0.0, hi), rng.uniform(0.0, 1.0), 0.0, -0.0, round(rng.uniform(0.0, hi), 1))


def out(fn, args):
    print(json.dumps({"fn": fn, "args": args}))


for _ in range(642):
    minm = pick(fl(), -rng.uniform(0.0, 5.0), -999.0, -0.0)
    maxm = pick(fl(), minm, -rng.uniform(0.0, 5.0), -0.0)
    for which in range(7):
        out("InitNodes", [which, minm, maxm])
    t = pick(rng.uniform(-20.0, 99.0), rng.uniform(100.0, 200.0))
    mmin, mmax = fl(), fl()
    mminav, mmaxav = pick(mmin, fl()), pick(mmax, fl(), -0.0)
    args = [pick(0, 1, 2), t, fl(), pick(0.0, rng.uniform(0.0, 1.0), 1.0), rng.uniform(-1.0e5, 3.0e6),
            pick(-20.0, 0.0, rng.uniform(-50.0, 10.0)), pick(100.0, 200.0, rng.uniform(50.0, 300.0)),
            mmin, mminav, mmax, mmaxav, pick(0.0, rng.uniform(0.0, 0.02)), pick(101325.0, rng.uniform(1.0e5, 6.0e5)),
            pick(101325.0, rng.uniform(1.0e5, 6.0e5))]
    for which in range(10):
        out("SafeCopy", [which] + args)
    lo, hi = fl(), fl()
    v = pick(fl(), lo, hi, -0.0, 0.0, lo - 1.0, hi + 1.0)
    out("BoundNode", [v, lo, hi])
    out("BoundTwo", [v, lo, hi])
    oldmin, oldmax = sorted([fl(), fl()]) if rng.random() < 0.8 else [fl(), fl()]
    newmin = pick(fl(), oldmin, oldmax, oldmin + 0.1, oldmax + 0.1)
    newmax = pick(fl(), oldmin, oldmax, oldmax - 0.1, oldmin - 0.1)
    for which in range(2):
        out("Tighten", [which, oldmin, oldmax, newmin, newmax])
for v, lo, hi in [(0.0, -0.0, 0.0), (-0.0, 0.0, -0.0), (0.0, 0.0, -0.0), (-0.0, -0.0, 0.0), (1.0, 2.0, 0.5), (0.5, 0.0, -0.0)]:
    for fn in ("BoundNode", "BoundTwo"):
        out(fn, [v, lo, hi])
for which in range(6):
    out("Tighten", [which % 2, 0.0, -0.0, -0.0, 0.0])
