// Written by hand alongside shim/ep_glhesim_shim.cc.
#ifndef EP_GLHESIM_H
#define EP_GLHESIM_H

double GSRun(int which, int steps, int tsPerHour, double tGround, double tIn0, double tInAmp, double flow0, int offEvery, double gA, double gB, double gRefRatio, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, int numBH);

#endif
