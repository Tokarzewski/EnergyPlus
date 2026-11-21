"""Realistic inputs for the curve unit, one {"fn": name, "args": [...]} per line, for
`rust-anything record --inputs`. Coefficients are at the scale of real equipment fits
(normalised curves around 1.0, higher-order terms shrinking with the input's size), inputs are
in the ranges those curves see (temperatures, part-load ratios, flow fractions), and the limits
are sometimes narrower than the inputs so that clamping is exercised. An output limit of NaN
means "not set". Deterministic: the same file every run.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import importlib.util
import json
import math
import random
from pathlib import Path

spec = importlib.util.spec_from_file_location("gen_curves", Path(__file__).with_name("gen_curves.py"))
gen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gen)

rng = random.Random(20261121)
NAN = float("nan")

# Typical input ranges per curve type (one per input).
TEMP = (5.0, 45.0)
PLR = (0.0, 1.2)
FLOW = (0.2, 1.5)
RANGES = {
    "CurveBiQuadratic": (TEMP, TEMP),
    "CurveBiCubic": (TEMP, TEMP),
    "CurveQuadraticLinear": (PLR, TEMP),
    "CurveCubicLinear": (PLR, TEMP),
    "CurveFanPressureRise": ((0.0, 20.0), (0.0, 1500.0)),
    "CurveChillerPartLoadWithLift": ((0.2, 2.0), (0.0, 1.2), (0.0, 1.5)),
    "CurveTriQuadratic": (TEMP, TEMP, TEMP),
    "CurveQuadLinear": (TEMP, PLR, FLOW, TEMP),
    "CurveQuintLinear": (TEMP, PLR, FLOW, TEMP, FLOW),
    "CurveExponent": ((0.0, 2.0),),
    "CurveExponentialSkewNormal": ((-2.0, 2.0),),
    "CurveSigmoid": ((0.0, 2.0),),
    "CurveRectangularHyperbola1": ((0.0, 2.0),),
    "CurveRectangularHyperbola2": ((0.0, 2.0),),
    "CurveExponentialDecay": ((0.0, 2.0),),
    "CurveDoubleExponentialDecay": ((0.0, 2.0),),
}


def coeffs(name, nc, ranges):
    big = max(abs(hi) for _, hi in ranges)
    if name in ("CurveExponentialSkewNormal", "CurveSigmoid"):
        # widths and exponents must stay away from zero to be a real curve
        c = [rng.uniform(-1, 1), rng.uniform(0.2, 2.0), rng.uniform(-1, 1), rng.uniform(0.2, 2.0)]
        return (c + [rng.uniform(0.5, 2.0)])[:nc]
    if name == "CurveExponent":
        return [rng.uniform(-1, 1), rng.uniform(0.5, 2.0), rng.uniform(0.3, 3.0)]
    if name in ("CurveExponentialDecay", "CurveDoubleExponentialDecay"):
        return [rng.uniform(-1, 1) if i % 2 == 0 or i == 0 else rng.uniform(-3, 1) for i in range(nc)]
    if name.startswith("CurveRectangularHyperbola"):
        return [rng.uniform(0.2, 2.0), rng.uniform(0.05, 1.0), rng.uniform(-0.5, 0.5)]
    # polynomials: term i of roughly order k shrinks like big**-k
    out = []
    for i in range(nc):
        order = 0 if i == 0 else min(3, 1 + (i - 1) // max(1, len(ranges)))
        out.append(rng.uniform(-1.0, 1.0) * (1.0 if i == 0 else big ** -order))
    out[0] = rng.uniform(0.5, 1.5)
    return out


for name, _, nc, nv in gen.CURVES:
    ranges = RANGES.get(name, (PLR,) * nv)
    for k in range(60):
        c = coeffs(name, nc, ranges)
        lims = []
        for lo, hi in ranges:
            if k % 4 == 0:  # limits narrower than the inputs: clamping
                a, b = sorted((rng.uniform(lo, hi), rng.uniform(lo, hi)))
                lims += [a, b]
            else:
                lims += [lo, hi]
        r = k % 5
        out = {0: [NAN, NAN], 1: [0.0, NAN], 2: [NAN, 1.0], 3: [0.0, 1.5], 4: [rng.uniform(0, 0.8), rng.uniform(0.9, 1.5)]}[r]
        v = []
        for lo, hi in ranges:
            span = hi - lo
            v.append(rng.uniform(lo - 0.1 * span, hi + 0.1 * span))
        if k == 0:
            v = [lo for lo, _ in ranges]  # exactly on the lower limits
        if k == 1:
            v = [hi for _, hi in ranges]  # exactly on the upper limits
        args = c + lims + out + v
        print(json.dumps({"fn": name, "args": [x if not math.isnan(x) else {"$num": "NaN"} for x in args]}))
