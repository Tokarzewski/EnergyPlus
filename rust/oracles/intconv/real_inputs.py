"""Inputs for InitIntConvCoeff: the inside convection coefficient of heat transfer surfaces as
EnergyPlus meets them, in SI units. Each surface's model is one of the algorithms EnergyPlus
offers: TARP (the default, ISO 15099 for windows), ASHRAE simple, ASTM C1340 or a user value, set
on the surface or on its zone. The surfaces are walls, ceilings, floors, tilted roofs and floors,
and windows and skylights, facing the outside air or other zones. Face temperatures run from cold
glazing on winter nights to sunlit floors, with zone air at 15 to 30 C. Some cases sit on the
edges the code tests: the face exactly at the air temperature, tilts of exactly 0, 90 and 180
degrees and near the correlations' tilt and Rayleigh-number limits, faces outside the ISO 15099
temperature range, coefficients below the lower limit. Some surfaces have an EMS override or a
frame adjustment ratio.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261125)


def pick(*xs):
    return rng.choice(xs)


def maybe_round(v, nd=2):
    return round(v, nd) if rng.random() < 0.3 else v


for _ in range(12000):
    model = rng.choices([4, 3, 8, 0], weights=[50, 20, 15, 15])[0]
    setbyzone = 0 if model == 0 else (1 if rng.random() < 0.5 else 0)
    window = 1 if rng.random() < 0.3 else 0
    if window:
        tilt = pick(90.0, 90.0, 90.0, 0.0, rng.uniform(0.0, 30.0), rng.uniform(30.0, 90.0), rng.uniform(90.0, 180.0))
    else:
        tilt = pick(90.0, 90.0, 0.0, 180.0, rng.uniform(0.0, 180.0), pick(1.0, 2.0, 14.0, 15.0, 75.0, 105.0, 165.0,
                                                                          178.0, 179.0))
    costilt = math.cos(math.radians(tilt))
    sintilt = math.sin(math.radians(tilt))
    if tilt == 90.0:
        costilt, sintilt = 0.0, 1.0
    elif tilt == 0.0:
        costilt, sintilt = 1.0, 0.0
    elif tilt == 180.0:
        costilt, sintilt = -1.0, 0.0
    mat = rng.uniform(15.0, 30.0)
    tsurf = mat + pick(rng.uniform(-25.0, 5.0), rng.uniform(-3.0, 3.0), rng.uniform(-2.0, 35.0))
    edge = rng.random()
    if edge < 0.04:
        tsurf = mat
    elif edge < 0.05:
        tsurf = pick(-100.0, 180.0, 200.0)
    humrat = rng.uniform(0.002, 0.02)
    height = pick(rng.uniform(0.3, 3.0), rng.uniform(2.5, 6.0))
    area = height * rng.uniform(0.3, 20.0)
    extbound = 0 if rng.random() < 0.6 else 1
    outwind = 0.0 if rng.random() < 0.1 else rng.uniform(0.0, 10.0)
    outbaro = pick(101325.0, rng.uniform(85000.0, 102000.0))
    volume = rng.uniform(20.0, 5000.0)
    hvalue = pick(rng.uniform(0.5, 15.0), rng.uniform(0.0, 0.2)) if model == 0 else 0.0
    emsover = 1 if rng.random() < 0.05 else 0
    emsval = rng.uniform(0.5, 20.0) if emsover else 0.0
    adjratio = 1.0 if rng.random() < 0.8 else rng.uniform(1.0, 1.6)
    lowlimit = 0.1 if rng.random() < 0.8 else pick(0.0, rng.uniform(0.1, 3.0))
    args = [model, setbyzone, maybe_round(tsurf), mat, humrat, costilt, tilt, sintilt, maybe_round(height),
            maybe_round(area), window, extbound, maybe_round(outwind), outbaro, maybe_round(volume), hvalue, emsover,
            emsval, adjratio, lowlimit]
    print(json.dumps({"fn": "IntConvCoeff", "args": args}))
