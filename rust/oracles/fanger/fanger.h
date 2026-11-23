// Written by hand alongside shim/ep_fanger_shim.cc.
#ifndef EP_FANGER_H
#define EP_FANGER_H

double FangerPMV(int which, double at, double rt, double rh, double vel, double act, double clo, double work);
double StandardEffectiveTemp(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work);

#endif
