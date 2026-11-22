// Written by hand: the interior radiation exchange routines take area, angle and emissivity
// vectors, view factor matrices and the surfaces' classes, which gen_plain.py cannot build. Plain
// extern "C" entry points over the real EnergyPlus HeatBalanceIntRadExchange code, on a box zone.
//
// The zone (as EnergyPlus orders an enclosure's surfaces): the surfaces whose bit is set in
// `mask`, from bit 0: floor (L by W, tilt 180), roof (L by W, tilt 0), four walls (L by H facing
// rot, W by H facing rot + 90, L by H facing rot + 180, W by H facing rot + 270, tilt 90), and an
// internal mass of area `mass` (azimuth rot, tilt 90). Surface emissivities are e0 + de k for the
// k-th included surface (from 0). Indices i and j are 1-based in the included surfaces.
//
// One EnergyPlusData for the process, built when the library loads, with console output off; its
// surfaces and zone are set up per call. ShowFatalError (FixViewFactors' "should never happen"
// case) throws, and the shim returns NaN for it.
#include <cmath>
#include <ObjexxFCL/Array1D.hh>
#include <ObjexxFCL/Array2D.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/EnergyPlus.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/HeatBalanceIntRadExchange.hh>
#include <EnergyPlus/UtilityRoutines.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::HeatBalanceIntRadExchange;
using DataSurfaces::SurfaceClass;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataSurface->Surface.allocate(7);
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "BOX";
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

struct Box
{
    int n = 0;
    Array1D<Real64> area, azimuth, tilt, emiss;
    Array1D_int sptr;
    bool ok = false;

    Box(int mask, double L, double W, double H, double mass, double rot, double e0 = 0.9, double de = 0.0)
    {
        if (mask <= 0 || mask > 127) {
            return;
        }
        double const a[7] = {L * W, L * W, L * H, W * H, L * H, W * H, mass};
        double const az[7] = {rot, rot, rot, rot + 90, rot + 180, rot + 270, rot};
        double const tl[7] = {180, 0, 90, 90, 90, 90, 90};
        SurfaceClass const cls[7] = {SurfaceClass::Floor, SurfaceClass::Roof, SurfaceClass::Wall, SurfaceClass::Wall,
                                     SurfaceClass::Wall,  SurfaceClass::Wall, SurfaceClass::IntMass};
        for (int k = 0; k < 7; ++k) {
            n += (mask >> k) & 1;
        }
        area.allocate(n);
        azimuth.allocate(n);
        tilt.allocate(n);
        emiss.allocate(n);
        sptr.allocate(n);
        int i = 0;
        for (int k = 0; k < 7; ++k) {
            if (((mask >> k) & 1) == 0) {
                continue;
            }
            ++i;
            auto &s = st().dataSurface->Surface(i);
            s.Class = cls[k];
            s.Zone = 1;
            s.Name = "S" + std::to_string(k);
            area(i) = a[k];
            azimuth(i) = az[k];
            tilt(i) = tl[k];
            emiss(i) = e0 + de * (i - 1);
            sptr(i) = i;
        }
        st().dataHeatBalIntRadExchg->LargestSurf = 0;
        ok = true;
    }
};

bool in(int i, int n)
{
    return i >= 1 && i <= n;
}
} // namespace

extern "C" {
// The approximate (fixed 0) or fixed (fixed 1) view factor F(i, j). With i 0 and fixed 1, j picks
// FixViewFactors' OriginalCheckValue (1), FixedCheckValue (2), FinalCheckValue (3), NumIterations
// (4) or RowSum (5).
double ep_IRView(int fixed, int i, int j, int mask, double L, double W, double H, double mass, double rot)
{
    Box b(mask, L, W, H, mass, rot);
    if (!b.ok) {
        return NAN;
    }
    int const n = b.n;
    Array2D<Real64> F(n, n, 0.0);
    CalcApproximateViewFactors(st(), n, b.area, b.azimuth, b.tilt, F, b.sptr);
    if (fixed == 0) {
        return in(i, n) && in(j, n) ? F(i, j) : NAN;
    }
    std::string name = "BOX";
    std::vector<int> const spaces;
    Real64 original = 0.0, fixedCheck = 0.0, finalCheck = 0.0, rowSum = 0.0;
    int iterations = 0;
    try {
        FixViewFactors(st(), n, b.area, F, name, spaces, original, fixedCheck, finalCheck, iterations, rowSum, false);
    } catch (FatalError const &) {
        return NAN;
    }
    if (i == 0) {
        switch (j) {
        case 1:
            return original;
        case 2:
            return fixedCheck;
        case 3:
            return finalCheck;
        case 4:
            return iterations;
        case 5:
            return rowSum;
        default:
            return NAN;
        }
    }
    return in(i, n) && in(j, n) ? F(i, j) : NAN;
}

// ScriptF(i, j) (stored transposed, as CalcScriptF stores it) from the fixed view factors.
double ep_IRScriptF(int i, int j, int mask, double L, double W, double H, double mass, double rot, double e0, double de)
{
    Box b(mask, L, W, H, mass, rot, e0, de);
    if (!b.ok || !in(i, b.n) || !in(j, b.n)) {
        return NAN;
    }
    int const n = b.n;
    Array2D<Real64> F(n, n, 0.0);
    CalcApproximateViewFactors(st(), n, b.area, b.azimuth, b.tilt, F, b.sptr);
    std::string name = "BOX";
    std::vector<int> const spaces;
    Real64 original = 0.0, fixedCheck = 0.0, finalCheck = 0.0, rowSum = 0.0;
    int iterations = 0;
    try {
        FixViewFactors(st(), n, b.area, F, name, spaces, original, fixedCheck, finalCheck, iterations, rowSum, false);
    } catch (FatalError const &) {
        return NAN;
    }
    Array2D<Real64> scriptF(n, n, 0.0);
    CalcScriptF(st(), n, b.area, F, b.emiss, scriptF);
    return scriptF(i, j);
}

// CalcFMRT's FMRT(i) (fp 0), or CalcFp's Fp(i) from it (fp 1).
double ep_IRFmrt(int fp, int i, int mask, double L, double W, double H, double mass, double e0, double de)
{
    Box b(mask, L, W, H, mass, 0.0, e0, de);
    if (!b.ok || !in(i, b.n)) {
        return NAN;
    }
    Array1D<Real64> fmrt(b.n, 0.0);
    CalcFMRT(st(), b.n, b.area, fmrt);
    if (fp == 0) {
        return fmrt(i);
    }
    Array1D<Real64> fps(b.n, 0.0);
    CalcFp(b.n, b.emiss, fmrt, fps);
    return fps(i);
}
}
