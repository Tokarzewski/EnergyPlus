// Written by hand alongside shim/ep_corrhum_shim.cc.
#ifndef EP_CORRHUM_H
#define EP_CORRHUM_H

double CorrectHumRat(int which, int controlled, int ninlets, double m1, double w1in, double m2, double w2in, double m3, double w3in, int hasnode, int mult, int listmult, double zt, double outbaro, double airhumrat, double latentgain, double latentradsys, double latentpool, double tsyssec, double oamfl, double vamfl, double ctmfl, double eamfl, double eamflxw, double sumhmaraw, double sumhmara, double mixw, double mixm, double mdotoa, double outhumrat, double volume, double volcapmult, int algo, double wprev0, double wprev1, double wprev2, double w1);

#endif
