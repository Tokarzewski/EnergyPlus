// Written by hand alongside shim/ep_extconv_shim.cc.
#ifndef EP_EXTCONV_H
#define EP_EXTCONV_H

double ExtConvCoeff(int which, int model, int setbyzone, double hmov, int rough, double absext, double tempext, double outdb, double skytempk, int hassrd, double srdtemp, double vfsrd, int usegnd, double gndtemp, int extwind, int window, int extshade, double windspeed, double winddir, double grossarea, double height, double costilt, double azimuth, double hvalue, int emsover, double emsval, double adjratio, double vfsky, double vfgnd, double airskysplit);

#endif
