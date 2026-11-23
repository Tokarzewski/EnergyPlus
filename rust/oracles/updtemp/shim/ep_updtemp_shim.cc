// Written by hand: ZoneSpaceHeatBalanceData::updateTemperatures reads the zone's air histories,
// system node and system time step counts from the EnergyPlusData state, which gen_plain.py cannot
// set up. Plain extern "C" entry points over the real EnergyPlus ZoneTempPredictorCorrector code.
//
// UpdateTemperatures sets up zone 1, fully mixed, with system node 1 when hasnode is 1. The zone
// air is at mat with humidity ratio w; x0 to x3 and wp0 to wp3 are its zone time step histories
// (XMAT, WPrevZoneTS), ds0 to ds3 and dsw0 to dsw3 its down-stepped ones (DSXMAT, DSWPrevZoneTS).
// The system time step is tsys hours after a prior one of prior hours, with nsys system time steps
// now and nsyslast in the last zone time step. Then the real updateTemperatures runs for zone 1 (no
// space), shortening the time step when shorten is 1 and using the zone time step history when
// usezonehist is 1. DownInterpolate5 calls the five-value DownInterpolate4HistoryValues that the
// contaminant predictor uses. One EnergyPlusData for the process, built when the library loads,
// with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 21 (UpdateTemperatures) or 0 to 4
// (DownInterpolate5), and a flag outside 0 or 1.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataRoomAirModel.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataHeatBalFanSys->TempTstatAir.dimension(1, 0.0);
        p->dataLoopNodes->Node.allocate(1);
        p->dataRoomAir->anyNonMixingRoomAirModel = false;
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of updateTemperatures for zone 1 (see the file comment): which picks MAT (0),
// airHumRat (1), DSXMAT[0..3] (2 to 5), DSWPrevZoneTS[0..3] (6 to 9), ZTM[0..3] (10 to 13),
// WPrevZoneTSTemp[0..3] (14 to 17), and the system node's Temp (18), HumRat (19) and Enthalpy (20)
// and the thermostat air temperature (21), these four starting at 0.
double ep_UpdateTemperatures(int which,
                             int shorten,
                             int usezonehist,
                             double prior,
                             double tsys,
                             int nsys,
                             int nsyslast,
                             int hasnode,
                             double mat,
                             double w,
                             double x0,
                             double x1,
                             double x2,
                             double x3,
                             double ds0,
                             double ds1,
                             double ds2,
                             double ds3,
                             double wp0,
                             double wp1,
                             double wp2,
                             double wp3,
                             double dsw0,
                             double dsw1,
                             double dsw2,
                             double dsw3)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 21 || !flag(shorten) || !flag(usezonehist) || !flag(hasnode)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataHeatBal->Zone(1).SystemZoneNodeNumber = hasnode == 1 ? 1 : 0;
    state.dataHVACGlobal->TimeStepSys = tsys;
    state.dataHVACGlobal->NumOfSysTimeSteps = nsys;
    state.dataHVACGlobal->NumOfSysTimeStepsLastZoneTimeStep = nsyslast;
    auto &node = state.dataLoopNodes->Node(1);
    node.Temp = 0.0;
    node.HumRat = 0.0;
    node.Enthalpy = 0.0;
    state.dataHeatBalFanSys->TempTstatAir(1) = 0.0;
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.MAT = mat;
    zhb.airHumRat = w;
    zhb.XMAT = {x0, x1, x2, x3};
    zhb.DSXMAT = {ds0, ds1, ds2, ds3};
    zhb.WPrevZoneTS = {wp0, wp1, wp2, wp3};
    zhb.DSWPrevZoneTS = {dsw0, dsw1, dsw2, dsw3};
    zhb.ZTM = {0.0, 0.0, 0.0, 0.0};
    zhb.WPrevZoneTSTemp = {0.0, 0.0, 0.0, 0.0};

    zhb.updateTemperatures(state, shorten == 1, usezonehist == 1, prior, 1);

    if (which >= 2 && which <= 17) {
        int const i = (which - 2) % 4;
        switch ((which - 2) / 4) {
        case 0:
            return zhb.DSXMAT[i];
        case 1:
            return zhb.DSWPrevZoneTS[i];
        case 2:
            return zhb.ZTM[i];
        default:
            return zhb.WPrevZoneTSTemp[i];
        }
    }
    switch (which) {
    case 0:
        return zhb.MAT;
    case 1:
        return zhb.airHumRat;
    case 18:
        return node.Temp;
    case 19:
        return node.HumRat;
    case 20:
        return node.Enthalpy;
    default:
        return state.dataHeatBalFanSys->TempTstatAir(1);
    }
}

// One of the five new history values (which 0 to 4) that DownInterpolate4HistoryValues
// interpolates from the old values v0 to v2 at time step old (hours) for the time step new.
double ep_DownInterpolate5(int which, double old, double nw, double v0, double v1, double v2)
{
    if (which < 0 || which > 4) {
        return NAN;
    }
    double n[5] = {0.0, 0.0, 0.0, 0.0, 0.0};
    ZoneTempPredictorCorrector::DownInterpolate4HistoryValues(old, nw, v0, v1, v2, n[0], n[1], n[2], n[3], n[4]);
    return n[which];
}
}
