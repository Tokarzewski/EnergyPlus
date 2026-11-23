// The tempest unit as rust-anything sees it: the function forwards to the real EnergyPlus
// SetPointManager SPMTempest calculate through
// libep_tempest_shim (shim/ep_tempest_shim.cc), (see the shim).
#include "tempest.h"

double ep_Tempest(int warmest, int nc, int nh, double mint, double maxt, double load1, double zt1, double mmax1, double w1, double load2, double zt2, double mmax2, double w2, double load3, double zt3, double mmax3, double w3, double load4, double zt4, double mmax4, double w4);

double Tempest(int warmest, int nc, int nh, double mint, double maxt, double load1, double zt1, double mmax1, double w1, double load2, double zt2, double mmax2, double w2, double load3, double zt3, double mmax3, double w3, double load4, double zt4, double mmax4, double w4)
{
    return ep_Tempest(warmest, nc, nh, mint, maxt, load1, zt1, mmax1, w1, load2, zt2, mmax2, w2, load3, zt3, mmax3, w3, load4, zt4, mmax4, w4);
}

