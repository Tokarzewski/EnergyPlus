// The ksu unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ThermalComfort CalcThermalComfortKSU through
// libep_ksu_shim (shim/ep_ksu_shim.cc), (see the shim).
#include "ksu.h"

double ep_KsuTsv(int which, double outbaro, double ztav, double w, double mrt, double qrad, double act, double eff, double clo, double vel);

double KsuTsv(int which, double outbaro, double ztav, double w, double mrt, double qrad, double act, double eff, double clo, double vel)
{
    return ep_KsuTsv(which, outbaro, ztav, w, mrt, qrad, act, eff, clo, vel);
}

