// The groundtemp unit as rust-anything sees it: each function forwards to the real EnergyPlus
// ground temperature models through libep_groundtemp_shim (shim/ep_groundtemp_shim.cc), which
// builds each model from plain numbers (see the shim).
#include "groundtemp.h"

double ep_GTKusudaSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity);
double ep_GTKusudaMonth(double depth, int month, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity);
double ep_GTXingSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity);
double ep_GTXingMonth(double depth, int month, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity);
double ep_GTSite(int kind, int byMonth, double when, int numDaysInYear, double t0, double dt);

double GTKusudaSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity)
{
    return ep_GTKusudaSeconds(depth, seconds, numDaysInYear, ave, amp, phaseSecs, diffusivity);
}

double GTKusudaMonth(double depth, int month, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity)
{
    return ep_GTKusudaMonth(depth, month, numDaysInYear, ave, amp, phaseSecs, diffusivity);
}

double GTXingSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity)
{
    return ep_GTXingSeconds(depth, seconds, numDaysInYear, ave, amp1, phase1, amp2, phase2, diffusivity);
}

double GTXingMonth(double depth, int month, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2, double diffusivity)
{
    return ep_GTXingMonth(depth, month, numDaysInYear, ave, amp1, phase1, amp2, phase2, diffusivity);
}

double GTSite(int kind, int byMonth, double when, int numDaysInYear, double t0, double dt)
{
    return ep_GTSite(kind, byMonth, when, numDaysInYear, t0, dt);
}

