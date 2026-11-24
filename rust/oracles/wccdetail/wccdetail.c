// The wccdetail unit as rust-anything sees it: the function forwards to the real EnergyPlus
// WaterCoils CoolingCoil (detailed analysis) through
// libep_wccdetail_shim (shim/ep_wccdetail_shim.cc), (see the shim).
#include "wccdetail.h"

double ep_WaterCoolDetail(int which, int cycling, int design, int hex, int first, double avail, double uaint, double uaext, double area, double saved, double desvol, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr, double pb);

double WaterCoolDetail(int which, int cycling, int design, int hex, int first, double avail, double uaint, double uaext, double area, double saved, double desvol, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr, double pb)
{
    return ep_WaterCoolDetail(which, cycling, design, hex, first, avail, uaint, uaext, area, saved, desvol, mdota, tain, win, mdotw, mdotwmax, twin, hwin, plr, pb);
}

