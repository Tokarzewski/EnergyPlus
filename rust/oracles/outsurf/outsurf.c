// The outsurf unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceSurfaceManager CalcOutsideSurfTemp through
// libep_outsurf_shim (shim/ep_outsurf_shim.cc), which sets up one exterior wall (see the shim).
#include "outsurf.h"

double ep_OutsideSurfTemp(int which, double ctfcross, double ctfinside, double ctfoutside, double ctfsourceout, double ctfsourcein, int sourcesink, double hconvint, double hmovinsul, double tempext, double skytemp, double outdrybulb, int hassrd, int srdskyon, double srdskyval, int srdgndon, double srdgndval, double srdsurftemp, int usegndsurf, double gndsurftavg, int oscm, double oscmtrad, double oscmhrad, double hconvext, double hairext, double hskyext, double hgrdext, double hsrdsurfext, double qaddout, double ctfconstout, double qradswoutabs, double tempin, double qsrchist, double ctfconstin, double qradswinabs, double qdotradintgainsin, double mat, double qdotradnetlwin, double qradswoutmvins);

double OutsideSurfTemp(int which, double ctfcross, double ctfinside, double ctfoutside, double ctfsourceout, double ctfsourcein, int sourcesink, double hconvint, double hmovinsul, double tempext, double skytemp, double outdrybulb, int hassrd, int srdskyon, double srdskyval, int srdgndon, double srdgndval, double srdsurftemp, int usegndsurf, double gndsurftavg, int oscm, double oscmtrad, double oscmhrad, double hconvext, double hairext, double hskyext, double hgrdext, double hsrdsurfext, double qaddout, double ctfconstout, double qradswoutabs, double tempin, double qsrchist, double ctfconstin, double qradswinabs, double qdotradintgainsin, double mat, double qdotradnetlwin, double qradswoutmvins)
{
    return ep_OutsideSurfTemp(which, ctfcross, ctfinside, ctfoutside, ctfsourceout, ctfsourcein, sourcesink, hconvint, hmovinsul, tempext, skytemp, outdrybulb, hassrd, srdskyon, srdskyval, srdgndon, srdgndval, srdsurftemp, usegndsurf, gndsurftavg, oscm, oscmtrad, oscmhrad, hconvext, hairext, hskyext, hgrdext, hsrdsurfext, qaddout, ctfconstout, qradswoutabs, tempin, qsrchist, ctfconstin, qradswinabs, qdotradintgainsin, mat, qdotradnetlwin, qradswoutmvins);
}

