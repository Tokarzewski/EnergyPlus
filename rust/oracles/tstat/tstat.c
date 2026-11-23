// The tstat unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector CalcZoneAirTempSetPoints through
// libep_tstat_shim (shim/ep_tstat_shim.cc), which sets up one thermostat (see the shim).
#include "tstat.h"

double ep_TstatSetPoints(int which, int ctype, int heatused, double heatsp, double coolsp, double hcsp, double dhsp, double dcsp, double setpt0, double hi0, double lo0, int anyop, int opctrl, double frac, double mrt, int anyhum, int ovctrl, double ovrange, double ovratio, double relhum, double dehum, int emsheat, double emsheatval, int emscool, double emscoolval);

double TstatSetPoints(int which, int ctype, int heatused, double heatsp, double coolsp, double hcsp, double dhsp, double dcsp, double setpt0, double hi0, double lo0, int anyop, int opctrl, double frac, double mrt, int anyhum, int ovctrl, double ovrange, double ovratio, double relhum, double dehum, int emsheat, double emsheatval, int emscool, double emscoolval)
{
    return ep_TstatSetPoints(which, ctype, heatused, heatsp, coolsp, hcsp, dhsp, dcsp, setpt0, hi0, lo0, anyop, opctrl, frac, mrt, anyhum, ovctrl, ovrange, ovratio, relhum, dehum, emsheat, emsheatval, emscool, emscoolval);
}

