"""Inputs for ZoneSpaceHeatBalanceData::calcPredictedSystemLoad: one zone's predicted sensible
load to its thermostat set points as EnergyPlus meets it, in SI units. Zones are uncontrolled or
have single heating, single cooling, single heating and cooling, or dual set point thermostats,
under the third order, analytical or Euler zone air solution, with and without RoomAirflowNetwork
fractions, IT equipment return air adjustment, a system node and staged thermostats at every
stage. The zone air heat balance terms (temperature dependent and independent loads, air heat
capacity over the time step, last temperature) span real zones heating, cooling and floating in
the deadband; set points, load correction factors and multipliers are typical ones, and a few
dual set points cross so that EnergyPlus stops. Each zone is asked for all ten results.
Usage: python3 real_inputs.py > real-inputs.jsonl"""

import json
import random

rng = random.Random(20261128)


def pick(*xs):
    return rng.choice(xs)


for _ in range(1250):
    ctrl = rng.choices([0, 1, 2, 3, 4, -1], weights=[8, 15, 15, 12, 48, 2])[0]
    algo = rng.choices([0, 1, 2], weights=[50, 35, 15])[0]
    rafnfrac = 0.0 if rng.random() < 0.85 else rng.uniform(0.2, 1.0)
    tempdep = 0.0 if rng.random() < 0.05 else rng.uniform(5.0, 3000.0)
    aircap = rng.uniform(50.0, 20000.0)
    t1 = rng.uniform(12.0, 32.0)
    setpt = round(rng.uniform(18.0, 26.0), 1)
    setptlo = round(rng.uniform(15.0, 22.0), 1)
    setpthi = round(setptlo + pick(0.0, 1.0, 2.0, 3.0, 5.0, rng.uniform(0.5, 8.0)), 2)
    if ctrl == 4 and rng.random() < 0.03:
        setptlo, setpthi = setpthi + 1.0, setptlo
    # The temperature at which the zone floats with no system: the load is tempdep * (T - Tfloat).
    tfloat = rng.uniform(10.0, 34.0)
    tempind = tempdep * tfloat + rng.uniform(-50.0, 50.0)
    if algo == 2 or (algo == 1 and tempdep == 0.0):
        tempind = tempdep * tfloat + aircap * (t1 - tfloat) * rng.uniform(-0.2, 0.2)
    hasite = 1 if rng.random() < 0.1 else 0
    itetemp = rng.uniform(20.0, 35.0) if hasite else 0.0
    beginsim = 1 if rng.random() < 0.05 else 0
    hasnode = 1 if rng.random() < 0.8 else 0
    nodetemp = rng.uniform(14.0, 30.0)
    staged = 1 if rng.random() < 0.1 else 0
    stagenum = pick(0, 1, 2, 3, -1, -2, -3) if staged else 0
    setptlast = pick(0.0, setpt, setptlo, setpthi, rng.uniform(15.0, 27.0))
    lcf = 1.0 if rng.random() < 0.7 else rng.uniform(0.8, 1.2)
    mult = pick(1, 1, 1, 2, 5, 10)
    listmult = pick(1, 1, 1, 1, 3)
    args = [ctrl, algo, rafnfrac, tempdep, tempind, aircap, t1, setpt, setptlo, setpthi, hasite, itetemp, beginsim,
            hasnode, nodetemp, staged, stagenum, setptlast, lcf, mult, listmult]
    for which in range(10):
        print(json.dumps({"fn": "PredictedSystemLoad", "args": [which] + args}))
