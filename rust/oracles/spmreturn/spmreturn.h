// Written by hand alongside shim/ep_spmreturn_shim.cc.
#ifndef EP_SPMRETURN_H
#define EP_SPMRETURN_H

double RetWater(int hot, int ttype, double mdot, double tsup, double tret, double target, double retsp, double mint, double maxt);
double SysNode(int ctrl, double ref, double lowref, double highref, double splow, double sphigh);

#endif
