// The fanger unit as rust-anything sees it: each function forwards to the real EnergyPlus
// ThermalComfort CalcFangerPMV or CalcStandardEffectiveTemp through
// libep_fanger_shim (shim/ep_fanger_shim.cc), (see the shim).
#include "fanger.h"

double ep_FangerPMV(int which, double at, double rt, double rh, double vel, double act, double clo, double work);
double ep_StandardEffectiveTemp(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work);

double FangerPMV(int which, double at, double rt, double rh, double vel, double act, double clo, double work)
{
    return ep_FangerPMV(which, at, rt, rh, vel, act, clo, work);
}

double StandardEffectiveTemp(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work)
{
    return ep_StandardEffectiveTemp(which, outbaro, at, rt, rh, vel, act, clo, work);
}

