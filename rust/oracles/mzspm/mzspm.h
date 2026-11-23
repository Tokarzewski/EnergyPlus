// Written by hand alongside shim/ep_mzspm_shim.cc.
#ifndef EP_MZSPM_H
#define EP_MZSPM_H

double MZTemp(int heating, int nc, double mint, double maxt, double m1, double load1, double zt1, double zw1, double iw1, double m2, double load2, double zt2, double zw2, double iw2, double m3, double load3, double zt3, double zw3, double iw3, double m4, double load4, double zt4, double zw4, double iw4);
double MZHum(int kind, int nc, double minh, double maxh, double m1, double hum1, double dehum1, double zw1, double m2, double hum2, double dehum2, double zw2, double m3, double hum3, double dehum3, double zw3, double m4, double hum4, double dehum4, double zw4);

#endif
