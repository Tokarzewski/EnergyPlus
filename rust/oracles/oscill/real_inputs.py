"""Inputs for ZoneTempPredictorCorrector::DetectOscillatingZoneTemp as EnergyPlus meets it, in C
and hours: one to three zones whose air temperature histories are steady, drifting, swinging just
under and well over the 0.15 C oscillation magnitude in either direction, occupied or not (with
and without the ASH55 comfort results), in the dead band or not, at system time steps of one
minute to an hour, with run period sums already counting. Each case is asked for all 27 results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261126)


def pick(*xs):
    return rng.choice(xs)


def zone():
    t = pick(rng.uniform(18.0, 26.0), rng.uniform(-5.0, 40.0))
    kind = rng.choices(["steady", "drift", "swing", "near"], weights=[20, 20, 45, 15])[0]
    if kind == "steady":
        h = [t, t, t, t]
    elif kind == "drift":
        d = rng.uniform(-0.5, 0.5)
        h = [t + 3 * d, t + 2 * d, t + d, t]
    else:
        a = rng.uniform(0.16, 2.0) if kind == "swing" else pick(0.15, 0.149, 0.151, rng.uniform(0.1, 0.2))
        s = pick(1.0, -1.0)
        # zt, then rows 1 to 3: zt - row1 = s*a, row1 - row2 = -s*a, row2 - row3 = s*a.
        b = [a * rng.uniform(0.9, 1.1) for _ in range(3)]
        r3 = t
        r2 = r3 + s * b[2]
        r1 = r2 - s * b[1]
        zt = r1 + s * b[0]
        h = [r1, r2, r3, zt]
    return h + [1 if rng.random() < 0.6 else 0, 1 if rng.random() < 0.3 else 0]


for _ in range(510):
    nzones = rng.choice([1, 2, 3])
    tsys = pick(1.0 / 60.0, 0.05, 0.1, 1.0 / 6.0, 0.25, 1.0)
    comfort = 1 if rng.random() < 0.6 else 0
    zs = []
    for _ in range(3):
        zs += zone()
    sums = [pick(0.0, rng.uniform(0.0, 100.0)) for _ in range(3)]
    args = [nzones, tsys, comfort] + zs + sums
    for which in range(27):
        print(json.dumps({"fn": "Oscillate", "args": [which] + args}))
