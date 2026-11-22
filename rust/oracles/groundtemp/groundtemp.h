// Written by hand alongside shim/ep_groundtemp_shim.cc.
#ifndef EP_GROUNDTEMP_H
#define EP_GROUNDTEMP_H

double GTKusudaSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity);
double GTKusudaMonth(double depth, int month, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity);
double GTXingSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity);
double GTXingMonth(double depth, int month, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity);
double GTSite(int kind, int byMonth, double when, int numDaysInYear, double t0, double dt);

#endif
