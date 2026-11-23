// The corrtemp unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector correctAirTemp through
// libep_corrtemp_shim (shim/ep_corrtemp_shim.cc), which sets up one fully mixed zone (see the shim).
#include "corrtemp.h"

double ep_CorrectAirTemp(int which, int usezonehist, int algo, int hasnode, int controlled, int mult, int listmult, double volume, double capsens, double capmoist, double tsyssec, double outbaro, double outhumrat, double mat, double airhumrat, double xmat0, double xmat1, double xmat2, double dsxmat0, double dsxmat1, double dsxmat2, double wprev0, double wprev1, double wprev2, double dswprev0, double dswprev1, double dswprev2, double t1, double w1, int ndev, double cg1, double rg1, double cg2, double rg2, int noheatret, double radsys, double mcpi, double mcpti, double mcpv, double mcptv, double mdotcpoa, double outdb, int ninlets, double m1, double tin1, double win1, double m2, double tin2, double win2, double nodemflow, double nonair, double sysdeplag, double latentgain, double oamfl, double vamfl, double mdotoa);

double CorrectAirTemp(int which, int usezonehist, int algo, int hasnode, int controlled, int mult, int listmult, double volume, double capsens, double capmoist, double tsyssec, double outbaro, double outhumrat, double mat, double airhumrat, double xmat0, double xmat1, double xmat2, double dsxmat0, double dsxmat1, double dsxmat2, double wprev0, double wprev1, double wprev2, double dswprev0, double dswprev1, double dswprev2, double t1, double w1, int ndev, double cg1, double rg1, double cg2, double rg2, int noheatret, double radsys, double mcpi, double mcpti, double mcpv, double mcptv, double mdotcpoa, double outdb, int ninlets, double m1, double tin1, double win1, double m2, double tin2, double win2, double nodemflow, double nonair, double sysdeplag, double latentgain, double oamfl, double vamfl, double mdotoa)
{
    return ep_CorrectAirTemp(which, usezonehist, algo, hasnode, controlled, mult, listmult, volume, capsens, capmoist, tsyssec, outbaro, outhumrat, mat, airhumrat, xmat0, xmat1, xmat2, dsxmat0, dsxmat1, dsxmat2, wprev0, wprev1, wprev2, dswprev0, dswprev1, dswprev2, t1, w1, ndev, cg1, rg1, cg2, rg2, noheatret, radsys, mcpi, mcpti, mcpv, mcptv, mdotcpoa, outdb, ninlets, m1, tin1, win1, m2, tin2, win2, nodemflow, nonair, sysdeplag, latentgain, oamfl, vamfl, mdotoa);
}

