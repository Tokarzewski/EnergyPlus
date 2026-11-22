// Written by hand alongside shim/ep_intrad_shim.cc.
#ifndef EP_INTRAD_H
#define EP_INTRAD_H

double IRView(int fixed, int i, int j, int mask, double L, double W, double H, double mass, double rot);
double IRScriptF(int i, int j, int mask, double L, double W, double H, double mass, double rot, double e0, double de);
double IRFmrt(int fp, int i, int mask, double L, double W, double H, double mass, double e0, double de);

#endif
