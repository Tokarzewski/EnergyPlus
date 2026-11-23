// The airmixer unit as rust-anything sees it: the function forwards to the real EnergyPlus
// MixerComponent CalcAirMixer through
// libep_airmixer_shim (shim/ep_airmixer_shim.cc), (see the shim).
#include "airmixer.h"

double ep_AirMixer(int which, int n, double m1, double maxav1, double minav1, double t1, double w1, double h1, double p1, double f1, double m2, double maxav2, double minav2, double t2, double w2, double h2, double p2, double f2, double m3, double maxav3, double minav3, double t3, double w3, double h3, double p3, double f3, double m4, double maxav4, double minav4, double t4, double w4, double h4, double p4, double f4);

double AirMixer(int which, int n, double m1, double maxav1, double minav1, double t1, double w1, double h1, double p1, double f1, double m2, double maxav2, double minav2, double t2, double w2, double h2, double p2, double f2, double m3, double maxav3, double minav3, double t3, double w3, double h3, double p3, double f3, double m4, double maxav4, double minav4, double t4, double w4, double h4, double p4, double f4)
{
    return ep_AirMixer(which, n, m1, maxav1, minav1, t1, w1, h1, p1, f1, m2, maxav2, minav2, t2, w2, h2, p2, f2, m3, maxav3, minav3, t3, w3, h3, p3, f3, m4, maxav4, minav4, t4, w4, h4, p4, f4);
}

