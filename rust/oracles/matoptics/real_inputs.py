"""Inputs for the material optics helpers: insect and solar screens (diameter-to-spacing ratios
0.05 to 0.6, dark to bright wire) under all three beam reflectance models at sun angles over the
quarter and back spheres, the 0 and 90 degree edges included; window and sun directions for the
relative angle and normalisation helpers, angles past 2 pi included; the screen, profile and slat
index helpers on and between their grid points; and simple glazing systems over the U-factor and
SHGC ranges EnergyPlus accepts, with and without a visible transmittance.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261205)
PI = math.pi


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


for _ in range(450):
    model = rng.randrange(3)
    g = rng.choice([0.05, 0.15, 0.25, 0.4, 0.6, rng.uniform(0.01, 0.8)])
    r = rng.choice([0.05, 0.2, 0.5, 0.8, rng.uniform(0.01, 0.9)])
    rv = rng.choice([r, rng.uniform(0.01, 0.9)])
    phi = rng.choice([0.0, PI / 2, PI, PI / 4, rng.uniform(0, PI), rng.uniform(0, PI / 2)])
    theta = rng.choice([0.0, PI / 2, PI, PI / 3, rng.uniform(0, PI), rng.uniform(0, PI / 2)])
    for w in rng.sample(range(12), 6):
        out("ScreenTrans", w, model, g, r, rv, phi, theta)

for _ in range(300):
    alt, az = math.radians(rng.uniform(0, 90)), math.radians(rng.uniform(0, 360))
    s = (math.cos(alt) * math.sin(az), math.cos(alt) * math.cos(az), math.sin(alt))
    pw = rng.choice([PI / 2, 0.0, PI / 4, rng.uniform(0, PI)])
    tw = rng.choice([0.0, PI, 3 * PI / 2, rng.uniform(-PI, 2 * PI)])
    for w in range(2):
        out("RelPhiTheta", w, pw, tw, *s)

for _ in range(250):
    phi = rng.choice([rng.uniform(0, 2 * PI), rng.uniform(0, 20), 2 * PI, PI, 7 * PI])
    theta = rng.choice([rng.uniform(0, 2 * PI), rng.uniform(0, 20), 2 * PI, PI])
    for w in range(2):
        out("NormPhiTheta", w, phi, theta)

d5 = math.radians(5)
for _ in range(300):
    phi = rng.choice([rng.uniform(0, PI), PI, 0.0, d5 * rng.randint(0, 36)])
    theta = rng.choice([rng.uniform(0, PI), PI, d5 * rng.randint(0, 36)])
    for w in range(4):
        out("PhiThetaIdx", w, phi, theta, d5, d5)

for _ in range(300):
    p = rng.choice([rng.uniform(-PI / 2, PI / 2), -PI / 2, PI / 2, -PI / 2 + PI / 36 * rng.randint(0, 36)])
    for w in range(2):
        out("ProfIdx", w, p)
    s = rng.choice([rng.uniform(0, PI), 0.0, PI, PI / 180 * rng.randint(0, 180)])
    for w in range(3):
        out("SlatIdx", w, s)

for _ in range(400):
    u = rng.choice([0.5, 1.2, 2.0, 3.4, 4.0, 4.5, 5.0, 5.85, 6.5, 7.0, rng.uniform(0.3, 7.0)])
    shgc = rng.choice([0.1, 0.15, 0.25, 0.4, 0.7206, 0.8, rng.uniform(0.05, 0.95)])
    vt_by_user = rng.randrange(2)
    vt = rng.choice([0.1, 0.5, 0.7, 0.9, rng.uniform(0.05, 0.95)])
    for w in range(9):
        out("SimpleGlazing", w, u, shgc, vt_by_user, vt)
