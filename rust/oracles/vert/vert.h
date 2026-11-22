// Written by hand alongside shim/ep_vert_shim.cc.
#ifndef EP_VERT_H
#define EP_VERT_H

double VertDistance(double xi, double yi, double zi, double xj, double yj, double zj, int which);
double VertResponse(double d0, double d1, double diffusivity, double currTime);
double VertIntegral(double xi, double yi, double zi, double xj, double yj, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime);
double VertDoubleIntegral(double xi, double yi, double xj, double yj, int same, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime);
int VertNumLntts(double bhLength, double diffusivity, double maxSimYears);
double VertLntts(double bhLength, double diffusivity, double maxSimYears, int k);
double VertGFunction(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double maxSimYears, int k);
double VertFrictionFactor(double reynoldsNum);
double VertPipeConductionResistance(double outDia, double innerDia, double k);
double VertTimeSS(double bhLength, double diffusivity);

#endif
