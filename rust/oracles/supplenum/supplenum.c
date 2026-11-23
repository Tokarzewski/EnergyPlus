// The supplenum unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZonePlenum CalcAirZoneSupplyPlenum and UpdateAirZoneSupplyPlenum through
// libep_supplenum_shim (shim/ep_supplenum_shim.cc), (see the shim).
#include "supplenum.h"

double ep_SupPlenum(int which, int first, int nout, double inm, double inmx, double inmn, double m1, double x1, double n1, double m2, double x2, double n2, double m3, double x3, double n3, double m4, double x4, double n4, double tz, double wz, double hz);

double SupPlenum(int which, int first, int nout, double inm, double inmx, double inmn, double m1, double x1, double n1, double m2, double x2, double n2, double m3, double x3, double n3, double m4, double x4, double n4, double tz, double wz, double hz)
{
    return ep_SupPlenum(which, first, nout, inm, inmx, inmn, m1, x1, n1, m2, x2, n2, m3, x3, n3, m4, x4, n4, tz, wz, hz);
}

