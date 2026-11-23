"""Inputs for UpdateThermalHistories: one opaque surface's CTF history update as EnergyPlus meets
it, in SI units. Most models are "simple CTF" (construction time step equal to the zone time step,
no sources and no construction overrides), so the histories shift by one term; the rest have
constructions whose time step spans several zone time steps (the master histories, updated at the
end of a series and interpolated between), construction overrides, or an embedded source or sink
(a radiant slab), with the surface anywhere in its series. Constructions have 1 to 7 CTF terms
(a few none) and the model shifts as many terms as its longest construction; a few surfaces use
EMPD (updated like CTF) or CondFD (skipped). Zone time steps from 1 to 60 minutes; face
temperatures, CTF terms and constant parts span winter to summer. Each surface is asked for the
face conduction outputs, the source temperatures, its series position and a sample of history
terms, biased to the terms the update touches.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261123)


def pick(*xs):
    return rng.choice(xs)


def maybe_round(v, nd=2):
    return round(v, nd) if rng.random() < 0.3 else v


for _ in range(500):
    seed = rng.randint(-2**31, 2**31 - 1)
    n = pick(0, 1, 2, 3, 3, 4, 4, 5, 6, 7) if rng.random() < 0.97 else 0
    if n == 0 and rng.random() < 0.7:
        n = rng.randint(1, 7)
    m = rng.randint(max(n, 1), 7) if rng.random() < 0.6 else max(n, 1)
    src = 1 if rng.random() < 0.15 else 0
    anysrc = 1 if src or rng.random() < 0.15 else 0
    algo = 0 if rng.random() < 0.88 else pick(1, 1, 2)
    tsz = pick(1.0, 0.5, 0.25, 1 / 6, 0.25, 1 / 6, 0.1, 1 / 12, 1 / 60)
    numhist = 1 if rng.random() < 0.55 else rng.randint(2, 6)
    simple = 1 if numhist == 1 and not anysrc and rng.random() < 0.85 else 0
    overrides = 1 if rng.random() < 0.08 else 0
    ctfts = numhist * tsz if rng.random() < 0.9 else tsz * rng.uniform(1.0, 6.0)
    curnum = rng.randint(0, numhist - 1)
    area = pick(rng.uniform(0.5, 5.0), rng.uniform(5.0, 60.0), rng.uniform(60.0, 900.0))
    tout = rng.uniform(-30.0, 60.0)
    tin = rng.uniform(10.0, 35.0)
    cross0 = pick(rng.uniform(0.0, 0.01), rng.uniform(0.01, 1.0), rng.uniform(1.0, 6.0))
    inside0 = cross0 + rng.uniform(0.3, 12.0)
    outside0 = cross0 + rng.uniform(0.3, 12.0)
    ctfin = rng.uniform(-50.0, 50.0) * (inside0 + 0.1)
    ctfout = rng.uniform(-50.0, 50.0) * (outside0 + 0.1)
    if src:
        qsrc = rng.uniform(-60.0, 250.0)
        srcin0, srcout0 = rng.uniform(0.0, 1.5), rng.uniform(0.0, 1.5)
        tsrcout0, tsrcin0, tsrcq0 = rng.uniform(0.0, 1.0), rng.uniform(0.0, 1.0), rng.uniform(0.0, 0.2)
        tuserout0, tuserin0, tusersrc0 = rng.uniform(0.0, 1.0), rng.uniform(0.0, 1.0), rng.uniform(0.0, 0.2)
        tsrcconst, tuserconst = rng.uniform(-20.0, 40.0), rng.uniform(-20.0, 40.0)
    else:
        qsrc = rng.uniform(-60.0, 250.0) if anysrc and rng.random() < 0.3 else 0.0
        srcin0 = srcout0 = tsrcout0 = tsrcin0 = tsrcq0 = tuserout0 = tuserin0 = tusersrc0 = 0.0
        tsrcconst = tuserconst = 0.0
    args = [seed, n, m, simple, overrides, anysrc, src, algo, curnum, numhist, tsz, ctfts]
    args += [maybe_round(v) for v in (area, tout, tin, ctfin, ctfout, qsrc, tsrcconst, tuserconst, cross0, inside0,
                                      outside0, srcin0, srcout0, tsrcout0, tsrcin0, tsrcq0, tuserout0, tuserin0,
                                      tusersrc0)]
    top = min(8, max(n, m) + 2)
    whichs = {112 + k for k in range(8)}
    whichs |= {8 * rng.randint(0, 7) + rng.randint(0, top - 1) for _ in range(12)}
    whichs |= {8 * rng.randint(8, 13) + rng.randint(0, top - 1) for _ in range(4 if anysrc else 1)}
    whichs |= {120 + rng.randint(0, 7) for _ in range(2)}
    for which in sorted(whichs):
        print(json.dumps({"fn": "ThermalHist", "args": [which] + args}))
