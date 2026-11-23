// The spmreturn unit as rust-anything sees it: each function forwards to the real EnergyPlus
// SetPointManager return water and system node managers through
// libep_spmreturn_shim (shim/ep_spmreturn_shim.cc), (see the shim).
#include "spmreturn.h"

double ep_RetWater(int hot, int ttype, double mdot, double tsup, double tret, double target, double retsp, double mint, double maxt);
double ep_SysNode(int ctrl, double ref, double lowref, double highref, double splow, double sphigh);

double RetWater(int hot, int ttype, double mdot, double tsup, double tret, double target, double retsp, double mint, double maxt)
{
    return ep_RetWater(hot, ttype, mdot, tsup, tret, target, retsp, mint, maxt);
}

double SysNode(int ctrl, double ref, double lowref, double highref, double splow, double sphigh)
{
    return ep_SysNode(ctrl, ref, lowref, highref, splow, sphigh);
}

