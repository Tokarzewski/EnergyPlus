// The plantiface unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HVACInterfaceManager UpdatePlantLoopInterface through
// libep_plantiface_shim (shim/ep_plantiface_shim.cc), (see the shim).
#include "plantiface.h"

double ep_PlantIface(int which, int side, int newstep, int haspress, int oot0, double tin, double mdot, double minav, double maxav, double q, double p, double otemp, double omdot, double pump, double mass, double last, double prev, double tssec, double hist);

double PlantIface(int which, int side, int newstep, int haspress, int oot0, double tin, double mdot, double minav, double maxav, double q, double p, double otemp, double omdot, double pump, double mass, double last, double prev, double tssec, double hist)
{
    return ep_PlantIface(which, side, newstep, haspress, oot0, tin, mdot, minav, maxav, q, p, otemp, omdot, pump, mass, last, prev, tssec, hist);
}

