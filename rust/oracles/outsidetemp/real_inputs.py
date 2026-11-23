"""Inputs for CalcOutsideSurfTemp: the outside face heat balance of exterior surfaces as
EnergyPlus meets them, in SI units. Constructions are heavy (cross CTF term below 0.01: "slow"
conduction, the inside face from the last iteration) or light (above 0.01: "quick", the inside
heat balance folded in), with the cross term sometimes exactly 0.01; about one in ten carries an
embedded source or sink (a radiant slab). Convection and linearized radiation coefficients,
CTF history parts, absorbed shortwave, inside face and zone air temperatures, weather and sky
temperatures span winter nights to summer afternoons. Some surfaces have exterior movable
insulation (never with a source or sink, which EnergyPlus refuses), a surrounding surfaces list,
a ground surfaces temperature or an other side conditions model. Each surface is asked for all
six outputs.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261223)


def pick(*xs):
    return rng.choice(xs)


def maybe_round(v, nd=2):
    return round(v, nd) if rng.random() < 0.3 else v


for _ in range(2000):
    heavy = rng.random() < 0.5
    if heavy:
        cross0 = pick(rng.uniform(1e-6, 0.0099), rng.uniform(1e-4, 0.005), 0.0)
    else:
        cross0 = pick(rng.uniform(0.0101, 1.0), rng.uniform(0.2, 6.0), rng.uniform(0.01, 0.05))
    if rng.random() < 0.03:
        cross0 = 0.01
    inside0 = cross0 + rng.uniform(0.3, 12.0)
    outside0 = cross0 + rng.uniform(0.3, 12.0)
    src = 1 if rng.random() < 0.1 else 0
    srcout0, srcin0 = (rng.uniform(0.0, 1.5), rng.uniform(0.0, 1.5)) if src else (0.0, 0.0)
    hconvint = pick(rng.uniform(0.1, 1.0), rng.uniform(1.0, 5.0), rng.uniform(5.0, 10.0))
    hconvext = pick(rng.uniform(1.0, 6.0), rng.uniform(6.0, 25.0), rng.uniform(25.0, 60.0))
    hairext = rng.uniform(0.0, 5.0)
    hskyext = rng.uniform(0.0, 5.0)
    hgrdext = rng.uniform(0.0, 5.0)
    hsrdext = 0.0 if rng.random() < 0.7 else rng.uniform(0.1, 3.0)
    text = rng.uniform(-30.0, 45.0)
    odb = text if rng.random() < 0.7 else rng.uniform(-30.0, 45.0)
    skytemp = odb - rng.uniform(0.0, 30.0)
    tempin = rng.uniform(10.0, 35.0)
    ctfout = outside0 * rng.uniform(-20.0, 40.0) + rng.uniform(-50.0, 50.0) * (cross0 + 0.1)
    ctfin = inside0 * rng.uniform(10.0, 35.0) + rng.uniform(-50.0, 50.0) * (cross0 + 0.1)
    qswout = 0.0 if rng.random() < 0.3 else rng.uniform(0.0, 900.0)
    qswin = 0.0 if rng.random() < 0.3 else rng.uniform(0.0, 150.0)
    qsrc = rng.uniform(-60.0, 250.0) if src else 0.0
    qaddout = 0.0 if rng.random() < 0.9 else rng.uniform(-50.0, 300.0)
    qintgains = rng.uniform(0.0, 30.0)
    qnetlwin = rng.uniform(-40.0, 40.0)
    mat = rng.uniform(15.0, 30.0)
    hmov = 0.0
    if not src and rng.random() < 0.15:
        hmov = pick(rng.uniform(0.2, 2.0), rng.uniform(2.0, 15.0), 1e-9)
    qswmvins = rng.uniform(0.0, 600.0) if hmov > 0 else 0.0
    hassrd = 1 if rng.random() < 0.15 else 0
    srdtemp = rng.uniform(-20.0, 45.0) if hassrd else 0.0
    usegnd = 1 if rng.random() < 0.1 else 0
    gndtemp = rng.uniform(-10.0, 40.0) if usegnd else 0.0
    osc = 1 if rng.random() < 0.1 else 0
    osctrad, oschrad = (rng.uniform(-20.0, 45.0), rng.uniform(0.0, 6.0)) if osc else (0.0, 0.0)
    args = [maybe_round(v) for v in (cross0, inside0, outside0, srcout0, srcin0)]
    args += [src] + [maybe_round(v) for v in (hconvint, hconvext, hairext, hskyext, hgrdext, hsrdext, ctfout, ctfin,
                                                qswout, qswin, qsrc, tempin, qaddout, qintgains, qnetlwin, mat,
                                                qswmvins)]
    args += [hmov] + [maybe_round(v) for v in (text, skytemp, odb)]
    args += [hassrd, srdtemp, usegnd, gndtemp, osc, osctrad, oschrad]
    for which in range(6):
        print(json.dumps({"fn": "OutsideSurfTemp", "args": [which] + args}))
