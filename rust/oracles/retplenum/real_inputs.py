"""Inputs for ZonePlenum's CalcAirZoneReturnPlenum as EnergyPlus meets it: return plenums with 1 to
4 zone return inlets carrying 0 to 5 kg/s each (a few at no flow, the loop off) at near-atmospheric
pressures, available flows around the inlet flows, 0 to 2 terminal units leaking into the plenum
(upstream, downstream or parallel PIU leaks, or none), 0 to 2 induced-air outlets drawing from it
(sometimes more than flows in), and plenum zones at 10 to 35 C. All seven results are asked for each
plenum.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261215)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1500):
    nin = rng.randint(1, 4)
    nadu = rng.randint(0, 2)
    nind = rng.randint(0, 2)
    off = rng.random() < 0.1
    inl = []
    for _ in range(4):
        m = 0.0 if off else pick(rng.uniform(0.0, 5.0), rng.uniform(0.0, 0.5), 0.0)
        x = pick(m, m * rng.uniform(1.0, 1.5), rng.uniform(0.0, 6.0))
        n = pick(0.0, m * rng.uniform(0.0, 1.0))
        p = pick(101325.0, rng.uniform(95000.0, 102000.0), rng.uniform(84000.0, 101325.0))
        inl += [m, x, n, p]
    adu = []
    for _ in range(2):
        u = rng.randint(0, 1)
        d = rng.randint(0, 1)
        lu = rng.uniform(0.0, 0.2) if u else 0.0
        ld = rng.uniform(0.0, 0.1) if d else 0.0
        lp = pick(0.0, 0.0, rng.uniform(0.0, 0.1))
        dx = pick(0.0, rng.uniform(-0.2, 0.2))
        dn = pick(0.0, rng.uniform(-0.2, 0.2))
        adu += [u, d, lu, ld, lp, dx, dn]
    ind = [pick(rng.uniform(0.0, 1.0), rng.uniform(0.0, 8.0), 0.0) for _ in range(2)]
    tz = rng.uniform(10.0, 35.0)
    wz = rng.uniform(0.002, 0.018)
    hz = 1.00484e3 * tz + wz * (2.50094e6 + 1.85895e3 * tz)
    for which in range(7):
        print(json.dumps({"fn": "RetPlenum", "args": [which, nin, nadu, nind] + inl + adu + ind + [tz, wz, hz]}))
for _ in range(3000):
    nin = rng.randint(1, 4)
    args = [rng.choice([0, 1, 2, 5]), nin, rng.randint(0, 2), rng.randint(0, 2)]
    for _ in range(4):
        m = rng.uniform(0.0, 3.0)
        args += [m, m + rng.uniform(0.0, 1.0), 0.0, 101325.0 + rng.uniform(-500.0, 500.0)]
    for _ in range(2):
        args += [rng.randint(0, 1), rng.randint(0, 1), rng.uniform(0.0, 0.3), rng.uniform(0.0, 0.3), rng.uniform(0.0, 0.1), rng.uniform(-0.5, 0.5), rng.uniform(-0.5, 0.5)]
    args += [rng.uniform(0.0, 10.0), rng.uniform(0.0, 10.0), 22.0, 0.008, 42500.0]
    print(json.dumps({"fn": "RetPlenum", "args": args}))
