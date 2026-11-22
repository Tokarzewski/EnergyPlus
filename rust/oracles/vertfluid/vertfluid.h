// Written by hand alongside shim/ep_vertfluid_shim.cc.
#ifndef EP_VERTFLUID_H
#define EP_VERTFLUID_H

double VFConvectionResistance(double outDia, double thickness, double massFlow, int numBH, double inletTemp);
double VFPipeResistance(double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp);
double VFBoreholeResistance(int which, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp);
int VFShortSteps(double bhLength, double soilK, double soilRhoCp);
double VFShortGFunction(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp);
double VFShortLntts(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp);

#endif
