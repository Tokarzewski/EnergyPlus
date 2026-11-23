// The retplenum unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZonePlenum CalcAirZoneReturnPlenum through
// libep_retplenum_shim (shim/ep_retplenum_shim.cc), (see the shim).
#include "retplenum.h"

double ep_RetPlenum(int which, int nin, int nadu, int nind, double m1, double x1, double n1, double p1, double m2, double x2, double n2, double p2, double m3, double x3, double n3, double p3, double m4, double x4, double n4, double p4, int u1, int d1, double lu1, double ld1, double lp1, double dx1, double dn1, int u2, int d2, double lu2, double ld2, double lp2, double dx2, double dn2, double i1, double i2, double tz, double wz, double hz);

double RetPlenum(int which, int nin, int nadu, int nind, double m1, double x1, double n1, double p1, double m2, double x2, double n2, double p2, double m3, double x3, double n3, double p3, double m4, double x4, double n4, double p4, int u1, int d1, double lu1, double ld1, double lp1, double dx1, double dn1, int u2, int d2, double lu2, double ld2, double lp2, double dx2, double dn2, double i1, double i2, double tz, double wz, double hz)
{
    return ep_RetPlenum(which, nin, nadu, nind, m1, x1, n1, p1, m2, x2, n2, p2, m3, x3, n3, p3, m4, x4, n4, p4, u1, d1, lu1, ld1, lp1, dx1, dn1, u2, d2, lu2, ld2, lp2, dx2, dn2, i1, i2, tz, wz, hz);
}

