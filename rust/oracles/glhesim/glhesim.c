// The glhesim unit as rust-anything sees it: GSRun forwards to the real EnergyPlus
// GLHEBase::calcGroundHeatExchanger through libep_glhesim_shim (shim/ep_glhesim_shim.cc), which
// runs it for `steps` time steps and returns one of its results (see the shim).
#include "glhesim.h"

double ep_GSRun(int which, int steps, int tsPerHour, double tGround, double tIn0, double tInAmp, double flow0, int offEvery, double gA, double gB, double gRefRatio, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, int numBH);

double GSRun(int which, int steps, int tsPerHour, double tGround, double tIn0, double tInAmp, double flow0, int offEvery, double gA, double gB, double gRefRatio, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, int numBH)
{
    return ep_GSRun(which, steps, tsPerHour, tGround, tIn0, tInAmp, flow0, offEvery, gA, gB, gRefRatio, bhLength, bhDiameter, uTubeDist, groutK, soilK, soilRhoCp, outDia, thickness, kPipe, numBH);
}

