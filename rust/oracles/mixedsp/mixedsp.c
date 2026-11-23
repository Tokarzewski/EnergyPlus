// The mixedsp unit as rust-anything sees it: the function forwards to the real EnergyPlus
// SetPointManager SPMMixedAir calculate through
// libep_mixedsp_shim (shim/ep_mixedsp_shim.cc), (see the shim).
#include "mixedsp.h"

double ep_MixedSP(int which, int layout, double refsp, double reft, double fanin, double fanout, double coilin, double coilout, double mincoil, double oat);

double MixedSP(int which, int layout, double refsp, double reft, double fanin, double fanout, double coilin, double coilout, double mincoil, double oat)
{
    return ep_MixedSP(which, layout, refsp, reft, fanin, fanout, coilin, coilout, mincoil, oat);
}

