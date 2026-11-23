// The outsidetemp unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceSurfaceManager CalcOutsideSurfTemp through libep_outsidetemp_shim
// (shim/ep_outsidetemp_shim.cc), which sets up the surface's heat balance terms (see the shim).
#include "outsidetemp.h"

double ep_OutsideSurfTemp(int which, double cross0, double inside0, double outside0, double srcout0, double srcin0, int src, double hconvint, double hconvext, double hairext, double hskyext, double hgrdext, double hsrdext, double ctfout, double ctfin, double qswout, double qswin, double qsrc, double tempin, double qaddout, double qintgains, double qnetlwin, double mat, double qswmvins, double hmov, double text, double skytemp, double odb, int hassrd, double srdtemp, int usegnd, double gndtemp, int osc, double osctrad, double oschrad);

double OutsideSurfTemp(int which, double cross0, double inside0, double outside0, double srcout0, double srcin0, int src, double hconvint, double hconvext, double hairext, double hskyext, double hgrdext, double hsrdext, double ctfout, double ctfin, double qswout, double qswin, double qsrc, double tempin, double qaddout, double qintgains, double qnetlwin, double mat, double qswmvins, double hmov, double text, double skytemp, double odb, int hassrd, double srdtemp, int usegnd, double gndtemp, int osc, double osctrad, double oschrad)
{
    return ep_OutsideSurfTemp(which, cross0, inside0, outside0, srcout0, srcin0, src, hconvint, hconvext, hairext, hskyext, hgrdext, hsrdext, ctfout, ctfin, qswout, qswin, qsrc, tempin, qaddout, qintgains, qnetlwin, mat, qswmvins, hmov, text, skytemp, odb, hassrd, srdtemp, usegnd, gndtemp, osc, osctrad, oschrad);
}

