"""Inputs for the weather sun position and sky helpers: sites from the poles to the equator in
both hemispheres and every time zone, every day of a leap year, hours through the day with one or
several time steps per hour, suns below, at and above the 0.00001 sun-is-up limit, clear to
overcast design-day taus, sky models over cold to hot and dry to humid air, and luminous efficacy
over the eight sky clearness bins. Dry-bulb temperatures avoid exactly 0 C, where the saturation
pressure cache's first slot starts with a matching key.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import math
import random

rng = random.Random(20261130)


def out(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


def site():
    lat = rng.choice([0.0, 23.44, -33.9, 40.0, 51.5, 64.8, -77.8, 89.9, rng.uniform(-90, 90)])
    lon = rng.choice([0.0, -87.6, 151.2, -179.9, 179.9, rng.uniform(-180, 180)])
    tzm = 15.0 * round(lon / 15.0) + rng.choice([0.0, 0.0, 15.0, -15.0])
    return lat, lon, tzm


def nonzero(x):
    return x if x != 0.0 else 0.1


for _ in range(400):
    model = rng.randrange(4)
    db = nonzero(rng.choice([-30.0, -5.0, 10.0, 20.0, 35.0, 45.0, round(rng.uniform(-40, 50), 1), rng.uniform(-40, 50)]))
    dp = db - rng.choice([0.0, 2.0, 10.0, 25.0, rng.uniform(-3, 30)])
    rh = rng.choice([5.0, 30.0, 50.0, 80.0, 100.0, rng.uniform(0, 100)])
    osky = rng.choice([0.0, 2.0, 5.0, 10.0, rng.uniform(0, 10)])
    out("SkyEmissivity", model, osky, db, dp, rh)

for _ in range(250):
    cz = rng.choice([0.0, 0.00001, 0.000009, 0.05, 0.3, 0.7, 1.0, rng.random()])
    taub = rng.choice([0.0, 0.3, 0.45, 0.6, rng.uniform(0.2, 1.0)])
    taud = rng.choice([0.0, 1.8, 2.2, 2.6, rng.uniform(1.0, 3.5)])
    etr = rng.choice([1321.0, 1367.0, 1412.0, rng.uniform(1300, 1420)])
    t17 = rng.randrange(2)
    for w in range(3):
        out("TauModel", w, t17, etr, cz, taub, taud)

for doy in list(range(1, 367)) + [0, 367]:
    lat = rng.choice([40.0, -33.9, 0.0, -0.0001])
    for w in range(7):
        out("DailySolar", w, doy, lat)


def day():
    x = 2 * math.pi / 366 * rng.randint(1, 366)
    sd = 0.398 * math.sin(x - 1.4)
    return rng.uniform(-0.25, 0.25), sd, math.sqrt(1 - sd * sd)


for _ in range(300):
    lat, lon, tzm = site()
    eqt, sd, cd = day()
    t = rng.choice([0.0, 6.0, 12.0, 18.0, 23.75, rng.uniform(0, 24)])
    for w in range(3):
        out("SunCosines", w, t, eqt, sd, cd, lat, lon, tzm)
    ts = rng.choice([1, 1, 4, 6, 60])
    off = rng.choice([0.0, -0.5, 0.25])
    for w in range(7):
        out("SunUpDown", w, ts, t, off, eqt, sd, cd, lat, lon, tzm)

for _ in range(300):
    out("WaterMains", rng.randint(1, 366), rng.choice([40.0, -33.9, 0.0, rng.uniform(-60, 60)]),
        rng.choice([-5.0, 0.0, 10.0, 25.0, rng.uniform(-10, 30)]), rng.choice([10.0, 20.0, 30.0, rng.uniform(0, 40)]),
        rng.choice([1.0, 1.0, 0.8, 1.2]), rng.choice([0.0, 0.0, 1.5, -2.0]))

for _ in range(500):
    alt = rng.choice([90.0, 60.0, 30.0, 10.0, 2.0, rng.uniform(0.1, 90)])
    cz = math.sin(math.radians(alt))
    dif = rng.choice([0.0, 5.0, 40.0, 100.0, 250.0, rng.uniform(0, 400)])
    beam = rng.choice([0.0, 0.0, 100.0, 400.0, 850.0, rng.uniform(0, 900)])
    m = rng.choice([rng.randint(1, 12), rng.randint(1, 12), 0, 13])
    for w in range(4):
        out("LumEff", w, m, cz, dif, beam, rng.choice([0.0, 1500.0]), rng.choice([-10.0, 5.0, 20.0]))
