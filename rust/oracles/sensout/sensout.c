// The sensout unit as rust-anything sees it: each function forwards to the real EnergyPlus
// GeneralRoutines sensible and latent output routines through
// libep_sensout_shim (shim/ep_sensout_shim.cc), (see the shim).
#include "sensout.h"

double ep_CompOutput(int which, double m, double t2, double w2, double t1, double w1);
double ep_ZoneOutput(int which, double m, double te, double we, double tz, double wz);
double ep_ZoneSensOutput(double m, double te, double tz, double wz);

double CompOutput(int which, double m, double t2, double w2, double t1, double w1)
{
    return ep_CompOutput(which, m, t2, w2, t1, w1);
}

double ZoneOutput(int which, double m, double te, double we, double tz, double wz)
{
    return ep_ZoneOutput(which, m, te, we, tz, wz);
}

double ZoneSensOutput(double m, double te, double tz, double wz)
{
    return ep_ZoneSensOutput(m, te, tz, wz);
}

