// The predhum unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio through
// libep_predhum_shim (shim/ep_predhum_shim.cc), which sets up one zone (see the shim).
#include "predhum.h"

double ep_PredictedHumidityRatio(int which, int ctrltype, double humsp, double dehumsp, int emshumon, double emshumval, int emsdehumon, double emsdehumval, double zt, double outbaro, double airhumrat, double latentgain, double latentradsys, double latentpool, double tsyssec, double oamfl, double vamfl, double ctmfl, double eamfl, double eamflxw, double sumhmaraw, double sumhmara, double mixw, double mixm, double mdotoa, double outhumrat, double volume, double volcapmult, int algo, double wprev0, double wprev1, double wprev2, double w1, double rafnfrac, int mult, int listmult);

double PredictedHumidityRatio(int which, int ctrltype, double humsp, double dehumsp, int emshumon, double emshumval, int emsdehumon, double emsdehumval, double zt, double outbaro, double airhumrat, double latentgain, double latentradsys, double latentpool, double tsyssec, double oamfl, double vamfl, double ctmfl, double eamfl, double eamflxw, double sumhmaraw, double sumhmara, double mixw, double mixm, double mdotoa, double outhumrat, double volume, double volcapmult, int algo, double wprev0, double wprev1, double wprev2, double w1, double rafnfrac, int mult, int listmult)
{
    return ep_PredictedHumidityRatio(which, ctrltype, humsp, dehumsp, emshumon, emshumval, emsdehumon, emsdehumval, zt, outbaro, airhumrat, latentgain, latentradsys, latentpool, tsyssec, oamfl, vamfl, ctmfl, eamfl, eamflxw, sumhmaraw, sumhmara, mixw, mixm, mdotoa, outhumrat, volume, volcapmult, algo, wprev0, wprev1, wprev2, w1, rafnfrac, mult, listmult);
}

