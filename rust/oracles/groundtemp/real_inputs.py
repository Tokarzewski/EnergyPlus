"""Inputs for the ground temperature models: depths from the surface to 100 m, every month and
months outside 1 to 12, times through the year and past it (where the models fold time with
remainder), common and leap years, and soils and climates as the input objects give them.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261126)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def depth():
    return rng.choice([0.0, 0.5, 1.0, 2.0, 4.0, 10.0, 100.0, rng.uniform(0, 20)])


def seconds():
    return rng.choice([0.0, 3600.0, 86400.0, 1.5e7, 3.1536e7, 3.1622e7, 3.2e7, 6.4e7, 1e9, rng.uniform(0, 3.2e7), rng.uniform(0, 3e8)])


def days():
    return rng.choice([365, 365, 366])


def diff():
    return rng.choice([3.0e-7, 5.0e-7, 7.0e-7, 1.0e-6, rng.uniform(2e-7, 1.5e-6)])


def diff_days():
    return rng.choice([0.026, 0.0432, 0.06, rng.uniform(0.02, 0.1)])  # m2/day, as Xing's input gives it


MONTHS = list(range(-13, 26))
for _ in range(150):
    ka = [rng.uniform(5, 20), rng.uniform(2, 15), rng.choice([0.0, 1.5e6, 2.6e6, rng.uniform(0, 3.1e7)]), diff()]
    out("GTKusudaSeconds", depth(), seconds(), days(), *ka)
    out("GTKusudaMonth", depth(), rng.choice(MONTHS), days(), *ka)
for _ in range(150):
    x = [rng.uniform(5, 20), rng.uniform(2, 15), rng.uniform(0, 365), rng.uniform(0, 3), rng.uniform(0, 365), diff_days()]
    out("GTXingSeconds", depth(), seconds(), days(), *x)
    out("GTXingMonth", depth(), rng.choice(MONTHS), days(), *x)
for kind in range(4):
    for m in MONTHS:
        out("GTSite", kind, 1, float(m), days(), rng.uniform(5, 18), rng.uniform(-1, 1))
    for _ in range(40):
        out("GTSite", kind, 0, seconds(), days(), rng.uniform(5, 18), rng.uniform(-1, 1))
    out("GTSite", kind, 1, 2.5, 365, 10.0, 0.5)
