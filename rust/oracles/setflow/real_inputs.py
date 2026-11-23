"""Inputs for PlantUtilities::SetComponentFlowRate as EnergyPlus meets it: pumps, boilers, chillers,
coils and other plant components on branches of one to three components, asking for 0 to 20 kg/s
on nodes whose hardware and available limits the loop solver has set (often equal, often zero,
now and then tighter downstream), with the loop side unlocked or locked, the loop still to be
sized or not, series active or active flow control, demand or other operation schemes, and EMS
overrides on some components. Each case is asked for all eighteen results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261204)


def pick(*xs):
    return rng.choice(xs)


def node(design):
    mx = pick(design, design, design * rng.uniform(0.5, 1.2), 0.0)
    mn = pick(0.0, 0.0, 0.0, mx * rng.uniform(0.0, 0.5), mx)
    maxav = pick(mx, mx, design * rng.uniform(0.0, 1.0), 0.0)
    minav = pick(0.0, 0.0, mn, maxav * rng.uniform(0.0, 0.5))
    req = pick(0.0, design * rng.uniform(0.0, 1.0), mx, design * rng.uniform(0.0, 1e-9))
    mdot = pick(0.0, req, maxav, design * rng.uniform(0.0, 1.0))
    return [mdot, mn, mx, minav, maxav, req]


for _ in range(750):
    design = pick(rng.uniform(0.01, 1.0), rng.uniform(1.0, 20.0), 0.5)
    ncomp = pick(1, 1, 2, 3)
    k = rng.randint(1, ncomp)
    lock = pick(0, 0, 1)
    autosize = pick(0, 0, 0, 0, 1)
    series = pick(0, 0, 1)
    demand = pick(0, 1)
    ems = pick(0, 0, 0, 0, rng.randint(0, 63))
    flow = pick(design * rng.uniform(0.0, 1.0), design * rng.uniform(0.0, 1.5), 0.0, design, design * rng.uniform(0.0, 1e-9))
    nodes = [node(design) for _ in range(4)]
    flat = [v for n in nodes for v in n]
    for which in range(18):
        print(json.dumps({"fn": "SetFlow", "args": [which, ncomp, k, lock, autosize, series, demand, ems, flow] + flat}))
