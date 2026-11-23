// The spmfollow unit as rust-anything sees it: each function forwards to the real EnergyPlus
// SetPointManager calculations through
// libep_spmfollow_shim (shim/ep_spmfollow_shim.cc), (see the shim).
#include "spmfollow.h"

double ep_OneStage(int heating, int stage, double on, double off);
double ep_SZHum(int maxhum, double zonem, double zonew, double hum, double dehum);
double ep_Pretreat(int ctrl, double refsp, double retv, double mixm, double oam, double minsp, double maxsp);
double ep_FollowOA(int wet, double oadb, double oawb, double offset, double mint, double maxt);
double ep_FollowNode(int wet, int more, double nodet, double nodewb, double offset, double mint, double maxt);

double OneStage(int heating, int stage, double on, double off)
{
    return ep_OneStage(heating, stage, on, off);
}

double SZHum(int maxhum, double zonem, double zonew, double hum, double dehum)
{
    return ep_SZHum(maxhum, zonem, zonew, hum, dehum);
}

double Pretreat(int ctrl, double refsp, double retv, double mixm, double oam, double minsp, double maxsp)
{
    return ep_Pretreat(ctrl, refsp, retv, mixm, oam, minsp, maxsp);
}

double FollowOA(int wet, double oadb, double oawb, double offset, double mint, double maxt)
{
    return ep_FollowOA(wet, oadb, oawb, offset, mint, maxt);
}

double FollowNode(int wet, int more, double nodet, double nodewb, double offset, double mint, double maxt)
{
    return ep_FollowNode(wet, more, nodet, nodewb, offset, mint, maxt);
}

