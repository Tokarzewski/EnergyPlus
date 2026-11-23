"""Inputs for the zone air history updates of ZoneTempPredictorCorrector: pushZoneTimestepHistory,
pushSystemTimestepHistory and revertZoneTimestepHistory for a fully mixed zone, and both forms of
DownInterpolate4HistoryValues, as EnergyPlus meets them, in SI units. Zone air temperature and
humidity ratio histories drift as in real zones, under the third order, analytical and Euler
solutions. The interpolations step a zone time step down to system time steps 2, 3, 4 or more times
shorter (exact ratios and near ones, as from 60 / 7 minute steps), and a few just under and over
the 0.01 tolerance. Each case is asked for every result.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261201)


def pick(*xs):
    return rng.choice(xs)


def hist(base, step):
    v = [base]
    for _ in range(3):
        v.append(v[-1] + rng.uniform(-step, step))
    return v


for _ in range(420):
    op = rng.choices([0, 1, 2, 3, 4], weights=[25, 25, 15, 17, 18])[0]
    algo = rng.choices([0, 1, 2], weights=[50, 35, 15])[0]
    t = rng.uniform(15.0, 30.0)
    w = rng.uniform(0.003, 0.016)
    x = hist(t, 0.8)
    ws = hist(w, 0.0004)
    dx = hist(t, 0.3)
    dw = hist(w, 0.0002)
    ztav = t + rng.uniform(-0.3, 0.3)
    zt = t + rng.uniform(-0.5, 0.5)
    mat = zt + rng.uniform(-0.2, 0.2)
    airhumrat = w + rng.uniform(-0.0002, 0.0002)
    airhumratavg = w + rng.uniform(-0.0002, 0.0002)
    airhumrattemp = w + rng.uniform(-0.0002, 0.0002)
    airrelhum = rng.uniform(20.0, 70.0)
    xmpt = t + rng.uniform(-0.5, 0.5)
    wtimeminusp = w
    tm2, tmx = t + rng.uniform(-1, 1), t + rng.uniform(-1, 1)
    wm2, wmx = w + rng.uniform(-0.0003, 0.0003), w + rng.uniform(-0.0003, 0.0003)
    outbaro = pick(101325.0, rng.uniform(80000.0, 103000.0))
    oldts = pick(900.0, 600.0, 3600.0, 360.0)
    r = pick(2.0, 3.0, 4.0, 5.0, 6.0, 10.0, rng.uniform(1.5, 12.0), 2.0 + pick(-0.0099, 0.0099, -0.0101, 0.0101),
             3.0 + pick(-0.0099, 0.0101), oldts / (oldts / 7.0) if rng.random() < 0.3 else 2.0)
    newts = oldts / r
    args = [op, algo] + x + ws + dx + dw + [ztav, zt, mat, airhumrat, airhumratavg, airhumrattemp, airrelhum, xmpt,
                                            wtimeminusp, tm2, tmx, wm2, wmx, outbaro, oldts, newts]
    for which in range(30):
        print(json.dumps({"fn": "ZoneHistory", "args": [which] + args}))
