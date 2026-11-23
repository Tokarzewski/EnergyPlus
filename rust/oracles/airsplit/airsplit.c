// The airsplit unit as rust-anything sees it: the function forwards to the real EnergyPlus
// SplitterComponent init, calc and update through
// libep_airsplit_shim (shim/ep_airsplit_shim.cc), (see the shim).
#include "airsplit.h"

double ep_AirSplit(int which, int n, int fhvac, int first, double m0, double maxav0, double minav0, double t0, double w0, double h0, double p0, double m1, double maxav1, double minav1, double t1, double w1, double h1, double p1, double m2, double maxav2, double minav2, double t2, double w2, double h2, double p2, double m3, double maxav3, double minav3, double t3, double w3, double h3, double p3, double m4, double maxav4, double minav4, double t4, double w4, double h4, double p4);

double AirSplit(int which, int n, int fhvac, int first, double m0, double maxav0, double minav0, double t0, double w0, double h0, double p0, double m1, double maxav1, double minav1, double t1, double w1, double h1, double p1, double m2, double maxav2, double minav2, double t2, double w2, double h2, double p2, double m3, double maxav3, double minav3, double t3, double w3, double h3, double p3, double m4, double maxav4, double minav4, double t4, double w4, double h4, double p4)
{
    return ep_AirSplit(which, n, fhvac, first, m0, maxav0, minav0, t0, w0, h0, p0, m1, maxav1, minav1, t1, w1, h1, p1, m2, maxav2, minav2, t2, w2, h2, p2, m3, maxav3, minav3, t3, w3, h3, p3, m4, maxav4, minav4, t4, w4, h4, p4);
}

