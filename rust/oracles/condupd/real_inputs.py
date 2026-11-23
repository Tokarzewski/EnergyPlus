"""Inputs for PlantUtilities' UpdateChillerComponentCondenserSide and UpdateComponentHeatRecoverySide
as EnergyPlus meets them: chillers and heat pumps rejecting 0 to 2 MW (or taking heat, negative) to
condenser and heat recovery water at 1 to 99 kg/s down to no flow, entering at 5 to 45 C, with node
states that already equal the model's (converged: the flag clears), differ in one field, or carry
no flow while heat moves; on the first HVAC iteration or not, with 0 to 2 connected loops. All four
results are asked for each call, both routines interleaved.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261217)


def pick(*xs):
    return rng.choice(xs)


for _ in range(3374):
    q = pick(rng.uniform(0.0, 2.0e6), rng.uniform(0.0, 5.0e4), 0.0, -rng.uniform(0.0, 1.0e5))
    tmin = rng.uniform(5.0, 45.0)
    mmod = pick(rng.uniform(1.0, 99.0), rng.uniform(0.0, 1.0), 0.0)
    tmout = tmin + pick(rng.uniform(0.0, 10.0), 0.0, -rng.uniform(0.0, 5.0))
    inm, intemp, outm, outt = mmod, tmin, mmod, tmout
    mode = rng.randint(0, 5)
    if mode == 1:
        inm = pick(rng.uniform(0.0, 99.0), 0.0)
    elif mode == 2:
        outm = rng.uniform(0.0, 99.0)
    elif mode == 3:
        intemp = rng.uniform(5.0, 45.0)
    elif mode == 4:
        outt = rng.uniform(5.0, 55.0)
    elif mode == 5:
        inm = outm = mmod = 0.0
    first = pick(0, 0, 1)
    sim0 = rng.randint(0, 1)
    nconn = rng.randint(0, 2)
    dem1, dem2 = rng.randint(0, 1), rng.randint(0, 1)
    hr = rng.randint(0, 1)
    for which in range(4):
        print(json.dumps({"fn": "CondUpd", "args": [which, hr, first, sim0, nconn, dem1, dem2, q, tmin, tmout, mmod, inm, intemp, outm, outt]}))
for k in range(4):
    print(json.dumps({"fn": "CondUpd", "args": [k % 2, 0, 0, 1, 0, 0, 0, 0.0, 20.0, 20.0, 0.0, 0.0, 20.0, 0.0, 20.0]}))
