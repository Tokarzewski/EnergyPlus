// Written by hand: CalcOutsideSurfTemp reads one surface's heat balance terms from the
// EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry point over the
// real EnergyPlus HeatBalanceSurfaceManager code.
//
// Each call sets up surface 1 (a heat transfer surface, not a tubular daylighting dome) in
// space 1 with construction 1, whose first CTF terms are cross0, inside0, outside0, srcout0
// and srcin0 and which has an embedded source or sink when src is 1, then the surface's heat
// balance terms (the arguments named after the state arrays they fill), and runs
// HeatBalanceSurfaceManager::CalcOutsideSurfTemp for movable insulation coefficient hmov and
// exterior temperature text. With hassrd 1 the surface has a surrounding surfaces list with no
// sky or ground temperature schedules, at temperature srdtemp; with usegnd 1 it takes its
// ground temperature, gndtemp, from a ground surfaces list; with osc 1 it has an other side
// conditions model of radiation temperature osctrad and coefficient oschrad. One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: src, hassrd, usegnd or osc outside 0 or 1; and movable
// insulation (hmov above 0) on a construction with a source or sink, which the C++ reports as a
// severe error through the error file this oracle does not set up.
#include <cmath>
#include <EnergyPlus/Construction.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalSurface.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/HeatBalanceSurfaceManager.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 1;
        surf.Surface.allocate(1);
        surf.Surface(1).Class = DataSurfaces::SurfaceClass::Wall;
        surf.Surface(1).HeatTransSurf = true;
        surf.OSCM.allocate(1);
        surf.GroundSurfsProperty.allocate(1);
        surf.SurroundingSurfsProperty.allocate(1);
        surf.SurroundingSurfsProperty(1).skyTempSched = nullptr;
        surf.SurroundingSurfsProperty(1).groundTempSched = nullptr;
        p->dataConstruction->Construct.allocate(1);
        auto &hbs = *p->dataHeatBalSurf;
        for (auto *a : {&hbs.SurfHConvInt, &hbs.SurfHConvExt, &hbs.SurfHAirExt, &hbs.SurfHSkyExt, &hbs.SurfHGrdExt, &hbs.SurfHSrdSurfExt,
                        &hbs.SurfCTFConstOutPart, &hbs.SurfCTFConstInPart, &hbs.SurfOpaqQRadSWOutAbs, &hbs.SurfOpaqQRadSWInAbs, &hbs.SurfTempIn,
                        &hbs.SurfQAdditionalHeatSourceOutside, &hbs.SurfQdotRadNetLWInPerArea, &hbs.SurfQRadSWOutMvIns,
                        &hbs.SurfQdotRadOutRepPerArea}) {
            a->dimension(1, 0.0);
        }
        hbs.SurfOutsideTempHist.dimension(4);
        for (auto &h : hbs.SurfOutsideTempHist) {
            h.dimension(1, 0.0);
        }
        hbs.SurfQsrcHist.dimension(1, 4, 0.0);
        p->dataHeatBal->SurfQdotRadIntGainsInPerArea.dimension(1, 0.0);
        p->dataZoneTempPredictorCorrector->spaceHeatBalance.allocate(1);
        auto &fs = *p->dataHeatBalFanSys;
        fs.RadSysToHBConstCoef.dimension(1, 0.0);
        fs.RadSysToHBTinCoef.dimension(1, 0.0);
        fs.RadSysToHBQsrcCoef.dimension(1, 0.0);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// Outside surface temperature of surface 1 and what CalcOutsideSurfTemp sets with it (see the
