// The setflow unit as rust-anything sees it: the function forwards to the real EnergyPlus
// PlantUtilities SetComponentFlowRate through
// libep_setflow_shim (shim/ep_setflow_shim.cc), (see the shim).
#include "setflow.h"

double ep_SetFlow(int which, int ncomp, int k, int lock, int autosize, int series, int demand, int ems, double flow, double mdot1, double min1, double max1, double minav1, double maxav1, double req1, double mdot2, double min2, double max2, double minav2, double maxav2, double req2, double mdot3, double min3, double max3, double minav3, double maxav3, double req3, double mdot4, double min4, double max4, double minav4, double maxav4, double req4);

double SetFlow(int which, int ncomp, int k, int lock, int autosize, int series, int demand, int ems, double flow, double mdot1, double min1, double max1, double minav1, double maxav1, double req1, double mdot2, double min2, double max2, double minav2, double maxav2, double req2, double mdot3, double min3, double max3, double minav3, double maxav3, double req3, double mdot4, double min4, double max4, double minav4, double maxav4, double req4)
{
    return ep_SetFlow(which, ncomp, k, lock, autosize, series, demand, ems, flow, mdot1, min1, max1, minav1, maxav1, req1, mdot2, min2, max2, minav2, maxav2, req2, mdot3, min3, max3, minav3, maxav3, req3, mdot4, min4, max4, minav4, maxav4, req4);
}

