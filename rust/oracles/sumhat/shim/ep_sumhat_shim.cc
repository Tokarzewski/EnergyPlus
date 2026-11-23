// Written by hand: SpaceHeatBalanceData::calcSumHAT reads each surface's convection and window
// terms from the EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry
// point over the real EnergyPlus ZoneTempPredictorCorrector code.
//
// Each call sets up zone 1 (controlled when controlled is 1, with no heat to return air when
// noheatret is 1, and zone system sums sysmcp and sysmcpt) with space 1 holding heat transfer
// surface 1: a window when window is 1 (shading flag shading, as DataSurfaces::WinShadingType;
// an equivalent layer construction when eql is 1), else a wall. The surface has area area, inside
// convection coefficient hconv, inside face temperature tin, reference air temperature tairref
// (DataSurfaces::RefAirTemp, -1 to 2) and adjacent air temperature tempeff. Its window terms are
// the divider area, convective heat gain, projection correction and inside temperature
// (dividerarea, dividergain, projdiv, dividertemp), the frame area, projection correction and
// inside temperature (framearea, projfr, frametemp), the equivalent layer and shade gap gains
// (otherconv, natconv), and the airflow window flow and gains (airflow, convgain, retgain); its
// window heat gain so far is heatgain0 and the zone time step tszsec seconds. Then the real
// SpaceHeatBalanceData::calcSumHAT runs for space 1. One EnergyPlusData for the process, built
// when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 9; a flag outside 0 or 1; shading
// outside -1 to 13; tairref outside -1 to 2; and the supply air reference in a zone that is not
// controlled, which EnergyPlus stops on as a fatal error.
#include <cmath>
#include <EnergyPlus/Construction.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalSurface.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).spaceIndexes.push_back(1);
        p->dataHeatBal->space.allocate(1);
        p->dataHeatBal->space(1).HTSurfaceFirst = 1;
        p->dataHeatBal->space(1).HTSurfaceLast = 1;
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        p->dataZoneTempPredictorCorrector->spaceHeatBalance.allocate(1);
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 1;
        surf.Surface.allocate(1);
        surf.Surface(1).HeatTransSurf = true;
        surf.Surface(1).Zone = 1;
        surf.Surface(1).spaceNum = 1;
        surf.Surface(1).Construction = 1;
        p->dataConstruction->Construct.allocate(1);
        surf.SurfWinShadingFlag.allocate(1);
        for (auto *a : {&surf.SurfWinDividerArea, &surf.SurfWinDividerHeatGain, &surf.SurfWinOtherConvHeatGain, &surf.SurfWinConvHeatFlowNatural,
                        &surf.SurfWinAirflowThisTS, &surf.SurfWinConvHeatGainToZoneAir, &surf.SurfWinRetHeatGainToZoneAir, &surf.SurfWinHeatGain,
                        &surf.SurfWinHeatGainRep, &surf.SurfWinHeatGainRepEnergy, &surf.SurfWinHeatLossRep, &surf.SurfWinHeatLossRepEnergy,
                        &surf.SurfWinHeatTransferRepEnergy, &surf.SurfWinFrameArea, &surf.SurfWinProjCorrFrIn, &surf.SurfWinFrameTempIn,
                        &surf.SurfWinProjCorrDivIn, &surf.SurfWinDividerTempIn}) {
            a->dimension(1, 0.0);
        }
        surf.SurfTAirRef.dimension(1, 0);
        p->dataHeatBalSurf->SurfHConvInt.dimension(1, 0.0);
        p->dataHeatBalSurf->SurfTempInTmp.dimension(1, 0.0);
        p->dataHeatBal->SurfTempEffBulkAir.dimension(1, 0.0);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of calcSumHAT for space 1 (see the file comment): which picks sumIntGain (0), sumHA
