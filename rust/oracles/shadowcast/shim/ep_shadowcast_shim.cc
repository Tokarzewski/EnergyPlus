// Written by hand: SHADOW works on the surfaces, shadowing combinations and homogeneous-coordinate
// figure arrays in the EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C"
// entry point over the real EnergyPlus SolarShading code.
//
// Each call sets up a sunlit base surface 1 (receiving, nvr world vertices r1..r4, local
// coordinate system lcsx, lcsy, lcsz about origin X0, Y0, Z0, NetAreaShadowCalc area) and a heat
// transfer surface 2 facing away from the sun (the caster, nvc world vertices c1..c4, schedule
// minimum value sched) that is its one general shadowing surface, with no back surfaces or
// subsurfaces, and runs SHADOW for the sun direction s. SHADOW then works out the shadow
// projections, transforms the receiving surface into its own plane (CTRANS, reversed to clockwise,
// HTRANS1) and calls SHDGSS, which transforms the caster (CTRANS), clips it at the receiving plane
// (CLIP), projects it along the sun, snaps its vertices to the receiving surface's within 6 units,
// overlaps it in place with DeterminePolygonOverlap and sets the sunlit area, which SHADOW clamps
// to [0, area]. algo picks the clipping: 0 SutherlandHodgman, 1 SlaterBarskyandSutherlandHodgman.
// The figure and scratch arrays are sized and zeroed on every call as AllocateModuleArrays would
// size them for surfaces of up to 4 vertices.
//
// Guards (ADR-009), which return NaN: algo outside 0 or 1; vertex counts other than 3 or 4;
// vertices and origin past 30 m, local axis and sun components past 1, sched outside [0, 1] and
// area outside [0, 1e6] (or not finite); and a sun so low over the receiving plane that a shadow
// projection |XS / ZS| or |YS / ZS| passes 4 (with |ZS| above SmallDistance), which with the
// 30 m limit keeps every projected coordinate within 1000 m, where the homogeneous products stay
// inside Int64. One EnergyPlusData for the process, built when the library loads, with console
// output off.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataShadowingCombinations.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/DataSystemVariables.hh>
#include <EnergyPlus/EnergyPlus.hh>
#include <EnergyPlus/SolarShading.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto &surf = *p->dataSurface;
        surf.TotSurfaces = 2;
        surf.Surface.allocate(2);
        for (int i = 1; i <= 2; ++i) {
            auto &s = surf.Surface(i);
            s.Vertex.dimension(4);
            s.BaseSurf = 1;
            s.IsShadowing = false;
            s.HeatTransSurf = true;
            s.IsTransparent = false;
            s.shadowSurfSched = nullptr;
            s.Zone = 1;
        }
        surf.Surface(2).BaseSurf = 2;
        surf.X0.dimension(2, 0.0);
        surf.Y0.dimension(2, 0.0);
        surf.Z0.dimension(2, 0.0);
        auto &sc = p->dataShadowComb->ShadowComb;
        sc.allocate(2);
        sc(1).UseThisSurf = true;
        sc(1).NumGenSurf = 1;
        sc(1).GenSurf.dimension(1, 2);
        sc(1).NumBackSurf = 0;
        sc(1).NumSubSurf = 0;
        sc(2).UseThisSurf = false;
        auto &ss = *p->dataSolarShading;
        ss.SurfSunCosTheta.dimension(2, 0.0);
        ss.SurfSunCosTheta(1) = 1.0;  // the receiving surface faces the sun
        ss.SurfSunCosTheta(2) = -1.0; // the caster faces away from it, so it casts a shadow
        ss.SurfSunlitArea.dimension(2, 0.0);
        ss.CalcSkyDifShading = false;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

void reset(EnergyPlusData &state, int const algo)
{
    auto &ss = *state.dataSolarShading;
    int const maxV = state.dataSurface->MaxVerticesPerSurface = 4;
    state.dataSysVars->SutherlandHodgman = true;
    state.dataSysVars->SlaterBarsky = algo == 1;
    state.dataSysVars->DisableAllSelfShading = false;
    state.dataSysVars->DisableGroupSelfShading = false;
    ss.MaxHCV = 15;
    ss.MaxHCS = 4;
    ss.MAXHCArrayBounds = 2 * (maxV + 1);
    ss.MAXHCArrayIncrement = maxV + 1;
    for (auto *a : {&ss.HCA, &ss.HCB, &ss.HCC, &ss.HCX, &ss.HCY}) {
        a->dimension(2 * ss.MaxHCS, ss.MaxHCV + 1, 0);
    }
    ss.HCAREA.dimension(2 * ss.MaxHCS, 0.0);
    ss.HCNV.dimension(2 * ss.MaxHCS, 0);
    ss.HCT.dimension(2 * ss.MaxHCS, 0.0);
    for (auto *a : {&ss.XTEMP, &ss.YTEMP, &ss.XTEMP1, &ss.YTEMP1, &ss.ATEMP, &ss.BTEMP, &ss.CTEMP}) {
        a->dimension(2 * (maxV + 1), 0.0);
    }
    for (auto *a : {&ss.XVS, &ss.YVS, &ss.XVC, &ss.YVC, &ss.ZVC}) {
        a->dimension(maxV + 1, 0.0);
    }
    ss.NVS = 0;
    ss.NumVertInShadowOrClippedSurface = 0;
    ss.OverlapStatus = 0;
}

bool within(double v, double lim)
{
    return std::abs(v) <= lim;
}
} // namespace

