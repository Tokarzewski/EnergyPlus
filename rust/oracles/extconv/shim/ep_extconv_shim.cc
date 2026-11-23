// Written by hand: InitExtConvCoeff reads one surface's geometry, weather and convection model
// from the EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry point
// over the real EnergyPlus ConvectionCoefficients code.
//
// Each call sets up surface 1 in zone 1, its own base surface, facing the outside air: a wall,
// or a window when window is 1 (with an exterior shade when extshade is 1), exposed to the wind
// when extwind is 1, of gross area grossarea, height height, tilt cosine costilt and azimuth
// azimuth, with sky and ground infrared view factors vfsky and vfgnd, air/sky radiation split
// airskysplit and convection adjustment ratio adjratio. Its outside convection model is model
// (Convect::HcExt: 0 a user value hvalue, 3 ASHRAE simple, 4 ASHRAE TARP, 5 TARP, 6 MoWiTT,
// 7 DOE-2, 8 BLAST), set on the zone instead when setbyzone is 1. Outside air dry bulb outdb,
// wind speed windspeed from direction winddir, sky temperature skytempk (K). With hassrd 1 the
// surface has a surrounding surfaces list with no sky temperature schedule, at srdtemp with view
// factor vfsrd; with usegnd 1 its ground temperature is gndtemp from a ground surfaces list. With
// emsover 1 EMS overrides the coefficient with emsval. Then the real Convect::InitExtConvCoeff
// runs for movable insulation hmov, roughness rough (0 very rough to 5 very smooth), thermal
// absorptance absext and outside face temperature tempext. One EnergyPlusData for the process,
// built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 4; a flag outside 0 or 1; rough outside
// 0 to 5; a model outside the list above (schedules, user curves and the adaptive algorithm need
// input objects); and a user value set on the zone, which EnergyPlus does not allow.
#include <cmath>
#include <EnergyPlus/ConvectionCoefficients.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalSurface.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataConvect->GetUserSuppliedConvectionCoeffs = false;
        p->dataHeatBal->Zone.allocate(1);
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 1;
        surf.Surface.allocate(1);
        auto &s1 = surf.Surface(1);
        s1.HeatTransSurf = true;
        s1.Zone = 1;
        s1.BaseSurf = 1;
        s1.ExtBoundCond = DataSurfaces::ExternalEnvironment;
        surf.SurroundingSurfsProperty.allocate(1);
        surf.SurroundingSurfsProperty(1).skyTempSched = nullptr;
        surf.GroundSurfsProperty.allocate(1);
        surf.surfExtConv.allocate(1);
        surf.surfExtConv(1).userModelNum = 1;
        surf.userExtConvModels.allocate(1);
        surf.userExtConvModels(1).overrideType = Convect::OverrideType::Value;
        for (auto *a : {&surf.SurfOutDryBulbTemp, &surf.SurfOutWindSpeed, &surf.SurfOutWindDir, &surf.SurfAirSkyRadSplit,
                        &surf.SurfEMSValueForExtConvCoef}) {
            a->dimension(1, 0.0);
        }
        surf.SurfEMSOverrideExtConvCoef.dimension(1, false);
        surf.SurfWinShadingFlag.allocate(1);
        p->dataHeatBalSurf->SurfWinCoeffAdjRatio.dimension(1, 1.0);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One coefficient from InitExtConvCoeff (see the file comment): which picks HExt (0), HSky (1),
// HGround (2), HAir (3) or HSrdSurf (4).
double ep_ExtConvCoeff(int which,
                       int model,
                       int setbyzone,
                       double hmov,
                       int rough,
                       double absext,
                       double tempext,
                       double outdb,
                       double skytempk,
                       int hassrd,
                       double srdtemp,
                       double vfsrd,
                       int usegnd,
                       double gndtemp,
                       int extwind,
                       int window,
                       int extshade,
                       double windspeed,
                       double winddir,
                       double grossarea,
                       double height,
                       double costilt,
                       double azimuth,
                       double hvalue,
                       int emsover,
                       double emsval,
                       double adjratio,
                       double vfsky,
                       double vfgnd,
                       double airskysplit)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    bool const known = model == 0 || (model >= 3 && model <= 8);
    if (which < 0 || which > 4 || !known || !flag(setbyzone) || !flag(hassrd) || !flag(usegnd) || !flag(extwind) || !flag(window) ||
        !flag(extshade) || !flag(emsover) || rough < 0 || rough > 5 || (setbyzone == 1 && model == 0)) {
        return NAN;
    }
    EnergyPlusData &state = st();
    auto &surf = *state.dataSurface;
    auto &s1 = surf.Surface(1);
    s1.Class = window == 1 ? DataSurfaces::SurfaceClass::Window : DataSurfaces::SurfaceClass::Wall;
    s1.ExtWind = extwind == 1;
    s1.GrossArea = grossarea;
    s1.Height = height;
    s1.CosTilt = costilt;
    s1.Azimuth = azimuth;
    s1.ViewFactorSkyIR = vfsky;
    s1.ViewFactorGroundIR = vfgnd;
    s1.SurfHasSurroundingSurfProperty = hassrd == 1;
    s1.SurfSurroundingSurfacesNum = hassrd;
    s1.SrdSurfTemp = srdtemp;
    s1.ViewFactorSrdSurfs = vfsrd;
    s1.UseSurfPropertyGndSurfTemp = usegnd == 1;
    s1.SurfPropertyGndSurfIndex = usegnd;
    surf.GroundSurfsProperty(1).SurfsTempAvg = gndtemp;
    surf.SurfWinShadingFlag(1) = extshade == 1 ? DataSurfaces::WinShadingType::ExtShade : DataSurfaces::WinShadingType::NoShade;
    surf.SurfOutDryBulbTemp(1) = outdb;
    surf.SurfOutWindSpeed(1) = windspeed;
    surf.SurfOutWindDir(1) = winddir;
    surf.SurfAirSkyRadSplit(1) = airskysplit;
    surf.SurfEMSOverrideExtConvCoef(1) = emsover == 1;
    surf.SurfEMSValueForExtConvCoef(1) = emsval;
    auto const hc = static_cast<Convect::HcExt>(model);
    surf.surfExtConv(1).model = setbyzone == 1 ? Convect::HcExt::SetByZone : hc;
    state.dataHeatBal->Zone(1).ExtConvAlgo = hc;
    surf.userExtConvModels(1).OverrideValue = hvalue;
    state.dataHeatBalSurf->SurfWinCoeffAdjRatio(1) = adjratio;
    state.dataEnvrn->SkyTempKelvin = skytempk;
    Real64 h[5] = {0.0, 0.0, 0.0, 0.0, 0.0};

    Convect::InitExtConvCoeff(state, 1, hmov, static_cast<Material::SurfaceRoughness>(rough), absext, tempext, h[0], h[1], h[2], h[3], h[4]);

    return h[which];
}
}
