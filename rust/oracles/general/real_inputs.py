"""Inputs for the General helpers: dates as day of year and month and day, hours 0 to 24, angles
in degrees, and the zero and near-zero divisors SafeDivide and epexp special-case.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("plain_inputs", Path(__file__).parents[1] / "plain_inputs.py")
plain = importlib.util.module_from_spec(spec)
spec.loader.exec_module(plain)
NAN = float("nan")

DATE = (1, 366, [1, 365, 366, 0, 59, 60])
HOUR = (0, 24, [0, 24, 12, 1])
plain.emit(Path(__file__).parent, {
    "Month": (1, 12, [1, 2, 3, 12, 0, 13, -1]), "Day": (1, 31, [1, 28, 29, 31, 0, 32]),
    "LeapYearValue": (0, 1, [0, 1]),
    "TestDate": DATE, "StartDate": DATE, "EndDate": DATE,
    "TestHour": HOUR, "StartHour": HOUR, "EndHour": HOUR,
    "a": (-1000.0, 1000.0, [0.0, 1.0, -1.0]),
    "b": (-1.0, 1.0, [0.0, -0.0, 1e-10, -1e-10, 1e-11, -1e-11, NAN, 2.0]),
    "AzmA": (0.0, 360.0, [0.0, 180.0, 360.0, -180.0, 540.0]),
    "AzmB": (0.0, 360.0, [0.0, 180.0, 360.0, 359.0, 1.0]),
    "numerator": (-50.0, 50.0, [0.0, 1.0, -800.0, 800.0]),
    "denominator": (-10.0, 10.0, [0.0, -0.0, 1.0, 1e-300, NAN]),
    "Lower": (-100.0, 100.0, [0.0, 1.0]), "Upper": (-100.0, 100.0, [0.0, 2.0]),
    "InterpFac": (0.0, 1.0, [0.0, 1.0, 0.5, -0.5, 1.5]),
})
