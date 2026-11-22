"""Inputs for the leaf helpers: interpolation points inside and outside their intervals, water
tank temperatures and flows (with the no-loss, no-flow and steady cases), window sizes, triangle
and quadrilateral corners with points on their edges, ice storage temperatures around freezing,
glass angles and indices, collector gaps at every tilt, and micro-CHP engine states.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import importlib.util
from pathlib import Path

spec = importlib.util.spec_from_file_location("plain_inputs", Path(__file__).parents[1] / "plain_inputs.py")
plain = importlib.util.module_from_spec(spec)
spec.loader.exec_module(plain)
NAN = float("nan")

T = (-10.0, 90.0, [0.0, 20.0, 60.0])
P = (-2.0, 2.0, [0.0, 1.0, -1.0, 0.5])
FLOW = (0.0, 5.0, [0.0, 1.0])
plain.emit(Path(__file__).parent, {
    "X": (-1.0, 2.0, [0.0, 1.0, 0.5]), "Y": (-1.0, 2.0, [0.0, 1.0, 0.5]),
    "X0": (0.0, 0.0, [0.0]), "X1": (0.0, 1.0, [1.0, 0.0, 0.5]), "X2": (1.0, 2.0, [1.0, 2.0]),
    "Y1": (0.0, 1.0, [0.0]), "Y2": (1.0, 2.0, [1.0, 2.0]),
    "F0": (-5.0, 5.0, [0.0]), "F1": (-5.0, 5.0, [1.0]),
    "Fx1y1": (-5.0, 5.0, [0.0]), "Fx1y2": (-5.0, 5.0, [1.0]), "Fx2y1": (-5.0, 5.0, [2.0]), "Fx2y2": (-5.0, 5.0, [3.0]),
    "SwitchFac": (-0.5, 1.5, [0.0, 1.0, 0.5, NAN]), "A": (-10.0, 10.0, [0.0]), "B": (-10.0, 10.0, [1.0]),
    "Ti": T, "Tf": T, "Ta": T, "T1": T, "T2": T,
    "m": (1.0, 500.0, [100.0, 0.0]), "Cp": (4000.0, 4200.0, [4180.0, 0.0]),
    "m1": (0.0, 2.0, [0.0]), "m2": (0.0, 2.0, [0.0]), "UA": (0.0, 10.0, [0.0, 2.0]),
    "Q": (-5000.0, 20000.0, [0.0, 4500.0]), "t": (0.0, 3600.0, [0.0, 60.0, 1e6]),
    "Width": (0.1, 3.0, [1.0, 0.0]), "Height": (0.1, 3.0, [1.0, 0.0]), "x": (-5.0, 5.0, [0.0, -0.0, -1.0]),
    "WaterTotal": (0.0, 1e6, [0.0, 1.0]), "ConversionFactor": (0.001, 1000.0, [1.0, 0.0, 3.785]),
    "qx": P, "qy": P, "x1": P, "y1": P, "x2": P, "y2": P, "x3": P, "y3": P,
    "ax": P, "ay": P, "bx": P, "by": P, "cx": P, "cy": P, "dx": P, "dy": P,
    "Tin": (-10.0, 20.0, [0.0, -5.0, 10.0]), "Tout": (-10.0, 20.0, [0.0, -5.0, 10.0]), "Tfr": (-5.0, 0.0, [0.0]),
    "Temp": (-40.0, 100.0, [0.0, -40.0, 32.0, 212.0]),
    "ValueToBound": (-10.0, 10.0, [0.0, NAN]), "LowerBound": (-5.0, 0.0, [0.0]), "UpperBound": (0.0, 5.0, [0.0]),
    "IntegerIsWithinTwoValues": {"ValueToCheck": (-5, 5, [0, -5, 5]), "LowerBound": (-5, 0, [0]), "UpperBound": (0, 5, [0])},
    "theta": (0.0, 1.5, [0.0, 0.5, 1.0]), "glass_thickness": (0.001, 0.01, [0.003]),
    "refrac_index_glass": (1.3, 1.7, [1.526]), "k_glass": (4.0, 32.0, [4.0]),
    "AspRat": (1.0, 100.0, [10.0, 0.0]), "Tilt": (0.0, 90.0, [0.0, 30.0, 60.0, 90.0, 75.0]),
    "Tso": (0.0, 60.0, [20.0]), "Tsi": (0.0, 60.0, [20.0]), "Gr": (0.0, 1e7, [0.0, 1e4, 5e4, 1e5, 14085.0]),
    "v": (-2.0, 2.0, [0.0, 1.0, NAN]), "min": (-1.0, 0.0, [0.0]), "max": (0.0, 1.0, [1.0]),
    "MakeUpWaterMassFlowRate": FLOW, "Density": (900.0, 1000.0, [999.2, 0.0]), "MakeUpWaterMass": (0.0, 500.0, [0.0]),
    "TcwOut": T, "TcwIn": T, "Teng": T, "Troom": T, "TengLast": T, "TcwoutLast": T,
    "MCeng": (100.0, 5e4, [1000.0]), "MCcw": (100.0, 5e4, [1000.0]), "UAHX": (1.0, 1000.0, [100.0]),
    "UAskin": (0.0, 100.0, [10.0, 0.0]), "Qgenss": (0.0, 2e4, [0.0]), "MdotCpcw": (0.0, 2000.0, [0.0, 400.0]),
    "time": (0.0, 3600.0, [0.0, 60.0, 1e7]), "NomHeatGen": (0.0, 2e4, [1000.0]),
})
