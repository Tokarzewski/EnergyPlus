"""Inputs for ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio: one zone's predicted moisture
load to its humidistat set points as EnergyPlus meets it, in SI units. Zones have no humidistat,
or one on relative humidity or dew point set points, some crossed or equal, with and without EMS
overrides, under the third order, analytical or Euler zone air solution, with and without
RoomAirflowNetwork fractions. Air states, latent gains, infiltration, ventilation, earth tube,
cooling tower, mixing and outdoor air flows, convective moisture terms, volumes, time steps and
humidity ratio histories span real zones humidifying, dehumidifying and floating in the deadband.
Each zone is asked for all six results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261129)


def pick(*xs):
    return rng.choice(xs)


def flow(p=0.5, hi=0.5):
    return rng.uniform(0.0, hi) if rng.random() < p else 0.0


for _ in range(2000):
    ctrltype = rng.choices([-1, 1, 0], weights=[15, 60, 25])[0]
    if ctrltype == 0:
        humsp = round(rng.uniform(-5.0, 12.0), 1)
        dehumsp = round(humsp + pick(0.0, 2.0, 5.0, rng.uniform(0.0, 10.0), -1.0), 1)
    else:
        humsp = float(pick(20, 25, 30, 35, 40, 45, round(rng.uniform(10.0, 60.0), 1)))
        dehumsp = float(pick(50, 55, 60, 65, 70, humsp, round(rng.uniform(30.0, 90.0), 1), humsp - 5.0))
    emshumon = 1 if rng.random() < 0.08 else 0
    emshumval = round(rng.uniform(20.0, 50.0), 1) if emshumon else 0.0
    emsdehumon = 1 if rng.random() < 0.08 else 0
    emsdehumval = round(rng.uniform(45.0, 80.0), 1) if emsdehumon else 0.0
    zt = rng.uniform(14.0, 32.0)
    outbaro = pick(101325.0, rng.uniform(80000.0, 103000.0))
    airhumrat = rng.uniform(0.002, 0.018)
    latentgain = pick(0.0, rng.uniform(0.0, 3000.0))
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
    volume = pick(rng.uniform(20.0, 300.0), rng.uniform(300.0, 5000.0))
    volcapmult = pick(1.0, 1.0, 1.0, rng.uniform(1.0, 10.0))
    algo = rng.choices([0, 1, 2], weights=[50, 35, 15])[0]
    w1 = rng.uniform(0.002, 0.018)
    wprev0 = w1
    wprev1 = w1 + rng.uniform(-0.0005, 0.0005)
    wprev2 = wprev1 + rng.uniform(-0.0005, 0.0005)
    rafnfrac = 0.0 if rng.random() < 0.9 else rng.uniform(0.2, 1.0)
    mult = pick(1, 1, 1, 2, 5, 10)
    listmult = pick(1, 1, 1, 1, 3)
    args = [ctrltype, humsp, dehumsp, emshumon, emshumval, emsdehumon, emsdehumval, zt, outbaro, airhumrat, latentgain,
            latentradsys, latentpool, tsyssec, oamfl, vamfl, ctmfl, eamfl, eamflxw, sumhmaraw, sumhmara, mixw, mixm,
            mdotoa, outhumrat, volume, volcapmult, algo, wprev0, wprev1, wprev2, w1, rafnfrac, mult, listmult]
    for which in range(6):
        print(json.dumps({"fn": "PredictedHumidityRatio", "args": [which] + args}))
