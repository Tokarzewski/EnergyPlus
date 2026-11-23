// The intconv unit as rust-anything sees it: the function forwards to the real EnergyPlus
// Convect::InitIntConvCoeff through libep_intconv_shim (shim/ep_intconv_shim.cc), which sets
// up one zone with one heat transfer surface (see the shim).
#include "intconv.h"

double ep_IntConvCoeff(int model, int setbyzone, double tsurf, double mat, double humrat, double costilt, double tilt, double sintilt, double height, double area, int window, int extbound, double outwind, double outbaro, double volume, double hvalue, int emsover, double emsval, double adjratio, double lowlimit);

double IntConvCoeff(int model, int setbyzone, double tsurf, double mat, double humrat, double costilt, double tilt, double sintilt, double height, double area, int window, int extbound, double outwind, double outbaro, double volume, double hvalue, int emsover, double emsval, double adjratio, double lowlimit)
{
    return ep_IntConvCoeff(model, setbyzone, tsurf, mat, humrat, costilt, tilt, sintilt, height, area, window, extbound, outwind, outbaro, volume, hvalue, emsover, emsval, adjratio, lowlimit);
}

