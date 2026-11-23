// The predload unit as rust-anything sees it: the function forwards to the real EnergyPlus
// ZoneTempPredictorCorrector ZoneSpaceHeatBalanceData::calcPredictedSystemLoad through
// libep_predload_shim (shim/ep_predload_shim.cc), which sets up one zone (see the shim).
#include "predload.h"

double ep_PredictedSystemLoad(int which, int ctrl, int algo, double rafnfrac, double tempdep, double tempind, double aircap, double t1, double setpt, double setptlo, double setpthi, int hasite, double itetemp, int beginsim, int hasnode, double nodetemp, int staged, int stagenum, double setptlast, double lcf, int mult, int listmult);

double PredictedSystemLoad(int which, int ctrl, int algo, double rafnfrac, double tempdep, double tempind, double aircap, double t1, double setpt, double setptlo, double setpthi, int hasite, double itetemp, int beginsim, int hasnode, double nodetemp, int staged, int stagenum, double setptlast, double lcf, int mult, int listmult)
{
    return ep_PredictedSystemLoad(which, ctrl, algo, rafnfrac, tempdep, tempind, aircap, t1, setpt, setptlo, setpthi, hasite, itetemp, beginsim, hasnode, nodetemp, staged, stagenum, setptlast, lcf, mult, listmult);
}

