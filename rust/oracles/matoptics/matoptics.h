// Written by hand alongside shim/ep_matoptics_shim.cc.
#ifndef EP_MATOPTICS_H
#define EP_MATOPTICS_H

double ScreenTrans(int which, int model, double gamma, double refl, double reflVis, double phi, double theta);
double RelPhiTheta(int which, double phiWin, double thetaWin, double sx, double sy, double sz);
double NormPhiTheta(int which, double phi, double theta);
double PhiThetaIdx(int which, double phi, double theta, double dPhi, double dTheta);
double ProfIdx(int which, double profAng);
double SlatIdx(int which, double slatAng);
double SimpleGlazing(int which, double u, double shgc, int vtByUser, double vt);

#endif
