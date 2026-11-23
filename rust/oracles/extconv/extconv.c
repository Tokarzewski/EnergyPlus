// The extconv unit as rust-anything sees it: the function forwards to the real EnergyPlus
// Convect::InitExtConvCoeff through libep_extconv_shim (shim/ep_extconv_shim.cc), which sets
// up one exterior surface (see the shim).
#include "extconv.h"

double ep_ExtConvCoeff(int which, int model, int setbyzone, double hmov, int rough, double absext, double tempext, double outdb, double skytempk, int hassrd, double srdtemp, double vfsrd, int usegnd, double gndtemp, int extwind, int window, int extshade, double windspeed, double winddir, double grossarea, double height, double costilt, double azimuth, double hvalue, int emsover, double emsval, double adjratio, double vfsky, double vfgnd, double airskysplit);

double ExtConvCoeff(int which, int model, int setbyzone, double hmov, int rough, double absext, double tempext, double outdb, double skytempk, int hassrd, double srdtemp, double vfsrd, int usegnd, double gndtemp, int extwind, int window, int extshade, double windspeed, double winddir, double grossarea, double height, double costilt, double azimuth, double hvalue, int emsover, double emsval, double adjratio, double vfsky, double vfgnd, double airskysplit)
{
    return ep_ExtConvCoeff(which, model, setbyzone, hmov, rough, absext, tempext, outdb, skytempk, hassrd, srdtemp, vfsrd, usegnd, gndtemp, extwind, window, extshade, windspeed, winddir, grossarea, height, costilt, azimuth, hvalue, emsover, emsval, adjratio, vfsky, vfgnd, airskysplit);
}

