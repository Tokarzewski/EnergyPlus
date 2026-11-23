"""Inputs for the ventilation part of ZoneEquipmentManager::CalcAirFlowSimple for one zone as
EnergyPlus meets it, in SI units: ZoneVentilation:DesignFlowRate objects (natural, intake, exhaust
and balanced fans with typical pressure rises and efficiencies, the usual coefficient sets) and
ZoneVentilation:WindandStackOpenArea objects (openings facing every direction, autocalculated and
given effectiveness and discharge coefficients), indoor, outdoor and delta temperature limits as
values or schedules, wind speed limits, hybrid ventilation closing, EMS overrides, all three
density bases, zones with and without an outdoor air node, the first and later system time steps,
winter, spring and summer. Each case is asked for all eight results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261207)
AUTO = -99999.0


def pick(*xs):
    return rng.choice(xs)


def h_fn_tdb_w(t, w):
    return 1.00484e3 * t + max(w, 1.0e-5) * (2.50094e6 + 1.85895e3 * t)


for _ in range(1700):
    nvent = 1 if rng.random() < 0.95 else 0
    mod = 0 if rng.random() < 0.7 else 1
    den = rng.choices([0, 1, 2], weights=[70, 15, 15])[0]
    avail = pick(1.0, 1.0, 0.0, 0.5, rng.uniform(0.0, 1.0))
    dl = pick(rng.uniform(0.01, 2.0), rng.uniform(0.0, 0.3))
    c0, ct, cv, cvv = pick((1.0, 0.0, 0.0, 0.0), (0.606, 0.03636, 0.1177, 0.0),
                           (rng.uniform(0.0, 1.0), rng.uniform(0.0, 0.05), rng.uniform(0.0, 0.3), rng.uniform(0.0, 0.02)))
    fantype = rng.choices([0, 1, 2, 3], weights=[40, 20, 25, 15])[0]
    fanp = 0.0 if fantype == 0 else pick(67.0, 125.0, rng.uniform(0.0, 400.0))
    fane = 1.0 if fantype == 0 else pick(0.7, 0.5, 0.0, rng.uniform(0.3, 0.9))
    openarea = rng.uniform(0.1, 5.0)
    openfrac = pick(1.0, 0.5, rng.uniform(0.0, 1.0))
    openeff = pick(AUTO, AUTO, 0.55, rng.uniform(0.25, 0.6))
    disc = pick(AUTO, AUTO, 0.6, rng.uniform(0.4, 0.7))
    effangle = rng.uniform(0.0, 360.0)
    dh = rng.uniform(0.0, 3.0)
    scheds = 1 if rng.random() < 0.3 else 0
    mint = pick(-100.0, 18.0, 21.0, rng.uniform(10.0, 24.0))
    maxt = pick(100.0, 26.0, rng.uniform(22.0, 30.0))
    minot = pick(-100.0, 12.0, rng.uniform(-5.0, 18.0))
    maxot = pick(100.0, 28.0, rng.uniform(20.0, 35.0))
    delt = pick(-100.0, 1.0, 2.0, rng.uniform(-5.0, 5.0))
    maxwind = pick(40.0, 40.0, rng.uniform(3.0, 15.0))
    hybrid = 1 if rng.random() < 0.08 else 0
    ems = 1 if rng.random() < 0.08 else 0
    emsflow = rng.uniform(-0.05, 1.0)
    systs = pick(1, 1, 2, 3)
    outdb = rng.uniform(-15.0, 36.0)
    mat = rng.uniform(16.0, 30.0)
    hum = rng.uniform(0.003, 0.015)
    xmpt = mat + rng.uniform(-1.0, 1.0)
    wprev = hum * rng.uniform(0.95, 1.05)
    wind = pick(0.0, rng.uniform(0.0, 14.0))
    winddir = rng.uniform(0.0, 360.0)
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    outhum = rng.uniform(0.0008, 0.02)
    outenth = h_fn_tdb_w(outdb, outhum)
    linked = 1 if rng.random() < 0.1 else 0
    linkedhum = outhum * rng.uniform(0.9, 1.1)
    linkedenth = h_fn_tdb_w(outdb + rng.uniform(-1.0, 1.0), linkedhum)
    stdrho = pick(1.2041, rng.uniform(1.0, 1.21))
    tsyssec = pick(900.0, 600.0, 300.0, 60.0)
    args = [nvent, mod, den, avail, dl, c0, ct, cv, cvv, fantype, fanp, fane, openarea, openfrac, openeff, disc,
            effangle, dh, scheds, mint, maxt, minot, maxot, delt, maxwind, hybrid, ems, emsflow, systs, mat, hum,
            xmpt, wprev, outdb, wind, winddir, outbaro, outhum, outenth, linked, linkedhum, linkedenth, stdrho,
            tsyssec]
    for which in range(8):
        print(json.dumps({"fn": "Ventilation", "args": [which] + args}))
