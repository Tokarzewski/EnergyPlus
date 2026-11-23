// The halfloop unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HVACInterfaceManager UpdateHalfLoopInletTemp through
// libep_halfloop_shim (shim/ep_halfloop_shim.cc), (see the shim).
#include "halfloop.h"

double ep_HalfLoop(int which, int side, int newstep, double tin, double mdot, double pump, double mass, double last, double prev, double tssec);

double HalfLoop(int which, int side, int newstep, double tin, double mdot, double pump, double mass, double last, double prev, double tssec)
{
    return ep_HalfLoop(which, side, newstep, tin, mdot, pump, mass, last, prev, tssec);
}

