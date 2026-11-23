// The zonehist unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector zone air history pushes and DownInterpolate4HistoryValues through
// libep_zonehist_shim (shim/ep_zonehist_shim.cc), which sets up one zone (see the shim).
#include "zonehist.h"

double ep_ZoneHistory(int which, int op, int algo, double x0, double x1, double x2, double x3, double w0, double w1, double w2, double w3, double dx0, double dx1, double dx2, double dx3, double dw0, double dw1, double dw2, double dw3, double ztav, double zt, double mat, double airhumrat, double airhumratavg, double airhumrattemp, double airrelhum, double xmpt, double wtimeminusp, double tm2, double tmx, double wm2, double wmx, double outbaro, double oldts, double newts);

double ZoneHistory(int which, int op, int algo, double x0, double x1, double x2, double x3, double w0, double w1, double w2, double w3, double dx0, double dx1, double dx2, double dx3, double dw0, double dw1, double dw2, double dw3, double ztav, double zt, double mat, double airhumrat, double airhumratavg, double airhumrattemp, double airrelhum, double xmpt, double wtimeminusp, double tm2, double tmx, double wm2, double wmx, double outbaro, double oldts, double newts)
{
    return ep_ZoneHistory(which, op, algo, x0, x1, x2, x3, w0, w1, w2, w3, dx0, dx1, dx2, dx3, dw0, dw1, dw2, dw3, ztav, zt, mat, airhumrat, airhumratavg, airhumrattemp, airrelhum, xmpt, wtimeminusp, tm2, tmx, wm2, wmx, outbaro, oldts, newts);
}

