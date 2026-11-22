// The glazing unit as rust-anything sees it: each function forwards to the real EnergyPlus
// WindowManager code through libep_glazing_shim (shim/ep_glazing_shim.cc), which
// builds the arrays from plain numbers (see the shim).
#include "glazing.h"

double ep_GlassPhi(int which, double cs, double tf0, double rf0, double rb0, int simple, double shgc, double u);
double ep_GlassFit(int which, double tf0, double rf0, double rb0);
double ep_SlatViewFac(int i, int j, double s, double h, double phib, double phis);
double ep_MatInverse(int which, int n, double d, double k, double o, double p);
double ep_Interp(double xin, int npts, double x0, double x1, double x2, double x3, double x4, double y0, double y1, double y2, double y3, double y4);

double GlassPhi(int which, double cs, double tf0, double rf0, double rb0, int simple, double shgc, double u)
{
    return ep_GlassPhi(which, cs, tf0, rf0, rb0, simple, shgc, u);
}

double GlassFit(int which, double tf0, double rf0, double rb0)
{
    return ep_GlassFit(which, tf0, rf0, rb0);
}

double SlatViewFac(int i, int j, double s, double h, double phib, double phis)
{
    return ep_SlatViewFac(i, j, s, h, phib, phis);
}

double MatInverse(int which, int n, double d, double k, double o, double p)
{
    return ep_MatInverse(which, n, d, k, o, p);
}

double Interp(double xin, int npts, double x0, double x1, double x2, double x3, double x4, double y0, double y1, double y2, double y3, double y4)
{
    return ep_Interp(xin, npts, x0, x1, x2, x3, x4, y0, y1, y2, y3, y4);
}

