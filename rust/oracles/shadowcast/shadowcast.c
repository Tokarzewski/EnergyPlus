// The shadowcast unit as rust-anything sees it: the function forwards to the real EnergyPlus
// SolarShading SHADOW through libep_shadowcast_shim (shim/ep_shadowcast_shim.cc), which sets
// up the receiving surface, its one shadowing surface and the sun (see the shim).
#include "shadowcast.h"

double ep_ShadowCast(int which, int algo, int nvr, double rx1, double ry1, double rz1, double rx2, double ry2, double rz2, double rx3, double ry3, double rz3, double rx4, double ry4, double rz4, int nvc, double cx1, double cy1, double cz1, double cx2, double cy2, double cz2, double cx3, double cy3, double cz3, double cx4, double cy4, double cz4, double ox, double oy, double oz, double lxx, double lxy, double lxz, double lyx, double lyy, double lyz, double lzx, double lzy, double lzz, double sx, double sy, double sz, double sched, double area);

double ShadowCast(int which, int algo, int nvr, double rx1, double ry1, double rz1, double rx2, double ry2, double rz2, double rx3, double ry3, double rz3, double rx4, double ry4, double rz4, int nvc, double cx1, double cy1, double cz1, double cx2, double cy2, double cz2, double cx3, double cy3, double cz3, double cx4, double cy4, double cz4, double ox, double oy, double oz, double lxx, double lxy, double lxz, double lyx, double lyy, double lyz, double lzx, double lzy, double lzz, double sx, double sy, double sz, double sched, double area)
{
    return ep_ShadowCast(which, algo, nvr, rx1, ry1, rz1, rx2, ry2, rz2, rx3, ry3, rz3, rx4, ry4, rz4, nvc, cx1, cy1, cz1, cx2, cy2, cz2, cx3, cy3, cz3, cx4, cy4, cz4, ox, oy, oz, lxx, lxy, lxz, lyx, lyy, lyz, lzx, lzy, lzz, sx, sy, sz, sched, area);
}

