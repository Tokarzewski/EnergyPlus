// The warmflow unit as rust-anything sees it: each function forwards to the real EnergyPlus
// SetPointManager warmest temperature flow and return air bypass managers through
// libep_warmflow_shim (shim/ep_warmflow_shim.cc), (see the shim).
#include "warmflow.h"

double ep_WarmFlow(int which, int flowfirst, int nc, double mint, double maxt, double minturn, double load1, double zt1, double mmax1, double w1, double load2, double zt2, double mmax2, double w2, double load3, double zt3, double mmax3, double w3, double load4, double zt4, double mmax4, double w4);
double ep_RABypass(double spt, double tloop, double tmix, double msup, double tsup, double mtot, double trab);

double WarmFlow(int which, int flowfirst, int nc, double mint, double maxt, double minturn, double load1, double zt1, double mmax1, double w1, double load2, double zt2, double mmax2, double w2, double load3, double zt3, double mmax3, double w3, double load4, double zt4, double mmax4, double w4)
{
    return ep_WarmFlow(which, flowfirst, nc, mint, maxt, minturn, load1, zt1, mmax1, w1, load2, zt2, mmax2, w2, load3, zt3, mmax3, w3, load4, zt4, mmax4, w4);
}

double RABypass(double spt, double tloop, double tmix, double msup, double tsup, double mtot, double trab)
{
    return ep_RABypass(spt, tloop, tmix, msup, tsup, mtot, trab);
}

