// Written by hand alongside shim/ep_glazing_shim.cc.
#ifndef EP_GLAZING_H
#define EP_INTRAD_H

double GlassPhi(int which, double cs, double tf0, double rf0, double rb0, int simple, double shgc, double u);
double GlassFit(int which, double tf0, double rf0, double rb0);
double SlatViewFac(int i, int j, double s, double h, double phib, double phis);
double MatInverse(int which, int n, double d, double k, double o, double p);
double Interp(double xin, int npts, double x0, double x1, double x2, double x3, double x4, double y0, double y1, double y2, double y3, double y4);

#endif
