// Written by hand: DeterminePolygonOverlap works on the homogeneous-coordinate figure arrays in
// state.dataSolarShading, which gen_plain.py cannot set up. Plain extern "C" entry point over the
// real EnergyPlus SolarShading code.
//
// Each call loads two polygons of 3 or 4 vertices into figures 1 and 2 with HTRANS1, gives them
// transmittances t1 and t2, and overlaps figure 2 on figure 1 into figure 3 with
// DeterminePolygonOverlap under one of the three ShadowCalculation polygon clipping algorithms:
// algo 0 SutherlandHodgman (CLIPPOLY), 1 SlaterBarskyandSutherlandHodgman (CLIPPOLY, which hands
// an axis-aligned rectangle clip to CLIPRECT and CLIPLINE) and 2 ConvexWeilerAtherton (INCLOS,
// INTCPT and ORDER). The figure and scratch arrays are sized and zeroed on every call as
// AllocateModuleArrays would size them for surfaces of up to 4 vertices; SLOPE, which ORDER sizes
// to 10 on first use, is sized to 16 here so an overlap of 11 to 14 vertices stays in bounds.
// Guards (ADR-009), which return NaN: algo outside 0 to 2; vertex counts other than 3 or 4;
// coordinates that are not finite or lie past 1000 m (whose homogeneous products would overflow
// Int64), or past 10 m for ConvexWeilerAtherton, whose INTCPT multiplies a side's constant term
// by another side's coefficient in Int64. One EnergyPlusData for the process, built when the
// library loads, with console output off.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
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
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

void reset(EnergyPlusData &state, int const algo)
{
    auto &ss = *state.dataSolarShading;
    int const maxV = state.dataSurface->MaxVerticesPerSurface = 4;
    state.dataSysVars->SutherlandHodgman = algo != 2;
    state.dataSysVars->SlaterBarsky = algo == 1;
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
    ss.XVS.dimension(maxV + 1, 0.0);
    ss.YVS.dimension(maxV + 1, 0.0);
    ss.SLOPE.dimension(16, 0.0);
    ss.ORDERFirstTimeFlag = false;
    ss.OverlapStatus = 0;
}
} // namespace

extern "C" {
// Overlap, under clipping algorithm algo, of polygon 2 (nv2 vertices b1..b4) on polygon 1 (nv1
// vertices a1..a4), coordinates in metres, transmittances t1 and t2. which picks OverlapStatus
// (0: 1 NoOverlap, 2 FirstSurfWithinSecond, 3 SecondSurfWithinFirst, 4 PartialOverlap, 5
// TooManyVertices), the overlap's vertex count (1), area (2) and transmittance (3), the areas of
// figures 1 (4) and 2 (5), or the overlap's homogeneous X (6 + k) and Y (21 + k) of vertex k,
// k < the vertex count.
double ep_ClipOverlap(int which,
                     int algo,
                     int nv1,
                     double ax1,
                     double ay1,
                     double ax2,
                     double ay2,
                     double ax3,
                     double ay3,
                     double ax4,
                     double ay4,
                     int nv2,
                     double bx1,
                     double by1,
                     double bx2,
                     double by2,
                     double bx3,
                     double by3,
                     double bx4,
                     double by4,
                     double t1,
                     double t2)
{
    double const a[8] = {ax1, ay1, ax2, ay2, ax3, ay3, ax4, ay4};
    double const b[8] = {bx1, by1, bx2, by2, bx3, by3, bx4, by4};
    if (algo < 0 || algo > 2 || nv1 < 3 || nv1 > 4 || nv2 < 3 || nv2 > 4) {
        return NAN;
    }
    double const lim = algo == 2 ? 10.0 : 1000.0;
    for (int i = 0; i < 8; ++i) {
        if (!(std::abs(a[i]) <= lim) || !(std::abs(b[i]) <= lim)) {
            return NAN;
        }
    }
    EnergyPlusData &state = st();
    reset(state, algo);
    auto &ss = *state.dataSolarShading;
    for (int n = 1; n <= nv1; ++n) {
        ss.XVS(n) = a[2 * n - 2];
        ss.YVS(n) = a[2 * n - 1];
    }
    SolarShading::HTRANS1(state, 1, nv1);
    for (int n = 1; n <= nv2; ++n) {
        ss.XVS(n) = b[2 * n - 2];
        ss.YVS(n) = b[2 * n - 1];
    }
    SolarShading::HTRANS1(state, 2, nv2);
    ss.HCT(1) = t1;
    ss.HCT(2) = t2;
    SolarShading::DeterminePolygonOverlap(state, 1, 2, 3);
    int const nv3 = ss.HCNV(3);
    switch (which) {
    case 0:
        return ss.OverlapStatus;
    case 1:
        return nv3;
    case 2:
        return ss.HCAREA(3);
    case 3:
        return ss.HCT(3);
    case 4:
        return ss.HCAREA(1);
    case 5:
        return ss.HCAREA(2);
    default:
        break;
    }
    if (which >= 6 && which < 6 + 15 && which - 6 < nv3) {
        return static_cast<double>(ss.HCX(3, which - 6 + 1));
    }
    if (which >= 21 && which < 21 + 15 && which - 21 < nv3) {
        return static_cast<double>(ss.HCY(3, which - 21 + 1));
    }
    return NAN;
}
}
