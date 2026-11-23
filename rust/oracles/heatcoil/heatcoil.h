// Written by hand alongside shim/ep_heatcoil_shim.cc.
#ifndef EP_HEATCOIL_H
#define EP_HEATCOIL_H

double HeatCoil(int which, int fuel, int cycling, int sensed, int plf, double avail, double mdot, double tin, double win, double tset, double qreq, double plr, double eff, double cap, double parelec, double parfuel, double c0, double c1, double c2);

#endif
