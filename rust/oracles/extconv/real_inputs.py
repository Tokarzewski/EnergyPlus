"""Inputs for InitExtConvCoeff: the outside convection and linearized radiation coefficients of
exterior surfaces as EnergyPlus meets them, in SI units. Each surface's model is one of the
algorithms EnergyPlus offers: TARP (the default), DOE-2, ASHRAE simple, MoWiTT, BLAST or a user
value, set on the surface or on its zone. The surfaces are walls, roofs, floors, tilted surfaces
and windows (some with an exterior shade), facing every compass direction under wind from every
direction, with or without wind exposure. Surface temperatures run from winter nights to sunlit
summer roofs. Some cases sit on the edges the code tests: the face exactly at the air or sky
temperature, horizontal-enough tilts at |cos| = 0.98, wind exactly 90 or 180 degrees off the
normal, no wind, and zero gross area or height. Some surfaces have exterior movable insulation, a
surrounding surfaces list, a ground surfaces temperature, an EMS override or a frame
adjustment ratio. Each surface is asked for all five coefficients.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261124)


def pick(*xs):
    return rng.choice(xs)


def maybe_round(v, nd=2):
    return round(v, nd) if rng.random() < 0.3 else v


for _ in range(2500):
    model = rng.choices([5, 4, 7, 3, 6, 8, 0], weights=[25, 15, 20, 15, 10, 5, 10])[0]
    setbyzone = 0 if model == 0 else (1 if rng.random() < 0.5 else 0)
    rough = rng.randint(0, 5)
    window = 1 if rng.random() < 0.25 else 0
    extshade = 1 if window and rng.random() < 0.2 else 0
    if window:
        rough = 5
    absext = pick(0.84, 0.9, rng.uniform(0.6, 0.95))
    outdb = rng.uniform(-30.0, 42.0)
    tempext = outdb + pick(rng.uniform(-15.0, 35.0), rng.uniform(-3.0, 3.0), rng.uniform(-30.0, 0.0))
    skytempk = outdb + 273.15 - rng.uniform(0.0, 30.0)
    edge = rng.random()
    if edge < 0.04:
        tempext = outdb
    elif edge < 0.07:
        skytempk = tempext + 273.15
    tilt = pick(90.0, 90.0, 0.0, 180.0, rng.uniform(0.0, 180.0), pick(11.478340954533572, 168.52165904546643))
    costilt = math.cos(math.radians(tilt))
    if tilt == 90.0:
        costilt = 0.0
    elif rng.random() < 0.05:
        costilt = pick(0.98, -0.98)
    azimuth = pick(0.0, 90.0, 180.0, 270.0, rng.uniform(0.0, 360.0))
    winddir = rng.uniform(0.0, 360.0)
    if rng.random() < 0.08:
        winddir = (azimuth + pick(90.0, -90.0, 180.0, 270.0)) % 360.0
    windspeed = 0.0 if rng.random() < 0.1 else rng.uniform(0.0, 15.0)
    extwind = 0 if rng.random() < 0.1 else 1
    grossarea = rng.uniform(0.5, 400.0)
    height = rng.uniform(0.5, 20.0)
    if rng.random() < 0.04:
        grossarea, height = pick((0.0, height), (grossarea, 0.0))
    vfsky = 0.5 * (1.0 + costilt)
    vfgnd = 1.0 - vfsky
    airskysplit = math.sqrt(vfsky)
    hmov = 0.0 if window or rng.random() < 0.88 else pick(rng.uniform(0.2, 2.0), rng.uniform(2.0, 15.0))
    hassrd = 1 if rng.random() < 0.15 else 0
    srdtemp, vfsrd = (rng.uniform(-20.0, 45.0), rng.uniform(0.0, 0.5)) if hassrd else (0.0, 0.0)
    if hassrd and rng.random() < 0.05:
        srdtemp = tempext
    usegnd = 1 if rng.random() < 0.1 else 0
    gndtemp = rng.uniform(-10.0, 40.0) if usegnd else 0.0
    hvalue = rng.uniform(1.0, 40.0) if model == 0 else 0.0
    emsover = 1 if rng.random() < 0.05 else 0
    emsval = rng.uniform(0.5, 50.0) if emsover else 0.0
    adjratio = 1.0 if rng.random() < 0.85 else rng.uniform(1.0, 1.6)
    args = [model, setbyzone, hmov, rough, absext, tempext, outdb, skytempk, hassrd, srdtemp, vfsrd, usegnd, gndtemp,
            extwind, window, extshade]
    args += [maybe_round(v) for v in (windspeed, winddir, grossarea, height)]
    args += [costilt, azimuth, hvalue, emsover, emsval, adjratio, vfsky, vfgnd, airskysplit]
    for which in range(5):
        print(json.dumps({"fn": "ExtConvCoeff", "args": [which] + args}))
