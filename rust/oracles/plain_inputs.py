"""Realistic inputs for a plain-number unit, shared by rust/oracles/<unit>/real_inputs.py.

A unit's real_inputs.py calls `emit(unit_dir, RANGES)`: for every function in the unit's C header
(<unit>.h, written by gen_plain.py) it prints `count` cases, one {"fn", "args"} JSON object per
line, for `rust-anything record --inputs`. Each parameter's values come from RANGES by name:
(low, high, values worth hitting exactly); an int parameter gets integers. Every third case puts
each parameter on one of its exact values in turn, the others are uniform in [low, high]. A
`{"fn": [...]}` entry in RANGES overrides a parameter for one function only. Deterministic."""

import importlib.util
import json
import math
import random
from pathlib import Path

_spec = importlib.util.spec_from_file_location("gen_plain", Path(__file__).with_name("gen_plain.py"))
_gen = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_gen)


def _value(rng, kind, rule, k):
    lo, hi, special = rule
    if k % 3 == 0 and special:
        v = special[(k // 3) % len(special)]
    elif kind == "int":
        v = rng.randint(int(lo), int(hi))
    elif kind == "bool":
        v = rng.random() < 0.5
    else:
        v = rng.uniform(lo, hi)
    if isinstance(v, float) and math.isnan(v):
        return {"$num": "NaN"}
    return v


def emit(unit_dir, ranges, count=60, seed=20261122):
    unit_dir = Path(unit_dir)
    header = next(p for p in unit_dir.glob("*.h"))
    rng = random.Random(seed)
    for name, _, ps in _gen.functions(header):
        local = ranges.get(name, {})
        for k in range(count):
            args = []
            for kind, p in ps:
                rule = local.get(p) or ranges.get(p) or ((0, 1, [True, False]) if kind == "bool" else None)
                if rule is None:
                    raise SystemExit(f"{name}: no range for {p}")
                args.append(_value(rng, kind, rule, k))
            print(json.dumps({"fn": name, "args": args}))
