"""Inputs for the weather helpers: water and wind as a ship or a lake sees them, sun angles,
longitudes on and across the time-zone edges, years around the leap-year rules, and months and
days just outside their ranges. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("plain_inputs", Path(__file__).parents[1] / "plain_inputs.py")
plain = importlib.util.module_from_spec(spec)
spec.loader.exec_module(plain)
NAN = float("nan")

WIND = (0.0, 360.0, [0.0, 1.0, 180.0, 359.0, 360.0])
plain.emit(Path(__file__).parent, {
    "curWaterTemp": (0.0, 35.0, [25.0, 0.0, 4.0]),
    "freeStreamVelocity": (0.0, 15.0, [0.0, 1.0]), "curWaterVelocity": (0.0, 15.0, [0.0, 1.0]),
    "distanceFromLeadingEdge": (0.1, 100.0, [1.0, 0.0, 50.0]),
    "prevHrWindDir": WIND, "curHrWindDir": WIND, "curHrWeight": (0.0, 1.0, [0.0, 0.5, 1.0]),
    "CosZen": (-1.0, 1.0, [0.001, 0.0011, 1.0, 0.999999, 0.0, -1.0, 0.5]),
    "Longitude": (-180.0, 180.0, [0.0, 7.5, -7.5, 180.0, -180.0, 187.5, 360.0, 540.0, -172.5]),
    "Year": (1900, 2100, [1900, 2000, 2100, 2024, 2023, 0, -4]),
    "gyyyy": (1900, 2100, [1900, 2000, 2024]), "gmm": (1, 12, [1, 2, 3, 12]), "gdd": (1, 31, [1, 28, 29, 31]),
    "Month": (1, 12, [1, 2, 12, 0, 13]), "Day": (1, 31, [1, 29, 31]), "leapYear": (0, 1, [True, False]),
    "month": (1, 12, [1, 2, 4, 12, 0, 13]), "day": (1, 31, [28, 29, 30, 31, 32]), "leapYearAdd": (0, 1, [0, 1]),
})
