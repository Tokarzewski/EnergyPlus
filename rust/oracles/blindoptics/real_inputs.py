"""Inputs for the blind optics: slats of common widths, separations and thicknesses (25 mm and
50 mm Venetian blinds, louvres), slat angles over 0 to 180 degrees in the 1-degree steps
CalcWindowBlindProperties uses, profile angles from -90 to 90 degrees in its 5-degree steps,
white, grey and dark slats (front and back reflectances, a little diffuse transmittance) and IR
emissivities from 0.1 to 0.9, for solar and visible.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261204)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def geometry():
    w, s = rng.choice([(0.025, 0.01875), (0.05, 0.0425), (0.0889, 0.075), (0.025, 0.025)])
    if rng.random() < 0.3:
        w, s = rng.uniform(0.01, 0.1), rng.uniform(0.008, 0.09)
    t = rng.choice([0.00025, 0.001, 0.0019, rng.uniform(0.0001, 0.003)])
    return w, s, t


def slat():
    tra = rng.choice([0.0, 0.0, 0.05, rng.uniform(0, 0.2)])
    rf = rng.choice([0.7, 0.8, 0.5, 0.2, rng.uniform(0.05, 0.9 - tra)])
    rb = rf if rng.random() < 0.5 else rng.uniform(0.05, 0.9 - tra)
    return tra, rf, rb


def slat_angle():
    return math.pi / 180 * rng.choice([0, 1, 45, 89, 90, 91, 135, 180, rng.randint(0, 180)])


for _ in range(400):
    w, s, t = geometry()
    tra, rf, rb = slat()
    ir_tra = rng.choice([0.0, 0.0, 0.02])
    ef, eb = rng.choice([0.9, 0.8, 0.1, rng.uniform(0.05, 0.95)]), rng.choice([0.9, 0.8, rng.uniform(0.05, 0.95)])
    isv = rng.choice([1, 2])
    el = slat_angle()
    for k in range(8, 15):
        out("BlindDiffuse", k, isv, w, s, t, el, tra, rf, rb, tra, rf, rb, ir_tra, ef, eb)

for _ in range(350):
    w, s, t = geometry()
    tra, rf, rb = slat()
    el = slat_angle()
    prof = -math.pi / 2 + math.pi / 36 * rng.randint(0, 36)
    bb = rng.choice([0.0, 0.0, 0.01])
    for k in range(8):
        out("BlindBeam", k, w, s, t, el, prof, bb, bb, bb, tra, rf, rb, tra, rf, rb)

for _ in range(600):
    w, s, t = geometry()
    out("BlindBeamBeam", w, s, t, rng.uniform(-math.pi / 2, math.pi / 2), slat_angle())
