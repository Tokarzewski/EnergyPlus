"""Inputs for HeatBalanceIntRadExchange::CalcInteriorRadExchange as EnergyPlus meets it in box zones:
rooms of 2 to 20 m by 2 to 15 m, 2.4 to 6 m high, with all six faces or some left out (a zone
closed by surfaces in other zones' enclosures), with and without internal mass, inside thermal
absorptances of 0.6 to 0.95, inside face temperatures spread over a few degrees (or up to 10 C for
a floor or ceiling heating case), with no window or one bare window, by ScriptF and now and then by
the Carroll method, at the first and a later inside heat balance iteration. Each case asks for one
surface's net long-wave radiation or, for windows, its incident long-wave radiation.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261130)


def pick(*xs):
    return rng.choice(xs)


n_cases = 0
while n_cases < 13500:
    mask = pick(63, 63, 127, 127, rng.randint(1, 127))
    if (mask >> 6) & 1 == 0 and rng.random() < 0.3:
        mask |= 64
    n = bin(mask).count("1")
    L = pick(rng.uniform(3.0, 8.0), rng.uniform(2.0, 20.0))
    W = pick(rng.uniform(3.0, 6.0), rng.uniform(2.0, 15.0))
    H = pick(2.7, 3.0, rng.uniform(2.4, 6.0))
    mass = rng.uniform(5.0, 2.0 * L * W) if (mask >> 6) & 1 else 0.0
    rot = pick(0.0, 90.0, rng.uniform(0.0, 360.0))
    e0 = pick(0.9, rng.uniform(0.6, 0.95))
    de = pick(0.0, 0.0, rng.uniform(-0.3, 0.3) / max(n - 1, 1))
    e_last = e0 + de * (n - 1)
    if not 0.05 <= e_last <= 0.99:
        de = 0.0
    carroll = pick(0, 0, 0, 1)
    win = pick(0, 0, rng.randint(1, n))
    t0 = pick(rng.uniform(18.0, 24.0), rng.uniform(5.0, 35.0))
    dt = pick(rng.uniform(-1.0, 1.0), rng.uniform(-3.0, 3.0), rng.uniform(-10.0, 10.0) / max(n, 1))
    twin = t0 + pick(0.0, rng.uniform(-10.0, 2.0))
    twin = min(max(twin, -30.0), 80.0)
    it = pick(0, 1)
    for i in range(1, n + 1):
        for which in (0, 1) if win == i else (0,):
            print(json.dumps({"fn": "IRExchange", "args": [which, i, mask, L, W, H, mass, rot, e0, de, carroll, win, t0, dt, twin, it]}))
            n_cases += 1
