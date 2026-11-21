"""Realistic inputs for the FluidProperties unit (plant loop and steam system ranges, a little past
the table edges), one {"fn": name, "args": [...]} per line, for `rust-anything record --inputs`.
Deterministic: the same file every run. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261121)

WATER_T = (-45.0, 135.0)  # tables cover -35..125 C, with zeros below 0 C for water
STEAM_T = (-10.0, 400.0)
STEAM_P = (100.0, 3.0e7)
QUALITY = (0.0, 1.0)

ARGS = {
    "WaterSpecificHeat": (WATER_T,),
    "WaterSpecificHeat_raw": (WATER_T,),
    "WaterDensity": (WATER_T,),
    "WaterConductivity": (WATER_T,),
    "WaterViscosity": (WATER_T,),
    "SteamSatPressure": (STEAM_T,),
    "SteamSatTemperature": (STEAM_P,),
    "SteamSatEnthalpy": (STEAM_T, QUALITY),
    "SteamSatDensity": (STEAM_T, QUALITY),
    "SteamSatSpecificHeat": (STEAM_T, QUALITY),
}

PER_FN = 80

for fn, ranges in ARGS.items():
    for i in range(PER_FN):
        if i < 3:
            # the ends and the middle of each range
            args = [(lo, hi, (lo + hi) / 2)[i] for lo, hi in ranges]
        elif i < 3 + 24 and ranges[0] in (WATER_T, STEAM_T):
            # exactly on and just either side of whole-5-degree table points
            t = -35.0 + 5.0 * rng.randrange(0, 34)
            t += (0.0, 1e-9, -1e-9)[i % 3]
            args = [t] + [rng.uniform(lo, hi) for lo, hi in ranges[1:]]
        else:
            args = [rng.uniform(lo, hi) for lo, hi in ranges]
        print(json.dumps({"fn": fn, "args": args}))
