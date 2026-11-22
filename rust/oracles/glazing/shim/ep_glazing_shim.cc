// Written by hand: these WindowManager routines take fixed-size arrays and return several results
// through references, which gen_plain.py cannot build. Plain extern "C" entry points over the real
// EnergyPlus code.
//
// One EnergyPlusData for the process (LUdecomposition's fatal error needs one), built when the
// library loads, with console output off. ShowFatalError (a singular matrix) throws, and the shim
// returns NaN for it.
#include <array>
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/EnergyPlus.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <EnergyPlus/WindowManager.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::Window;

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
} // namespace

extern "C" {
// TransAndReflAtPhi: which picks tfp (0), rfp (1) or rbp (2). A NaN SHGC or U-factor with the
// simple glazing model would reach the routine's assert(false), so the shim returns NaN for it.
double ep_GlassPhi(int which, double cs, double tf0, double rf0, double rb0, int simple, double shgc, double u)
{
    if (simple != 0 && (std::isnan(shgc) || std::isnan(u))) {
        return NAN;
    }
    Real64 tfp = 0.0, rfp = 0.0, rbp = 0.0;
    TransAndReflAtPhi(cs, tf0, rf0, rb0, tfp, rfp, rbp, simple != 0, shgc, u);
    return which == 0 ? tfp : which == 1 ? rfp : which == 2 ? rbp : NAN;
}

// As W5InitGlassParameters uses them: the glass's transmittance and front reflectance at the ten
// incidence angles (TransAndReflAtPhi, not the simple model), then W5LsqFit of the transmittance
// against cos(phi) (which 0 to 5: the coefficients), or DiffuseAverage of the transmittance (6) or
// of the front reflectance (7).
double ep_GlassFit(int which, double tf0, double rf0, double rb0)
{
    std::array<Real64, numPhis> t{}, r{};
    for (int i = 0; i < numPhis; ++i) {
        Real64 rb = 0.0;
        TransAndReflAtPhi(cosPhis[i], tf0, rf0, rb0, t[i], r[i], rb, false, 0.0, 0.0);
    }
    if (which >= 0 && which < maxPolyCoef) {
        std::array<Real64, maxPolyCoef> c{};
        W5LsqFit(cosPhis, t, c);
        return c[which];
    }
    return which == 6 ? DiffuseAverage(t) : which == 7 ? DiffuseAverage(r) : NAN;
}

// ViewFac for slats of width s, spacing h, slat angle phib and profile angle phis: F[i][j] for i
// and j from 0 to 5.
double ep_SlatViewFac(int i, int j, double s, double h, double phib, double phis)
{
    if (i < 0 || i > 5 || j < 0 || j > 5) {
        return NAN;
    }
    std::array<std::array<Real64, 6>, 6> F{};
    ViewFac(s, h, phib, phis, F);
    return F[i][j];
}

// InvertMatrix on the n by n matrix (n 1 to 10) a[r][c] = d + k r on the diagonal and
// o cos(1.7 r + 0.9 c + p) off it: y[which / n][which % n].
double ep_MatInverse(int which, int n, double d, double k, double o, double p)
{
    if (n < 1 || n > maxArraySize || which < 0 || which >= n * n) {
        return NAN;
    }
    std::array<std::array<Real64, maxArraySize>, maxArraySize> a{}, y{};
    for (int r = 0; r < n; ++r) {
        for (int c = 0; c < n; ++c) {
            a[r][c] = r == c ? d + k * r : o * std::cos(1.7 * r + 0.9 * c + p);
        }
    }
    std::array<int, maxArraySize> indx{};
    try {
        InvertMatrix(st(), a, y, indx, n);
    } catch (FatalError const &) {
        return NAN;
    }
    return y[which / n][which % n];
}

// Interpolate over the first npts (1 to 5) of the points (x0, y0) to (x4, y4).
double ep_Interp(double xin, int npts, double x0, double x1, double x2, double x3, double x4, double y0, double y1, double y2, double y3, double y4)
{
    if (npts < 1 || npts > 5) {
        return NAN;
    }
    std::array<Real64, 5> const x = {x0, x1, x2, x3, x4}, y = {y0, y1, y2, y3, y4};
    return Interpolate(x, y, npts, xin);
}
}
