// The twowaypipe unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HVACInterfaceManager ManageTwoWayCommonPipe through
// libep_twowaypipe_shim (shim/ep_twowaypipe_shim.cc), (see the shim).
#include "twowaypipe.h"

double ep_TwoWayPipe(int which, int side, int psp, int ssp, double tank, double other, double tpsp, double tssp, double tpin, double tsin, double msec, double mpri, double pts, double prc, double src);

double TwoWayPipe(int which, int side, int psp, int ssp, double tank, double other, double tpsp, double tssp, double tpin, double tsin, double msec, double mpri, double pts, double prc, double src)
{
    return ep_TwoWayPipe(which, side, psp, ssp, tank, other, tpsp, tssp, tpin, tsin, msec, mpri, pts, prc, src);
}

