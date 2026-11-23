// The corrhum unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector ZoneSpaceHeatBalanceData::calcCorrectHumRat through
// libep_corrhum_shim (shim/ep_corrhum_shim.cc), which sets up one zone (see the shim).
#include "corrhum.h"

double ep_CorrectHumRat(int which, int controlled, int ninlets, double m1, double w1in, double m2, double w2in, double m3, double w3in, int hasnode, int mult, int listmult, double zt, double outbaro, double airhumrat, double latentgain, double latentradsys, double latentpool, double tsyssec, double oamfl, double vamfl, double ctmfl, double eamfl, double eamflxw, double sumhmaraw, double sumhmara, double mixw, double mixm, double mdotoa, double outhumrat, double volume, double volcapmult, int algo, double wprev0, double wprev1, double wprev2, double w1);

double CorrectHumRat(int which, int controlled, int ninlets, double m1, double w1in, double m2, double w2in, double m3, double w3in, int hasnode, int mult, int listmult, double zt, double outbaro, double airhumrat, double latentgain, double latentradsys, double latentpool, double tsyssec, double oamfl, double vamfl, double ctmfl, double eamfl, double eamflxw, double sumhmaraw, double sumhmara, double mixw, double mixm, double mdotoa, double outhumrat, double volume, double volcapmult, int algo, double wprev0, double wprev1, double wprev2, double w1)
{
    return ep_CorrectHumRat(which, controlled, ninlets, m1, w1in, m2, w2in, m3, w3in, hasnode, mult, listmult, zt, outbaro, airhumrat, latentgain, latentradsys, latentpool, tsyssec, oamfl, vamfl, ctmfl, eamfl, eamflxw, sumhmaraw, sumhmara, mixw, mixm, mdotoa, outhumrat, volume, volcapmult, algo, wprev0, wprev1, wprev2, w1);
}

