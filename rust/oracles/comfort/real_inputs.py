"""Inputs for the thermal comfort helpers: PMV across and beyond the 7-point scale, air speeds
and activity levels of occupants, and air temperatures, plus the pole of the Torr formula at
-235 C. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("plain_inputs", Path(__file__).parents[1] / "plain_inputs.py")
plain = importlib.util.module_from_spec(spec)
spec.loader.exec_module(plain)
NAN = float("nan")

plain.emit(Path(__file__).parent, {
    "PMV": (-3.0, 3.0, [0.0, 3.0, -3.0, 0.5, 10.0, -10.0, 4.85, NAN]),
    "AirVel": (0.0, 2.0, [0.0, 0.1]), "ActMet": (0.7, 4.0, [1.0, 1.0000001, 0.8, 2.0]),
    "Temp": (-10.0, 50.0, [0.0, 20.0, 37.0, -235.0, 100.0]),
})
