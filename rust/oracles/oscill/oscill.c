// The oscill unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector DetectOscillatingZoneTemp through
// libep_oscill_shim (shim/ep_oscill_shim.cc), which sets up up to three zones (see the shim).
#include "oscill.h"

double ep_Oscillate(int which, int nzones, double tsys, int comfort, double ha1, double hb1, double hc1, double zt1, int occ1, int db1, double ha2, double hb2, double hc2, double zt2, int occ2, int db2, double ha3, double hb3, double hc3, double zt3, int occ3, int db3, double sum1, double sum2, double sum3);

double Oscillate(int which, int nzones, double tsys, int comfort, double ha1, double hb1, double hc1, double zt1, int occ1, int db1, double ha2, double hb2, double hc2, double zt2, int occ2, int db2, double ha3, double hb3, double hc3, double zt3, int occ3, int db3, double sum1, double sum2, double sum3)
{
    return ep_Oscillate(which, nzones, tsys, comfort, ha1, hb1, hc1, zt1, occ1, db1, ha2, hb2, hc2, zt2, occ2, db2, ha3, hb3, hc3, zt3, occ3, db3, sum1, sum2, sum3);
}

