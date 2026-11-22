// Written by hand: AnisoSkyViewFactors reads the sun's direction, the weather and the exterior
// surfaces from EnergyPlusData, which gen_plain.py cannot build. Plain extern "C" entry points over
// the real EnergyPlus SolarShading code: the Perez anisotropic sky model on one exterior surface,
// and the CLIPLINE line clipper.
//
// One EnergyPlusData for the process, built when the library loads, with console output off; the
// surface and its shading ratios are set up per call (hour 1, time step 1). ShowFatalError (a beam
// incidence cosine past 1 + 0.0001) throws, and the shim returns NaN for it.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/DataSystemVariables.hh>
#include <EnergyPlus/EnergyPlus.hh>
#include <EnergyPlus/SolarShading.hh>
#include <EnergyPlus/UtilityRoutines.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->HourOfDay = 1;
        p->dataGlobal->TimeStep = 1;
        p->dataSurface->Surface.allocate(1);
        p->dataSurface->Surface(1).Name = "WALL";
        p->dataSurface->AllExtSolarSurfaceList = {1};
        auto &ss = *p->dataSolarShading;
        for (auto *a : {&ss.SurfAnisoSkyMult, &ss.SurfDifShdgRatioIsoSky, &ss.SurfCurDifShdgRatioIsoSky, &ss.SurfDifShdgRatioHoriz,
                        &ss.SurfMultIsoSky, &ss.SurfMultCircumSolar, &ss.SurfMultHorizonZenith}) {
            a->allocate(1);
        }
        ss.SurfDifShdgRatioIsoSkyHRTS.allocate(1, 1, 1);
        ss.SurfDifShdgRatioHorizHRTS.allocate(1, 1, 1);
        p->dataHeatBal->SurfSunlitFrac.allocate(1, 1, 1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// The sky model on one surface. sx, sy, sz: the sun's direction cosines (SOLCOS); n*: the
// surface's outward normal; tilt in degrees (SinTilt is sin(tilt)); iso, sunlit, horiz: its
// isotropic-sky shading ratio, sunlit fraction and horizon shading ratio. detailed 1 takes the
// detailed sky diffuse branch (the ratios come from the hourly arrays). which picks
// SurfAnisoSkyMult (0), SurfMultIsoSky (1), SurfMultCircumSolar (2), SurfMultHorizonZenith (3) or
// SurfCurDifShdgRatioIsoSky (4).
double ep_PerezSky(int which, int detailed, double sx, double sy, double sz, double elev, double beam, double dif, double nx, double ny,
                double nz, double vfsky, double tilt, double iso, double sunlit, double horiz)
{
    auto &s = st();
    auto &ss = *s.dataSolarShading;
    auto &surf = s.dataSurface->Surface(1);
    s.dataEnvrn->SOLCOS = {sx, sy, sz};
    s.dataEnvrn->Elevation = elev;
    s.dataEnvrn->BeamSolarRad = beam;
    s.dataEnvrn->DifSolarRad = dif;
    surf.OutNormVec = {nx, ny, nz};
    surf.ViewFactorSky = vfsky;
    surf.Tilt = tilt;
    surf.SinTilt = std::sin(tilt * Constant::DegToRad);
    s.dataSysVars->DetailedSkyDiffuseAlgorithm = detailed != 0;
    s.dataSurface->ShadingTransmittanceVaries = detailed != 0;
    ss.SurfDifShdgRatioIsoSky(1) = iso;
    ss.SurfDifShdgRatioHoriz(1) = horiz;
    ss.SurfDifShdgRatioIsoSkyHRTS(1, 1, 1) = iso;
    ss.SurfDifShdgRatioHorizHRTS(1, 1, 1) = horiz;
    s.dataHeatBal->SurfSunlitFrac(1, 1, 1) = sunlit;
    ss.SurfCurDifShdgRatioIsoSky(1) = 0.0;
    try {
        SolarShading::AnisoSkyViewFactors(s);
    } catch (FatalError const &) {
        return NAN;
    }
    switch (which) {
    case 0:
        return ss.SurfAnisoSkyMult(1);
    case 1:
        return ss.SurfMultIsoSky(1);
    case 2:
        return ss.SurfMultCircumSolar(1);
    case 3:
        return ss.SurfMultHorizonZenith(1);
    case 4:
        return ss.SurfCurDifShdgRatioIsoSky(1);
    default:
        return NAN;
    }
}

// CLIPLINE on the segment (x0, y0) to (x1, y1): which picks the clipped x0 (0), x1 (1), y0 (2),
// y1 (3), or whether any of it is visible (4, 0 or 1).
double ep_ClipLine(int which, double x0, double x1, double y0, double y1, double maxX, double minX, double maxY, double minY)
{
    bool visible = false;
    SolarShading::CLIPLINE(x0, x1, y0, y1, maxX, minX, maxY, minY, visible);
    switch (which) {
    case 0:
        return x0;
    case 1:
        return x1;
    case 2:
        return y0;
    case 3:
        return y1;
    case 4:
        return visible ? 1.0 : 0.0;
    default:
        return NAN;
    }
}
}
