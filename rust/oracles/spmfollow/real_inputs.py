"""Inputs for five SetPointManager calculations as EnergyPlus meets them: staged thermostats at
stages -4 to 4 with supply set points of 10 to 50 C; humidity-controlled zones with 0 to 3 kg/s
of supply (some below the small-flow cutoff), humidity ratios of 0.002 to 0.02 and moisture loads
that floor the result or not; DOAS pretreat with 0 to 100 percent outdoor air, temperature and
humidity ratio references (including the zero humidity set point that means off) and limits that
bind or not; and supply resets that follow the outdoor or a node's dry or wet bulb (with and
without the extra node data) plus offsets of -5 to 5 K, limited. The five managers' cases are
interleaved.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261219)


def pick(*xs):
    return rng.choice(xs)


def out(fn, args):
    print(json.dumps({"fn": fn, "args": args}))


def lims(lo, hi):
    a, b = rng.uniform(lo, hi), rng.uniform(lo, hi)
    return pick([min(a, b), max(a, b)], [lo, hi], [a, a])


for _ in range(2700):
    out("OneStage", [rng.randint(0, 1), rng.randint(-4, 4), rng.uniform(10.0, 50.0), rng.uniform(10.0, 50.0)])
    zonem = pick(rng.uniform(0.0, 3.0), rng.uniform(0.0, 0.002), 0.0, 0.001)
    zonew = rng.uniform(0.002, 0.02)
    out("SZHum", [rng.randint(0, 1), zonem, zonew, pick(rng.uniform(-1e-3, 1e-3), 0.0, rng.uniform(-0.05, 0.05)), pick(rng.uniform(-1e-3, 1e-3), 0.0, -rng.uniform(0.0, 0.05))])
    ctrl = rng.randint(0, 3)
    mixm = pick(rng.uniform(0.1, 5.0), 0.0)
    oam = pick(mixm * rng.uniform(0.05, 1.0), mixm, 0.0)
    if ctrl == 0:
        mn, mx = lims(-10.0, 40.0)
        out("Pretreat", [ctrl, rng.uniform(10.0, 25.0), rng.uniform(18.0, 30.0), mixm, oam, mn, mx])
    else:
        mn, mx = lims(0.00001, 0.03)
        out("Pretreat", [ctrl, pick(rng.uniform(0.004, 0.012), 0.0), rng.uniform(0.004, 0.015), mixm, oam, mn, mx])
    oadb = rng.uniform(-30.0, 45.0)
    oawb = oadb - rng.uniform(0.0, 15.0)
    mn, mx = lims(-10.0, 40.0)
    out("FollowOA", [rng.randint(0, 1), oadb, oawb, rng.uniform(-5.0, 5.0), mn, mx])
    nodet = rng.uniform(-20.0, 45.0)
    mn, mx = lims(-10.0, 40.0)
    out("FollowNode", [rng.randint(0, 1), pick(1, 1, 0), nodet, nodet - rng.uniform(0.0, 12.0), rng.uniform(-5.0, 5.0), mn, mx])
