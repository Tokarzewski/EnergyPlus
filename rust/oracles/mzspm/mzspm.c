// The mzspm unit as rust-anything sees it: each function forwards to the real EnergyPlus
// SetPointManager multizone managers through
// libep_mzspm_shim (shim/ep_mzspm_shim.cc), (see the shim).
#include "mzspm.h"

double ep_MZTemp(int heating, int nc, double mint, double maxt, double m1, double load1, double zt1, double zw1, double iw1, double m2, double load2, double zt2, double zw2, double iw2, double m3, double load3, double zt3, double zw3, double iw3, double m4, double load4, double zt4, double zw4, double iw4);
double ep_MZHum(int kind, int nc, double minh, double maxh, double m1, double hum1, double dehum1, double zw1, double m2, double hum2, double dehum2, double zw2, double m3, double hum3, double dehum3, double zw3, double m4, double hum4, double dehum4, double zw4);

double MZTemp(int heating, int nc, double mint, double maxt, double m1, double load1, double zt1, double zw1, double iw1, double m2, double load2, double zt2, double zw2, double iw2, double m3, double load3, double zt3, double zw3, double iw3, double m4, double load4, double zt4, double zw4, double iw4)
{
    return ep_MZTemp(heating, nc, mint, maxt, m1, load1, zt1, zw1, iw1, m2, load2, zt2, zw2, iw2, m3, load3, zt3, zw3, iw3, m4, load4, zt4, zw4, iw4);
}

double MZHum(int kind, int nc, double minh, double maxh, double m1, double hum1, double dehum1, double zw1, double m2, double hum2, double dehum2, double zw2, double m3, double hum3, double dehum3, double zw3, double m4, double hum4, double dehum4, double zw4)
{
    return ep_MZHum(kind, nc, minh, maxh, m1, hum1, dehum1, zw1, m2, hum2, dehum2, zw2, m3, hum3, dehum3, zw3, m4, hum4, dehum4, zw4);
}

