// Written by hand alongside shim/ep_tstat_shim.cc.
#ifndef EP_TSTAT_H
#define EP_TSTAT_H

double TstatSetPoints(int which, int ctype, int heatused, double heatsp, double coolsp, double hcsp, double dhsp, double dcsp, double setpt0, double hi0, double lo0, int anyop, int opctrl, double frac, double mrt, int anyhum, int ovctrl, double ovrange, double ovratio, double relhum, double dehum, int emsheat, double emsheatval, int emscool, double emscoolval);

#endif
