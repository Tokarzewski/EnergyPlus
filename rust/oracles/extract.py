"""Print the C++ bodies (comments removed) of a header's plain-number functions, found in the
header itself or in the given .cc files. Usage: python3 extract.py <src dir> <header> <cc>... [-- names]"""

import importlib.util
import re
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("g", Path(__file__).with_name("gen_plain.py"))
g = importlib.util.module_from_spec(spec)
spec.loader.exec_module(g)

argv = sys.argv[1:]
names = argv[argv.index("--") + 1:] if "--" in argv else []
argv = argv[: argv.index("--")] if "--" in argv else argv
src, header, *ccs = argv
src = Path(src)
texts = {f: re.sub(r"//[^\n]*", "", (src / f).read_text()) for f in [header, *ccs]}
for name, ret, ps in g.functions(src / header):
    if names and name not in names:
        continue
    for fname, text in texts.items():
        hit = None
        for m in re.finditer(r"\n[ \t]*(?:(?:inline|constexpr|static)\s+)*(?:Real64|double|int|bool)\s+(?:\w+::)?" + name + r"\s*\(([^{;]*?)\)\s*(?:noexcept\s*)?\{", text):
            if "state" in m.group(1) or "Array" in m.group(1):
                continue
            i, depth = m.end(), 1
            while depth:
                depth += {"{": 1, "}": -1}.get(text[i], 0)
                i += 1
            hit = (text[: m.start()].count("\n") + 2, re.sub(r"\n\s*\n", "\n", text[m.start(): i]).strip("\n"))
            break
        if hit:
            print(f"// {fname}:{hit[0]}\n{hit[1]}\n")
            break
    else:
        print(f"// MISSING {name}\n")
