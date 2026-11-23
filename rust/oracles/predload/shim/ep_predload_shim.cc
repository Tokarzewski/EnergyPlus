// Written by hand: ZoneSpaceHeatBalanceData::calcPredictedSystemLoad reads the zone's thermostat,
// solution algorithm, staging and node from the EnergyPlusData state, which gen_plain.py cannot
// set up. Plain extern "C" entry point over the real EnergyPlus ZoneTempPredictorCorrector code.
//
// Each call sets up zone 1 with thermostat type ctrl (HVAC::SetptType, -1 to 4) and set points
// setpt, setptlo and setpthi, the zone air solution algorithm algo (DataHeatBalance::SolutionAlgo,
// 0 to 2), the zone air heat balance terms tempdep, tempind, aircap and t1, and the last set point
// setptlast. When hasite is 1 the zone's return air is adjusted by IT equipment to itetemp, which
// counts only when beginsim is 0 (BeginSimFlag). When hasnode is 1 the zone has system node 1 at
// nodetemp. When staged is 1 the zone has a staged thermostat at stage stagenum. The zone has
// load correction factor lcf, multiplier mult and list multiplier listmult. Then the real
// calcPredictedSystemLoad runs for zone 1 (no space) with RoomAirflowNetwork fraction rafnfrac.
// One EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 9; a flag outside 0 or 1; ctrl outside
// -1 to 4; algo outside 0 to 2 (EnergyPlus asserts on it); mult or listmult outside 1 to 10000
// (their int product must not overflow); and the fatal errors EnergyPlus stops on, a heating load
// above the cooling load (or loads that compare as neither) under a single heating and cooling or
// dual set point thermostat.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataZoneControls.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataHeatBalFanSys->zoneTstatSetpts.allocate(1);
        p->dataHeatBalFanSys->TempControlType.allocate(1);
        p->dataHeatBalFanSys->LoadCorrectionFactor.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        p->dataZoneEnergyDemand->ZoneSysEnergyDemand.allocate(1);
        p->dataZoneEnergyDemand->Setback.dimension(1, false);
        p->dataZoneEnergyDemand->DeadBandOrSetback.dimension(1, false);
        p->dataZoneEnergyDemand->CurDeadBandOrSetback.dimension(1, false);
        p->dataZoneCtrls->StageZoneLogic.dimension(1, false);
        p->dataLoopNodes->Node.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of calcPredictedSystemLoad for zone 1 (see the file comment): which picks the zone
// system demand's predictedRate (0), predictedHSPRate (1), predictedCSPRate (2),
// TotalOutputRequired (3), OutputRequiredToHeatingSP (4) and OutputRequiredToCoolingSP (5), the
// thermostat set point after the call (6), the system node's TempSetPoint (7, starting at 0),
// Setback (8) and DeadBandOrSetback (9) as 0 or 1.
double ep_PredictedSystemLoad(int which,
                              int ctrl,
                              int algo,
                              double rafnfrac,
                              double tempdep,
                              double tempind,
                              double aircap,
                              double t1,
                              double setpt,
                              double setptlo,
                              double setpthi,
                              int hasite,
                              double itetemp,
                              int beginsim,
                              int hasnode,
                              double nodetemp,
                              int staged,
                              int stagenum,
                              double setptlast,
                              double lcf,
                              int mult,
                              int listmult)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 9 || !flag(hasite) || !flag(beginsim) || !flag(hasnode) || !flag(staged) || ctrl < -1 || ctrl > 4 || algo < 0 ||
        algo > 2 || mult < 1 || mult > 10000 || listmult < 1 || listmult > 10000) {
        return NAN;
    }
    EnergyPlusData &state = st();
    auto &zone = state.dataHeatBal->Zone(1);
    zone.HasAdjustedReturnTempByITE = hasite == 1;
    zone.AdjustedReturnTempByITE = itetemp;
    zone.SystemZoneNodeNumber = hasnode == 1 ? 1 : 0;
    zone.Multiplier = mult;
    zone.ListMultiplier = listmult;
    state.dataGlobal->BeginSimFlag = beginsim == 1;
    state.dataHeatBal->ZoneAirSolutionAlgo = static_cast<DataHeatBalance::SolutionAlgo>(algo);
    auto &fs = *state.dataHeatBalFanSys;
    fs.TempControlType(1) = static_cast<HVAC::SetptType>(ctrl);
    fs.zoneTstatSetpts(1).setpt = setpt;
    fs.zoneTstatSetpts(1).setptLo = setptlo;
    fs.zoneTstatSetpts(1).setptHi = setpthi;
    fs.LoadCorrectionFactor(1) = lcf;
    state.dataLoopNodes->Node(1).Temp = nodetemp;
    state.dataLoopNodes->Node(1).TempSetPoint = 0.0;
    state.dataZoneTempPredictorCorrector->NumStageCtrZone = staged;
    state.dataZoneCtrls->StageZoneLogic(1) = staged == 1;
    auto &dem = *state.dataZoneEnergyDemand;
    dem.ZoneSysEnergyDemand(1).StageNum = stagenum;
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.tempDepLoad = tempdep;
    zhb.tempIndLoad = tempind;
    zhb.AirPowerCap = aircap;
    zhb.T1 = t1;
    zhb.setPointLast = setptlast;

    try {
        zhb.calcPredictedSystemLoad(state, rafnfrac, 1);
    } catch (FatalError const &) {
        return NAN;
    }

    auto const &d = dem.ZoneSysEnergyDemand(1);
    switch (which) {
    case 0:
        return d.predictedRate;
    case 1:
        return d.predictedHSPRate;
    case 2:
        return d.predictedCSPRate;
    case 3:
        return d.TotalOutputRequired;
    case 4:
        return d.OutputRequiredToHeatingSP;
    case 5:
        return d.OutputRequiredToCoolingSP;
    case 6:
        return fs.zoneTstatSetpts(1).setpt;
    case 7:
        return state.dataLoopNodes->Node(1).TempSetPoint;
    case 8:
        return dem.Setback(1) ? 1.0 : 0.0;
    default:
        return dem.DeadBandOrSetback(1) ? 1.0 : 0.0;
    }
}
}
