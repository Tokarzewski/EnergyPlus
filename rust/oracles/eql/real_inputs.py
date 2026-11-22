"""Inputs for the window equivalent-layer helpers: insect-screen wire and spacing, venetian slat
width and crown, gap gas properties at window temperatures (air coefficients), Rayleigh numbers
across the Nusselt fit's ranges, surface temperatures and emittances, and the tolerances FEQX
compares with. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("plain_inputs", Path(__file__).parents[1] / "plain_inputs.py")
plain = importlib.util.module_from_spec(spec)
spec.loader.exec_module(plain)
NAN = float("nan")

TK = (250.0, 330.0, [273.15, 293.15])
plain.emit(Path(__file__).parent, {
    "D": (0.0, 0.002, [0.0, 0.001]), "S": (0.0, 0.004, [0.0, -0.001, 0.001]),
    "OPENNESS": (0.0, 1.0, [0.0, 1.0, 1.5, -0.1]),
    "W": (0.0, 0.05, [0.0, 0.025]), "C": (0.0, 0.01, [0.0, 0.03, 0.0125]),
    "TM": TK, "T": (0.005, 0.03, [0.0127]), "DT": (0.0, 30.0, [0.0, 10.0]),
    "AK": (2.8e-3, 2.9e-3, [2.873e-3]), "BK": (7.5e-5, 8e-5, [7.76e-5]), "CK": (0.0, 1e-9, [0.0]),
    "ACP": (1000.0, 1010.0, [1002.737]), "BCP": (0.0, 0.02, [0.012324]), "CCP": (0.0, 1e-5, [0.0]),
    "AVISC": (3.5e-6, 4e-6, [3.723e-6]), "BVISC": (4.5e-8, 5e-8, [4.94e-8]), "CVISC": (0.0, 1e-12, [0.0]),
    "RHOGAS": (1.0, 1.5, [1.29]),
    "RA": (0.0, 1e6, [0.0, 10000.0, 10000.000001, 50000.0, -20000.0, 1e9]),
    "T1": TK, "T2": TK, "E1": (0.0, 1.0, [0.84, 0.001, 0.0011, 0.0]), "E2": (0.0, 1.0, [0.84, 0.001, 0.0011]),
    "L": (0.0, 0.03, [0.0, 0.001, 0.0127]), "TG": TK, "TI": TK,
    "OMEGA_DEG": (-90.0, 90.0, [0.0, 90.0, -90.0]),
    "a": (-10.0, 10.0, [0.0, 1.0]), "b": (-10.0, 10.0, [0.0, 1.0, 1.0000001]),
    "tolF": (0.0, 1e-3, [0.0, 1e-6]), "tolAbs": (0.0, 1e-6, [0.0, 1e-10, 1e-12]),
    "J": (0.0, 1000.0, [0.0, 400.0]), "Emiss": (0.0, 1.0, [0.0, 0.001, 0.84]),
})
