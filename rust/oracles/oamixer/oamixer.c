// The oamixer unit as rust-anything sees it: the function forwards to the real EnergyPlus
// MixedAir CalcOAMixer through
// libep_oamixer_shim (shim/ep_oamixer_shim.cc), (see the shim).
#include "oamixer.h"

double ep_OAMixer(int which, double rett, double retw, double reth, double retp, double retm, double oat, double oaw, double oah, double oap, double oam, double relm);

double OAMixer(int which, double rett, double retw, double reth, double retp, double retm, double oat, double oaw, double oah, double oap, double oam, double relm)
{
    return ep_OAMixer(which, rett, retw, reth, retp, retm, oat, oaw, oah, oap, oam, relm);
}

