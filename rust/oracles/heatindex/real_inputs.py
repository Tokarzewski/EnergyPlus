"""Realistic inputs for the heat index unit, one {"fn": name, "args": [...]} per line, for
`rust-anything record --inputs`. Temperatures are in kelvin, as EnergyPlus passes them (zone air
from -40 to 80 C), relative humidity is a fraction, and the helpers get the skin, clothing and
vapour-pressure ranges the model reaches. heatindex also gets a grid across the model's regimes
(cold, comfortable, clothed, sweating, overheating) and the special cases 0 K and the triple
point. Deterministic. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261122)
AIR = (233.15, 353.15)
SKIN = (270.0, 320.0)


def emit(fn, *args):
    print(json.dumps({"fn": fn, "args": list(args)}))


for T in [0.0, 273.16, 273.15, 310.0, 150.0, 400.0]:
    emit("pvstar", T)
    emit("Le", T)
for _ in range(60):
    T = rng.uniform(150.0, 400.0)
    emit("pvstar", T)
    emit("Le", T)
    emit("Qv", rng.uniform(*AIR), rng.uniform(0.0, 10000.0))
    emit("Zs", rng.choice([0.0387, 0.0, rng.uniform(0.0, 0.1)]))
    for fn in ("Ra", "Ra_bar", "Ra_un"):
        emit(fn, rng.uniform(*SKIN), rng.uniform(*AIR))

RH = [0.0, 0.05, 0.2, 0.4, 0.6, 0.8, 1.0]
for fn in ("find_eqvar_phi", "find_eqvar_Rf", "find_eqvar_rs", "find_eqvar_dTcdt",
           "find_eqvar_value", "find_eqvar_name", "heatindex"):
    for _ in range(60):
        emit(fn, rng.uniform(*AIR), rng.uniform(0.0, 1.0))
    for Ta in [233.15, 263.15, 273.15, 288.15, 298.15, 303.15, 310.0, 318.15, 333.15, 353.15]:
        for rh in RH:
            emit(fn, Ta, rh)
emit("heatindex", 0.0, 0.5)
emit("heatindex", 273.16, 1.0)

# find_T on its own: each equivalent variable over the values heatindex hands it.
EQVAR = {0: (0.0, 1.0), 1: (0.0, 1.0), 2: (0.0, 0.05), 3: (0.0, 0.002)}
for name, (lo, hi) in EQVAR.items():
    for _ in range(30):
        emit("find_T", name, rng.uniform(lo, hi))
