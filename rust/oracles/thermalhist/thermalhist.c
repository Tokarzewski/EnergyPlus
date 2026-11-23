// The thermalhist unit as rust-anything sees it: the function forwards to the real EnergyPlus
// HeatBalanceSurfaceManager UpdateThermalHistories through libep_thermalhist_shim
// (shim/ep_thermalhist_shim.cc), which sets up one surface's CTF histories (see the shim).
#include "thermalhist.h"

double ep_ThermalHist(int which, int seed, int n, int m, int simple, int overrides, int anysrc, int src, int algo, int curnum, int numhist, double tsz, double ctfts, double area, double tout, double tin, double ctfin, double ctfout, double qsrc, double tsrcconst, double tuserconst, double cross0, double inside0, double outside0, double srcin0, double srcout0, double tsrcout0, double tsrcin0, double tsrcq0, double tuserout0, double tuserin0, double tusersrc0);

double ThermalHist(int which, int seed, int n, int m, int simple, int overrides, int anysrc, int src, int algo, int curnum, int numhist, double tsz, double ctfts, double area, double tout, double tin, double ctfin, double ctfout, double qsrc, double tsrcconst, double tuserconst, double cross0, double inside0, double outside0, double srcin0, double srcout0, double tsrcout0, double tsrcin0, double tsrcq0, double tuserout0, double tuserin0, double tusersrc0)
{
    return ep_ThermalHist(which, seed, n, m, simple, overrides, anysrc, src, algo, curnum, numhist, tsz, ctfts, area, tout, tin, ctfin, ctfout, qsrc, tsrcconst, tuserconst, cross0, inside0, outside0, srcin0, srcout0, tsrcout0, tsrcin0, tsrcq0, tuserout0, tuserin0, tusersrc0);
}

