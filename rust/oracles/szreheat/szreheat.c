// The szreheat unit as rust-anything sees it: the function forwards to the real EnergyPlus
// SetPointManager SPMSingleZoneReheat calculate through
// libep_szreheat_shim (shim/ep_szreheat_shim.cc), (see the shim).
#include "szreheat.h"

double ep_SZReheat(int hasoa, int hasfan, int deadband, double oafrac, double zonem, double inw, double load, double tocool, double toheat, double zonet, double retw, double reth, double oaw, double oah, double loopt, double fanin, double fanout, double mint, double maxt);

double SZReheat(int hasoa, int hasfan, int deadband, double oafrac, double zonem, double inw, double load, double tocool, double toheat, double zonet, double retw, double reth, double oaw, double oah, double loopt, double fanin, double fanout, double mint, double maxt)
{
    return ep_SZReheat(hasoa, hasfan, deadband, oafrac, zonem, inw, load, tocool, toheat, zonet, retw, reth, oaw, oah, loopt, fanin, fanout, mint, maxt);
}

