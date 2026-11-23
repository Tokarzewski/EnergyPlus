// The vent unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneEquipmentManager CalcAirFlowSimple ventilation through
// libep_vent_shim (shim/ep_vent_shim.cc), which sets up one zone (see the shim).
#include "vent.h"

double ep_Ventilation(int which, int nvent, int mod, int den, double avail, double dl, double c0, double ct, double cv, double cvv, int fantype, double fanp, double fane, double openarea, double openfrac, double openeff, double disc, double effangle, double dh, int scheds, double mint, double maxt, double minot, double maxot, double delt, double maxwind, int hybrid, int ems, double emsflow, int systs, double mat, double hum, double xmpt, double wprev, double outdb, double wind, double winddir, double outbaro, double outhum, double outenth, int linked, double linkedhum, double linkedenth, double stdrho, double tsyssec);

double Ventilation(int which, int nvent, int mod, int den, double avail, double dl, double c0, double ct, double cv, double cvv, int fantype, double fanp, double fane, double openarea, double openfrac, double openeff, double disc, double effangle, double dh, int scheds, double mint, double maxt, double minot, double maxot, double delt, double maxwind, int hybrid, int ems, double emsflow, int systs, double mat, double hum, double xmpt, double wprev, double outdb, double wind, double winddir, double outbaro, double outhum, double outenth, int linked, double linkedhum, double linkedenth, double stdrho, double tsyssec)
{
    return ep_Ventilation(which, nvent, mod, den, avail, dl, c0, ct, cv, cvv, fantype, fanp, fane, openarea, openfrac, openeff, disc, effangle, dh, scheds, mint, maxt, minot, maxot, delt, maxwind, hybrid, ems, emsflow, systs, mat, hum, xmpt, wprev, outdb, wind, winddir, outbaro, outhum, outenth, linked, linkedhum, linkedenth, stdrho, tsyssec);
}

