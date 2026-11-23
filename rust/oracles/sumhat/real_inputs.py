"""Inputs for SpaceHeatBalanceData::calcSumHAT: one heat transfer surface's contribution to the
zone air heat balance sums as EnergyPlus meets it, in SI units. Surfaces are walls, floors and
ceilings, or windows with or without frames and dividers, interior or exterior shades and blinds
(and the conditionally-off variants), equivalent layer constructions and airflow windows that
exhaust to the zone or, in zones with no heat to return air, to the return. Each surface refers
its convection to the zone mean air (the usual case), to the adjacent air, or to the supply air of
a controlled zone with or without system flow yet. Inside convection coefficients, areas and face
temperatures span real zones; window heat gains so far are positive and negative. Each surface is
asked for all ten results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261127)


def pick(*xs):
    return rng.choice(xs)


def maybe_round(v, nd=2):
    return round(v, nd) if rng.random() < 0.3 else v


for _ in range(1250):
    window = 1 if rng.random() < 0.55 else 0
    shading = rng.choices([0, 1, 2, 6, 4, 7, 10, 13, 3, 8, -1], weights=[30, 10, 15, 15, 5, 5, 5, 5, 4, 3, 3])[0]
    eql = 1 if window and rng.random() < 0.1 else 0
    area = pick(rng.uniform(0.5, 6.0), rng.uniform(5.0, 80.0))
    hconv = pick(rng.uniform(0.1, 2.0), rng.uniform(2.0, 6.0))
    tin = rng.uniform(5.0, 35.0)
    tairref = rng.choices([0, 1, 2, -1], weights=[70, 15, 10, 5])[0]
    tempeff = rng.uniform(15.0, 30.0)
    controlled = 1 if tairref == 2 or rng.random() < 0.6 else 0
    has_div = window and rng.random() < 0.4
    dividerarea = rng.uniform(0.02, 0.4) if has_div else 0.0
    dividergain = rng.uniform(-20.0, 30.0) if has_div else 0.0
    projdiv = rng.uniform(0.0, 0.5) if has_div else 0.0
    dividertemp = rng.uniform(5.0, 35.0) if has_div else 0.0
    has_frame = window and rng.random() < 0.5
    framearea = rng.uniform(0.05, 0.6) if has_frame else 0.0
    projfr = rng.uniform(0.0, 0.5) if has_frame else 0.0
    frametemp = rng.uniform(5.0, 35.0) if has_frame else 0.0
    otherconv = rng.uniform(-30.0, 60.0) if eql else 0.0
    natconv = rng.uniform(-10.0, 80.0) if window and shading in (2, 6) else 0.0
    airflow = 0.0 if not window or rng.random() < 0.85 else pick(rng.uniform(0.001, 0.05), 0.0)
    convgain = rng.uniform(-20.0, 150.0) if airflow > 0.0 else 0.0
    retgain = rng.uniform(-20.0, 150.0) if airflow > 0.0 else 0.0
    noheatret = 1 if rng.random() < 0.3 else 0
    heatgain0 = rng.uniform(-300.0, 600.0) if window else 0.0
    if airflow > 0.0 and rng.random() < 0.1:
        heatgain0, retgain = 25.0, -25.0
    sysmcp = 0.0 if rng.random() < 0.2 else rng.uniform(10.0, 3000.0)
    sysmcpt = sysmcp * rng.uniform(12.0, 40.0)
    tszsec = pick(900.0, 600.0, 3600.0, 60.0)
    args = [window, shading, eql, maybe_round(area), maybe_round(hconv), maybe_round(tin), tairref, tempeff, dividerarea,
            dividergain, projdiv, dividertemp, framearea, projfr, frametemp, otherconv, natconv, airflow, convgain,
            retgain, noheatret, heatgain0, controlled, sysmcp, sysmcpt, tszsec]
    for which in range(10):
        print(json.dumps({"fn": "SumHAT", "args": [which] + args}))
