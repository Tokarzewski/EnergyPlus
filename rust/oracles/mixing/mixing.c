// The mixing unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneEquipmentManager CalcAirFlowSimple zone mixing through
// libep_mixing_shim (shim/ep_mixing_shim.cc), which sets up two zones (see the shim).
#include "mixing.h"

double ep_Mixing(int which, int nmix, int hybrid, double saved, double td, int hastd, int hmint, double mint, int hmaxt, double maxt, int hmins, double mins, int hmaxs, double maxs, int hmino, double mino, int hmaxo, double maxo, int systs, double mat1, double hum1, double xmpt1, double wprev1, double mat2, double hum2, double xmpt2, double wprev2, double outdb, double outbaro);

double Mixing(int which, int nmix, int hybrid, double saved, double td, int hastd, int hmint, double mint, int hmaxt, double maxt, int hmins, double mins, int hmaxs, double maxs, int hmino, double mino, int hmaxo, double maxo, int systs, double mat1, double hum1, double xmpt1, double wprev1, double mat2, double hum2, double xmpt2, double wprev2, double outdb, double outbaro)
{
    return ep_Mixing(which, nmix, hybrid, saved, td, hastd, hmint, mint, hmaxt, maxt, hmins, mins, hmaxs, maxs, hmino, mino, hmaxo, maxo, systs, mat1, hum1, xmpt1, wprev1, mat2, hum2, xmpt2, wprev2, outdb, outbaro);
}

