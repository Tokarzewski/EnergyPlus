"""Realistic inputs for the Sandia PV unit, one {"fn": name, "args": [...]} per line, for
`rust-anything record --inputs`. Each parameter gets the range it has in a simulation, chosen by
its name: weather (irradiance, wind, temperatures, sun angles) as EnergyPlus passes it, and module
coefficients at the scale of the Sandia module database. Every third case puts each parameter on a
value where the equations branch or degenerate (no irradiance, the 89.9 degree air-mass cut-off,
96.08 degrees where its power term blows up, zero effective irradiance before the logs).
Deterministic. Usage: python3 real_inputs.py <path to Photovoltaics.hh> > real-inputs.jsonl"""

import importlib.util
import json
import random
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("gen_pv", Path(__file__).with_name("gen_pv.py"))
gen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gen)

rng = random.Random(20261122)

TEMP = (-20.0, 75.0, [25.0, 0.0, -20.0])
COEF_I = (0.9, 1.05, [1.0, 0.0])  # C0, C4, C6: near 1
COEF_I2 = (-0.05, 0.1, [0.0])  # C1, C5, C7: near 0
# (low, high, values worth hitting exactly), matched by parameter name
RANGES = {
    "Ibc": (0.0, 1000.0, [0.0, 1000.0]), "Idc": (0.0, 300.0, [0.0, 100.0]),
    "Ws": (0.0, 15.0, [0.0]), "Ta": (-20.0, 45.0, [25.0, 0.0]), "Tm": TEMP, "Tc": TEMP,
    "fd": (0.8, 1.0, [1.0, 0.0]), "a": (-3.9, -3.2, [-3.56]), "b": (-0.1, -0.05, [-0.075]),
    "DT0": (1.0, 4.0, [3.0]),
    "Isc": (0.0, 10.0, [0.0, 5.0]), "Isc0": (3.0, 9.0, [5.0]), "aIsc": (0.0002, 0.001, [0.0]),
    "SolZen": (0.0, 100.0, [0.0, 89.9, 89.89999, 90.0, 96.08, 120.0, 60.0]),
    "Altitude": (0.0, 3000.0, [0.0, -100.0]),
    "AMa": (1.0, 10.0, [1.5, 36.32, 0.0]),
    "a0": (0.9, 0.95, [0.93]), "a1": (0.03, 0.08, [0.06]), "a2": (-0.01, -0.005, [-0.0087]),
    "a3": (0.0004, 0.0008, [0.00058]), "a4": (-2e-5, -1e-5, [-1.4e-5]),
    "IncAng": (0.0, 90.0, [0.0, 90.0]),
    "b0": (0.98, 1.0, [1.0]), "b1": (-0.004, -0.001, [-0.0024]), "b2": (2e-4, 4e-4, [3.1e-4]),
    "b3": (-2e-5, -1e-5, [-1.2e-5]), "b4": (1e-7, 3e-7, [2.1e-7]), "b5": (-2e-9, -1e-9, [-1.4e-9]),
    "Ee": (0.0, 1.2, [0.0, 1.0, -0.1, 1e-12]),
    "Imp0": (2.0, 8.0, [4.5]), "Ix0": (2.0, 8.0, [4.8]), "Ixx0": (2.0, 6.0, [3.5]),
    "aImp": (-0.0005, 0.0001, [0.0]),
    "C0": COEF_I, "C1": COEF_I2, "C4": COEF_I, "C5": COEF_I2, "C6": COEF_I, "C7": COEF_I2,
    "F1": (0.0, 1.05, [0.0, 1.0]), "F2": (0.0, 1.05, [0.0, 1.0]),
    "Vmp0": (30.0, 60.0, [40.0]), "Voc0": (40.0, 70.0, [50.0]),
    "NcellSer": (36.0, 96.0, [72.0]), "DiodeFactor": (1.0, 1.5, [1.2]),
    "BVmp0": (-0.25, -0.1, [-0.2]), "mBVmp": (-0.01, 0.01, [0.0]),
    "BVoc0": (-0.25, -0.15, [-0.2]), "mBVoc": (-0.01, 0.01, [0.0]),
    "C2": (-0.5, 0.5, [0.0]), "C3": (-10.0, 0.0, [-5.0]),
}

hh = sys.argv[1]
for name, _, ps in gen.functions(hh):
    missing = [n for _, n in ps if n not in RANGES]
    if missing:
        sys.exit(f"{name}: no range for {missing}")
    rs = [RANGES[n] for _, n in ps]
    for k in range(60):
        args = []
        for lo, hi, special in rs:
            if k % 3 == 0:  # an exact value for each parameter, in turn
                args.append(special[(k // 3) % len(special)])
            else:
                args.append(rng.uniform(lo, hi))
        print(json.dumps({"fn": name, "args": args}))
