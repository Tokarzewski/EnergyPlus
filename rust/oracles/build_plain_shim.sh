#!/usr/bin/env bash
# Builds shim/libep_<unit>_shim.so (<unit> is the first line of gen.args): the real EnergyPlus
# functions gen.args lists, behind plain C functions, statically linked from the EnergyPlus build
# (libenergyplusapi.so hides these symbols). Shared by the plain-number units: each unit's folder
# holds gen.args and real_inputs.py, and its build_shim.sh runs this one on that folder. Prints the CFLAGS rust-anything needs to link the C unit against it.
# Needs the EnergyPlus build in <repo>/build: cmake -S . -B build -G Ninja
#   -DCMAKE_BUILD_TYPE=Release -DLINK_WITH_PYTHON=OFF -DBUILD_FORTRAN=OFF, then ninja energyplusapi.
set -euo pipefail
here="$(cd "${1:?usage: build_plain_shim.sh <unit folder>}" && pwd)"
ep="$(cd "$here/../../.." && pwd)"
b="$ep/build"
p="$b/Products"
[ -f "$p/libenergypluslib.a" ] || { echo "no $p/libenergypluslib.a: build EnergyPlus first" >&2; exit 1; }

mapfile -t args < "$here/gen.args"
unit="${args[0]}"
python3 -I "$here/../gen_plain.py" "$here" "${args[@]}" >/dev/null

# The flags ninja uses for the EnergyPlus sources, minus -fvisibility=hidden so the
# shim's own entry points are exported.
defs=(-DEP_NO_OPENGL -DEP_cache_GlycolSpecificHeat -DEP_psych_errors -DHAVE_FEENABLEEXCEPT
      -DJSON_USE_IMPLICIT_CONVERSIONS=1 -DNDEBUG -DOBJEXXFCL_ALIGN=64 -DXML_STATIC)
incs=(-I"$ep/idd" -I"$ep/src" -I"$ep/src/EnergyPlus/AirflowNetwork/include")
for d in nlohmann ObjexxFCL/src SQLite re2 btwxt/include btwxt/vendor/courierr/include fast_float \
         CLI milo valijson; do
    incs+=(-isystem "$ep/third_party/$d")
done
libs=(libenergypluslib.a libembedded_epjson_source.a libsqlite.a libbcvtb.a libepfmiimport.a
      libepexpat.a libminiziplib.a libDElight.a ../third_party/kiva/libkiva.a
      libairflownetworklib.a libobjexx.a libre2.a libWindows-CalcEngine.a ssc.a shared.a)
largs=()
for l in "${libs[@]}"; do largs+=("$p/$l"); done

g++ -std=c++20 -O3 -fPIC -shared -ffp-contract=off -fno-stack-protector "${defs[@]}" "${incs[@]}" \
    -o "$here/shim/libep_${unit}_shim.so" "$here/shim/ep_${unit}_shim.cc" \
    "${largs[@]}" -lm "$p/liblibtk205.a" "$p/libbtwxt.a" -ldl -lutil

echo "CFLAGS=\"-L$here/shim -Wl,--no-as-needed -lep_${unit}_shim -Wl,-rpath,$here/shim\""
