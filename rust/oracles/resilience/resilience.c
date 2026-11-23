// The resilience unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceSurfaceManager CalcThermalResilience through
// libep_resilience_shim (shim/ep_resilience_shim.cc), which sets up one zone (see the shim).
#include "resilience.h"

double ep_ThermalResilience(int which, int method, double ztav, double airhumratavg, double outbaro);

double ThermalResilience(int which, int method, double ztav, double airhumratavg, double outbaro)
{
    return ep_ThermalResilience(which, method, ztav, airhumratavg, outbaro);
}

