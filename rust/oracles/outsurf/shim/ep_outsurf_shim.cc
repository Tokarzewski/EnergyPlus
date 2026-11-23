// Written by hand: HeatBalanceSurfaceManager::CalcOutsideSurfTemp reads the surface's
// construction, coefficients, fluxes and boundary conditions from the EnergyPlusData state, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// HeatBalanceSurfaceManager code.
//
// Each call sets up exterior wall 1 in space 1. Its construction has first CTF terms ctfcross,
// ctfinside, ctfoutside, ctfsourceout and ctfsourcein, with an embedded source or sink when
// sourcesink is 1. The wall has inside and outside convection coefficients hconvint and hconvext,
// outside radiation coefficients to the air, sky, ground and surrounding surfaces (hairext,
// hskyext, hgrdext, hsrdsurfext), the outdoor boundary temperature tempext, sky temperature
// skytemp and outdoor dry bulb outdrybulb. When hassrd is 1 it has a surrounding surfaces property
// with surrounding surface temperature srdsurftemp, and sky and ground temperature schedules at
// srdskyval and srdgndval when srdskyon and srdgndon are 1. When usegndsurf is 1 its ground
// surfaces have average temperature gndsurftavg. When oscm is 1 it has an other side conditions
// model with radiation temperature oscmtrad and coefficient oscmhrad. Its fluxes and terms are
// the additional outside heat source qaddout, the CTF constant parts ctfconstout and ctfconstin,
// the outside and inside absorbed short wave qradswoutabs and qradswinabs, the inside temperature
// tempin, the source history qsrchist, the inside radiant internal gains and net long wave
// qdotradintgainsin and qdotradnetlwin, the space mean air temperature mat, and the short wave
// absorbed on movable insulation qradswoutmvins. Then the real CalcOutsideSurfTemp runs for the
// wall with movable insulation coefficient hmovinsul. One EnergyPlusData for the process, built
// when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 4; a flag outside 0 or 1; and exterior
// movable insulation on a construction with a source or sink, which EnergyPlus reports as an error
// (ErrorFlag) before stopping.
#include <cmath>
#include <EnergyPlus/Construction.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalSurface.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/HeatBalanceSurfaceManager.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
Sched::Schedule *skySched = nullptr;
Sched::Schedule *gndSched = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 1;
        surf.Surface.allocate(1);
        surf.Surface(1).Class = DataSurfaces::SurfaceClass::Wall;
        surf.Surface(1).spaceNum = 1;
        surf.Surface(1).Construction = 1;
        surf.SurroundingSurfsProperty.allocate(1);
        surf.GroundSurfsProperty.allocate(1);
        surf.OSCM.allocate(1);
        skySched = Sched::AddScheduleConstant(*p, "SKY TEMPERATURE");
        gndSched = Sched::AddScheduleConstant(*p, "GROUND TEMPERATURE");
        p->dataConstruction->Construct.allocate(1);
        auto &hbs = *p->dataHeatBalSurf;
        for (auto *a : {&hbs.SurfHConvInt, &hbs.SurfHConvExt, &hbs.SurfHAirExt, &hbs.SurfHSkyExt, &hbs.SurfHGrdExt, &hbs.SurfHSrdSurfExt,
                        &hbs.SurfQAdditionalHeatSourceOutside, &hbs.SurfCTFConstOutPart, &hbs.SurfCTFConstInPart, &hbs.SurfOpaqQRadSWOutAbs,
                        &hbs.SurfOpaqQRadSWInAbs, &hbs.SurfTempIn, &hbs.SurfQdotRadNetLWInPerArea, &hbs.SurfQRadSWOutMvIns,
                        &hbs.SurfQdotRadOutRepPerArea}) {
            a->dimension(1, 0.0);
        }
        hbs.SurfOutsideTempHist.allocate(1);
        hbs.SurfOutsideTempHist(1).dimension(1, 0.0);
        hbs.SurfQsrcHist.dimension(1, 2, 0.0);
        p->dataHeatBal->SurfQdotRadIntGainsInPerArea.dimension(1, 0.0);
        p->dataHeatBalFanSys->RadSysToHBConstCoef.dimension(1, 0.0);
        p->dataHeatBalFanSys->RadSysToHBTinCoef.dimension(1, 0.0);
        p->dataHeatBalFanSys->RadSysToHBQsrcCoef.dimension(1, 0.0);
        p->dataZoneTempPredictorCorrector->spaceHeatBalance.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of CalcOutsideSurfTemp for wall 1 (see the file comment): which picks the outside
