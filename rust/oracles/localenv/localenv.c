// The localenv unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceSurfaceManager local environment averages through
// libep_localenv_shim (shim/ep_localenv_shim.cc), which sets up one exterior surface (see the shim).
#include "localenv.h"

double ep_LocalEnvironment(int which, int op, int anylocal, int gnddefined, double vfsum, int ngnd, double gvf1, int ht1, double gt1, int hr1, double gr1, double gvf2, int ht2, double gt2, int hr2, double gr2, double gvf3, int ht3, double gt3, int hr3, double gr3, double gvf4, int ht4, double gt4, int hr4, double gr4, int hassrd, int nsrd, double svf1, double st1, double svf2, double st2, double svf3, double st3, double svf4, double st4, double vfsrd, double vfgir, double tavg0, double ravg0, double srdt0, int usegndtemp0, int usegndrefl0);

double LocalEnvironment(int which, int op, int anylocal, int gnddefined, double vfsum, int ngnd, double gvf1, int ht1, double gt1, int hr1, double gr1, double gvf2, int ht2, double gt2, int hr2, double gr2, double gvf3, int ht3, double gt3, int hr3, double gr3, double gvf4, int ht4, double gt4, int hr4, double gr4, int hassrd, int nsrd, double svf1, double st1, double svf2, double st2, double svf3, double st3, double svf4, double st4, double vfsrd, double vfgir, double tavg0, double ravg0, double srdt0, int usegndtemp0, int usegndrefl0)
{
    return ep_LocalEnvironment(which, op, anylocal, gnddefined, vfsum, ngnd, gvf1, ht1, gt1, hr1, gr1, gvf2, ht2, gt2, hr2, gr2, gvf3, ht3, gt3, hr3, gr3, gvf4, ht4, gt4, hr4, gr4, hassrd, nsrd, svf1, st1, svf2, st2, svf3, st3, svf4, st4, vfsrd, vfgir, tavg0, ravg0, srdt0, usegndtemp0, usegndrefl0);
}

