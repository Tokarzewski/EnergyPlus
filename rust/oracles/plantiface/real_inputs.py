"""Inputs for HVACInterfaceManager's UpdatePlantLoopInterface without a common pipe as EnergyPlus
meets it: chilled, condenser and hot water loops of 0 to 50000 kg with 0 to 100 kg/s leaving the
side just simulated (demand or supply), the other side's inlet last at the same flow (converged),
just within or beyond the 0.001 kg/s or 1 percent tolerance, or anything; its temperature at the
tank's or off it by more or less than 0.01 K; pump heat, loops with and without pressure
components, the first call of a time step or a repeat, and the out-of-tolerance flag already set or
not. Every result is asked for each call.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261224)


def pick(*xs):
    return rng.choice(xs)


for _ in range(964):
    tin = pick(rng.uniform(4.0, 14.0), rng.uniform(20.0, 40.0), rng.uniform(40.0, 80.0))
    last = min(max(tin + rng.uniform(-6.0, 6.0), 1.0), 99.0)
    prev = pick(last, min(max(last + rng.uniform(-2.0, 2.0), 1.0), 99.0))
    mdot = pick(rng.uniform(0.0, 100.0), rng.uniform(0.0, 0.5), 0.0)
    omdot = pick(mdot, mdot + rng.uniform(-0.0009, 0.0009), mdot * rng.uniform(0.98, 1.02), rng.uniform(0.0, 100.0))
    omdot = max(omdot, 0.0)
    mass = pick(rng.uniform(100.0, 50000.0), 0.0)
    otemp = pick(tin, prev, last, min(max(last + rng.uniform(-0.02, 0.02), 1.0), 99.0), rng.uniform(1.0, 99.0))
    args = [rng.randint(0, 1), rng.randint(0, 1), rng.randint(0, 1), pick(0, 0, 1), tin, mdot, pick(0.0, mdot * 0.5),
            pick(mdot, mdot * 1.2, 100.0), pick(0.0, rng.uniform(0.0, 1.0)), pick(101325.0, rng.uniform(1.0e5, 6.0e5)),
            otemp, omdot, pick(0.0, rng.uniform(0.0, 5.0e4)), mass, last, prev, pick(60.0, 600.0, 900.0, rng.uniform(1.0, 3600.0)),
            pick(0.0, rng.uniform(-1.0, 1.0))]
    for which in range(14):
        print(json.dumps({"fn": "PlantIface", "args": [which] + args}))
for k in range(4):
    print(json.dumps({"fn": "PlantIface", "args": [k, 0, 0, 0, 0, 20.0, 1.0, 0.0, 1.0, 0.0, 101325.0, 20.0, 1.0, 0.0, 0.0, 20.0, 20.0, 600.0, 0.0]}))
