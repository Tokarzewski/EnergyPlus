// The vertgfunc unit as rust-anything sees it: each function forwards to the real EnergyPlus
// GLHEVert code through libep_vertgfunc_shim (shim/ep_vertgfunc_shim.cc). The field is nx by ny
// boreholes `spacing` apart on a plant loop of water; calcGFunctions combines its short and long
// time step g-functions, and VGGFunc is getGFunc on the combined curve.
#include "vertgfunc.h"

int ep_VGCombinedLen(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);
double ep_VGCombinedLntts(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);
double ep_VGCombinedGFunction(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);
double ep_VGGFunc(double time, double gRefRatio, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);

int VGCombinedLen(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)
{
    return ep_VGCombinedLen(nx, ny, spacing, bhLength, bhTopDepth, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, inletTemp, maxSimYears);
}

double VGCombinedLntts(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)
{
    return ep_VGCombinedLntts(k, nx, ny, spacing, bhLength, bhTopDepth, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, inletTemp, maxSimYears);
}

double VGCombinedGFunction(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)
{
    return ep_VGCombinedGFunction(k, nx, ny, spacing, bhLength, bhTopDepth, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, inletTemp, maxSimYears);
}

double VGGFunc(double time, double gRefRatio, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears)
{
    return ep_VGGFunc(time, gRefRatio, nx, ny, spacing, bhLength, bhTopDepth, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, inletTemp, maxSimYears);
}

