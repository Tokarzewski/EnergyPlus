// The zonesums unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector calcZoneOrSpaceSums through
// libep_zonesums_shim (shim/ep_zonesums_shim.cc), which sets up one zone (see the shim).
#include "zonesums.h"

double ep_ZoneSums(int which, int corrector, int kind, int ndev, double cg1, double rg1, double cg2, double rg2, double cg3, double rg3, double radsys, double pool, int noheatret, double mcpi, double mcpti, double mcpv, double mcptv, double mcpm, double mcptm, double mcpe, double mcpte, double mcpc, double mcptc, double mdotcpoa, double outdb, double airhumrat, int ninlets, double m1, double t1, double m2, double t2, double m3, double t3, int hasadu, int upleak, double mup, double tup, int dnleak, double mdn, double tdn, int haspiu, double leak, double tpri, int hasnode, int mult, int listmult);

double ZoneSums(int which, int corrector, int kind, int ndev, double cg1, double rg1, double cg2, double rg2, double cg3, double rg3, double radsys, double pool, int noheatret, double mcpi, double mcpti, double mcpv, double mcptv, double mcpm, double mcptm, double mcpe, double mcpte, double mcpc, double mcptc, double mdotcpoa, double outdb, double airhumrat, int ninlets, double m1, double t1, double m2, double t2, double m3, double t3, int hasadu, int upleak, double mup, double tup, int dnleak, double mdn, double tdn, int haspiu, double leak, double tpri, int hasnode, int mult, int listmult)
{
    return ep_ZoneSums(which, corrector, kind, ndev, cg1, rg1, cg2, rg2, cg3, rg3, radsys, pool, noheatret, mcpi, mcpti, mcpv, mcptv, mcpm, mcptm, mcpe, mcpte, mcpc, mcptc, mdotcpoa, outdb, airhumrat, ninlets, m1, t1, m2, t2, m3, t3, hasadu, upleak, mup, tup, dnleak, mdn, tdn, haspiu, leak, tpri, hasnode, mult, listmult);
}

