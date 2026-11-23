// Written by hand: UpdateThermalHistories reads and shifts one surface's CTF histories in the
// EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry point over the
// real EnergyPlus HeatBalanceSurfaceManager code.
//
// Each call sets up zone 1 with space 1 holding surface 1 (heat transfer algorithm algo: 0 CTF,
// 1 EMPD, 2 CondFD, which the update skips), area area, construction 1 with n CTF terms, n
// histories per construction time step numhist, construction time step ctfts and the first CTF
// terms given (cross0 ... tusersrc0); the construction has a source or sink when src is 1. The
// model has m history terms to shift (MaxCTFTerms), SimpleCTFOnly simple, construction overrides
// overrides and any internal heat source anysrc. The zone time step is tsz hours and the surface
// is curnum steps into its master history.
//
// Every history value (the eight temperature and flux histories and their masters, terms 1 to 8;
// the six source, user location and source flux histories and their masters; TempExt1, TempInt1,
// QExt1, QInt1, Tsrc1, Tuser1 and Qsrc1; and the six face conduction and source temperature
// outputs) is drawn from splitmix64 seeded with seed, in that order, as
// double(z >> 11) * 2^-53 * 200 - 100. Then the outside face temperature (SurfOutsideTempHist
// term 1) is tout, the source flux (SurfQsrcHist term 1) qsrc, the inside face temperature tin,
// the CTF constant parts ctfin and ctfout, and the source and user location constant parts
// tsrcconst and tuserconst; SumTime starts at 0. Then the real
// HeatBalanceSurfaceManager::UpdateThermalHistories runs. One EnergyPlusData for the process,
// built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 127; a flag outside 0 or 1; algo outside
// 0 to 2; n outside 0 to 7; m outside 1 to 7 or below n; numhist below 1; curnum outside 0 to
// numhist - 1; and a source or sink with anysrc 0, which EnergyPlus never sets up.
#include <cmath>
#include <cstdint>
#include <EnergyPlus/Construction.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalSurface.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/HeatBalanceSurfaceManager.hh>

using namespace EnergyPlus;

namespace {
constexpr int Terms = 8;

EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).spaceIndexes.push_back(1);
        p->dataHeatBal->space.allocate(1);
        p->dataHeatBal->space(1).OpaqOrIntMassSurfaceFirst = 1;
        p->dataHeatBal->space(1).OpaqOrIntMassSurfaceLast = 1;
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 1;
        surf.Surface.allocate(1);
        surf.Surface(1).Class = DataSurfaces::SurfaceClass::Wall;
        surf.Surface(1).HeatTransSurf = true;
        surf.Surface(1).Construction = 1;
        p->dataConstruction->Construct.allocate(1);
        auto &hbs = *p->dataHeatBalSurf;
        for (auto *h : {&hbs.SurfInsideTempHist, &hbs.SurfOutsideTempHist, &hbs.SurfInsideFluxHist, &hbs.SurfOutsideFluxHist,
                        &hbs.SurfInsideTempHistMaster, &hbs.SurfOutsideTempHistMaster, &hbs.SurfInsideFluxHistMaster,
                        &hbs.SurfOutsideFluxHistMaster}) {
            h->dimension(Terms);
            for (auto &a : *h) {
                a.dimension(1, 0.0);
            }
        }
        for (auto *a : {&hbs.SurfTsrcHist, &hbs.SurfQsrcHist, &hbs.SurfTuserHist, &hbs.SurfTsrcHistM, &hbs.SurfQsrcHistM, &hbs.SurfTuserHistM}) {
            a->dimension(1, Terms, 0.0);
        }
        for (auto *a : {&hbs.SurfOpaqInsFaceCond, &hbs.SurfOpaqInsFaceCondFlux, &hbs.SurfOpaqOutFaceCond, &hbs.SurfOpaqOutFaceCondFlux,
                        &hbs.SurfTempSource, &hbs.SurfTempUserLoc, &hbs.SurfTempIn, &hbs.SurfCTFConstInPart, &hbs.SurfCTFConstOutPart}) {
            a->dimension(1, 0.0);
        }
        hbs.SurfCurrNumHist.dimension(1, 0);
        p->dataHeatBalFanSys->CTFTsrcConstPart.dimension(1, 0.0);
        p->dataHeatBalFanSys->CTFTuserConstPart.dimension(1, 0.0);
        auto &mgr = *p->dataHeatBalSurfMgr;
        for (auto *a : {&mgr.QExt1, &mgr.QInt1, &mgr.TempInt1, &mgr.TempExt1, &mgr.SumTime, &mgr.Qsrc1, &mgr.Tsrc1, &mgr.Tuser1}) {
            a->dimension(1, 0.0);
        }
        mgr.UpdateThermalHistoriesFirstTimeFlag = false;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

