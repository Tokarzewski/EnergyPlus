// Written by hand: the Vectors functions take Array1D<Vector> polygons and write through
// reference arguments, which gen_plain.py cannot build. Plain extern "C" entry points over the
// real EnergyPlus Vectors.cc (with Geometry/Plane and Vector3D). A polygon is its first n of five
// given vertices; a function with several results returns the one `which` picks.
//
// The domain (ADR-009): 3 to 5 vertices (AreaPolygon and DetermineAzimuthAndTilt read three,
// Plane::fromVertices asserts on fewer); VecRound's scaled values must fit an int64 (nint64 casts).
#include <cmath>
#include <ObjexxFCL/Array1D.hh>
#include <EnergyPlus/DataVectorTypes.hh>
#include <EnergyPlus/Vectors.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::Vectors;
using DataVectorTypes::Vector;

namespace {
#define POLY_PARAMS                                                                                                                          \
    int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4,         \
        double z4, double x5, double y5, double z5
#define POLY_ARGS n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5

bool poly(Array1D<Vector> &p, POLY_PARAMS)
{
    if (n < 3 || n > 5) {
        return false;
    }
    Vector const all[5] = {{x1, y1, z1}, {x2, y2, z2}, {x3, y3, z3}, {x4, y4, z4}, {x5, y5, z5}};
    p.allocate(n);
    for (int i = 0; i < n; ++i) {
        p(i + 1) = all[i];
    }
    return true;
}

double component(Vector const &v, int which)
{
    return which == 0 ? v.x : which == 1 ? v.y : which == 2 ? v.z : NAN;
}

// nint64 of v * roundto is defined when it fits an int64.
bool round_ok(double v, double roundto)
{
    double const s = v * roundto;
    double const r = s + (s >= 0.0 ? 1 : -1) * 0.5;
    return r > -9223372036854775808.0 && r < 9223372036854775808.0;
}
} // namespace

extern "C" {
double ep_VAreaPolygon(POLY_PARAMS)
{
    Array1D<Vector> p;
    return poly(p, POLY_ARGS) ? AreaPolygon(n, p) : NAN;
}

// CreateNewellAreaVector's component `which`.
double ep_VNewellArea(int which, POLY_PARAMS)
{
    Array1D<Vector> p;
    if (!poly(p, POLY_ARGS)) {
        return NAN;
    }
    Vector v;
    CreateNewellAreaVector(p, n, v);
    return component(v, which);
}

// CreateNewellSurfaceNormalVector's component `which`.
double ep_VNewellNormal(int which, POLY_PARAMS)
{
    Array1D<Vector> p;
    if (!poly(p, POLY_ARGS)) {
        return NAN;
    }
    Vector v;
    CreateNewellSurfaceNormalVector(p, n, v);
    return component(v, which);
}

// DetermineAzimuthAndTilt with the polygon's Newell normal, as SurfaceGeometry calls it: which 0
// azimuth, 1 tilt, 2 to 10 the components of lcsx, lcsy and lcsz.
double ep_VAzimuthTilt(int which, POLY_PARAMS)
{
    Array1D<Vector> p;
    if (!poly(p, POLY_ARGS) || which < 0 || which > 10) {
        return NAN;
    }
    Vector normal;
    CreateNewellSurfaceNormalVector(p, n, normal);
    Real64 az = 0.0;
    Real64 tilt = 0.0;
    Vector lcsx, lcsy, lcsz;
    DetermineAzimuthAndTilt(p, az, tilt, lcsx, lcsy, lcsz, normal);
    if (which == 0) {
        return az;
    }
    if (which == 1) {
        return tilt;
    }
    Vector const &v = which < 5 ? lcsx : which < 8 ? lcsy : lcsz;
    return component(v, (which - 2) % 3);
}

// CalcCoPlanarNess: which 0 IsCoPlanar (1 or 0), 1 MaxDist, 2 ErrorVertex.
double ep_VCoPlanar(int which, POLY_PARAMS)
{
    Array1D<Vector> p;
    if (!poly(p, POLY_ARGS) || which < 0 || which > 2) {
        return NAN;
    }
    bool coPlanar = false;
    Real64 maxDist = 0.0;
    int errorVertex = 0;
    CalcCoPlanarNess(p, n, coPlanar, maxDist, errorVertex);
    return which == 0 ? (coPlanar ? 1.0 : 0.0) : which == 1 ? maxDist : errorVertex;
}

// PointsInPlane of all five vertices against the plane of the first n: which -2 ErrorFound (1 or
// 0), -1 the number of vertices in the plane, k >= 0 the k-th of them (1-based), 0 past the end.
int ep_VPointsInPlane(int which, POLY_PARAMS)
{
    Array1D<Vector> base;
    Array1D<Vector> query;
    if (!poly(base, POLY_ARGS) || !poly(query, 5, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5)) {
        return -1;
    }
    bool error = false;
    std::vector<int> const in = PointsInPlane(base, n, query, 5, error);
    if (which == -2) {
        return error ? 1 : 0;
    }
    if (which == -1) {
        return static_cast<int>(in.size());
    }
    return which >= 0 && which < static_cast<int>(in.size()) ? in[which] : 0;
}

// VecRound's component `which`.
double ep_VRound(int which, double x, double y, double z, double roundto)
{
    if (!round_ok(x, roundto) || !round_ok(y, roundto) || !round_ok(z, roundto)) {
        return NAN;
    }
    Vector v(x, y, z);
    VecRound(v, roundto);
    return component(v, which);
}

// VecNormalize's component `which`, or VecLength (which 3) or VecSquaredLength (4).
double ep_VNormalize(int which, double x, double y, double z)
{
    Vector const v(x, y, z);
    if (which == 3) {
        return VecLength(v);
    }
    if (which == 4) {
        return VecSquaredLength(v);
    }
    return component(VecNormalize(v), which);
}
}
