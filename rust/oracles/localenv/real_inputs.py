"""Inputs for the local environment averages of HeatBalanceSurfaceManager:
GetGroundSurfacesTemperatureAverage, GetGroundSurfacesReflectanceAverage,
ReSetGroundSurfacesViewFactor and GetSurroundingSurfacesTemperatureAverage, for one exterior
surface as EnergyPlus meets them, in SI units. Surfaces have ground surfaces properties of one to
four ground surfaces (grass, paving, snow; with and without temperature and reflectance schedules;
some view factors zero) and surrounding surfaces properties of one to four neighbouring buildings
or trees, in models with and without local environments, summer and winter. Each case is asked for
all seven results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261204)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1800):
    op = rng.choices([0, 1, 2, 3], weights=[30, 25, 15, 30])[0]
    anylocal = 1 if rng.random() < 0.92 else 0
    gnddefined = 1 if rng.random() < 0.85 else 0
    ngnd = rng.choices([0, 1, 2, 3, 4], weights=[5, 35, 30, 15, 15])[0]
    outdoor = rng.uniform(-15.0, 38.0)
    gnd = []
    vfs = 0.0
    for i in range(4):
        vf = 0.0 if rng.random() < 0.1 else round(rng.uniform(0.05, 0.5), pick(2, 3, 6))
        if i < ngnd:
            vfs += vf
        gnd += [vf, 1 if rng.random() < 0.85 else 0, outdoor + rng.uniform(-5.0, 25.0),
                1 if rng.random() < 0.85 else 0, pick(0.2, 0.25, 0.3, 0.7, rng.uniform(0.05, 0.9))]
    vfsum = pick(vfs, vfs, 0.0, rng.uniform(0.1, 0.5))
    hassrd = 1 if rng.random() < 0.85 else 0
    nsrd = rng.choices([0, 1, 2, 3, 4], weights=[3, 40, 30, 15, 12])[0]
    srd = []
    svfs = 0.0
    for i in range(4):
        vf = round(rng.uniform(0.02, 0.4), pick(2, 3, 6))
        if i < nsrd:
            svfs += vf
        srd += [vf, outdoor + rng.uniform(-3.0, 20.0)]
    vfsrd = pick(svfs, svfs, rng.uniform(0.05, 0.6))
    vfgir = pick(0.5, 0.0, rng.uniform(0.1, 0.6))
    args = ([op, anylocal, gnddefined, vfsum, ngnd] + gnd + [hassrd, nsrd] + srd +
            [vfsrd, vfgir, rng.uniform(-10, 30), rng.uniform(0.1, 0.5), rng.uniform(-10, 30),
             pick(0, 1), pick(0, 1)])
    for which in range(7):
        print(json.dumps({"fn": "LocalEnvironment", "args": [which] + args}))
