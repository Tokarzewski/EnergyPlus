"""Inputs for ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums of ZoneTempPredictorCorrector for one
zone as EnergyPlus meets it, in SI units: offices, homes and plenums with zero to three internal
gain devices (people, lights, equipment, some with return air gains), infiltration, ventilation,
mixing, earth tube and cooling tower air flows, uncontrolled and controlled zones with up to three
supply air inlets, return plenums with leaky air terminals and supply plenums, parallel PIU
backdraft leakage, zone multipliers, as the predictor and the corrector, winter and summer. Each
case is asked for all five sums.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261205)


def pick(*xs):
    return rng.choice(xs)


def cp(w):
    return 1.00484e3 + max(w, 1.0e-5) * 1.85895e3


for _ in range(2600):
    corrector = 1 if rng.random() < 0.6 else 0
    kind = rng.choices([0, 1, 2, 3], weights=[20, 50, 20, 10])[0]
    ndev = rng.choices([0, 1, 2, 3], weights=[15, 25, 30, 30])[0]
    gains = []
    for _ in range(3):
        gains += [round(rng.uniform(0.0, 3000.0), pick(1, 3, 6)), pick(0.0, 0.0, rng.uniform(0.0, 400.0))]
    radsys = pick(0.0, 0.0, 0.0, rng.uniform(0.0, 2000.0))
    pool = pick(0.0, 0.0, 0.0, rng.uniform(0.0, 500.0))
    noheatret = 1 if rng.random() < 0.3 else 0
    outdb = rng.uniform(-20.0, 38.0)
    zt = rng.uniform(16.0, 30.0)
    w = rng.uniform(0.002, 0.016)
    flows = []
    for t in (outdb, outdb, rng.uniform(15.0, 28.0), rng.uniform(8.0, 22.0), rng.uniform(14.0, 25.0)):
        mcp = pick(0.0, 0.0, rng.uniform(0.0, 0.3) * cp(w), rng.uniform(0.0, 2.0) * cp(w))
        flows += [mcp, mcp * t]
    mdotcpoa = pick(0.0, 0.0, rng.uniform(0.0, 0.2) * cp(w))
    ninlets = rng.choices([0, 1, 2, 3], weights=[10, 50, 25, 15])[0]
    inl = []
    for _ in range(3):
        inl += [pick(0.0, rng.uniform(0.0, 1.5), rng.uniform(0.0, 0.3)), rng.uniform(12.0, 40.0)]
    hasadu = 1 if rng.random() < 0.5 else 0
    adu = [pick(0, 1), rng.uniform(0.0, 0.05), rng.uniform(12.0, 35.0), pick(0, 1), rng.uniform(0.0, 0.05),
           rng.uniform(12.0, 35.0)]
    haspiu = 1 if rng.random() < 0.15 else 0
    piu = [pick(0.0, rng.uniform(0.0, 0.05)), rng.uniform(12.0, 18.0)]
    hasnode = 1 if (kind > 0 or rng.random() < 0.3) else 0
    mult = pick(1, 1, 1, 2, 3, 10)
    listmult = pick(1, 1, 1, 2, 5)
    args = ([corrector, kind, ndev] + gains + [radsys, pool, noheatret] + flows + [mdotcpoa, outdb, w, ninlets] + inl +
            [hasadu] + adu + [haspiu] + piu + [hasnode, mult, listmult])
    for which in range(5):
        print(json.dumps({"fn": "ZoneSums", "args": [which] + args}))
