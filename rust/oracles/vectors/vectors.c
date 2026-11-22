// The vectors unit as rust-anything sees it: each function forwards to the real EnergyPlus
// Vectors.cc functions through libep_vectors_shim (shim/ep_vectors_shim.cc), which
// builds each polygon from its first n of five vertices (see the shim).
#include "vectors.h"

double ep_VAreaPolygon(int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double ep_VNewellArea(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double ep_VNewellNormal(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double ep_VAzimuthTilt(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double ep_VCoPlanar(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
int ep_VPointsInPlane(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double ep_VRound(int which, double x, double y, double z, double roundto);
double ep_VNormalize(int which, double x, double y, double z);

double VAreaPolygon(int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)
{
    return ep_VAreaPolygon(n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5);
}

double VNewellArea(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)
{
    return ep_VNewellArea(which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5);
}

double VNewellNormal(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)
{
    return ep_VNewellNormal(which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5);
}

double VAzimuthTilt(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)
{
    return ep_VAzimuthTilt(which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5);
}

double VCoPlanar(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)
{
    return ep_VCoPlanar(which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5);
}

int VPointsInPlane(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5)
{
    return ep_VPointsInPlane(which, n, x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5);
}

double VRound(int which, double x, double y, double z, double roundto)
{
    return ep_VRound(which, x, y, z, roundto);
}

double VNormalize(int which, double x, double y, double z)
{
    return ep_VNormalize(which, x, y, z);
}

