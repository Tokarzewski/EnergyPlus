// The sumhat unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector SpaceHeatBalanceData::calcSumHAT through libep_sumhat_shim
// (shim/ep_sumhat_shim.cc), which sets up one space with one surface (see the shim).
#include "sumhat.h"

double ep_SumHAT(int which, int window, int shading, int eql, double area, double hconv, double tin, int tairref, double tempeff, double dividerarea, double dividergain, double projdiv, double dividertemp, double framearea, double projfr, double frametemp, double otherconv, double natconv, double airflow, double convgain, double retgain, int noheatret, double heatgain0, int controlled, double sysmcp, double sysmcpt, double tszsec);

double SumHAT(int which, int window, int shading, int eql, double area, double hconv, double tin, int tairref, double tempeff, double dividerarea, double dividergain, double projdiv, double dividertemp, double framearea, double projfr, double frametemp, double otherconv, double natconv, double airflow, double convgain, double retgain, int noheatret, double heatgain0, int controlled, double sysmcp, double sysmcpt, double tszsec)
{
    return ep_SumHAT(which, window, shading, eql, area, hconv, tin, tairref, tempeff, dividerarea, dividergain, projdiv, dividertemp, framearea, projfr, frametemp, otherconv, natconv, airflow, convgain, retgain, noheatret, heatgain0, controlled, sysmcp, sysmcpt, tszsec);
}

