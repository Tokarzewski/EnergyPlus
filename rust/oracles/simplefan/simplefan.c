// The simplefan unit as rust-anything sees it: the function forwards to the real EnergyPlus
// Fans FanComponent simulateConstant, simulateVAV and simulateOnOff through
// libep_simplefan_shim (shim/ep_simplefan_shim.cc), (see the shim).
#include "simplefan.h"

double ep_SimpleFan(int which, int kind, int turn, int ems, double avail, double mdot, double mdotmax, double mdotmin, double volmax, double volmin, double dp, double eff, double moteff, double motinair, double rho, double tin, double win, double plf, double c0, double c1, double c2, double c3, double c4, double pr, double emsflow, double emspress, double emseff);

double SimpleFan(int which, int kind, int turn, int ems, double avail, double mdot, double mdotmax, double mdotmin, double volmax, double volmin, double dp, double eff, double moteff, double motinair, double rho, double tin, double win, double plf, double c0, double c1, double c2, double c3, double c4, double pr, double emsflow, double emspress, double emseff)
{
    return ep_SimpleFan(which, kind, turn, ems, avail, mdot, mdotmax, mdotmin, volmax, volmin, dp, eff, moteff, motinair, rho, tin, win, plf, c0, c1, c2, c3, c4, pr, emsflow, emspress, emseff);
}

