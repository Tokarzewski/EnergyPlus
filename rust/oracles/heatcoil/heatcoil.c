// The heatcoil unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatingCoils CalcElectricHeatingCoil and CalcFuelHeatingCoil through
// libep_heatcoil_shim (shim/ep_heatcoil_shim.cc), (see the shim).
#include "heatcoil.h"

double ep_HeatCoil(int which, int fuel, int cycling, int sensed, int plf, double avail, double mdot, double tin, double win, double tset, double qreq, double plr, double eff, double cap, double parelec, double parfuel, double c0, double c1, double c2);

double HeatCoil(int which, int fuel, int cycling, int sensed, int plf, double avail, double mdot, double tin, double win, double tset, double qreq, double plr, double eff, double cap, double parelec, double parfuel, double c0, double c1, double c2)
{
    return ep_HeatCoil(which, fuel, cycling, sensed, plf, avail, mdot, tin, win, tset, qreq, plr, eff, cap, parelec, parfuel, c0, c1, c2);
}

