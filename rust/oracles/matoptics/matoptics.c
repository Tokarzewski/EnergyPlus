// The matoptics unit as rust-anything sees it: each function forwards to the real EnergyPlus
// Material code through libep_matoptics_shim (shim/ep_matoptics_shim.cc), which
// builds the screen and glass materials from plain numbers (see the shim).
#include "matoptics.h"

double ep_ScreenTrans(int which, int model, double gamma, double refl, double reflVis, double phi, double theta);
double ep_RelPhiTheta(int which, double phiWin, double thetaWin, double sx, double sy, double sz);
double ep_NormPhiTheta(int which, double phi, double theta);
double ep_PhiThetaIdx(int which, double phi, double theta, double dPhi, double dTheta);
double ep_ProfIdx(int which, double profAng);
double ep_SlatIdx(int which, double slatAng);
double ep_SimpleGlazing(int which, double u, double shgc, int vtByUser, double vt);

double ScreenTrans(int which, int model, double gamma, double refl, double reflVis, double phi, double theta)
{
    return ep_ScreenTrans(which, model, gamma, refl, reflVis, phi, theta);
}

double RelPhiTheta(int which, double phiWin, double thetaWin, double sx, double sy, double sz)
{
    return ep_RelPhiTheta(which, phiWin, thetaWin, sx, sy, sz);
}

double NormPhiTheta(int which, double phi, double theta)
{
    return ep_NormPhiTheta(which, phi, theta);
}

double PhiThetaIdx(int which, double phi, double theta, double dPhi, double dTheta)
{
    return ep_PhiThetaIdx(which, phi, theta, dPhi, dTheta);
}

double ProfIdx(int which, double profAng)
{
    return ep_ProfIdx(which, profAng);
}

double SlatIdx(int which, double slatAng)
{
    return ep_SlatIdx(which, slatAng);
}

double SimpleGlazing(int which, double u, double shgc, int vtByUser, double vt)
{
    return ep_SimpleGlazing(which, u, shgc, vtByUser, vt);
}

