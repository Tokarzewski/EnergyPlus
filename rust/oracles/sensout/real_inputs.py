"""Inputs for GeneralRoutines' CalcComponentSensibleLatentOutput, CalcZoneSensibleLatentOutput and
calcZoneSensibleOutput as EnergyPlus meets them: coils, fans and heat exchangers moving 0 to 20
kg/s of air between states from -30 to 60 C at humidity ratios of 0.0005 to 0.025 (cooling and
dehumidifying, heating, and no flow), and zone equipment supplying 10 to 50 C air to zones at 15
to 30 C. The two routines with three results are asked for all of them; the three routines'
cases are interleaved.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261214)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1500):
    m = pick(rng.uniform(0.05, 2.0), rng.uniform(2.0, 20.0), 0.0)
    t1 = rng.uniform(-30.0, 40.0)
    w1 = rng.uniform(0.0005, 0.02)
    t2 = pick(t1 - rng.uniform(0.0, 20.0), t1 + rng.uniform(0.0, 30.0), t1)
    w2 = pick(w1, w1 * rng.uniform(0.5, 1.0), rng.uniform(0.0005, 0.025))
    te = pick(rng.uniform(10.0, 16.0), rng.uniform(25.0, 50.0), rng.uniform(15.0, 30.0))
    we = rng.uniform(0.003, 0.015)
    tz = rng.uniform(15.0, 30.0)
    wz = rng.uniform(0.003, 0.015)
    mz = pick(rng.uniform(0.01, 3.0), 0.0)
    for which in range(3):
        print(json.dumps({"fn": "CompOutput", "args": [which, m, t2, w2, t1, w1]}))
        print(json.dumps({"fn": "ZoneOutput", "args": [which, mz, te, we, tz, wz]}))
        print(json.dumps({"fn": "ZoneSensOutput", "args": [pick(mz, m), te, tz, wz]}))