extern "C" {
// Sunlit area of receiving surface 1 under the shadow of surface 2 (see the file comment). which
// picks the final SurfSunlitArea (0), OverlapStatus (1: 1 NoOverlap, 2 FirstSurfWithinSecond,
// 3 SecondSurfWithinFirst, 4 PartialOverlap, 5 TooManyVertices), the clipped shadow's NVS (2)
// and NumVertInShadowOrClippedSurface (3), the vertex count (4), area (5) and transmittance (6)
// of figure 2 (the shadow, overlapped in place), LOCHCA (7), NGSSHC (8), the receiving figure's
// area (9), the shadow projections XShadowProjection (10) and YShadowProjection (11), or figure
// 2's homogeneous X (12 + k) and Y (27 + k) of vertex k, k < its vertex count.
double ep_ShadowCast(int which,
                     int algo,
                     int nvr,
                     double rx1,
                     double ry1,
                     double rz1,
                     double rx2,
                     double ry2,
                     double rz2,
                     double rx3,
                     double ry3,
                     double rz3,
                     double rx4,
                     double ry4,
                     double rz4,
                     int nvc,
                     double cx1,
                     double cy1,
                     double cz1,
                     double cx2,
                     double cy2,
                     double cz2,
                     double cx3,
                     double cy3,
                     double cz3,
                     double cx4,
                     double cy4,
                     double cz4,
                     double ox,
                     double oy,
                     double oz,
                     double lxx,
                     double lxy,
                     double lxz,
                     double lyx,
                     double lyy,
                     double lyz,
                     double lzx,
                     double lzy,
                     double lzz,
                     double sx,
                     double sy,
                     double sz,
                     double sched,
                     double area)
{
    double const r[12] = {rx1, ry1, rz1, rx2, ry2, rz2, rx3, ry3, rz3, rx4, ry4, rz4};
    double const c[12] = {cx1, cy1, cz1, cx2, cy2, cz2, cx3, cy3, cz3, cx4, cy4, cz4};
    double const o[3] = {ox, oy, oz};
    double const l[12] = {lxx, lxy, lxz, lyx, lyy, lyz, lzx, lzy, lzz, sx, sy, sz};
    if (algo < 0 || algo > 1 || nvr < 3 || nvr > 4 || nvc < 3 || nvc > 4) {
        return NAN;
    }
    for (int i = 0; i < 12; ++i) {
        if (!within(r[i], 30.0) || !within(c[i], 30.0) || !within(l[i], 1.0)) {
            return NAN;
        }
    }
    if (!within(o[0], 30.0) || !within(o[1], 30.0) || !within(o[2], 30.0) || !(sched >= 0.0 && sched <= 1.0) ||
        !(area >= 0.0 && area <= 1.0e6)) {
        return NAN;
    }
    // SHADOW's shadow displacements, refused where the projection would pass 4.
    double const XS = lxx * sx + lxy * sy + lxz * sz;
    double const YS = lyx * sx + lyy * sy + lyz * sz;
    double const ZS = lzx * sx + lzy * sy + lzz * sz;
    if (std::abs(ZS) > Constant::SmallDistance && !(std::abs(XS / ZS) <= 4.0 && std::abs(YS / ZS) <= 4.0)) {
        return NAN;
    }

    EnergyPlusData &state = st();
    reset(state, algo);
    auto &surf = *state.dataSurface;
    auto &ss = *state.dataSolarShading;
    auto &recv = surf.Surface(1);
    auto &cast = surf.Surface(2);
    recv.Sides = nvr;
    for (int n = 1; n <= nvr; ++n) {
        recv.Vertex(n).x = r[3 * n - 3];
        recv.Vertex(n).y = r[3 * n - 2];
        recv.Vertex(n).z = r[3 * n - 1];
    }
    cast.Sides = nvc;
    for (int n = 1; n <= nvc; ++n) {
        cast.Vertex(n).x = c[3 * n - 3];
        cast.Vertex(n).y = c[3 * n - 2];
        cast.Vertex(n).z = c[3 * n - 1];
    }
    recv.lcsx.x = lxx;
    recv.lcsx.y = lxy;
    recv.lcsx.z = lxz;
    recv.lcsy.x = lyx;
    recv.lcsy.y = lyy;
    recv.lcsy.z = lyz;
    recv.lcsz.x = lzx;
    recv.lcsz.y = lzy;
    recv.lcsz.z = lzz;
    recv.NetAreaShadowCalc = area;
    cast.SchedMinValue = sched;
    surf.X0(1) = ox;
    surf.Y0(1) = oy;
    surf.Z0(1) = oz;
    ss.SUNCOS.x = sx;
    ss.SUNCOS.y = sy;
    ss.SUNCOS.z = sz;

    SolarShading::SHADOW(state, 1, 1);

    int const nv2 = ss.HCNV(2);
    switch (which) {
    case 0:
        return ss.SurfSunlitArea(1);
    case 1:
        return ss.OverlapStatus;
    case 2:
        return ss.NVS;
    case 3:
        return ss.NumVertInShadowOrClippedSurface;
    case 4:
        return nv2;
    case 5:
        return ss.HCAREA(2);
    case 6:
        return ss.HCT(2);
    case 7:
        return ss.LOCHCA;
    case 8:
        return ss.NGSSHC;
    case 9:
        return ss.HCAREA(1);
    case 10:
        return ss.XShadowProjection;
    case 11:
        return ss.YShadowProjection;
    default:
        break;
    }
    if (which >= 12 && which < 12 + 15 && which - 12 < nv2) {
        return static_cast<double>(ss.HCX(2, which - 12 + 1));
    }
    if (which >= 27 && which < 27 + 15 && which - 27 < nv2) {
        return static_cast<double>(ss.HCY(2, which - 27 + 1));
    }
    return NAN;
}
}
