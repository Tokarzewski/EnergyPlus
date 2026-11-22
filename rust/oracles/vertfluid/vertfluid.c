// The vertfluid unit as rust-anything sees it: each function forwards to the real EnergyPlus
// GLHEVert code through libep_vertfluid_shim (shim/ep_vertfluid_shim.cc), on a plant loop of
// water. VFBoreholeResistance's `which` picks the average (0), total internal (1), grout (2) or
// effective (3) borehole resistance; VFShortGFunction and VFShortLntts are the k-th short
// time step g-function and its LNTTS.
#include "vertfluid.h"

double ep_VFConvectionResistance(double outDia, double thickness, double massFlow, int numBH, double inletTemp);
double ep_VFPipeResistance(double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp);
double ep_VFBoreholeResistance(int which, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp);
int ep_VFShortSteps(double bhLength, double soilK, double soilRhoCp);
double ep_VFShortGFunction(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp);
double ep_VFShortLntts(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp);

double VFConvectionResistance(double outDia, double thickness, double massFlow, int numBH, double inletTemp)
{
    return ep_VFConvectionResistance(outDia, thickness, massFlow, numBH, inletTemp);
}

double VFPipeResistance(double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp)
{
    return ep_VFPipeResistance(outDia, thickness, kPipe, massFlow, numBH, inletTemp);
}

double VFBoreholeResistance(int which, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp)
{
    return ep_VFBoreholeResistance(which, bhLength, bhDiameter, uTubeDist, groutK, soilK, outDia, thickness, kPipe, massFlow, numBH, inletTemp);
}

int VFShortSteps(double bhLength, double soilK, double soilRhoCp)
{
    return ep_VFShortSteps(bhLength, soilK, soilRhoCp);
}

double VFShortGFunction(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp)
{
    return ep_VFShortGFunction(k, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp);
}

double VFShortLntts(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp)
{
    return ep_VFShortLntts(k, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp);
}

