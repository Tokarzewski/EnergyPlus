// The polyclip unit as rust-anything sees it: the function forwards to the real EnergyPlus
// SolarShading polygon code through libep_polyclip_shim (shim/ep_polyclip_shim.cc), which
// loads the two polygons into the homogeneous-coordinate figure arrays (see the shim).
#include "polyclip.h"

double ep_PolyOverlap(int which, int nv1, double ax1, double ay1, double ax2, double ay2, double ax3, double ay3, double ax4, double ay4, int nv2, double bx1, double by1, double bx2, double by2, double bx3, double by3, double bx4, double by4, double t1, double t2);

double PolyOverlap(int which, int nv1, double ax1, double ay1, double ax2, double ay2, double ax3, double ay3, double ax4, double ay4, int nv2, double bx1, double by1, double bx2, double by2, double bx3, double by3, double bx4, double by4, double t1, double t2)
{
    return ep_PolyOverlap(which, nv1, ax1, ay1, ax2, ay2, ax3, ay3, ax4, ay4, nv2, bx1, by1, bx2, by2, bx3, by3, bx4, by4, t1, t2);
}

