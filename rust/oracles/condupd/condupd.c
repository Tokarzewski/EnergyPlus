// The condupd unit as rust-anything sees it: the function forwards to the real EnergyPlus
// PlantUtilities UpdateChillerComponentCondenserSide and UpdateComponentHeatRecoverySide through
// libep_condupd_shim (shim/ep_condupd_shim.cc), (see the shim).
#include "condupd.h"

double ep_CondUpd(int which, int hr, int first, int sim0, int nconn, int dem1, int dem2, double q, double tmin, double tmout, double mmod, double inm, double intemp, double outm, double outt);

double CondUpd(int which, int hr, int first, int sim0, int nconn, int dem1, int dem2, double q, double tmin, double tmout, double mmod, double inm, double intemp, double outm, double outt)
{
    return ep_CondUpd(which, hr, first, sim0, nconn, dem1, dem2, q, tmin, tmout, mmod, inm, intemp, outm, outt);
}

