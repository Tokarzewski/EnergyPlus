// Written by hand: TARCOG's gas property and solver routines take Array1D/Array2D tables and
// return several results through references, which gen_plain.py cannot build. Plain extern "C"
// entry points over the real EnergyPlus TARCOGGasses90 and TARCOGCommon code.
//
// The gas table holds the ISO 15099 coefficients of air (1), argon (2), krypton (3) and xenon (4);
// `quad` adds small multiples of it to each gas's quadratic coefficients so those terms are
// exercised. One EnergyPlusData for the process (the routines keep scratch arrays in it), built
// when the library loads, with console output off.
#include <cmath>
#include <string>
#include <ObjexxFCL/Array1D.hh>
#include <ObjexxFCL/Array2D.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/TARCOGCommon.hh>
#include <EnergyPlus/TARCOGGasses90.hh>
#include <EnergyPlus/TARCOGGassesParams.hh>

using namespace EnergyPlus;
using TARCOGGassesParams::maxgas;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

// a, b, c of conductivity, viscosity and specific heat, and the molecular weight.
double const GAS[4][10] = {
    {2.873e-3, 7.760e-5, 0.0, 3.723e-6, 4.940e-8, 0.0, 1002.7370, 1.2324e-2, 0.0, 28.97},
    {2.285e-3, 5.149e-5, 0.0, 3.379e-6, 6.451e-8, 0.0, 521.9285, 0.0, 0.0, 39.948},
    {9.443e-4, 2.826e-5, 0.0, 2.213e-6, 7.777e-8, 0.0, 248.0907, 0.0, 0.0, 83.8},
    {4.538e-4, 1.723e-5, 0.0, 1.069e-6, 7.414e-8, 0.0, 158.3397, 0.0, 0.0, 131.3},
};
} // namespace

extern "C" {
// GASSES90 for a mixture of nmix (1 to 4) gases g1..g4 (1 to 4) with fractions f1..f4, under
// standard 0 (ISO 15099), 1 (EN 673 declared) or 2 (EN 673 design): which picks con (0), visc (1),
// dens (2), cp (3), pr (4) or nperr (5). Results the routine does not set (it returns early on a
// zero fraction) read -1.
double ep_Gasses90(int which, int standard, double tmean, double pres, int nmix, int g1, int g2, int g3, int g4, double f1, double f2,
                   double f3, double f4, double quad)
{
    int const g[4] = {g1, g2, g3, g4};
    if (nmix < 1 || nmix > 4 || standard < 0 || standard > 2) {
        return NAN;
    }
    for (int i = 0; i < nmix; ++i) {
        if (g[i] < 1 || g[i] > 4) {
            return NAN;
        }
    }
    Array1D_int iprop(maxgas, 1);
    Array1D<Real64> frct(maxgas, 0.0), xwght(maxgas, 0.0);
    Array2D<Real64> xgcon(3, maxgas, 0.0), xgvis(3, maxgas, 0.0), xgcp(3, maxgas, 0.0);
    for (int k = 1; k <= 4; ++k) {
        double const *c = GAS[k - 1];
        for (int t = 1; t <= 3; ++t) {
            xgcon(t, k) = c[t - 1] + (t == 3 ? quad * 1e-9 : 0.0);
            xgvis(t, k) = c[t + 2] + (t == 3 ? quad * 1e-12 : 0.0);
            xgcp(t, k) = c[t + 5] + (t == 3 ? quad * 1e-6 : 0.0);
        }
        xwght(k) = c[9];
    }
    double const f[4] = {f1, f2, f3, f4};
    for (int i = 1; i <= nmix; ++i) {
        iprop(i) = g[i - 1];
        frct(i) = f[i - 1];
    }
    Real64 con = -1, visc = -1, dens = -1, cp = -1, pr = -1;
    int nperr = 0;
    std::string msg;
    TARCOGGasses90::GASSES90(st(), tmean, iprop, frct, pres, nmix, xwght, xgcon, xgvis, xgcp, con, visc, dens, cp, pr,
                             static_cast<TARCOGGassesParams::Stdrd>(standard), nperr, msg);
    double const r[6] = {con, visc, dens, cp, pr, static_cast<double>(nperr)};
    return which >= 0 && which < 6 ? r[which] : NAN;
}

// GassesLow: which picks con (0) or nperr (1); con reads -1 when the routine returns early.
double ep_GassesLow(int which, double tmean, double mwght, double pressure, double gama)
{
    Real64 con = -1;
    int nperr = 0;
    std::string msg;
    TARCOGGasses90::GassesLow(tmean, mwght, pressure, gama, con, nperr, msg);
    return which == 0 ? con : which == 1 ? static_cast<double>(nperr) : NAN;
}

// EquationsSolver on the n by n system (n 1 to 20) a(r, c) = d + k r on the diagonal and
// o cos(1.7 r + 0.9 c + p) off it (1-based r and c), b(i) = sin(0.5 i + p): which 0 to n - 1
// picks the solution's element which + 1, which n picks nperr.
double ep_TarcogSolve(int which, int n, double d, double k, double o, double p)
{
    if (n < 1 || n > 20 || which < 0 || which > n) {
        return NAN;
    }
    Array2D<Real64> a(n, n, 0.0);
    Array1D<Real64> b(n, 0.0);
    for (int r = 1; r <= n; ++r) {
        for (int c = 1; c <= n; ++c) {
            a(r, c) = r == c ? d + k * r : o * std::cos(1.7 * r + 0.9 * c + p);
        }
        b(r) = std::sin(0.5 * r + p);
    }
    int nperr = 0;
    std::string msg;
    TARCOGCommon::EquationsSolver(st(), a, b, n, nperr, msg);
    return which == n ? static_cast<double>(nperr) : b(which + 1);
}
}
