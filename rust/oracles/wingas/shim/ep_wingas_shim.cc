// Written by hand: these WindowManager routines read the gap's gases, fractions and width (and
// for NusseltNumber the window's tilt and height) from EnergyPlusData, which gen_plain.py cannot
// build. Plain extern "C" entry points over the real EnergyPlus code; gap 0 and surface 1 are set
// up per call.
//
// Gases g1..g4 are EnergyPlus's built-in Material::gases (1 air, 2 argon, 3 krypton, 4 xenon);
// `quad` adds small multiples of it to each gas's quadratic coefficients so those terms are
// exercised. One EnergyPlusData for the process, built when the library loads, with console
// output off.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/Material.hh>
#include <EnergyPlus/WindowManager.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataSurface->Surface.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

// Gap 0 with nmix (1 to 4) of gases g (1 to 4) and fractions f; false for anything else.
bool gap(double width, int nmix, int const g[4], double const f[4], double quad)
{
    if (nmix < 1 || nmix > 4) {
        return false;
    }
    auto &gp = st().dataWindowManager->gaps[0];
    gp = Window::WindowGap();
    gp.numGases = nmix;
    gp.width = width;
    for (int i = 0; i < nmix; ++i) {
        if (g[i] < 1 || g[i] > 4) {
            return false;
        }
        Material::Gas gas = Material::gases[g[i]];
        gas.con.c2 += quad * 1e-9;
        gas.vis.c2 += quad * 1e-12;
        gas.cp.c2 += quad * 1e-6;
        gp.gases[i] = gas;
        gp.gasFracts[i] = f[i];
    }
    return true;
}
} // namespace

extern "C" {
// WindowGasConductance for a gap between surfaces at tleft and tright (K): which picks con (0),
// pr (1) or gr (2).
double ep_GapCon(int which, double tleft, double tright, double width, int nmix, int g1, int g2, int g3, int g4, double f1, double f2,
                 double f3, double f4, double quad)
{
    int const g[4] = {g1, g2, g3, g4};
    double const f[4] = {f1, f2, f3, f4};
    if (!gap(width, nmix, g, f, quad)) {
        return NAN;
    }
    Real64 con = -1, pr = -1, gr = -1;
    Window::WindowGasConductance(st(), tleft, tright, 0, con, pr, gr);
    return which == 0 ? con : which == 1 ? pr : which == 2 ? gr : NAN;
}

// WindowGasPropertiesAtTemp: which picks dens (0) or visc (1).
double ep_GapProps(int which, double tmean, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad)
{
    int const g[4] = {g1, g2, g3, g4};
    double const f[4] = {f1, f2, f3, f4};
    if (!gap(0.0127, nmix, g, f, quad)) {
        return NAN;
    }
    Real64 dens = -1, visc = -1;
    Window::WindowGasPropertiesAtTemp(st(), tmean, 0, dens, visc);
    return which == 0 ? dens : which == 1 ? visc : NAN;
}

// NusseltNumber for a gap of the given width in a window of the given tilt (deg) and height (m);
// a height of 0 or less passes SurfNum 0 (the nominal 1.524 m window). The result reads -1 where
// the routine leaves it unset (a NaN tilt with the window heated from below).
double ep_GapNusselt(double tso, double tsi, double width, double height, double tilt, double gr, double pr)
{
    auto &s = st();
    s.dataWindowManager->gaps[0].width = width;
    s.dataWindowManager->tilt = tilt;
    s.dataSurface->Surface(1).Height = height;
    Real64 gnu = -1;
    Window::NusseltNumber(s, height > 0.0 ? 1 : 0, tso, tsi, 0, gr, pr, gnu);
    return gnu;
}
}
