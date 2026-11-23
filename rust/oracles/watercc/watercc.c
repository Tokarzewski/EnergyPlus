// The watercc unit as rust-anything sees it: the function forwards to the real EnergyPlus
// WaterCoils CoolingCoil through
// libep_watercc_shim (shim/ep_watercc_shim.cc), (see the shim).
#include "watercc.h"

double ep_WaterCoolCoil(int which, int cycling, int design, int hex, double avail, double uatot, double uaint, double uaext, double desvol, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr, double pb);

double WaterCoolCoil(int which, int cycling, int design, int hex, double avail, double uatot, double uaint, double uaext, double desvol, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr, double pb)
{
    return ep_WaterCoolCoil(which, cycling, design, hex, avail, uatot, uaint, uaext, desvol, mdota, tain, win, mdotw, mdotwmax, twin, hwin, plr, pb);
}

