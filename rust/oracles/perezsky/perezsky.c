// The perezsky unit as rust-anything sees it: each function forwards to the real EnergyPlus
// SolarShading code through libep_perezsky_shim (shim/ep_perezsky_shim.cc), which
// sets up one exterior surface from plain numbers (see the shim).
#include "perezsky.h"

double ep_PerezSky(int which, int detailed, double sx, double sy, double sz, double elev, double beam, double dif, double nx, double ny, double nz, double vfsky, double tilt, double iso, double sunlit, double horiz);
double ep_ClipLine(int which, double x0, double x1, double y0, double y1, double maxX, double minX, double maxY, double minY);

double PerezSky(int which, int detailed, double sx, double sy, double sz, double elev, double beam, double dif, double nx, double ny, double nz, double vfsky, double tilt, double iso, double sunlit, double horiz)
{
    return ep_PerezSky(which, detailed, sx, sy, sz, elev, beam, dif, nx, ny, nz, vfsky, tilt, iso, sunlit, horiz);
}

double ClipLine(int which, double x0, double x1, double y0, double y1, double maxX, double minX, double maxY, double minY)
{
    return ep_ClipLine(which, x0, x1, y0, y1, maxX, minX, maxY, minY);
}

