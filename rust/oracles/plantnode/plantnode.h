// Written by hand alongside shim/ep_plantnode_shim.cc.
#ifndef EP_PLANTNODE_H
#define EP_PLANTNODE_H

double InitNodes(int which, double minm, double maxm);
double SafeCopy(int which, int lp, double t, double m, double q, double h, double tmin, double tmax, double mmin, double mminav, double mmax, double mmaxav, double w, double p, double op);
double BoundNode(double v, double minav, double maxav);
double Tighten(int which, double oldmin, double oldmax, double newmin, double newmax);
double BoundTwo(double v, double lo, double hi);

#endif
