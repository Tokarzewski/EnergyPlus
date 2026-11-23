// Written by hand alongside shim/ep_spmfollow_shim.cc.
#ifndef EP_SPMFOLLOW_H
#define EP_SPMFOLLOW_H

double OneStage(int heating, int stage, double on, double off);
double SZHum(int maxhum, double zonem, double zonew, double hum, double dehum);
double Pretreat(int ctrl, double refsp, double retv, double mixm, double oam, double minsp, double maxsp);
double FollowOA(int wet, double oadb, double oawb, double offset, double mint, double maxt);
double FollowNode(int wet, int more, double nodet, double nodewb, double offset, double mint, double maxt);

#endif
