// The irexch unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceIntRadExchange CalcInteriorRadExchange through
// libep_irexch_shim (shim/ep_irexch_shim.cc), (see the shim).
#include "irexch.h"

double ep_IRExchange(int which, int i, int mask, double L, double W, double H, double mass, double rot, double e0, double de, int carroll, int win, double t0, double dt, double twin, int iter);

double IRExchange(int which, int i, int mask, double L, double W, double H, double mass, double rot, double e0, double de, int carroll, int win, double t0, double dt, double twin, int iter)
{
    return ep_IRExchange(which, i, mask, L, W, H, mass, rot, e0, de, carroll, win, t0, dt, twin, iter);
}

