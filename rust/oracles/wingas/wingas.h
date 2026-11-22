// Written by hand alongside shim/ep_wingas_shim.cc.
#ifndef EP_WINGAS_H
#define EP_WINGAS_H

double GapCon(int which, double tleft, double tright, double width, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad);
double GapProps(int which, double tmean, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad);
double GapNusselt(double tso, double tsi, double width, double height, double tilt, double gr, double pr);

#endif
