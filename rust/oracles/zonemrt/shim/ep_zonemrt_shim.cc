// Written by hand: CalculateZoneMRT reads every surface's area, inside thermal absorptance and
// inside face temperature, and the zone's spaces and radiant enclosures, from the EnergyPlusData
// state, which gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// HeatBalanceSurfaceManager code.
//
// Each call sets up zone 1 (mean air temperature mat) with two spaces, each its own radiant
// enclosure: space 1 (mean air temperature mat1, volume vol1) holds heat transfer surfaces 1 to
// n1 and space 2 (mat2, vol2) surfaces n1 + 1 to n1 + n2. Surface k has area ak, inside thermal
// absorptance ek and inside face temperature tk. Both enclosures' running sums start at 0, and
// the model has no user zone MRT objects. Then the real
// HeatBalanceSurfaceManager::CalculateZoneMRT runs for every zone. One EnergyPlusData for the
// process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 10; n1 or n2 below 0; and more than 6
// surfaces.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalSurface.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/DataViewFactorInformation.hh>
#include <EnergyPlus/HeatBalanceSurfaceManager.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
constexpr int MaxSurf = 6;

EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).spaceIndexes.push_back(1);
        p->dataHeatBal->Zone(1).spaceIndexes.push_back(2);
        p->dataHeatBal->space.allocate(2);
        p->dataViewFactor->EnclRadInfo.allocate(2);
        for (int k = 1; k <= 2; ++k) {
            p->dataHeatBal->space(k).radiantEnclosureNum = k;
            p->dataViewFactor->EnclRadInfo(k).spaceNums = {k};
        }
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        p->dataZoneTempPredictorCorrector->spaceHeatBalance.allocate(2);
        p->dataSurface->Surface.allocate(MaxSurf);
        for (auto &s1 : p->dataSurface->Surface) {
            s1.HeatTransSurf = true;
            s1.Zone = 1;
        }
        p->dataHeatBalSurf->SurfAbsThermalInt.dimension(MaxSurf, 0.0);
        p->dataHeatBalSurf->SurfTempInTmp.dimension(MaxSurf, 0.0);
        auto &mgr = *p->dataHeatBalSurfMgr;
        mgr.SurfaceAE.allocate(MaxSurf);
        mgr.ZoneAESum.allocate(1);
        // The first call would read the user zone MRT objects from the input file.
        mgr.CalculateZoneMRTfirstTime = false;
        p->dataHeatBal->totZoneMRT = 0;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of CalculateZoneMRT (see the file comment): which picks the zone MRT (0), its
// standard MRT (1), enclosure 1 and 2 MRT (2, 3), space 1 and 2 MRT (4, 5), enclosure 1 sumAE
// and sumAET (6, 7), enclosure 2 sumAE and sumAET (8, 9) and the zone's area-emissivity sum (10).
double ep_ZoneMRT(int which,
                  int n1,
                  int n2,
                  double a1,
                  double a2,
                  double a3,
                  double a4,
                  double a5,
                  double a6,
                  double e1,
                  double e2,
                  double e3,
                  double e4,
                  double e5,
                  double e6,
                  double t1,
                  double t2,
                  double t3,
                  double t4,
                  double t5,
                  double t6,
                  double mat,
                  double mat1,
                  double mat2,
                  double vol1,
                  double vol2)
{
    // Each count is checked before the sum so that n1 + n2 cannot overflow.
    if (which < 0 || which > 10 || n1 < 0 || n2 < 0 || n1 > MaxSurf || n2 > MaxSurf || n1 + n2 > MaxSurf) {
        return NAN;
    }
    EnergyPlusData &state = st();
    double const a[MaxSurf] = {a1, a2, a3, a4, a5, a6};
    double const e[MaxSurf] = {e1, e2, e3, e4, e5, e6};
    double const t[MaxSurf] = {t1, t2, t3, t4, t5, t6};
    int const n = n1 + n2;
    state.dataSurface->TotSurfaces = n;
    auto &encl = state.dataViewFactor->EnclRadInfo;
    encl(1).SurfacePtr.dimension(n1, 0);
    encl(2).SurfacePtr.dimension(n2, 0);
    for (int k = 1; k <= n; ++k) {
        auto &s1 = state.dataSurface->Surface(k);
        s1.Area = a[k - 1];
        s1.RadEnclIndex = k <= n1 ? 1 : 2;
        state.dataHeatBalSurf->SurfAbsThermalInt(k) = e[k - 1];
        state.dataHeatBalSurf->SurfTempInTmp(k) = t[k - 1];
        if (k <= n1) {
            encl(1).SurfacePtr(k) = k;
        } else {
            encl(2).SurfacePtr(k - n1) = k;
        }
    }
    auto &sp = state.dataHeatBal->space;
    sp(1).HTSurfaceFirst = 1;
    sp(1).HTSurfaceLast = n1;
    sp(2).HTSurfaceFirst = n1 + 1;
    sp(2).HTSurfaceLast = n;
    sp(1).Volume = vol1;
    sp(2).Volume = vol2;
    for (auto &x : encl) {
        x.sumAET = 0.0;
        x.MRT = 0.0;
    }
    auto &ztpc = *state.dataZoneTempPredictorCorrector;
    ztpc.zoneHeatBalance(1).MAT = mat;
    ztpc.spaceHeatBalance(1).MAT = mat1;
    ztpc.spaceHeatBalance(2).MAT = mat2;

    HeatBalanceSurfaceManager::CalculateZoneMRT(state);

    switch (which) {
    case 0:
        return ztpc.zoneHeatBalance(1).MRT;
    case 1:
        return ztpc.zoneHeatBalance(1).stdMRT;
    case 2:
    case 3:
        return encl(which - 1).MRT;
    case 4:
    case 5:
        return ztpc.spaceHeatBalance(which - 3).MRT;
    case 6:
        return encl(1).sumAE;
    case 7:
        return encl(1).sumAET;
    case 8:
        return encl(2).sumAE;
    case 9:
        return encl(2).sumAET;
    default:
        return state.dataHeatBalSurfMgr->ZoneAESum(1);
    }
}
}
