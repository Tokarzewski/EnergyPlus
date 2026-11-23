// The meanair unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceAirManager calcMeanAirTemps through
// libep_meanair_shim (shim/ep_meanair_shim.cc), which sets up one zone (see the shim).
#include "meanair.h"

double ep_MeanAirTemps(int which, double ztav, double w, double mrt, double outbaro, int anyop, int controlled, int opctrl, double frac, int wbgt);

double MeanAirTemps(int which, double ztav, double w, double mrt, double outbaro, int anyop, int controlled, int opctrl, double frac, int wbgt)
{
    return ep_MeanAirTemps(which, ztav, w, mrt, outbaro, anyop, controlled, opctrl, frac, wbgt);
}

