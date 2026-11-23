// Written by hand: InitIntConvCoeff reads each surface's geometry, zone air and convection model
// from the EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry point
// over the real EnergyPlus ConvectionCoefficients code.
//
// Each call sets up zone 1 (volume volume) with space 1 at mean air temperature mat and humidity
// ratio humrat, holding heat transfer surface 1: a window when window is 1, else an opaque
// surface, facing the outside air when extbound is 0 or an interior partition when it is 1. Its
// tilt is tilt degrees (cosine costilt, sine sintilt), height height and area area; its face is
// at tsurf; the outside wind speed is outwind and the outdoor pressure outbaro. Its inside
// convection model is model (Convect::HcInt: 0 a user value hvalue, 3 ASHRAE simple, 4 TARP,
// which windows take as ISO 15099, 8 ASTM C1340), set on the zone instead when setbyzone is 1.
// With emsover 1 EMS overrides the coefficient with emsval; adjratio is the window frame
// adjustment ratio and lowlimit the lowest allowed coefficient (LowHConvLimit). The zone has no
// air system (no loop nodes). Then the real Convect::InitIntConvCoeff runs over the surface
// temperatures and the call returns SurfHConvInt of surface 1. One EnergyPlusData for the
// process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: a flag outside 0 or 1; a model outside the list above
// (schedules, user curves, ceiling diffusers, Trombe walls and the adaptive algorithm need input
// objects); and a user value set on the zone, which EnergyPlus does not allow.
#include <cmath>
#include <EnergyPlus/Construction.hh>
#include <EnergyPlus/ConvectionCoefficients.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
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
        p->dataGlobal->BeginEnvrnFlag = false;
        p->dataConvect->GetUserSuppliedConvectionCoeffs = false;
        p->dataConvect->NodeCheck = false;
        p->dataConvect->ActiveSurfaceCheck = false;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).spaceIndexes.push_back(1);
        p->dataHeatBal->space.allocate(1);
        p->dataHeatBal->space(1).HTSurfaceFirst = 1;
        p->dataHeatBal->space(1).HTSurfaceLast = 1;
        p->dataZoneTempPredictorCorrector->spaceHeatBalance.allocate(1);
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 1;
        surf.Surface.allocate(1);
        auto &s1 = surf.Surface(1);
        s1.HeatTransSurf = true;
        s1.Zone = 1;
        s1.spaceNum = 1;
        s1.BaseSurf = 1;
        s1.Construction = 1;
        p->dataConstruction->Construct.allocate(1);
        surf.surfIntConv.allocate(1);
        surf.surfIntConv(1).userModelNum = 1;
        surf.userIntConvModels.allocate(1);
        surf.userIntConvModels(1).overrideType = Convect::OverrideType::Value;
        surf.SurfOutWindSpeed.dimension(1, 0.0);
        surf.SurfAdjacentZone.dimension(1, 0);
        surf.SurfEMSValueForIntConvCoef.dimension(1, 0.0);
        surf.SurfEMSOverrideIntConvCoef.dimension(1, false);
        p->dataHeatBalSurf->SurfHConvInt.dimension(1, 0.0);
        p->dataHeatBalSurf->SurfWinCoeffAdjRatio.dimension(1, 1.0);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// SurfHConvInt of surface 1 after InitIntConvCoeff (see the file comment).
double ep_IntConvCoeff(int model,
                       int setbyzone,
                       double tsurf,
                       double mat,
                       double humrat,
                       double costilt,
                       double tilt,
                       double sintilt,
                       double height,
                       double area,
                       int window,
                       int extbound,
                       double outwind,
                       double outbaro,
                       double volume,
                       double hvalue,
                       int emsover,
                       double emsval,
                       double adjratio,
                       double lowlimit)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    bool const known = model == 0 || model == 3 || model == 4 || model == 8;
    if (!known || !flag(setbyzone) || !flag(window) || !flag(extbound) || !flag(emsover) || (setbyzone == 1 && model == 0)) {
        return NAN;
    }
    EnergyPlusData &state = st();
    auto &surf = *state.dataSurface;
    auto &s1 = surf.Surface(1);
    s1.Class = window == 1 ? DataSurfaces::SurfaceClass::Window : DataSurfaces::SurfaceClass::Wall;
    s1.ExtBoundCond = extbound == 1 ? 1 : DataSurfaces::ExternalEnvironment;
    s1.CosTilt = costilt;
    s1.Tilt = tilt;
    s1.SinTilt = sintilt;
    s1.Height = height;
    s1.Area = area;
    state.dataConstruction->Construct(1).TypeIsWindow = window == 1;
    auto &space = state.dataHeatBal->space(1);
    space.WindowSurfaceFirst = window == 1 ? 1 : 0;
    space.WindowSurfaceLast = window == 1 ? 1 : -1;
    state.dataHeatBal->Zone(1).Volume = volume;
    auto const hc = static_cast<Convect::HcInt>(model);
    state.dataHeatBal->Zone(1).IntConvAlgo = hc;
    surf.surfIntConv(1).model = setbyzone == 1 ? Convect::HcInt::SetByZone : hc;
    surf.userIntConvModels(1).OverrideValue = hvalue;
    surf.SurfOutWindSpeed(1) = outwind;
    surf.SurfEMSOverrideIntConvCoef(1) = emsover == 1;
    surf.SurfEMSValueForIntConvCoef(1) = emsval;
    auto &zhb = state.dataZoneTempPredictorCorrector->spaceHeatBalance(1);
    zhb.MAT = mat;
    zhb.airHumRatAvg = humrat;
    state.dataEnvrn->OutBaroPress = outbaro;
    state.dataHeatBal->LowHConvLimit = lowlimit;
    state.dataHeatBalSurf->SurfWinCoeffAdjRatio(1) = adjratio;
    state.dataHeatBalSurf->SurfHConvInt(1) = 0.0;
    Array1D<Real64> temps(1, tsurf);

    Convect::InitIntConvCoeff(state, temps);

    return state.dataHeatBalSurf->SurfHConvInt(1);
}
}
