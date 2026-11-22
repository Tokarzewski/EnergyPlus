// The tarcoggas unit as rust-anything sees it: each function forwards to the real EnergyPlus
// TARCOG code through libep_tarcoggas_shim (shim/ep_tarcoggas_shim.cc), which
// builds the gas table and matrices from plain numbers (see the shim).
#include "tarcoggas.h"

double ep_Gasses90(int which, int standard, double tmean, double pres, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad);
double ep_GassesLow(int which, double tmean, double mwght, double pressure, double gama);
double ep_TarcogSolve(int which, int n, double d, double k, double o, double p);

double Gasses90(int which, int standard, double tmean, double pres, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad)
{
    return ep_Gasses90(which, standard, tmean, pres, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad);
}

double GassesLow(int which, double tmean, double mwght, double pressure, double gama)
{
    return ep_GassesLow(which, tmean, mwght, pressure, gama);
}

double TarcogSolve(int which, int n, double d, double k, double o, double p)
{
    return ep_TarcogSolve(which, n, d, k, o, p);
}

