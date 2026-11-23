// Written by hand alongside shim/ep_updtemp_shim.cc.
#ifndef EP_UPDTEMP_H
#define EP_UPDTEMP_H

double UpdateTemperatures(int which, int shorten, int usezonehist, double prior, double tsys, int nsys, int nsyslast, int hasnode, double mat, double w, double x0, double x1, double x2, double x3, double ds0, double ds1, double ds2, double ds3, double wp0, double wp1, double wp2, double wp3, double dsw0, double dsw1, double dsw2, double dsw3);
double DownInterpolate5(int which, double old, double nw, double v0, double v1, double v2);

#endif
