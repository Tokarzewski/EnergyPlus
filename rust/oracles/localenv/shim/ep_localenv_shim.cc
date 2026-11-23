// Written by hand: the local environment averages of HeatBalanceSurfaceManager read each exterior
// surface's ground and surrounding surfaces properties and their schedules from the EnergyPlusData
// state, which gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// HeatBalanceSurfaceManager code.
//
// Each call sets up exterior surface 1, in a model with local environments when anylocal is 1. When
// gnddefined is 1 it has ground surfaces property 1: view factor sum vfsum and ngnd (0 to 4) ground
// surfaces, ground surface i with view factor gvf_i and, when ht_i or hr_i is 1, temperature and
// reflectance schedules at gt_i and gr_i. When hassrd is 1 it has surrounding surfaces property 1
// with nsrd (0 to 4) surrounding surfaces, surrounding surface i with view factor svf_i and a
// temperature schedule at st_i. The surface sees the surrounding surfaces with view factor vfsrd
// and the ground with view factor vfgir. The values these functions set start at tavg0 (ground
// surfaces temperature), ravg0 (ground surfaces reflectance) and srdt0 (surrounding surfaces
// temperature), and the flags at usegndtemp0 and usegndrefl0. Then op picks what runs:
// GetGroundSurfacesTemperatureAverage (0), GetGroundSurfacesReflectanceAverage (1),
// ReSetGroundSurfacesViewFactor for surface 1 (2), or GetSurroundingSurfacesTemperatureAverage
// (3). One EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 6; op outside 0 to 3; a flag outside 0 or
// 1; ngnd or nsrd outside 0 to 4.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/HeatBalanceSurfaceManager.hh>
#include <EnergyPlus/ScheduleManager.hh>

using namespace EnergyPlus;

