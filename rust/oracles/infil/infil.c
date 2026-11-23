// The infil unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneEquipmentManager CalcAirFlowSimple infiltration through
// libep_infil_shim (shim/ep_infil_shim.cc), which sets up one zone (see the shim).
#include "infil.h"

double ep_Infiltration(int which, int ninf, int mod1, int den1, double frac1, double dl1, double c01, double ct1, double cv1, double cvv1, double la1, double cs1, double cw1, double fc1, double as1, double aw1, double n1, double sf1, int ems1, double emsv1, int mod2, int den2, double frac2, double dl2, double c02, double ct2, double cv2, double cvv2, double la2, double cs2, double cw2, double fc2, double as2, double aw2, double n2, double sf2, int ems2, double emsv2, int systs, double mat, double hum, double xmpt, double wprev, double outdb, double wind, double metwind, double outbaro, double outhum, int linked, double linkedhum, double stdrho);

double Infiltration(int which, int ninf, int mod1, int den1, double frac1, double dl1, double c01, double ct1, double cv1, double cvv1, double la1, double cs1, double cw1, double fc1, double as1, double aw1, double n1, double sf1, int ems1, double emsv1, int mod2, int den2, double frac2, double dl2, double c02, double ct2, double cv2, double cvv2, double la2, double cs2, double cw2, double fc2, double as2, double aw2, double n2, double sf2, int ems2, double emsv2, int systs, double mat, double hum, double xmpt, double wprev, double outdb, double wind, double metwind, double outbaro, double outhum, int linked, double linkedhum, double stdrho)
{
    return ep_Infiltration(which, ninf, mod1, den1, frac1, dl1, c01, ct1, cv1, cvv1, la1, cs1, cw1, fc1, as1, aw1, n1, sf1, ems1, emsv1, mod2, den2, frac2, dl2, c02, ct2, cv2, cvv2, la2, cs2, cw2, fc2, as2, aw2, n2, sf2, ems2, emsv2, systs, mat, hum, xmpt, wprev, outdb, wind, metwind, outbaro, outhum, linked, linkedhum, stdrho);
}