struct SplitMix
{
    std::uint64_t s;
    double next()
    {
        s += 0x9E3779B97F4A7C15ULL;
        std::uint64_t z = s;
        z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
        z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
        z ^= z >> 31;
        return double(z >> 11) * 0x1.0p-53 * 200.0 - 100.0;
    }
};
} // namespace

extern "C" {
// One value after UpdateThermalHistories (see the file comment). which = 8 * array + term - 1:
// arrays 0 to 7 are SurfInsideTempHist, SurfOutsideTempHist, SurfInsideFluxHist,
// SurfOutsideFluxHist and their masters in that order; 8 to 13 SurfTsrcHist, SurfQsrcHist,
// SurfTuserHist, SurfTsrcHistM, SurfQsrcHistM and SurfTuserHistM; 14 is SurfOpaqInsFaceCond,
// SurfOpaqInsFaceCondFlux, SurfOpaqOutFaceCond, SurfOpaqOutFaceCondFlux, SurfTempSource,
// SurfTempUserLoc, SurfCurrNumHist and SumTime by term; 15 is TempExt1, TempInt1, QExt1, QInt1,
// Tsrc1, Tuser1 and Qsrc1 by term (term 8 is 0).
double ep_ThermalHist(int which,
                      int seed,
                      int n,
                      int m,
                      int simple,
                      int overrides,
                      int anysrc,
                      int src,
                      int algo,
                      int curnum,
                      int numhist,
                      double tsz,
                      double ctfts,
                      double area,
                      double tout,
                      double tin,
                      double ctfin,
                      double ctfout,
                      double qsrc,
                      double tsrcconst,
                      double tuserconst,
                      double cross0,
                      double inside0,
                      double outside0,
                      double srcin0,
                      double srcout0,
                      double tsrcout0,
                      double tsrcin0,
                      double tsrcq0,
                      double tuserout0,
                      double tuserin0,
                      double tusersrc0)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 127 || !flag(simple) || !flag(overrides) || !flag(anysrc) || !flag(src) || algo < 0 || algo > 2 || n < 0 ||
        n > 7 || m < 1 || m > 7 || m < n || numhist < 1 || curnum < 0 || curnum >= numhist || (src == 1 && anysrc == 0)) {
        return NAN;
    }
    EnergyPlusData &state = st();
    auto &surface = state.dataSurface->Surface(1);
    static constexpr DataSurfaces::HeatTransferModel algos[] = {
        DataSurfaces::HeatTransferModel::CTF, DataSurfaces::HeatTransferModel::EMPD, DataSurfaces::HeatTransferModel::CondFD};
    surface.HeatTransferAlgorithm = algos[algo];
    surface.Area = area;
    auto &c = state.dataConstruction->Construct(1);
    c.NumCTFTerms = n;
    c.NumHistories = numhist;
    c.CTFTimeStep = ctfts;
    c.SourceSinkPresent = src == 1;
    c.CTFCross[0] = cross0;
    c.CTFInside[0] = inside0;
    c.CTFOutside[0] = outside0;
    c.CTFSourceIn[0] = srcin0;
    c.CTFSourceOut[0] = srcout0;
    c.CTFTSourceOut[0] = tsrcout0;
    c.CTFTSourceIn[0] = tsrcin0;
    c.CTFTSourceQ[0] = tsrcq0;
    c.CTFTUserOut[0] = tuserout0;
    c.CTFTUserIn[0] = tuserin0;
    c.CTFTUserSource[0] = tusersrc0;
    state.dataHeatBal->MaxCTFTerms = m;
    state.dataHeatBal->SimpleCTFOnly = simple == 1;
    state.dataGlobal->AnyConstrOverridesInModel = overrides == 1;
    state.dataHeatBal->AnyInternalHeatSourceInInput = anysrc == 1;
    state.dataGlobal->TimeStepZone = tsz;

    auto &hbs = *state.dataHeatBalSurf;
    auto &mgr = *state.dataHeatBalSurfMgr;
    SplitMix g{std::uint64_t(std::int64_t(seed))};
    Array1D<Array1D<Real64>> *hists[] = {&hbs.SurfInsideTempHist,
                                         &hbs.SurfOutsideTempHist,
                                         &hbs.SurfInsideFluxHist,
                                         &hbs.SurfOutsideFluxHist,
                                         &hbs.SurfInsideTempHistMaster,
                                         &hbs.SurfOutsideTempHistMaster,
                                         &hbs.SurfInsideFluxHistMaster,
                                         &hbs.SurfOutsideFluxHistMaster};
    for (auto *h : hists) {
        for (int k = 1; k <= Terms; ++k) {
            (*h)(k)(1) = g.next();
        }
    }
    Array2D<Real64> *srcs[] = {&hbs.SurfTsrcHist, &hbs.SurfQsrcHist, &hbs.SurfTuserHist, &hbs.SurfTsrcHistM, &hbs.SurfQsrcHistM, &hbs.SurfTuserHistM};
    for (auto *a : srcs) {
        for (int k = 1; k <= Terms; ++k) {
            (*a)(1, k) = g.next();
        }
    }
    Array1D<Real64> *ones[] = {&mgr.TempExt1, &mgr.TempInt1, &mgr.QExt1, &mgr.QInt1, &mgr.Tsrc1, &mgr.Tuser1, &mgr.Qsrc1};
    for (auto *a : ones) {
        (*a)(1) = g.next();
    }
    Array1D<Real64> *outs[] = {&hbs.SurfOpaqInsFaceCond,
                               &hbs.SurfOpaqInsFaceCondFlux,
                               &hbs.SurfOpaqOutFaceCond,
                               &hbs.SurfOpaqOutFaceCondFlux,
                               &hbs.SurfTempSource,
                               &hbs.SurfTempUserLoc};
    for (auto *a : outs) {
        (*a)(1) = g.next();
    }
    hbs.SurfOutsideTempHist(1)(1) = tout;
    hbs.SurfQsrcHist(1, 1) = qsrc;
    hbs.SurfTempIn(1) = tin;
    hbs.SurfCTFConstInPart(1) = ctfin;
    hbs.SurfCTFConstOutPart(1) = ctfout;
    state.dataHeatBalFanSys->CTFTsrcConstPart(1) = tsrcconst;
    state.dataHeatBalFanSys->CTFTuserConstPart(1) = tuserconst;
    hbs.SurfCurrNumHist(1) = curnum;
    mgr.SumTime(1) = 0.0;

    HeatBalanceSurfaceManager::UpdateThermalHistories(state);

    int const a = which / 8;
    int const k = which % 8 + 1;
    if (a < 8) {
        return (*hists[a])(k)(1);
    }
    if (a < 14) {
        return (*srcs[a - 8])(1, k);
    }
    if (a == 14) {
        if (k <= 6) {
            return (*outs[k - 1])(1);
        }
        return k == 7 ? double(hbs.SurfCurrNumHist(1)) : mgr.SumTime(1);
    }
    return k <= 7 ? (*ones[k - 1])(1) : 0.0;
}
}
