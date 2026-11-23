"""Inputs for ZoneSpaceHeatBalanceData::correctAirTemp for one fully mixed zone as EnergyPlus meets
it, in SI units: offices, homes and halls (30 to 5000 m3) under each zone air solution algorithm,
from the zone or the system time step history, at system time steps of one minute to an hour;
heated, cooled and free running zones with up to two internal gain devices, infiltration,
ventilation and outdoor air terms, up to two supply air inlets of hot, cold, dry or humid air,
zones with and without a system node, multipliers, non-air system responses (radiant and
baseboard heat) and lagged system loads, latent gains and moisture flows, from sea level to high
altitude. Each case is asked for all fourteen results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261123)


def pick(*xs):
    return rng.choice(xs)


def hist(t, spread):
    # A zone time step history drifting towards t.
    d = rng.uniform(-spread, spread)
    return [t + d, t + 2.0 * d + rng.uniform(-0.1, 0.1), t + 3.0 * d + rng.uniform(-0.2, 0.2)]


for _ in range(900):
    algo = rng.choices([0, 1, 2], weights=[50, 35, 15])[0]
    usezonehist = 1 if rng.random() < 0.7 else 0
    hasnode = 1 if rng.random() < 0.75 else 0
    controlled = hasnode if rng.random() < 0.9 else 1 - hasnode
    mult = pick(1, 1, 1, 2, 5, 10)
    listmult = pick(1, 1, 1, 3)
    volume = pick(rng.uniform(30.0, 300.0), rng.uniform(300.0, 5000.0))
    capsens = pick(1.0, 1.0, rng.uniform(1.0, 10.0))
    capmoist = pick(1.0, 1.0, rng.uniform(1.0, 10.0))
    tsyssec = pick(60.0, 300.0, 600.0, 900.0, 3600.0, rng.uniform(60.0, 900.0))
    outbaro = pick(101325.0, rng.uniform(80000.0, 102500.0))
    outhumrat = pick(rng.uniform(0.002, 0.012), rng.uniform(0.0005, 0.02))
    mat = pick(rng.uniform(18.0, 26.0), rng.uniform(-5.0, 40.0))
    airhumrat = pick(rng.uniform(0.004, 0.012), rng.uniform(0.001, 0.02))
    xmat = hist(mat, pick(0.05, 0.5, 2.0))
    dsxmat = hist(mat, pick(0.02, 0.2, 1.0))
    wprev = [airhumrat + rng.uniform(-0.0005, 0.0005) for _ in range(3)]
    dswprev = [airhumrat + rng.uniform(-0.0003, 0.0003) for _ in range(3)]
    t1 = mat + pick(0.0, rng.uniform(-0.5, 0.5))
    w1 = airhumrat + pick(0.0, rng.uniform(-0.0003, 0.0003))
    ndev = rng.choice([0, 1, 2])
    gain = volume * pick(3.0, 10.0, 25.0)
    cg1, cg2 = rng.uniform(0.0, gain), rng.uniform(0.0, gain / 2.0)
    rg1, rg2 = pick(0.0, rng.uniform(0.0, gain / 5.0)), pick(0.0, rng.uniform(0.0, gain / 5.0))
    noheatret = 1 if rng.random() < 0.3 else 0
    radsys = pick(0.0, 0.0, rng.uniform(0.0, 2000.0))
    outdb = pick(rng.uniform(-20.0, 5.0), rng.uniform(5.0, 25.0), rng.uniform(25.0, 40.0))
    ach = pick(0.0, rng.uniform(0.1, 1.0), rng.uniform(1.0, 5.0))
    mcpi = volume * ach / 3600.0 * 1.2 * 1005.0
    tinf = pick(outdb, rng.uniform(-20.0, 40.0))
    mcpti = mcpi * tinf
    mcpv = pick(0.0, 0.0, mcpi * rng.uniform(0.5, 3.0))
    mcptv = mcpv * pick(outdb, tinf)
    mdotcpoa = pick(0.0, 0.0, rng.uniform(0.0, 200.0))
    ninlets = rng.choice([0, 1, 2, 2])
    supply = volume * pick(0.0, rng.uniform(1.0, 6.0), rng.uniform(6.0, 15.0)) / 3600.0 * 1.2
    m1 = supply * rng.uniform(0.3, 1.0)
    m2 = supply - m1 if rng.random() < 0.8 else rng.uniform(0.0, supply)
    tin1 = pick(rng.uniform(12.0, 16.0), rng.uniform(28.0, 45.0), rng.uniform(16.0, 28.0))
    tin2 = tin1 + pick(0.0, rng.uniform(-3.0, 3.0))
    win1 = pick(rng.uniform(0.006, 0.009), rng.uniform(0.001, 0.015))
    win2 = win1 + pick(0.0, rng.uniform(-0.001, 0.001))
    nodemflow = (m1 + m2) * pick(1.0, 1.0, rng.uniform(0.0, 1.2)) * mult * listmult
    nonair = pick(0.0, 0.0, rng.uniform(-3000.0, 6000.0))
    sysdeplag = pick(0.0, 0.0, rng.uniform(-500.0, 500.0))
    latentgain = pick(0.0, rng.uniform(0.0, volume * 3.0))
    oamfl = mcpi / 1005.0 * pick(1.0, rng.uniform(0.9, 1.1))
    vamfl = mcpv / 1005.0
    mdotoa = mdotcpoa / 1005.0
    args = [usezonehist, algo, hasnode, controlled, mult, listmult, volume, capsens, capmoist, tsyssec,
            outbaro, outhumrat, mat, airhumrat] + xmat + dsxmat + wprev + dswprev + [
            t1, w1, ndev, cg1, rg1, cg2, rg2, noheatret, radsys, mcpi, mcpti, mcpv, mcptv, mdotcpoa, outdb,
            ninlets, m1, tin1, win1, m2, tin2, win2, nodemflow, nonair, sysdeplag, latentgain, oamfl, vamfl,
            mdotoa]
    for which in range(14):
        print(json.dumps({"fn": "CorrectAirTemp", "args": [which] + args}))
