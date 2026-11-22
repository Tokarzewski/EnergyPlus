// The vert unit as rust-anything sees it: each function forwards to the real EnergyPlus GLHEVert
// code through libep_vert_shim (shim/ep_vert_shim.cc). VertGFunction is the k-th uniform heat
// flux g-function of an nx by ny field of boreholes `spacing` apart.
#include "vert.h"

double ep_VertDistance(double xi, double yi, double zi, double xj, double yj, double zj, int which);
double ep_VertResponse(double d0, double d1, double diffusivity, double currTime);
double ep_VertIntegral(double xi, double yi, double zi, double xj, double yj, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime);
double ep_VertDoubleIntegral(double xi, double yi, double xj, double yj, int same, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime);
int ep_VertNumLntts(double bhLength, double diffusivity, double maxSimYears);
double ep_VertLntts(double bhLength, double diffusivity, double maxSimYears, int k);
double ep_VertGFunction(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double maxSimYears, int k);
double ep_VertFrictionFactor(double reynoldsNum);
double ep_VertPipeConductionResistance(double outDia, double innerDia, double k);
double ep_VertTimeSS(double bhLength, double diffusivity);

double VertDistance(double xi, double yi, double zi, double xj, double yj, double zj, int which)
{
    return ep_VertDistance(xi, yi, zi, xj, yj, zj, which);
}

double VertResponse(double d0, double d1, double diffusivity, double currTime)
{
    return ep_VertResponse(d0, d1, diffusivity, currTime);
}

double VertIntegral(double xi, double yi, double zi, double xj, double yj, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime)
{
    return ep_VertIntegral(xi, yi, zi, xj, yj, bhLength, bhTopDepth, bhDiameter, diffusivity, currTime);
}

double VertDoubleIntegral(double xi, double yi, double xj, double yj, int same, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double currTime)
{
    return ep_VertDoubleIntegral(xi, yi, xj, yj, same, bhLength, bhTopDepth, bhDiameter, diffusivity, currTime);
}

int VertNumLntts(double bhLength, double diffusivity, double maxSimYears)
{
    return ep_VertNumLntts(bhLength, diffusivity, maxSimYears);
}

double VertLntts(double bhLength, double diffusivity, double maxSimYears, int k)
{
    return ep_VertLntts(bhLength, diffusivity, maxSimYears, k);
}

double VertGFunction(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity, double maxSimYears, int k)
{
    return ep_VertGFunction(nx, ny, spacing, bhLength, bhTopDepth, bhDiameter, diffusivity, maxSimYears, k);
}

double VertFrictionFactor(double reynoldsNum)
{
    return ep_VertFrictionFactor(reynoldsNum);
}

double VertPipeConductionResistance(double outDia, double innerDia, double k)
{
    return ep_VertPipeConductionResistance(outDia, innerDia, k);
}

double VertTimeSS(double bhLength, double diffusivity)
{
    return ep_VertTimeSS(bhLength, diffusivity);
}

