// Written by hand: SHADOW works on the surfaces, shadowing combinations, constructions and
// homogeneous-coordinate figure arrays in the EnergyPlusData state, which gen_plain.py cannot set
// up. Plain extern "C" entry point over the real EnergyPlus SolarShading code.
//
// Each call sets up a sunlit base surface 1 (receiving, nvr world vertices r1..r4, local
// coordinate system lcsx, lcsy, lcsz about origin X0, Y0, Z0, NetAreaShadowCalc area), ncast
// (1 or 2) heat transfer surfaces 2 and 3 as its general shadowing surfaces (caster i: nvi world
// vertices ci1..ci4, the cosine qi of the sun's angle on it, schedule minimum value ti) and one
// subsurface 4 on it (a window or door: nvw vertices w1..w4 in the base surface's plane, as
// ShadeV holds them, reveal depth rev, NetAreaShadowCalc warea, Area wgross, a construction of
// diffuse transmittance tdiff and glazed fraction gfrac), and nback (0 to 2) back surfaces 5
// and 6 (back surface i: nbi world vertices bi1..bi4 and the cosine pi of the sun's angle on
// it), and runs SHADOW for the sun direction s at hour 1, time step 1. After the shadows of unit
// 31 (SHDGSS with MULTOL), SHDBKS projects each back surface the sun reaches through the
// receiving surface (pi below -SunIsUpValue) onto the receiving plane, snaps it to the receiving
// vertices and overlaps it in place on the receiving figure, keeping the overlaps found with
// their surface numbers (HCNS), surface 6 a subsurface of surface 5 when b2base is 5; the
// receiving surface's ExtBoundCond is ebc; then SHDSBS takes the subsurface: its figure, MULTOL of it on the general shadows, its
// sunlit area (taken off the receiving surface's) and SHDRVL, the shading by its reveal, which
// SHADOW and SHDSBS then clamp, and for a window that transmits (tdiff above 0) with sun on it,
// CalcInteriorSolarOverlaps: the area of the window (or of its reveal's opening) that sends beam
// to each back surface, MULTOL of each back surface figure on the window's figures, kept in
// SurfWinBackSurfOverlaps with a subsurface's area taken off its base surface's. algo picks the clipping: 0 SutherlandHodgman,
// 1 SlaterBarskyandSutherlandHodgman. The figure and scratch arrays are sized and zeroed on every
// call as AllocateModuleArrays would size them for surfaces of up to 4 vertices, with MaxHCS 20:
// two casters, two back surfaces, the subsurface and its reveal need at most 16 figures, so
// TooManyFigures cannot arise.
//
// Guards (ADR-009), which return NaN: algo outside 0 or 1; ncast outside 1 or 2; nback outside
// 0 to 2; vertex counts other than 3 or 4; vertices and origin past 30 m, local axis, sun and
// cosine values past 1, schedule values outside [0, 1], for the casters and back surfaces in use;
// tdiff or gfrac outside [0, 1], ebc outside 0 or 1, b2base other than 5 or 6; subsurface vertices past 30 m, a reveal
// outside [0, 2] m and areas outside [0, 1e6] (warea above 0, which divides); and a sun so low
// over the receiving plane that a shadow projection |XS / ZS| or |YS / ZS| passes 4 (with |ZS|
// above SmallDistance), which keeps every projected coordinate within 1000 m, where the
// homogeneous products stay inside Int64. One EnergyPlusData for the process, built when the
// library loads, with console output off.
#include <cmath>
#include <EnergyPlus/Construction.hh>
#include <EnergyPlus/DataBSDFWindow.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataHeatBalance.hh>
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
        surf.TotSurfaces = 6;
        surf.Surface.allocate(6);
        for (int i = 1; i <= 6; ++i) {
            auto &s = surf.Surface(i);
            s.Vertex.dimension(4);
            s.BaseSurf = 1;
            s.IsShadowing = false;
            s.HeatTransSurf = true;
            s.IsTransparent = false;
            s.shadowSurfSched = nullptr;
            s.Zone = 1;
        }
        for (int i = 2; i <= 6; ++i) {
            surf.Surface(i).BaseSurf = i;
        }
        surf.Surface(4).Construction = 1;
        surf.Surface(4).BaseSurf = 1;
        surf.ShadeV.allocate(6);
        surf.SurfaceWindow.allocate(6);
        auto &cons = p->dataConstruction->Construct;
        cons.allocate(1);
        cons(1).TransDiff = 0.0;
        p->dataGlobal->TimeStepsInHour = 1;
        p->dataHeatBal->SurfSunlitFracWithoutReveal.dimension(24, 1, 6, 0.0);
        p->dataSolarShading->SurfWinRevealStatus.dimension(24, 1, 6, 0);
        p->dataHeatBal->SurfWinBackSurfOverlaps.allocate(24, 1, p->dataBSDFWindow->MaxBkSurf, 6);
        surf.X0.dimension(6, 0.0);
        surf.Y0.dimension(6, 0.0);
        surf.Z0.dimension(6, 0.0);
        auto &sc = p->dataShadowComb->ShadowComb;
        sc.allocate(6);
        sc(1).UseThisSurf = true;
        sc(1).NumBackSurf = 0; // set per call
        sc(1).NumSubSurf = 1;
        sc(1).SubSurf.dimension(1, 4);
        for (int i = 2; i <= 6; ++i) {
            sc(i).UseThisSurf = false;
        }
        auto &ss = *p->dataSolarShading;
        ss.SurfSunCosTheta.dimension(6, 0.0);
        ss.SurfSunCosTheta(1) = 1.0; // the receiving surface faces the sun
        ss.SurfSunlitArea.dimension(6, 0.0);
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
    ss.MaxHCS = 20;
    ss.MAXHCArrayBounds = 2 * (maxV + 1);
    ss.MAXHCArrayIncrement = maxV + 1;
    for (auto *a : {&ss.HCA, &ss.HCB, &ss.HCC, &ss.HCX, &ss.HCY}) {
        a->dimension(2 * ss.MaxHCS, ss.MaxHCV + 1, 0);
    }
    ss.HCAREA.dimension(2 * ss.MaxHCS, 0.0);
    ss.HCNV.dimension(2 * ss.MaxHCS, 0);
    ss.HCT.dimension(2 * ss.MaxHCS, 0.0);
    ss.HCNS.dimension(2 * ss.MaxHCS, 0);
    for (auto *a : {&ss.XTEMP, &ss.YTEMP, &ss.XTEMP1, &ss.YTEMP1, &ss.ATEMP, &ss.BTEMP, &ss.CTEMP}) {
        a->dimension(2 * (maxV + 1), 0.0);
    }
    for (auto *a : {&ss.XVS, &ss.YVS, &ss.XVC, &ss.YVC, &ss.ZVC}) {
        a->dimension(maxV + 1, 0.0);
    }
    // SHDSBS and SHDRVL write these only on some paths: zero them so no call sees another's.
    state.dataHeatBal->SurfSunlitFracWithoutReveal = 0.0;
    ss.SurfWinRevealStatus = 0;
    state.dataHeatBal->SurfWinBackSurfOverlaps.reset();
    ss.FINSHC = 0;
    ss.NINSHC = 0;
    ss.FBKSHC = 0;
    ss.NBKSHC = 0;
    ss.FSBSHC = 0;
    ss.NSBSHC = 0;
    ss.FRVLHC = 0;
    ss.NRVLHC = 0;
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
// Sunlit areas of receiving surface 1 and its subsurface 4 under the shadows of the ncast
// casters and the subsurface's reveal, with the back surfaces the sun reaches through the
// receiving surface (see the file comment); caster i's arguments are nvi, its vertices ci1..ci4,
// qi and ti, back surface i's nbi, its vertices bi1..bi4 and pi. which picks the final
// SurfSunlitArea of the receiving surface (0) and of the subsurface (1),
// SurfSunlitFracWithoutReveal (2) and SurfWinRevealStatus (3) of the subsurface, OverlapStatus
// (4), LOCHCA (5), NGSSHC (6), FGSSHC (7), FSBSHC (8), NSBSHC (9), FRVLHC (10), NRVLHC (11), the
// receiving figure's area (12), FBKSHC (13), NBKSHC (14), FINSHC (559), NINSHC (560), the
// back surface number and overlap area of SurfWinBackSurfOverlaps entry i (559 + 2 * i and
// 560 + 2 * i, for i = 1 to 3), the vertex count, area, transmittance
// and HCNS of figure k (15 + 4 * (k - 2) + 0, 1, 2, 3, for k = 2 to 17), or figure k's
// homogeneous X (79 + 30 * (k - 2) + j) and Y (94 + 30 * (k - 2) + j) of vertex j, j < its
// vertex count.
double ep_ShadowInterior(int which,
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
                     int nback,
                     int nb1,
                     double b1x1,
                     double b1y1,
                     double b1z1,
                     double b1x2,
                     double b1y2,
                     double b1z2,
                     double b1x3,
                     double b1y3,
                     double b1z3,
                     double b1x4,
                     double b1y4,
                     double b1z4,
                     double p1,
                     int nb2,
                     double b2x1,
                     double b2y1,
                     double b2z1,
                     double b2x2,
                     double b2y2,
                     double b2z2,
                     double b2x3,
                     double b2y3,
                     double b2z3,
                     double b2x4,
                     double b2y4,
                     double b2z4,
                     double p2,
                     int nvw,
                     double wx1,
                     double wy1,
                     double wx2,
                     double wy2,
                     double wx3,
                     double wy3,
                     double wx4,
                     double wy4,
                     double rev,
                     double warea,
                     double wgross,
                     double tdiff,
                     int ebc,
                     double gfrac,
                     int b2base,
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
    double const c[2][12] = {{c1x1, c1y1, c1z1, c1x2, c1y2, c1z2, c1x3, c1y3, c1z3, c1x4, c1y4, c1z4},
                             {c2x1, c2y1, c2z1, c2x2, c2y2, c2z2, c2x3, c2y3, c2z3, c2x4, c2y4, c2z4}};
    int const nv[2] = {nv1, nv2};
    double const q[2] = {q1, q2};
    double const t[2] = {t1, t2};
    double const b[2][12] = {{b1x1, b1y1, b1z1, b1x2, b1y2, b1z2, b1x3, b1y3, b1z3, b1x4, b1y4, b1z4},
                             {b2x1, b2y1, b2z1, b2x2, b2y2, b2z2, b2x3, b2y3, b2z3, b2x4, b2y4, b2z4}};
    int const nb[2] = {nb1, nb2};
    double const p[2] = {p1, p2};
    double const w[8] = {wx1, wy1, wx2, wy2, wx3, wy3, wx4, wy4};
    double const o[3] = {ox, oy, oz};
    double const l[12] = {lxx, lxy, lxz, lyx, lyy, lyz, lzx, lzy, lzz, sx, sy, sz};
    if (algo < 0 || algo > 1 || nvr < 3 || nvr > 4 || ncast < 1 || ncast > 2 || nback < 0 || nback > 2 || nvw < 3 || nvw > 4) {
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
    for (int k = 0; k < nback; ++k) {
        if (nb[k] < 3 || nb[k] > 4 || !within(p[k], 1.0)) {
            return NAN;
        }
        for (int i = 0; i < 12; ++i) {
            if (!within(b[k][i], 30.0)) {
                return NAN;
            }
        }
    }
    if (!within(o[0], 30.0) || !within(o[1], 30.0) || !within(o[2], 30.0) || !(area >= 0.0 && area <= 1.0e6)) {
        return NAN;
    }
    for (int i = 0; i < 8; ++i) {
        if (!within(w[i], 30.0)) {
            return NAN;
        }
    }
    if (!(rev >= 0.0 && rev <= 2.0) || !(warea > 0.0 && warea <= 1.0e6) || !(wgross >= 0.0 && wgross <= 1.0e6)) {
        return NAN;
    }
    if (!(tdiff >= 0.0 && tdiff <= 1.0) || ebc < 0 || ebc > 1 || !(gfrac >= 0.0 && gfrac <= 1.0) || b2base < 5 || b2base > 6) {
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
    sc.NumBackSurf = nback;
    sc.BackSurf.dimension(nback, 0);
    for (int k = 0; k < nback; ++k) {
        auto &back = surf.Surface(k + 5);
        back.Sides = nb[k];
        for (int n = 1; n <= nb[k]; ++n) {
            back.Vertex(n).x = b[k][3 * n - 3];
            back.Vertex(n).y = b[k][3 * n - 2];
            back.Vertex(n).z = b[k][3 * n - 1];
        }
        ss.SurfSunCosTheta(k + 5) = p[k];
        sc.BackSurf(k + 1) = k + 5;
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
    recv.ExtBoundCond = ebc;
    surf.Surface(6).BaseSurf = b2base;
    state.dataConstruction->Construct(1).TransDiff = tdiff;
    surf.SurfaceWindow(4).glazedFrac = gfrac;
    auto &win = surf.Surface(4);
    win.Sides = nvw;
    win.Reveal = rev;
    win.NetAreaShadowCalc = warea;
    win.Area = wgross;
    auto &sv = surf.ShadeV(4);
    sv.XV.dimension(nvw, 0.0);
    sv.YV.dimension(nvw, 0.0);
    sv.ZV.dimension(nvw, 0.0);
    for (int n = 1; n <= nvw; ++n) {
        sv.XV(n) = w[2 * n - 2];
        sv.YV(n) = w[2 * n - 1];
    }
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
        return ss.SurfSunlitArea(4);
    case 2:
        return state.dataHeatBal->SurfSunlitFracWithoutReveal(1, 1, 4);
    case 3:
        return ss.SurfWinRevealStatus(1, 1, 4);
    case 4:
        return ss.OverlapStatus;
    case 5:
        return ss.LOCHCA;
    case 6:
        return ss.NGSSHC;
    case 7:
        return ss.FGSSHC;
    case 8:
        return ss.FSBSHC;
    case 9:
        return ss.NSBSHC;
    case 10:
        return ss.FRVLHC;
    case 11:
        return ss.NRVLHC;
    case 12:
        return ss.HCAREA(1);
    case 13:
        return ss.FBKSHC;
    case 14:
        return ss.NBKSHC;
    case 559:
        return ss.FINSHC;
    case 560:
        return ss.NINSHC;
    case 561:
    case 563:
    case 565:
        return state.dataHeatBal->SurfWinBackSurfOverlaps(1, 1, (which - 559) / 2, 4).backSurfNum;
    case 562:
    case 564:
    case 566:
        return state.dataHeatBal->SurfWinBackSurfOverlaps(1, 1, (which - 560) / 2, 4).overlapArea;
    default:
        break;
    }
    if (which >= 15 && which < 79) {
        int const k = 2 + (which - 15) / 4;
        switch ((which - 15) % 4) {
        case 0:
            return ss.HCNV(k);
        case 1:
            return ss.HCAREA(k);
        case 2:
            return ss.HCT(k);
        default:
            return ss.HCNS(k);
        }
    }
    if (which >= 79 && which < 79 + 30 * 16) {
        int const k = 2 + (which - 79) / 30;
        int const j = (which - 79) % 30;
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
