// The xmix unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneEquipmentManager CalcAirFlowSimple cross mixing through
// libep_xmix_shim (shim/ep_xmix_shim.cc), which sets up two zones (see the shim).
#include "xmix.h"

double ep_CrossMixing(int which, int nmix, double desired, double td, int hastd, int hmint, double mint, int hmaxt, double maxt, int hmins, double mins, int hmaxs, double maxs, int hmino, double mino, int hmaxo, double maxo, int systs, double mat1, double hum1, double xmpt1, double wprev1, double mat2, double hum2, double xmpt2, double wprev2, double outdb, double outbaro);

double CrossMixing(int which, int nmix, double desired, double td, int hastd, int hmint, double mint, int hmaxt, double maxt, int hmins, double mins, int hmaxs, double maxs, int hmino, double mino, int hmaxo, double maxo, int systs, double mat1, double hum1, double xmpt1, double wprev1, double mat2, double hum2, double xmpt2, double wprev2, double outdb, double outbaro)
{
    return ep_CrossMixing(which, nmix, desired, td, hastd, hmint, mint, hmaxt, maxt, hmins, mins, hmaxs, maxs, hmino, mino, hmaxo, maxo, systs, mat1, hum1, xmpt1, wprev1, mat2, hum2, xmpt2, wprev2, outdb, outbaro);
}

