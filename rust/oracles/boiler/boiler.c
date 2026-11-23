// The boiler unit as rust-anything sees it: the function forwards to the real EnergyPlus
// Boilers CalcBoilerModel through
// libep_boiler_shim (shim/ep_boiler_shim.cc), (see the shim).
#include "boiler.h"

double ep_Boiler(int which, int lock, int flowmode, int dual, int series, int run, int curve, int tempmode, double myload, double nomcap, double nomeff, double minplr, double maxplr, double tlimit, double desmdot, double parelec, double parfuel, double tin, double mdotin, double hwmax, double minav, double maxav, double tset, double tsetlo, double c0, double c1, double c2, double c3, double c4, double c5);

double Boiler(int which, int lock, int flowmode, int dual, int series, int run, int curve, int tempmode, double myload, double nomcap, double nomeff, double minplr, double maxplr, double tlimit, double desmdot, double parelec, double parfuel, double tin, double mdotin, double hwmax, double minav, double maxav, double tset, double tsetlo, double c0, double c1, double c2, double c3, double c4, double c5)
{
    return ep_Boiler(which, lock, flowmode, dual, series, run, curve, tempmode, myload, nomcap, nomeff, minplr, maxplr, tlimit, desmdot, parelec, parfuel, tin, mdotin, hwmax, minav, maxav, tset, tsetlo, c0, c1, c2, c3, c4, c5);
}

