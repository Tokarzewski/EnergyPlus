// The waterhc unit as rust-anything sees it: the function forwards to the real EnergyPlus
// WaterCoils CalcSimpleHeatingCoil through
// libep_waterhc_shim (shim/ep_waterhc_shim.cc), (see the shim).
#include "waterhc.h"

double ep_WaterHeatCoil(int which, int cycling, int design, double ua, double avail, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr);

double WaterHeatCoil(int which, int cycling, int design, double ua, double avail, double mdota, double tain, double win, double mdotw, double mdotwmax, double twin, double hwin, double plr)
{
    return ep_WaterHeatCoil(which, cycling, design, ua, avail, mdota, tain, win, mdotw, mdotwmax, twin, hwin, plr);
}

