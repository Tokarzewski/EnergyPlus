// Written by hand alongside shim/ep_sensout_shim.cc.
#ifndef EP_SENSOUT_H
#define EP_SENSOUT_H

double CompOutput(int which, double m, double t2, double w2, double t1, double w1);
double ZoneOutput(int which, double m, double te, double we, double tz, double wz);
double ZoneSensOutput(double m, double te, double tz, double wz);

#endif
