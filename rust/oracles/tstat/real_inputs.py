"""Inputs for ZoneTempPredictorCorrector::CalcZoneAirTempSetPoints for one thermostat as EnergyPlus
meets it, in SI units: each control type (uncontrolled, single heating, used or not, single
cooling, single heating and cooling, dual set point) with occupied and set back schedules,
operative temperature control with radiative fractions of 0 to 0.9 against mean radiant
temperatures near and away from the set points, temperature and humidity overcooling with
constant and scheduled ranges, zone relative humidities below and above the dehumidifying set
point, and EMS heating and cooling overrides. Each case is asked for all six results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261125)


def pick(*xs):
    return rng.choice(xs)


for _ in range(2200):
    ctype = rng.choices([0, 1, 2, 3, 4], weights=[10, 15, 15, 15, 45])[0]
    heatused = 1 if rng.random() < 0.85 else 0
    heat = pick(20.0, 21.0, 22.0, 15.6, rng.uniform(12.0, 23.0))
    cool = pick(24.0, 25.0, 26.7, 29.4, rng.uniform(23.0, 32.0))
    hc = pick(21.0, 22.0, rng.uniform(18.0, 26.0))
    setpt0 = pick(0.0, heat, cool, rng.uniform(10.0, 30.0))
    hi0 = pick(0.0, cool, rng.uniform(20.0, 35.0))
    lo0 = pick(0.0, heat, rng.uniform(5.0, 22.0))
    anyop = 1 if rng.random() < 0.35 else 0
    opctrl = rng.choices([0, 1, 2], weights=[30, 40, 30])[0]
    frac = pick(0.5, 0.3, 0.0, 0.9, rng.uniform(0.0, 0.9))
    mrt = pick(rng.uniform(15.0, 30.0), rng.uniform(-5.0, 45.0))
    anyhum = 1 if rng.random() < 0.35 else 0
    ovctrl = rng.choices([0, 1, 2], weights=[30, 40, 30])[0]
    ovrange = pick(1.0, 2.0, 3.0, rng.uniform(0.0, 5.0))
    ovratio = pick(3.6, 10.0, 0.0, rng.uniform(0.0, 20.0))
    relhum = pick(rng.uniform(30.0, 60.0), rng.uniform(55.0, 95.0))
    dehum = pick(60.0, 50.0, 70.0, rng.uniform(40.0, 80.0))
    emsheat = 1 if rng.random() < 0.15 else 0
    emscool = 1 if rng.random() < 0.15 else 0
    emsheatval = rng.uniform(15.0, 24.0)
    emscoolval = rng.uniform(22.0, 30.0)
    args = [ctype, heatused, heat, cool, hc, heat, cool, setpt0, hi0, lo0, anyop, opctrl, frac, mrt, anyhum,
            ovctrl, ovrange, ovratio, relhum, dehum, emsheat, emsheatval, emscool, emscoolval]
    for which in range(6):
        print(json.dumps({"fn": "TstatSetPoints", "args": [which] + args}))
