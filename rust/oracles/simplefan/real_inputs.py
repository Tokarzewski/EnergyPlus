"""Inputs for Fans::FanComponent::simulateConstant, simulateVAV and simulateOnOff as EnergyPlus meets
them: supply, return and unit fans of 0.05 to 20 kg/s at design pressure rises of 75 to 1500 Pa,
total efficiencies of 0.3 to 0.8, motors in or out of the air stream, inlet air of -20 to 40 C,
flows from off through part load to full, the DOE-2 inlet vane, discharge damper and variable
speed part load coefficients (and the ASHRAE 90.1 ones), on/off fans cycling with coil part load
fractions of 0.7 to 1, now and then an EMS override or TurnFansOn and TurnFansOff. Each fan is
asked for all nine results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261201)

CURVES = [
    [0.35071223, 0.30850535, -0.54137364, 0.87198823, 0.0],
    [0.37073425, 0.97250253, -0.34240761, 0.0, 0.0],
    [0.0015302446, 0.0052080574, 1.1086242, -0.11635563, 0.0],
    [0.0013, 0.147, 0.9506, -0.0998, 0.0],
]


def pick(*xs):
    return rng.choice(xs)


for _ in range(1500):
    kind = pick(0, 1, 1, 2, 3, 3)
    rho = pick(1.2, rng.uniform(0.95, 1.25))
    volmax = pick(rng.uniform(0.05, 2.0), rng.uniform(2.0, 16.0))
    mdotmax = volmax * rho
    volmin = pick(0.0, volmax * rng.uniform(0.1, 0.4)) if kind in (1, 2) else 0.0
    mdotmin = 0.0
    frac = pick(1.0, 1.0, rng.uniform(0.0, 1.0), rng.uniform(0.0, 0.15), 0.0, rng.uniform(1.0, 1.2))
    mdot = min(mdotmax * frac, 100.0)
    dp = pick(rng.uniform(75.0, 300.0), rng.uniform(300.0, 1500.0))
    eff = pick(0.7, 0.6, rng.uniform(0.3, 0.8))
    moteff = pick(0.9, rng.uniform(0.8, 0.95))
    motinair = pick(1.0, 0.0, rng.uniform(0.0, 1.0))
    tin = pick(rng.uniform(12.0, 28.0), rng.uniform(-20.0, 40.0))
    win = pick(rng.uniform(0.004, 0.012), rng.uniform(0.001, 0.02))
    plf = pick(1.0, 1.0, rng.uniform(0.7, 1.0), rng.uniform(0.5, 0.7))
    c = pick(*CURVES)
    pr = rng.uniform(100.0, 1500.0)
    avail = pick(1.0, 1.0, 1.0, 0.0)
    turn = pick(0, 0, 0, 0, 0, 0, 1, 2)
    ems = pick(0, 0, 0, 0, 0, 0, 1, 2, 4, rng.randint(1, 7))
    emsflow = mdotmax * rng.uniform(0.0, 1.1)
    emspress = rng.uniform(50.0, 1500.0)
    emseff = rng.uniform(0.3, 0.9)
    for which in range(9):
        print(json.dumps({"fn": "SimpleFan", "args": [which, kind, turn, ems, avail, mdot, mdotmax, mdotmin, volmax, volmin, dp, eff,
                                                     moteff, motinair, rho, tin, win, plf, *c, pr, emsflow, emspress, emseff]}))