// file comment). which picks TH11, the new outside face temperature (0), SurfQdotRadOutRepPerArea
// (1), RadSysToHBConstCoef (2), RadSysToHBTinCoef (3), RadSysToHBQsrcCoef (4) and the error flag
// (5).
double ep_OutsideSurfTemp(int which,
                          double cross0,
                          double inside0,
                          double outside0,
                          double srcout0,
                          double srcin0,
                          int src,
                          double hconvint,
                          double hconvext,
                          double hairext,
                          double hskyext,
                          double hgrdext,
                          double hsrdext,
                          double ctfout,
                          double ctfin,
                          double qswout,
                          double qswin,
                          double qsrc,
                          double tempin,
                          double qaddout,
                          double qintgains,
                          double qnetlwin,
                          double mat,
                          double qswmvins,
                          double hmov,
                          double text,
                          double skytemp,
                          double odb,
                          int hassrd,
                          double srdtemp,
                          int usegnd,
                          double gndtemp,
                          int osc,
                          double osctrad,
                          double oschrad)
{
    if (src < 0 || src > 1 || hassrd < 0 || hassrd > 1 || usegnd < 0 || usegnd > 1 || osc < 0 || osc > 1 || (src == 1 && hmov > 0.0)) {
        return NAN;
    }
    EnergyPlusData &state = st();
    auto &surface = state.dataSurface->Surface(1);
    surface.SurfHasSurroundingSurfProperty = hassrd == 1;
    surface.SurfSurroundingSurfacesNum = hassrd;
    surface.SrdSurfTemp = srdtemp;
    surface.UseSurfPropertyGndSurfTemp = usegnd == 1;
    surface.SurfPropertyGndSurfIndex = usegnd;
    state.dataSurface->GroundSurfsProperty(1).SurfsTempAvg = gndtemp;
    surface.OSCMPtr = osc;
    state.dataSurface->OSCM(1).TRad = osctrad;
    state.dataSurface->OSCM(1).HRad = oschrad;
    auto &c = state.dataConstruction->Construct(1);
    c.CTFCross[0] = cross0;
    c.CTFInside[0] = inside0;
    c.CTFOutside[0] = outside0;
    c.CTFSourceOut[0] = srcout0;
    c.CTFSourceIn[0] = srcin0;
    c.SourceSinkPresent = src == 1;
    auto &hbs = *state.dataHeatBalSurf;
    hbs.SurfHConvInt(1) = hconvint;
    hbs.SurfHConvExt(1) = hconvext;
    hbs.SurfHAirExt(1) = hairext;
    hbs.SurfHSkyExt(1) = hskyext;
    hbs.SurfHGrdExt(1) = hgrdext;
    hbs.SurfHSrdSurfExt(1) = hsrdext;
    hbs.SurfCTFConstOutPart(1) = ctfout;
    hbs.SurfCTFConstInPart(1) = ctfin;
    hbs.SurfOpaqQRadSWOutAbs(1) = qswout;
    hbs.SurfOpaqQRadSWInAbs(1) = qswin;
    hbs.SurfQsrcHist(1, 1) = qsrc;
    hbs.SurfTempIn(1) = tempin;
    hbs.SurfQAdditionalHeatSourceOutside(1) = qaddout;
    hbs.SurfQdotRadNetLWInPerArea(1) = qnetlwin;
    hbs.SurfQRadSWOutMvIns(1) = qswmvins;
    hbs.SurfQdotRadOutRepPerArea(1) = 0.0;
    hbs.SurfOutsideTempHist(1)(1) = 0.0;
    state.dataHeatBal->SurfQdotRadIntGainsInPerArea(1) = qintgains;
    state.dataZoneTempPredictorCorrector->spaceHeatBalance(1).MAT = mat;
    state.dataEnvrn->SkyTemp = skytemp;
    state.dataEnvrn->OutDryBulbTemp = odb;
    auto &fs = *state.dataHeatBalFanSys;
    fs.RadSysToHBConstCoef(1) = 0.0;
    fs.RadSysToHBTinCoef(1) = 0.0;
    fs.RadSysToHBQsrcCoef(1) = 0.0;
    bool err = false;

    HeatBalanceSurfaceManager::CalcOutsideSurfTemp(state, 1, 1, 1, hmov, text, err);

    switch (which) {
    case 0:
        return hbs.SurfOutsideTempHist(1)(1);
    case 1:
        return hbs.SurfQdotRadOutRepPerArea(1);
    case 2:
        return fs.RadSysToHBConstCoef(1);
    case 3:
        return fs.RadSysToHBTinCoef(1);
    case 4:
        return fs.RadSysToHBQsrcCoef(1);
    case 5:
        return err ? 1.0 : 0.0;
    default:
        return NAN;
    }
}
}
