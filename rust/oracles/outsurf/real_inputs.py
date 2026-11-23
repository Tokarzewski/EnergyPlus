"""Inputs for HeatBalanceSurfaceManager::CalcOutsideSurfTemp: the outside face heat balance of
one exterior surface as EnergyPlus meets it, in SI units. Walls and roofs are slow or quick
conducting (first cross CTF terms below and above 0.01), with and without embedded sources or
sinks, exterior movable insulation, an other side conditions model, a surrounding surfaces
property (with and without sky and ground temperature schedules) and ground surface temperatures.
Coefficients, CTF terms and fluxes span real constructions by day and night, summer and winter.
Each surface is asked for all five results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261202)


def pick(*xs):
    return rng.choice(xs)


for _ in range(2500):
    quick = rng.random() < 0.4
    ctfcross = rng.uniform(0.02, 3.0) if quick else pick(0.0, rng.uniform(1e-6, 0.0099), 0.01)
    ctfinside = ctfcross + rng.uniform(0.5, 8.0)
    ctfoutside = ctfcross + rng.uniform(0.5, 8.0)
    sourcesink = 1 if rng.random() < 0.12 else 0
    ctfsourceout = rng.uniform(0.0, 0.5) if sourcesink else 0.0
    ctfsourcein = rng.uniform(0.0, 0.5) if sourcesink else 0.0
    hconvint = rng.uniform(0.3, 5.0)
    hmovinsul = 0.0 if (sourcesink or rng.random() < 0.85) else rng.uniform(0.1, 5.0)
    outdrybulb = rng.uniform(-20.0, 38.0)
    tempext = outdrybulb + pick(0.0, 0.0, rng.uniform(-1.0, 1.0))
    skytemp = outdrybulb - rng.uniform(5.0, 25.0)
    hassrd = 1 if rng.random() < 0.15 else 0
    srdskyon = 1 if hassrd and rng.random() < 0.5 else 0
    srdgndon = 1 if hassrd and rng.random() < 0.5 else 0
    srdskyval = skytemp + rng.uniform(-3, 3)
    srdgndval = outdrybulb + rng.uniform(-5, 5)
    srdsurftemp = outdrybulb + rng.uniform(-5, 10) if hassrd else 0.0
    usegndsurf = 1 if rng.random() < 0.08 else 0
    gndsurftavg = outdrybulb + rng.uniform(-5, 15)
    oscm = 1 if rng.random() < 0.08 else 0
    oscmtrad = outdrybulb + rng.uniform(-5, 10)
    oscmhrad = rng.uniform(0.0, 6.0)
    hconvext = rng.uniform(2.0, 40.0)
    hairext = rng.uniform(0.0, 3.0)
    hskyext = rng.uniform(0.0, 4.0)
    hgrdext = rng.uniform(0.0, 3.0)
    hsrdsurfext = rng.uniform(0.0, 2.0) if hassrd else 0.0
    qaddout = 0.0 if rng.random() < 0.9 else rng.uniform(-50, 200)
    ctfconstout = rng.uniform(-200.0, 200.0)
    qradswoutabs = pick(0.0, rng.uniform(0.0, 800.0))
    tempin = rng.uniform(15.0, 30.0)
    qsrchist = rng.uniform(-50.0, 300.0) if sourcesink else 0.0
    ctfconstin = rng.uniform(-200.0, 200.0)
    qradswinabs = rng.uniform(0.0, 50.0)
    qdotradintgainsin = rng.uniform(0.0, 20.0)
    mat = rng.uniform(16.0, 30.0)
    qdotradnetlwin = rng.uniform(-30.0, 30.0)
    qradswoutmvins = rng.uniform(0.0, 600.0) if hmovinsul > 0 else 0.0
    args = [ctfcross, ctfinside, ctfoutside, ctfsourceout, ctfsourcein, sourcesink, hconvint, hmovinsul, tempext,
            skytemp, outdrybulb, hassrd, srdskyon, srdskyval, srdgndon, srdgndval, srdsurftemp, usegndsurf,
            gndsurftavg, oscm, oscmtrad, oscmhrad, hconvext, hairext, hskyext, hgrdext, hsrdsurfext, qaddout,
            ctfconstout, qradswoutabs, tempin, qsrchist, ctfconstin, qradswinabs, qdotradintgainsin, mat,
            qdotradnetlwin, qradswoutmvins]
    for which in range(5):
        print(json.dumps({"fn": "OutsideSurfTemp", "args": [which] + args}))
