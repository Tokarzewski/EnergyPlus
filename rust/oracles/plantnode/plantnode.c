// The plantnode unit as rust-anything sees it: each function forwards to the real EnergyPlus
// PlantUtilities node helpers through
// libep_plantnode_shim (shim/ep_plantnode_shim.cc), (see the shim).
#include "plantnode.h"

double ep_InitNodes(int which, double minm, double maxm);
double ep_SafeCopy(int which, int lp, double t, double m, double q, double h, double tmin, double tmax, double mmin, double mminav, double mmax, double mmaxav, double w, double p, double op);
double ep_BoundNode(double v, double minav, double maxav);
double ep_Tighten(int which, double oldmin, double oldmax, double newmin, double newmax);
double ep_BoundTwo(double v, double lo, double hi);

double InitNodes(int which, double minm, double maxm)
{
    return ep_InitNodes(which, minm, maxm);
}

double SafeCopy(int which, int lp, double t, double m, double q, double h, double tmin, double tmax, double mmin, double mminav, double mmax, double mmaxav, double w, double p, double op)
{
    return ep_SafeCopy(which, lp, t, m, q, h, tmin, tmax, mmin, mminav, mmax, mmaxav, w, p, op);
}

double BoundNode(double v, double minav, double maxav)
{
    return ep_BoundNode(v, minav, maxav);
}

double Tighten(int which, double oldmin, double oldmax, double newmin, double newmax)
{
    return ep_Tighten(which, oldmin, oldmax, newmin, newmax);
}

double BoundTwo(double v, double lo, double hi)
{
    return ep_BoundTwo(v, lo, hi);
}