namespace {
Sched::Schedule *gndTemp[4];
Sched::Schedule *gndRefl[4];
Sched::Schedule *srdTemp[4];

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 1;
        surf.Surface.allocate(1);
        surf.Surface(1).Class = DataSurfaces::SurfaceClass::Wall;
        surf.GroundSurfsProperty.allocate(1);
        surf.GroundSurfsProperty(1).GndSurfs.allocate(4);
        surf.SurroundingSurfsProperty.allocate(1);
        surf.SurroundingSurfsProperty(1).SurroundingSurfs.allocate(4);
        for (int i = 0; i < 4; ++i) {
            gndTemp[i] = Sched::AddScheduleConstant(*p, "GROUND TEMPERATURE " + std::to_string(i + 1));
            gndRefl[i] = Sched::AddScheduleConstant(*p, "GROUND REFLECTANCE " + std::to_string(i + 1));
            srdTemp[i] = Sched::AddScheduleConstant(*p, "SURROUNDING TEMPERATURE " + std::to_string(i + 1));
        }
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One value after the call for surface 1 (see the file comment): which picks the ground surfaces
// property's SurfsTempAvg (0), the surface's UseSurfPropertyGndSurfTemp (1, as 0 or 1), SurfsReflAvg
// (2), UseSurfPropertyGndSurfRefl (3), the surface's SrdSurfTemp (4), SurfsViewFactorSum (5) and the
// first ground surface's ViewFactor (6).
double ep_LocalEnvironment(int which,
                           int op,
                           int anylocal,
                           int gnddefined,
                           double vfsum,
                           int ngnd,
                           double gvf1,
                           int ht1,
                           double gt1,
                           int hr1,
                           double gr1,
                           double gvf2,
                           int ht2,
                           double gt2,
                           int hr2,
                           double gr2,
                           double gvf3,
                           int ht3,
                           double gt3,
                           int hr3,
                           double gr3,
                           double gvf4,
                           int ht4,
                           double gt4,
                           int hr4,
                           double gr4,
                           int hassrd,
                           int nsrd,
                           double svf1,
                           double st1,
                           double svf2,
                           double st2,
                           double svf3,
                           double st3,
                           double svf4,
                           double st4,
                           double vfsrd,
                           double vfgir,
                           double tavg0,
                           double ravg0,
                           double srdt0,
                           int usegndtemp0,
                           int usegndrefl0)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    for (int f : {anylocal, gnddefined, ht1, hr1, ht2, hr2, ht3, hr3, ht4, hr4, hassrd, usegndtemp0, usegndrefl0}) {
        if (!flag(f)) {
            return NAN;
        }
    }
    if (which < 0 || which > 6 || op < 0 || op > 3 || ngnd < 0 || ngnd > 4 || nsrd < 0 || nsrd > 4) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataGlobal->AnyLocalEnvironmentsInModel = anylocal == 1;
    auto &surf = *state.dataSurface;
    auto &s1 = surf.Surface(1);
    s1.IsSurfPropertyGndSurfacesDefined = gnddefined == 1;
    s1.SurfPropertyGndSurfIndex = 1;
    s1.UseSurfPropertyGndSurfTemp = usegndtemp0 == 1;
    s1.UseSurfPropertyGndSurfRefl = usegndrefl0 == 1;
    s1.SurfHasSurroundingSurfProperty = hassrd == 1;
    s1.SurfSurroundingSurfacesNum = 1;
    s1.ViewFactorSrdSurfs = vfsrd;
    s1.ViewFactorGroundIR = vfgir;
    s1.SrdSurfTemp = srdt0;
    auto &g = surf.GroundSurfsProperty(1);
    g.NumGndSurfs = ngnd;
    g.SurfsViewFactorSum = vfsum;
    g.SurfsTempAvg = tavg0;
    g.SurfsReflAvg = ravg0;
    double const gvf[4] = {gvf1, gvf2, gvf3, gvf4};
    int const ht[4] = {ht1, ht2, ht3, ht4};
    double const gt[4] = {gt1, gt2, gt3, gt4};
    int const hr[4] = {hr1, hr2, hr3, hr4};
    double const gr[4] = {gr1, gr2, gr3, gr4};
    for (int i = 0; i < 4; ++i) {
        auto &gs = g.GndSurfs(i + 1);
        gs.ViewFactor = gvf[i];
        gndTemp[i]->currentVal = gt[i];
        gndRefl[i]->currentVal = gr[i];
        gs.tempSched = ht[i] == 1 ? gndTemp[i] : nullptr;
        gs.reflSched = hr[i] == 1 ? gndRefl[i] : nullptr;
    }
    auto &sp = surf.SurroundingSurfsProperty(1);
    sp.TotSurroundingSurface = nsrd;
    double const svf[4] = {svf1, svf2, svf3, svf4};
    double const stv[4] = {st1, st2, st3, st4};
    for (int i = 0; i < 4; ++i) {
        sp.SurroundingSurfs(i + 1).ViewFactor = svf[i];
        srdTemp[i]->currentVal = stv[i];
        sp.SurroundingSurfs(i + 1).tempSched = srdTemp[i];
    }

    switch (op) {
    case 0:
        HeatBalanceSurfaceManager::GetGroundSurfacesTemperatureAverage(state);
        break;
    case 1:
        HeatBalanceSurfaceManager::GetGroundSurfacesReflectanceAverage(state);
        break;
    case 2:
        HeatBalanceSurfaceManager::ReSetGroundSurfacesViewFactor(state, 1);
        break;
    default:
        HeatBalanceSurfaceManager::GetSurroundingSurfacesTemperatureAverage(state);
        break;
    }

    switch (which) {
    case 0:
        return g.SurfsTempAvg;
    case 1:
        return s1.UseSurfPropertyGndSurfTemp ? 1.0 : 0.0;
    case 2:
        return g.SurfsReflAvg;
    case 3:
        return s1.UseSurfPropertyGndSurfRefl ? 1.0 : 0.0;
    case 4:
        return s1.SrdSurfTemp;
    case 5:
        return g.SurfsViewFactorSum;
    default:
        return g.GndSurfs(1).ViewFactor;
    }
}
}
