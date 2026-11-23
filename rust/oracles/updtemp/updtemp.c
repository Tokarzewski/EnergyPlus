// The updtemp unit as rust-anything sees it: each function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector updateTemperatures or DownInterpolate4HistoryValues through
// libep_updtemp_shim (shim/ep_updtemp_shim.cc), which sets up one fully mixed zone (see the shim).
#include "updtemp.h"

double ep_UpdateTemperatures(int which, int shorten, int usezonehist, double prior, double tsys, int nsys, int nsyslast, int hasnode, double mat, double w, double x0, double x1, double x2, double x3, double ds0, double ds1, double ds2, double ds3, double wp0, double wp1, double wp2, double wp3, double dsw0, double dsw1, double dsw2, double dsw3);
double ep_DownInterpolate5(int which, double old, double nw, double v0, double v1, double v2);

double UpdateTemperatures(int which, int shorten, int usezonehist, double prior, double tsys, int nsys, int nsyslast, int hasnode, double mat, double w, double x0, double x1, double x2, double x3, double ds0, double ds1, double ds2, double ds3, double wp0, double wp1, double wp2, double wp3, double dsw0, double dsw1, double dsw2, double dsw3)
{
    return ep_UpdateTemperatures(which, shorten, usezonehist, prior, tsys, nsys, nsyslast, hasnode, mat, w, x0, x1, x2, x3, ds0, ds1, ds2, ds3, wp0, wp1, wp2, wp3, dsw0, dsw1, dsw2, dsw3);
}

double DownInterpolate5(int which, double old, double nw, double v0, double v1, double v2)
{
    return ep_DownInterpolate5(which, old, nw, v0, v1, v2);
}

