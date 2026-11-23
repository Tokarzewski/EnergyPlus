"""Inputs for Boilers::BoilerSpecs::CalcBoilerModel as EnergyPlus meets it: Boiler:HotWater objects
of 5 kW to 5 MW with nominal efficiencies of 0.75 to 0.98 (and electric ones at 1), part load
limits as the input files give them, outlet limits of 80 to 100 C, with no efficiency curve, a
quadratic in the part load ratio or a biquadratic with the entering or leaving water temperature
(typical condensing and non-condensing coefficients), asked for 0 to more than the capacity at 20
to 90 C inlet water on unlocked and locked loop sides, in all three flow modes, single and dual
set point loops, active and series active control. Each case is asked for all eleven results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261205)


def pick(*xs):
    return rng.choice(xs)


CURVES = [
    [0.97, 0.0633, -0.0333, 0.0, 0.0, 0.0],
    [0.626428326, 0.645643582, -0.77720685, 0.0, 0.0, 0.0],
    [1.111720116, 0.078614078, -0.400425756, 0.0, -0.000156783, 0.009384599],
    [1.124970374, 0.014963852, -0.02599835, 0.0, -1.40464e-6, -0.00153624],
]

for _ in range(1227):
    nomcap = pick(rng.uniform(5.0e3, 1.0e5), rng.uniform(1.0e5, 5.0e6))
    nomeff = pick(0.8, rng.uniform(0.75, 0.98), 1.0)
    minplr = pick(0.0, 0.0, rng.uniform(0.0, 0.3))
    maxplr = pick(1.0, 1.0, 1.2, rng.uniform(max(minplr, 0.8), 1.2))
    tlimit = pick(99.9, 95.0, rng.uniform(80.0, 100.0))
    desmdot = nomcap / 4180.0 / pick(11.0, 20.0, rng.uniform(5.0, 25.0))
    parelec = pick(0.0, 0.0, rng.uniform(0.0, 0.01) * nomcap)
    parfuel = pick(0.0, 0.0, rng.uniform(0.0, 0.002) * nomcap)
    tin = pick(rng.uniform(50.0, 75.0), rng.uniform(20.0, 90.0))
    lock = pick(0, 1)
    flowmode = pick(0, 1, 2)
    dual = pick(0, 0, 1)
    series = pick(0, 0, 1)
    run = pick(1, 1, 1, 1, 0)
    curve = pick(0, 1, 2, 2)
    tempmode = pick(1, 2, 2, 0) if curve == 2 else pick(0, 1, 2)
    cs = rng.choice(CURVES[:2]) if curve == 1 else rng.choice(CURVES[2:]) if curve == 2 else [0.0] * 6
    myload = pick(rng.uniform(0.0, 1.0) * nomcap, rng.uniform(0.0, 1.3) * nomcap, 0.0, -rng.uniform(0.0, 1.0e4))
    mdotin = pick(desmdot, desmdot * rng.uniform(0.0, 1.2), 0.0)
    hwmax = pick(desmdot, desmdot, desmdot * rng.uniform(0.5, 1.5), 0.0)
    maxav = pick(hwmax, hwmax, desmdot * rng.uniform(0.0, 1.5), 0.0)
    minav = pick(0.0, 0.0, maxav * rng.uniform(0.0, 0.5))
    tset = pick(82.0, 60.0, rng.uniform(40.0, 90.0), tin - rng.uniform(0.0, 5.0))
    tsetlo = tset - pick(0.0, rng.uniform(0.0, 5.0))
    args = [lock, flowmode, dual, series, run, curve, tempmode, myload, nomcap, nomeff, minplr, maxplr, tlimit, desmdot,
            parelec, parfuel, tin, mdotin, hwmax, minav, maxav, tset, tsetlo] + cs
    for which in range(11):
        print(json.dumps({"fn": "Boiler", "args": [which] + args}))