// (1), sumHATsurf (2), sumHATref (3), then the window's SurfWinHeatGain (4), SurfWinHeatGainRep
// (5), SurfWinHeatGainRepEnergy (6), SurfWinHeatLossRep (7), SurfWinHeatLossRepEnergy (8) and
// SurfWinHeatTransferRepEnergy (9), the last five starting at 0.
double ep_SumHAT(int which,
                 int window,
                 int shading,
                 int eql,
                 double area,
                 double hconv,
                 double tin,
                 int tairref,
                 double tempeff,
                 double dividerarea,
                 double dividergain,
                 double projdiv,
                 double dividertemp,
                 double framearea,
                 double projfr,
                 double frametemp,
                 double otherconv,
                 double natconv,
                 double airflow,
                 double convgain,
                 double retgain,
                 int noheatret,
                 double heatgain0,
                 int controlled,
                 double sysmcp,
                 double sysmcpt,
                 double tszsec)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 9 || !flag(window) || !flag(eql) || !flag(noheatret) || !flag(controlled) || shading < -1 || shading > 13 ||
        tairref < -1 || tairref > 2 || (tairref == 2 && controlled == 0)) {
        return NAN;
    }
    EnergyPlusData &state = st();
    auto &surf = *state.dataSurface;
    auto &s1 = surf.Surface(1);
    s1.Class = window == 1 ? DataSurfaces::SurfaceClass::Window : DataSurfaces::SurfaceClass::Wall;
    s1.Area = area;
    state.dataConstruction->Construct(1).WindowTypeEQL = eql == 1;
    surf.SurfWinShadingFlag(1) = static_cast<DataSurfaces::WinShadingType>(shading);
    surf.SurfWinDividerArea(1) = dividerarea;
    surf.SurfWinDividerHeatGain(1) = dividergain;
    surf.SurfWinProjCorrDivIn(1) = projdiv;
    surf.SurfWinDividerTempIn(1) = dividertemp;
    surf.SurfWinFrameArea(1) = framearea;
    surf.SurfWinProjCorrFrIn(1) = projfr;
    surf.SurfWinFrameTempIn(1) = frametemp;
    surf.SurfWinOtherConvHeatGain(1) = otherconv;
    surf.SurfWinConvHeatFlowNatural(1) = natconv;
    surf.SurfWinAirflowThisTS(1) = airflow;
    surf.SurfWinConvHeatGainToZoneAir(1) = convgain;
    surf.SurfWinRetHeatGainToZoneAir(1) = retgain;
    surf.SurfWinHeatGain(1) = heatgain0;
    for (auto *a : {&surf.SurfWinHeatGainRep, &surf.SurfWinHeatGainRepEnergy, &surf.SurfWinHeatLossRep, &surf.SurfWinHeatLossRepEnergy,
                    &surf.SurfWinHeatTransferRepEnergy}) {
        (*a)(1) = 0.0;
    }
    surf.SurfTAirRef(1) = tairref;
    state.dataHeatBalSurf->SurfHConvInt(1) = hconv;
    state.dataHeatBalSurf->SurfTempInTmp(1) = tin;
    state.dataHeatBal->SurfTempEffBulkAir(1) = tempeff;
    auto &zone = state.dataHeatBal->Zone(1);
    zone.IsControlled = controlled == 1;
    zone.NoHeatToReturnAir = noheatret == 1;
    auto &ztpc = *state.dataZoneTempPredictorCorrector;
    ztpc.zoneHeatBalance(1).SumSysMCp = sysmcp;
    ztpc.zoneHeatBalance(1).SumSysMCpT = sysmcpt;
    state.dataGlobal->TimeStepZoneSec = tszsec;

    ZoneTempPredictorCorrector::SumHATOutput const r = ztpc.spaceHeatBalance(1).calcSumHAT(state, 1, 1);

    switch (which) {
    case 0:
        return r.sumIntGain;
    case 1:
        return r.sumHA;
    case 2:
        return r.sumHATsurf;
    case 3:
        return r.sumHATref;
    case 4:
        return surf.SurfWinHeatGain(1);
    case 5:
        return surf.SurfWinHeatGainRep(1);
    case 6:
        return surf.SurfWinHeatGainRepEnergy(1);
    case 7:
        return surf.SurfWinHeatLossRep(1);
    case 8:
        return surf.SurfWinHeatLossRepEnergy(1);
    default:
        return surf.SurfWinHeatTransferRepEnergy(1);
    }
}
}
