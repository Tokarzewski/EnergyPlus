// The commonpipe unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HVACInterfaceManager UpdateCommonPipe (single common pipe) through
// libep_commonpipe_shim (shim/ep_commonpipe_shim.cc), (see the shim).
#include "commonpipe.h"

double ep_CommonPipe(int which, int side, int newstep, double tin, double msec, double mpri, double pump, double mass, double last, double prev, double other, double tssec);

double CommonPipe(int which, int side, int newstep, double tin, double msec, double mpri, double pump, double mass, double last, double prev, double other, double tssec)
{
    return ep_CommonPipe(which, side, newstep, tin, msec, mpri, pump, mass, last, prev, other, tssec);
}

