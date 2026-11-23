// Written by hand alongside shim/ep_warmflow_shim.cc.
#ifndef EP_WARMFLOW_H
#define EP_WARMFLOW_H

double WarmFlow(int which, int flowfirst, int nc, double mint, double maxt, double minturn, double load1, double zt1, double mmax1, double w1, double load2, double zt2, double mmax2, double w2, double load3, double zt3, double mmax3, double w3, double load4, double zt4, double mmax4, double w4);
double RABypass(double spt, double tloop, double tmix, double msup, double tsup, double mtot, double trab);

#endif
