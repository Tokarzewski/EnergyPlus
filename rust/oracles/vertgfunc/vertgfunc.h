// Written by hand alongside shim/ep_vertgfunc_shim.cc.
#ifndef EP_VERTGFUNC_H
#define EP_VERTGFUNC_H

int VGCombinedLen(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);
double VGCombinedLntts(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);
double VGCombinedGFunction(int k, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);
double VGGFunc(double time, double gRefRatio, int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp, double maxSimYears);

#endif
