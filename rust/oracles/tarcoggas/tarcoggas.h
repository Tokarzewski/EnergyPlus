// Written by hand alongside shim/ep_tarcoggas_shim.cc.
#ifndef EP_TARCOGGAS_H
#define EP_TARCOGGAS_H

double Gasses90(int which, int standard, double tmean, double pres, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad);
double GassesLow(int which, double tmean, double mwght, double pressure, double gama);
double TarcogSolve(int which, int n, double d, double k, double o, double p);

#endif
