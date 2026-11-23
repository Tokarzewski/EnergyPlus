// The zonemrt unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceSurfaceManager CalculateZoneMRT through libep_zonemrt_shim
// (shim/ep_zonemrt_shim.cc), which sets up one zone of two spaces and their surfaces (see the shim).
#include "zonemrt.h"

double ep_ZoneMRT(int which, int n1, int n2, double a1, double a2, double a3, double a4, double a5, double a6, double e1, double e2, double e3, double e4, double e5, double e6, double t1, double t2, double t3, double t4, double t5, double t6, double mat, double mat1, double mat2, double vol1, double vol2);

double ZoneMRT(int which, int n1, int n2, double a1, double a2, double a3, double a4, double a5, double a6, double e1, double e2, double e3, double e4, double e5, double e6, double t1, double t2, double t3, double t4, double t5, double t6, double mat, double mat1, double mat2, double vol1, double vol2)
{
    return ep_ZoneMRT(which, n1, n2, a1, a2, a3, a4, a5, a6, e1, e2, e3, e4, e5, e6, t1, t2, t3, t4, t5, t6, mat, mat1, mat2, vol1, vol2);
}