// face temperature SurfOutsideTempHist(1) (0), SurfQdotRadOutRepPerArea (1), and the radiant
// system coefficients RadSysToHBConstCoef (2), RadSysToHBTinCoef (3) and RadSysToHBQsrcCoef (4),
// all starting at 0.
double ep_OutsideSurfTemp(int which,
                          double ctfcross,
                          double ctfinside,
                          double ctfoutside,
                          double ctfsourceout,
                          double ctfsourcein,
                          int sourcesink,
                          double hconvint,
                          double hmovinsul,
                          double tempext,
                          double skytemp,
                          double outdrybulb,
                          int hassrd,
                          int srdskyon,
                          double srdskyval,
                          int srdgndon,
                          double srdgndval,
                          double srdsurftemp,
                          int usegndsurf,
                          double gndsurftavg,
                          int oscm,
                          double oscmtrad,
                          double oscmhrad,
                          double hconvext,
                          double hairext,
                          double hskyext,
                          double hgrdext,
                          double hsrdsurfext,
                          double qaddout,
                          double ctfconstout,
                          double qradswoutabs,
                          double tempin,
                          double qsrchist,
                          double ctfconstin,
                          double qradswinabs,
                          double qdotradintgainsin,
                          double mat,
                          double qdotradnetlwin,
                          double qradswoutmvins)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 4 || !flag(sourcesink) || !flag(hassrd) || !flag(srdskyon) || !flag(srdgndon) || !flag(usegndsurf) || !flag(oscm) ||
        (sourcesink == 1 && hmovinsul > 0.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &c = state.dataConstruction->Construct(1);
    c.CTFCross[0] = ctfcross;
    c.CTFInside[0] = ctfinside;
    c.CTFOutside[0] = ctfoutside;
    c.CTFSourceOut[0] = ctfsourceout;
    c.CTFSourceIn[0] = ctfsourcein;
    c.SourceSinkPresent = sourcesink == 1;
    auto &surf = *state.dataSurface;
    auto &s1 = surf.Surface(1);
    s1.SurfHasSurroundingSurfProperty = hassrd == 1;
    s1.SurfSurroundingSurfacesNum = hassrd == 1 ? 1 : 0;
    s1.SrdSurfTemp = srdsurftemp;
    s1.UseSurfPropertyGndSurfTemp = usegndsurf == 1;
    s1.SurfPropertyGndSurfIndex = usegndsurf == 1 ? 1 : 0;
    s1.OSCMPtr = oscm == 1 ? 1 : 0;
    skySched->currentVal = srdskyval;
    gndSched->currentVal = srdgndval;
    surf.SurroundingSurfsProperty(1).skyTempSched = srdskyon == 1 ? skySched : nullptr;
    surf.SurroundingSurfsProperty(1).groundTempSched = srdgndon == 1 ? gndSched : nullptr;
    surf.GroundSurfsProperty(1).SurfsTempAvg = gndsurftavg;
    surf.OSCM(1).TRad = oscmtrad;
    surf.OSCM(1).HRad = oscmhrad;
    state.dataEnvrn->SkyTemp = skytemp;
    state.dataEnvrn->OutDryBulbTemp = outdrybulb;
    auto &hbs = *state.dataHeatBalSurf;
    hbs.SurfHConvInt(1) = hconvint;
    hbs.SurfHConvExt(1) = hconvext;
    hbs.SurfHAirExt(1) = hairext;
    hbs.SurfHSkyExt(1) = hskyext;
    hbs.SurfHGrdExt(1) = hgrdext;
    hbs.SurfHSrdSurfExt(1) = hsrdsurfext;
    hbs.SurfQAdditionalHeatSourceOutside(1) = qaddout;
    hbs.SurfCTFConstOutPart(1) = ctfconstout;
    hbs.SurfCTFConstInPart(1) = ctfconstin;
    hbs.SurfOpaqQRadSWOutAbs(1) = qradswoutabs;
    hbs.SurfOpaqQRadSWInAbs(1) = qradswinabs;
    hbs.SurfTempIn(1) = tempin;
    hbs.SurfQsrcHist(1, 1) = qsrchist;
    hbs.SurfQdotRadNetLWInPerArea(1) = qdotradnetlwin;
    hbs.SurfQRadSWOutMvIns(1) = qradswoutmvins;
    hbs.SurfOutsideTempHist(1)(1) = 0.0;
    hbs.SurfQdotRadOutRepPerArea(1) = 0.0;
    state.dataHeatBal->SurfQdotRadIntGainsInPerArea(1) = qdotradintgainsin;
    state.dataZoneTempPredictorCorrector->spaceHeatBalance(1).MAT = mat;
    auto &fs = *state.dataHeatBalFanSys;
    fs.RadSysToHBConstCoef(1) = 0.0;
    fs.RadSysToHBTinCoef(1) = 0.0;
    fs.RadSysToHBQsrcCoef(1) = 0.0;

    bool errorFlag = false;
    HeatBalanceSurfaceManager::CalcOutsideSurfTemp(state, 1, 1, 1, hmovinsul, tempext, errorFlag);

    switch (which) {
    case 0:
        return hbs.SurfOutsideTempHist(1)(1);
    case 1:
        return hbs.SurfQdotRadOutRepPerArea(1);
    case 2:
        return fs.RadSysToHBConstCoef(1);
    case 3:
        return fs.RadSysToHBTinCoef(1);
    default:
        return fs.RadSysToHBQsrcCoef(1);
    }
}
}
