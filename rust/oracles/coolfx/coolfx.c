// The coolfx unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ThermalComfort CalcCoolingEffectAdjustedPMV through
// libep_coolfx_shim (shim/ep_coolfx_shim.cc), (see the shim).
#include "coolfx.h"

double ep_CoolingEffect(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work);

double CoolingEffect(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work)
{
    return ep_CoolingEffect(which, outbaro, at, rt, rh, vel, act, clo, work);
}

