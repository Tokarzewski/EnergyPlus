"""Realistic inputs for the Psychrometrics unit (building HVAC ranges), one
{"fn": name, "args": [...]} per line, for `rust-anything record --inputs`.
Deterministic: the same file every run. Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261121)

TDB = (-40.0, 60.0)
W = (0.0, 0.03)
PB = (60000.0, 110000.0)
RH = (0.0, 1.0)
H = (-30000.0, 150000.0)
RHOV = (0.0, 0.08)
WATER_T = (0.0, 100.0)
PSAT_T = (-100.0, 200.0)
PRESS = (100.0, 2.0e6)

ARGS = {
    "PsyRhoAirFnPbTdbW": (PB, TDB, W),
    "PsyRhoAirFnPbTdbW_fast": (PB, TDB, W),
    "PsyHfgAirFnWTdb": (W, TDB),
    "PsyHgAirFnWTdb": (W, TDB),
    "PsyHFnTdbW": (TDB, W),
    "PsyHFnTdbW_fast": (TDB, W),
    "PsyCpAirFnW": (W,),
    "PsyCpAirFnW_fast": (W,),
    "PsyTdbFnHW": (H, W),
    "PsyRhovFnTdbRhLBnd0C": (TDB, RH),
    "PsyRhovFnTdbWPb": (TDB, W, PB),
    "PsyRhovFnTdbWPb_fast": (TDB, W, PB),
    "PsyRhFnTdbRhovLBnd0C": (TDB, RHOV),
    "PsyTwbFnTdbWPb": (TDB, W, PB),
    "PsyTwbFnTdbWPb_raw": (TDB, W, PB),
    "PsyVFnTdbWPb": (TDB, W, PB),
    "PsyWFnTdbH": (TDB, H),
    "PsyPsatFnTemp": (PSAT_T,),
    "PsyPsatFnTemp_raw": (PSAT_T,),
    "PsyTsatFnHPb": (H, PB),
    "PsyTsatFnHPb_raw": (H, PB),
    "PsyRhovFnTdbRh": (TDB, RH),
    "PsyRhFnTdbRhov": (TDB, RHOV),
    "PsyRhFnTdbWPb": (TDB, W, PB),
    "PsyWFnTdpPb": (TDB, PB),
    "PsyWFnTdbRhPb": (TDB, RH, PB),
    "PsyWFnTdbTwbPb": (TDB, TDB, PB),
    "PsyHFnTdbRhPb": (TDB, RH, PB),
    "PsyTsatFnPb": (PRESS,),
    "PsyTsatFnPb_raw": (PRESS,),
    "PsyTdpFnWPb": (W, PB),
    "PsyTdpFnTdbTwbPb": (TDB, TDB, PB),
    "CPCW": (WATER_T,),
    "CPHW": (WATER_T,),
    "RhoH2O": (WATER_T,),
    "PsyDeltaHSenFnTdb2Tdb1W": (TDB, TDB, W),
    "PsyDeltaHSenFnTdb2W2Tdb1W1": (TDB, W, TDB, W),
}

PER_FN = 40

for fn, ranges in ARGS.items():
    for i in range(PER_FN):
        if i < 3:
            # the ends and the middle of each range
            args = [(lo, hi, (lo + hi) / 2)[i] for lo, hi in ranges]
        else:
            args = [rng.uniform(lo, hi) for lo, hi in ranges]
        print(json.dumps({"fn": fn, "args": args}))
