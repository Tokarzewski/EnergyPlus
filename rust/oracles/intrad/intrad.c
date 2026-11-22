// The intrad unit as rust-anything sees it: each function forwards to the real EnergyPlus
// HeatBalanceIntRadExchange code through libep_intrad_shim (shim/ep_intrad_shim.cc), which
// builds a box zone from plain numbers (see the shim).
#include "intrad.h"

double ep_IRView(int fixed, int i, int j, int mask, double L, double W, double H, double mass, double rot);
double ep_IRScriptF(int i, int j, int mask, double L, double W, double H, double mass, double rot, double e0, double de);
double ep_IRFmrt(int fp, int i, int mask, double L, double W, double H, double mass, double e0, double de);

double IRView(int fixed, int i, int j, int mask, double L, double W, double H, double mass, double rot)
{
    return ep_IRView(fixed, i, j, mask, L, W, H, mass, rot);
}

double IRScriptF(int i, int j, int mask, double L, double W, double H, double mass, double rot, double e0, double de)
{
    return ep_IRScriptF(i, j, mask, L, W, H, mass, rot, e0, de);
}

double IRFmrt(int fp, int i, int mask, double L, double W, double H, double mass, double e0, double de)
{
    return ep_IRFmrt(fp, i, mask, L, W, H, mass, e0, de);
}

