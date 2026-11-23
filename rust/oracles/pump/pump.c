// The pump unit as rust-anything sees it: the function forwards to the real EnergyPlus
// Pumps CalcPumps through
// libep_pump_shim (shim/ep_pump_shim.cc), (see the shim).
#include "pump.h"

double ep_Pump(int which, int ptype, int lock, int series, int press, int ems, int nbank, double flowreq, double mmax, double mmin, double nomvol, double nompow, double c0, double c1, double c2, double c3, double motor, double peff, double frac, double pdrop, double emsp, double tin, double pin, double mdotin, double hwmax, double minav, double maxav);

double Pump(int which, int ptype, int lock, int series, int press, int ems, int nbank, double flowreq, double mmax, double mmin, double nomvol, double nompow, double c0, double c1, double c2, double c3, double motor, double peff, double frac, double pdrop, double emsp, double tin, double pin, double mdotin, double hwmax, double minav, double maxav)
{
    return ep_Pump(which, ptype, lock, series, press, ems, nbank, flowreq, mmax, mmin, nomvol, nompow, c0, c1, c2, c3, motor, peff, frac, pdrop, emsp, tin, pin, mdotin, hwmax, minav, maxav);
}

