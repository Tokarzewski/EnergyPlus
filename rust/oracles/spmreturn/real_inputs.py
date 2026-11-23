"""Inputs for SetPointManager's return water temperature and system node reset managers as
EnergyPlus meets them: chilled water loops at 4 to 12 C supply and hot water loops at 30 to 80 C
with 0 to 50 kg/s (some off), return temperatures either side of the supply, targets constant,
scheduled or taken from the return node's set point, and supply limits that bind or not; and
system node resets on a node's temperature (-20 to 40 C) or humidity ratio (0.002 to 0.02) between
low and high references (ordered, equal or crossed), set points rising or falling. Both managers'
cases are interleaved.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261221)


def pick(*xs):
    return rng.choice(xs)


def out(fn, args):
    print(json.dumps({"fn": fn, "args": args}))


for _ in range(6750):
    hot = rng.randint(0, 1)
    if hot:
        tsup = rng.uniform(30.0, 80.0)
        tret = tsup + pick(-rng.uniform(0.0, 20.0), rng.uniform(0.0, 3.0), 0.0)
        target = rng.uniform(30.0, 70.0)
        mint, maxt = pick((rng.uniform(25.0, 50.0), rng.uniform(50.0, 85.0)), (40.0, 40.0))
    else:
        tsup = rng.uniform(4.0, 12.0)
        tret = tsup + pick(rng.uniform(0.0, 10.0), -rng.uniform(0.0, 3.0), 0.0)
        target = rng.uniform(10.0, 18.0)
        mint, maxt = pick((rng.uniform(3.0, 7.0), rng.uniform(7.0, 15.0)), (7.0, 7.0))
    mdot = pick(rng.uniform(0.0, 50.0), 0.0, 0.0005, 0.001)
    out("RetWater", [hot, rng.randint(0, 2), mdot, tsup, tret, target, target + rng.uniform(-2.0, 2.0), mint, maxt])
    ctrl = rng.randint(0, 6)
    if 3 <= ctrl <= 5:
        lo, hi = sorted([rng.uniform(0.002, 0.02), rng.uniform(0.002, 0.02)])
        ref = pick(rng.uniform(0.0, 0.025), lo, hi)
        sp = [rng.uniform(0.004, 0.015), rng.uniform(0.004, 0.015)]
    else:
        lo, hi = sorted([rng.uniform(-20.0, 40.0), rng.uniform(-20.0, 40.0)])
        ref = pick(rng.uniform(-25.0, 45.0), lo, hi)
        sp = [rng.uniform(10.0, 40.0), rng.uniform(10.0, 40.0)]
    if rng.random() < 0.1:
        lo, hi = pick((hi, lo), (lo, lo))
    out("SysNode", [ctrl, ref, lo, hi] + sp)
