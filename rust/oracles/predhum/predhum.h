// Written by hand alongside shim/ep_predhum_shim.cc.
#ifndef EP_PREDHUM_H
#define EP_PREDHUM_H

double PredictedHumidityRatio(int which, int ctrltype, double humsp, double dehumsp, int emshumon, double emshumval, int emsdehumon, double emsdehumval, double zt, double outbaro, double airhumrat, double latentgain, double latentradsys, double latentpool, double tsyssec, double oamfl, double vamfl, double ctmfl, double eamfl, double eamflxw, double sumhmaraw, double sumhmara, double mixw, double mixm, double mdotoa, double outhumrat, double volume, double volcapmult, int algo, double wprev0, double wprev1, double wprev2, double w1, double rafnfrac, int mult, int listmult);

#endif
