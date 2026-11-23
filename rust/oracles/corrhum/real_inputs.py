"""Inputs for ZoneSpaceHeatBalanceData::correctHumRat: one zone's new air humidity ratio as
EnergyPlus meets it, in SI units. Zones are controlled, with one to three supply air inlets
humidifying, dehumidifying or off (and controlled zones with none), or uncontrolled, with and
without a system node, under the third order, analytical or Euler zone air solution. Air states,
latent gains, infiltration, ventilation, earth tube, cooling tower, mixing and outdoor air flows,
convective moisture terms, volumes, multipliers, time steps and humidity ratio histories span real
zones; a few are dried out below zero or pushed past saturation. Each zone is asked for all three
results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261130)


def pick(*xs):
    return rng.choice(xs)


def flow(p=0.5, hi=0.5):
    return rng.uniform(0.0, hi) if rng.random() < p else 0.0


for _ in range(4000):
    controlled = 1 if rng.random() < 0.75 else 0
    ninlets = rng.choices([0, 1, 2, 3], weights=[5, 70, 15, 10])[0] if controlled else rng.choice([0, 1])
    inlets = []
    for _k in range(3):
        m = pick(0.0, rng.uniform(0.01, 3.0), rng.uniform(0.01, 0.5))
        inlets += [m, rng.uniform(0.003, 0.016)]
    hasnode = 1 if controlled and rng.random() < 0.95 else (1 if rng.random() < 0.1 else 0)
    mult = pick(1, 1, 1, 2, 5, 10)
    listmult = pick(1, 1, 1, 1, 3)
    zt = rng.uniform(12.0, 34.0)
    outbaro = pick(101325.0, rng.uniform(80000.0, 103000.0))
    airhumrat = rng.uniform(0.002, 0.018)
    latentgain = pick(0.0, rng.uniform(0.0, 3000.0), rng.uniform(-200.0, 0.0))
    latentradsys = 0.0 if rng.random() < 0.95 else rng.uniform(0.0, 200.0)
    latentpool = 0.0 if rng.random() < 0.95 else rng.uniform(0.0, 2000.0)
    tsyssec = pick(900.0, 600.0, 300.0, 60.0, rng.uniform(60.0, 900.0))
    oamfl = flow(0.8, 0.3)
    vamfl = flow(0.4, 0.5)
    ctmfl = flow(0.03, 0.5)
    eamfl = flow(0.03, 0.3)
    outhumrat = rng.uniform(0.001, 0.02)
    eamflxw = eamfl * rng.uniform(0.002, 0.02)
    sumhmara = flow(0.1, 0.05)
    sumhmaraw = sumhmara * rng.uniform(0.002, 0.018)
    mixm = flow(0.2, 0.5)
    mixw = mixm * rng.uniform(0.002, 0.018)
    mdotoa = flow(0.05, 0.3)
    if rng.random() < 0.03:
        # Dried out below zero, or a big humidifier pushing past saturation.
        latentgain = pick(-200000.0, 4.0e6)
    volume = pick(rng.uniform(20.0, 300.0), rng.uniform(300.0, 5000.0))
    volcapmult = pick(1.0, 1.0, 1.0, rng.uniform(1.0, 10.0))
    algo = rng.choices([0, 1, 2], weights=[50, 35, 15])[0]
    w1 = rng.uniform(0.002, 0.018)
    wprev0 = w1
    wprev1 = w1 + rng.uniform(-0.0005, 0.0005)
    wprev2 = wprev1 + rng.uniform(-0.0005, 0.0005)
    args = [controlled, ninlets] + inlets + [hasnode, mult, listmult, zt, outbaro, airhumrat, latentgain, latentradsys,
            latentpool, tsyssec, oamfl, vamfl, ctmfl, eamfl, eamflxw, sumhmaraw, sumhmara, mixw, mixm, mdotoa,
            outhumrat, volume, volcapmult, algo, wprev0, wprev1, wprev2, w1]
    for which in range(3):
        print(json.dumps({"fn": "CorrectHumRat", "args": [which] + args}))
