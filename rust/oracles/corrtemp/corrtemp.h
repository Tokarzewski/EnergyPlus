// Written by hand alongside shim/ep_corrtemp_shim.cc.
#ifndef EP_LOCALENV_H
#define EP_LOCALENV_H

double CorrectAirTemp(int which, int usezonehist, int algo, int hasnode, int controlled, int mult, int listmult, double volume, double capsens, double capmoist, double tsyssec, double outbaro, double outhumrat, double mat, double airhumrat, double xmat0, double xmat1, double xmat2, double dsxmat0, double dsxmat1, double dsxmat2, double wprev0, double wprev1, double wprev2, double dswprev0, double dswprev1, double dswprev2, double t1, double w1, int ndev, double cg1, double rg1, double cg2, double rg2, int noheatret, double radsys, double mcpi, double mcpti, double mcpv, double mcptv, double mdotcpoa, double outdb, int ninlets, double m1, double tin1, double win1, double m2, double tin2, double win2, double nodemflow, double nonair, double sysdeplag, double latentgain, double oamfl, double vamfl, double mdotoa);

#endif
