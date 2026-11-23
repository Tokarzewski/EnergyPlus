"""Inputs for Pumps::CalcPumps as EnergyPlus meets it: loop and branch pumps of 0.0001 to 0.5 m3/s
and 50 W to 200 kW, constant and variable speed (with the default and typical VFD part load
coefficients), condensate pumps and banks of 2 to 6, asked for 0 to more than their design flow
at 5 to 90 C, on unlocked and locked loop sides with the loop's available limits, active and
series active control, with and without the pressure simulation and the EMS pressure override.
Each case is asked for all eleven results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261206)


def pick(*xs):
    return rng.choice(xs)


COEFS = [[0.0, 1.0, 0.0, 0.0], [0.0, 0.0216, -0.0325, 1.0095], [0.0, 0.0205, 0.4101, 0.5753], [0.0, 1.0, 0.0, 0.0]]

for _ in range(1227):
    ptype = pick(0, 1, 1, 0, 2, 3, 4)
    nbank = rng.randint(2, 6) if ptype >= 3 else pick(1, 1, 2)
    nomvol = pick(rng.uniform(1.0e-4, 0.01), rng.uniform(0.01, 0.5))
    rho0 = pick(999.9, 998.2, rng.uniform(970.0, 1000.0))
    mmax = nomvol * rho0
    mmin = pick(0.0, 0.0, mmax * rng.uniform(0.0, 0.3)) if ptype in (0, 3) else 0.0
    nompow = min(nomvol * pick(rng.uniform(2.0e5, 4.0e5), 348701.1), 1.0e7)
    cs = pick(*COEFS) if ptype in (0, 3) else [0.0, 1.0, 0.0, 0.0]
    motor = pick(0.9, rng.uniform(0.8, 0.95))
    peff = pick(0.78, rng.uniform(0.6, 0.9))
    frac = pick(0.0, 1.0, rng.uniform(0.0, 1.0))
    lock = pick(0, 0, 1)
    series = pick(0, 0, 1)
    press = pick(0, 0, 0, 1)
    ems = pick(0, 0, 0, 0, 1)
    pdrop = rng.uniform(1.0e4, 3.0e5)
    emsp = rng.uniform(1.0e4, 3.0e5)
    tin = pick(rng.uniform(5.0, 15.0), rng.uniform(20.0, 40.0), rng.uniform(50.0, 90.0))
    pin = pick(101325.0, rng.uniform(1.0e5, 6.0e5))
    flowreq = pick(mmax, mmax * rng.uniform(0.0, 1.0), mmax * rng.uniform(0.0, 2.0), 0.0, mmax * rng.uniform(0.0, 1e-9))
    hwmax = pick(mmax, mmax * rng.uniform(1.0, 3.0), mmax * rng.uniform(0.0, 1.0))
    maxav = pick(hwmax, hwmax, mmax * rng.uniform(0.0, 2.0), 0.0)
    minav = pick(0.0, 0.0, maxav * rng.uniform(0.0, 0.5))
    mdotin = pick(flowreq, mmax * rng.uniform(0.0, 1.2), 0.0, maxav)
    args = [ptype, lock, series, press, ems, nbank, flowreq, mmax, mmin, nomvol, nompow] + cs + \
        [motor, peff, frac, pdrop, emsp, tin, pin, mdotin, hwmax, minav, maxav]
    for which in range(11):
        print(json.dumps({"fn": "Pump", "args": [which] + args}))
