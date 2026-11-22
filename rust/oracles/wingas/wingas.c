// The wingas unit as rust-anything sees it: each function forwards to the real EnergyPlus
// WindowManager code through libep_wingas_shim (shim/ep_wingas_shim.cc), which
// sets up the gap from plain numbers (see the shim).
#include "wingas.h"

double ep_GapCon(int which, double tleft, double tright, double width, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad);
double ep_GapProps(int which, double tmean, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad);
double ep_GapNusselt(double tso, double tsi, double width, double height, double tilt, double gr, double pr);

double GapCon(int which, double tleft, double tright, double width, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad)
{
    return ep_GapCon(which, tleft, tright, width, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad);
}

double GapProps(int which, double tmean, int nmix, int g1, int g2, int g3, int g4, double f1, double f2, double f3, double f4, double quad)
{
    return ep_GapProps(which, tmean, nmix, g1, g2, g3, g4, f1, f2, f3, f4, quad);
}

double GapNusselt(double tso, double tsi, double width, double height, double tilt, double gr, double pr)
{
    return ep_GapNusselt(tso, tsi, width, height, tilt, gr, pr);
}

