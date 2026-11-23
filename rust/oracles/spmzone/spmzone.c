// The spmzone unit as rust-anything sees it: each function forwards to the real EnergyPlus
// SetPointManager SPMSingleZoneTemp and SPMOutsideAir calculate through
// libep_spmzone_shim (shim/ep_spmzone_shim.cc), (see the shim).
#include "spmzone.h"

double ep_SZTemp(int heating, double zonem, double inw, double toheat, double tocool, double zonet, double mint, double maxt);
double ep_OAReset(int schedv, double oat, double low1, double high1, double lsp1, double hsp1, double low2, double high2, double lsp2, double hsp2);

double SZTemp(int heating, double zonem, double inw, double toheat, double tocool, double zonet, double mint, double maxt)
{
    return ep_SZTemp(heating, zonem, inw, toheat, tocool, zonet, mint, maxt);
}

double OAReset(int schedv, double oat, double low1, double high1, double lsp1, double hsp1, double low2, double high2, double lsp2, double hsp2)
{
    return ep_OAReset(schedv, oat, low1, high1, lsp1, hsp1, low2, high2, lsp2, hsp2);
}

