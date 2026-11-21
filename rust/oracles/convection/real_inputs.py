"""Realistic inputs for the convection unit, one {"fn": name, "args": [...]} per line, for
`rust-anything record --inputs`. Each parameter gets the range it sees in a simulation, chosen by
its name, plus the values where the correlations branch (zero temperature difference, zero wind,
the ASHRAE tilt thresholds, wind exactly along or against the surface). Deterministic.
Usage: python3 real_inputs.py <path to ConvectionCoefficients.hh> > real-inputs.jsonl"""

import importlib.util
import json
import random
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("gen_conv", Path(__file__).with_name("gen_conv.py"))
gen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gen)

rng = random.Random(20261122)

# (low, high, values worth hitting exactly), matched by parameter name
TILT = (-1.0, 1.0, [0.0, 1.0, -1.0, 0.3827, -0.3827, 0.9239, -0.9239, 0.5])
ANGLE = (0.0, 360.0, [0.0, 90.0, 180.0, 270.0, 360.0, 100.0, -45.0, 405.0])
TEMP = (-20.0, 45.0, [0.0, 20.0, 21.0])
RANGES = {
    "CosTilt": TILT, "cosTilt": TILT, "CosineTilt": TILT,
    "Azimuth": ANGLE, "WindDirection": ANGLE, "WindDir": ANGLE, "SurfAzimuth": ANGLE,
    "LongAxisOutwardAzimuth": ANGLE,
    "Tsurf": TEMP, "Tamb": TEMP, "SurfTemp": TEMP, "SupplyAirTemp": TEMP,
    "DeltaTemp": (-30.0, 30.0, [0.0, 1e-9, -1e-9, 1.0, -1.0]),
    "HydraulicDiameter": (0.1, 20.0, [0.0, 1.0]), "Height": (0.5, 10.0, [0.0, 3.0]),
    "LengthScale": (0.1, 50.0, [0.0, 10.0]),
    "AirChangeRate": (0.0, 10.0, [0.0, 0.5]), "QdotConv": (-200.0, 200.0, [0.0]),
    "AirSystemFlowRate": (0.0, 5.0, [0.0]), "ZoneExtPerimLength": (0.0, 50.0, [0.0]),
    "WindAtZ": (0.0, 20.0, [0.0, 1e-6]), "WindAt10m": (0.0, 20.0, [0.0, 1e-6]),
}

hh = sys.argv[1]
for name, _, ps in gen.functions(hh):
    rs = [RANGES[n] for _, n in ps]
    for k in range(60):
        args = []
        for lo, hi, special in rs:
            if k % 3 == 0:  # an exact branch value for each parameter, in turn
                args.append(special[(k // 3) % len(special)])
            else:
                args.append(rng.uniform(lo, hi))
        print(json.dumps({"fn": name, "args": args}))
