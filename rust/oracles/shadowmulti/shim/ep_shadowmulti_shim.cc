// Written by hand: SHADOW works on the surfaces, shadowing combinations and homogeneous-coordinate
// figure arrays in the EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C"
// entry point over the real EnergyPlus SolarShading code.
//
// Each call sets up a sunlit base surface 1 (receiving, nvr world vertices r1..r4, local
// coordinate system lcsx, lcsy, lcsz about origin X0, Y0, Z0, NetAreaShadowCalc area) and ncast
// (1 to 3) heat transfer surfaces 2.. as its general shadowing surfaces (caster i: nvi world
// vertices ci1..ci4, the cosine qi of the sun's angle on it, schedule minimum value ti), with no
// back surfaces or subsurfaces, and runs SHADOW for the sun direction s. SHADOW works out the
// shadow projections and the receiving figure, then SHDGSS takes each caster in turn: a caster
// facing the sun (qi above SunIsUpValue) casts nothing; the others are brought into the receiving
// plane (CTRANS), clipped at it (CLIP), projected along the sun, snapped to the receiving
// vertices, and overlapped in place on the receiving figure with DeterminePolygonOverlap; each
// shadow kept is overlapped with the shadows before it by MULTOL, and the sunlit area sums every
// figure kept, which SHADOW clamps to [0, area]. algo picks the clipping: 0 SutherlandHodgman,
// 1 SlaterBarskyandSutherlandHodgman. The figure and scratch arrays are sized and zeroed on every
// call as AllocateModuleArrays would size them for surfaces of up to 4 vertices, with MaxHCS 8:
// three casters need at most 8 figures, so TooManyFigures cannot arise.
//
// Guards (ADR-009), which return NaN: algo outside 0 or 1; ncast outside 1 to 3; vertex counts
// other than 3 or 4; vertices and origin past 30 m, local axis, sun and cosine values past 1,
// schedule values outside [0, 1] and area outside [0, 1e6] (or not finite), for the casters in
// use; and a sun so low over the receiving plane that a shadow projection |XS / ZS| or |YS / ZS|
// passes 4 (with |ZS| above SmallDistance), which with the 30 m limit keeps every projected
// coordinate within 1000 m, where the homogeneous products stay inside Int64. One EnergyPlusData
// for the process, built when the library loads, with console output off.
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
        surf.TotSurfaces = 4;
        surf.Surface.allocate(4);
        for (int i = 1; i <= 4; ++i) {
            auto &s = surf.Surface(i);
            s.Vertex.dimension(4);
            s.BaseSurf = 1;
            s.IsShadowing = false;
            s.HeatTransSurf = true;
            s.IsTransparent = false;
            s.shadowSurfSched = nullptr;
            s.Zone = 1;
        }
        for (int i = 2; i <= 4; ++i) {
            surf.Surface(i).BaseSurf = i;
        }
        surf.X0.dimension(4, 0.0);
        surf.Y0.dimension(4, 0.0);
        surf.Z0.dimension(4, 0.0);
        auto &sc = p->dataShadowComb->ShadowComb;
        sc.allocate(4);
        sc(1).UseThisSurf = true;
        sc(1).NumBackSurf = 0;
        sc(1).NumSubSurf = 0;
        for (int i = 2; i <= 4; ++i) {
            sc(i).UseThisSurf = false;
        }
        auto &ss = *p->dataSolarShading;
        ss.SurfSunCosTheta.dimension(4, 0.0);
        ss.SurfSunCosTheta(1) = 1.0; // the receiving surface faces the sun
        ss.SurfSunlitArea.dimension(4, 0.0);
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
    ss.MaxHCS = 8;
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
// Sunlit area of receiving surface 1 under the shadows of the ncast casters (see the file
// comment); caster i's arguments are nvi, its vertices ci1..ci4, qi and ti. which picks the final
// SurfSunlitArea (0), OverlapStatus (1: 1 NoOverlap, 2 FirstSurfWithinSecond, 3
// SecondSurfWithinFirst, 4 PartialOverlap, 5 TooManyVertices), LOCHCA (2), NGSSHC (3), FGSSHC
// (4), the receiving figure's area (5), the vertex count, area and transmittance of figure k
// (6 + 3 * (k - 2) + 0, 1, 2, for k = 2 to 8), or figure k's homogeneous X (27 + 30 * (k - 2) + j)
// and Y (42 + 30 * (k - 2) + j) of vertex j, j < its vertex count.
double ep_ShadowMulti(int which,
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
                      int ncast,
                      int nv1,
                      double c1x1,
                      double c1y1,
                      double c1z1,
                      double c1x2,
                      double c1y2,
                      double c1z2,
                      double c1x3,
                      double c1y3,
                      double c1z3,
                      double c1x4,
                      double c1y4,
                      double c1z4,
                      double q1,
                      double t1,
                      int nv2,
                      double c2x1,
                      double c2y1,
                      double c2z1,
                      double c2x2,
                      double c2y2,
                      double c2z2,
                      double c2x3,
                      double c2y3,
                      double c2z3,
                      double c2x4,
                      double c2y4,
                      double c2z4,
                      double q2,
                      double t2,
                      int nv3,
                      double c3x1,
                      double c3y1,
                      double c3z1,
                      double c3x2,
                      double c3y2,
                      double c3z2,
                      double c3x3,
                      double c3y3,
                      double c3z3,
                      double c3x4,
                      double c3y4,
                      double c3z4,
                      double q3,
                      double t3,
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
                      double area)
{
    double const r[12] = {rx1, ry1, rz1, rx2, ry2, rz2, rx3, ry3, rz3, rx4, ry4, rz4};
    double const c[3][12] = {{c1x1, c1y1, c1z1, c1x2, c1y2, c1z2, c1x3, c1y3, c1z3, c1x4, c1y4, c1z4},
                             {c2x1, c2y1, c2z1, c2x2, c2y2, c2z2, c2x3, c2y3, c2z3, c2x4, c2y4, c2z4},
                             {c3x1, c3y1, c3z1, c3x2, c3y2, c3z2, c3x3, c3y3, c3z3, c3x4, c3y4, c3z4}};
    int const nv[3] = {nv1, nv2, nv3};
    double const q[3] = {q1, q2, q3};
    double const t[3] = {t1, t2, t3};
    double const o[3] = {ox, oy, oz};
    double const l[12] = {lxx, lxy, lxz, lyx, lyy, lyz, lzx, lzy, lzz, sx, sy, sz};
    if (algo < 0 || algo > 1 || nvr < 3 || nvr > 4 || ncast < 1 || ncast > 3) {
        return NAN;
    }
    for (int i = 0; i < 12; ++i) {
        if (!within(r[i], 30.0) || !within(l[i], 1.0)) {
            return NAN;
        }
    }
    for (int k = 0; k < ncast; ++k) {
        if (nv[k] < 3 || nv[k] > 4 || !within(q[k], 1.0) || !(t[k] >= 0.0 && t[k] <= 1.0)) {
            return NAN;
        }
        for (int i = 0; i < 12; ++i) {
            if (!within(c[k][i], 30.0)) {
                return NAN;
            }
        }
    }
    if (!within(o[0], 30.0) || !within(o[1], 30.0) || !within(o[2], 30.0) || !(area >= 0.0 && area <= 1.0e6)) {
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
    recv.Sides = nvr;
    for (int n = 1; n <= nvr; ++n) {
        recv.Vertex(n).x = r[3 * n - 3];
        recv.Vertex(n).y = r[3 * n - 2];
        recv.Vertex(n).z = r[3 * n - 1];
    }
    auto &sc = state.dataShadowComb->ShadowComb(1);
    sc.NumGenSurf = ncast;
    sc.GenSurf.dimension(ncast, 0);
    for (int k = 0; k < ncast; ++k) {
        auto &cast = surf.Surface(k + 2);
        cast.Sides = nv[k];
        for (int n = 1; n <= nv[k]; ++n) {
            cast.Vertex(n).x = c[k][3 * n - 3];
            cast.Vertex(n).y = c[k][3 * n - 2];
            cast.Vertex(n).z = c[k][3 * n - 1];
        }
        cast.SchedMinValue = t[k];
        ss.SurfSunCosTheta(k + 2) = q[k];
        sc.GenSurf(k + 1) = k + 2;
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
    surf.X0(1) = ox;
    surf.Y0(1) = oy;
    surf.Z0(1) = oz;
    ss.SUNCOS.x = sx;
    ss.SUNCOS.y = sy;
    ss.SUNCOS.z = sz;

    SolarShading::SHADOW(state, 1, 1);

    switch (which) {
    case 0:
        return ss.SurfSunlitArea(1);
    case 1:
        return ss.OverlapStatus;
    case 2:
        return ss.LOCHCA;
    case 3:
        return ss.NGSSHC;
    case 4:
        return ss.FGSSHC;
    case 5:
        return ss.HCAREA(1);
    default:
        break;
    }
    if (which >= 6 && which < 27) {
        int const k = 2 + (which - 6) / 3;
        switch ((which - 6) % 3) {
        case 0:
            return ss.HCNV(k);
        case 1:
            return ss.HCAREA(k);
        default:
            return ss.HCT(k);
        }
    }
    if (which >= 27 && which < 27 + 30 * 7) {
        int const k = 2 + (which - 27) / 30;
        int const j = (which - 27) % 30;
        int const nvk = ss.HCNV(k);
        if (j < 15 && j < nvk) {
            return static_cast<double>(ss.HCX(k, j + 1));
        }
        if (j >= 15 && j - 15 < nvk) {
            return static_cast<double>(ss.HCY(k, j - 15 + 1));
        }
    }
    return NAN;
}
}
