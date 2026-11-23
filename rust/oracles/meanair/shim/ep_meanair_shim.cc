// Written by hand: HeatBalanceAirManager::calcMeanAirTemps reads the zone's operative temperature
// control and the barometric pressure from the EnergyPlusData state, which gen_plain.py cannot
// set up. Plain extern "C" entry point over the real EnergyPlus HeatBalanceAirManager code.
//
// Each call reports zone 1, whose air averaged over the zone time step is at ztav with humidity
// ratio w and whose mean radiant temperature is mrt, at barometric pressure outbaro. When anyop
// is 1 the model has operative temperature control, and zone 1 is controlled when controlled is
// 1, with thermostat 1's operative temperature control opctrl (DataZoneControls::TempCtrl: 0 none,
// 1 constant, 2 scheduled) and radiative fraction frac (fixed or as a schedule). The wet bulb
// globe temperature is reported when wbgt is 1. Then the real calcMeanAirTemps runs for zone 1's
// report variables, which start at 0. One EnergyPlusData for the process, built when the library
// loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 5; opctrl outside 0 to 2; a flag outside
// 0 or 1; and the fatal errors EnergyPlus stops on.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataZoneControls.hh>
#include <EnergyPlus/HeatBalanceAirManager.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/UtilityRoutines.hh>

using namespace EnergyPlus;

namespace {
Sched::Schedule *fracSched;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataHeatBal->Zone(1).TempControlledZoneIndex = 1;
        p->dataHeatBal->ZnAirRpt.allocate(1);
        p->dataZoneCtrls->TempControlledZone.allocate(1);
        fracSched = Sched::AddScheduleConstant(*p, "RADIATIVE FRACTION");
        p->dataZoneCtrls->TempControlledZone(1).opTempRadiativeFractionSched = fracSched;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One report variable of zone 1 after calcMeanAirTemps (see the file comment): which picks
// MeanAirTemp (0), MeanAirHumRat (1), OperativeTemp (2), MeanAirDewPointTemp (3),
// ThermOperativeTemp (4) and WetbulbGlobeTemp (5).
double ep_MeanAirTemps(int which, double ztav, double w, double mrt, double outbaro, int anyop, int controlled, int opctrl, double frac, int wbgt)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 5 || opctrl < 0 || opctrl > 2 || !flag(anyop) || !flag(controlled) || !flag(wbgt)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataEnvrn->OutBaroPress = outbaro;
    state.dataZoneCtrls->AnyOpTempControl = anyop == 1;
    state.dataHeatBal->Zone(1).IsControlled = controlled == 1;
    auto &tcz = state.dataZoneCtrls->TempControlledZone(1);
    tcz.OpTempCtrl = static_cast<DataZoneControls::TempCtrl>(opctrl);
    tcz.FixedRadiativeFraction = frac;
    fracSched->currentVal = frac;
    auto &rpt = state.dataHeatBal->ZnAirRpt(1);
    rpt = DataHeatBalance::AirReportVars();
    rpt.ReportWBGT = wbgt == 1;

    try {
        HeatBalanceAirManager::calcMeanAirTemps(state, ztav, w, mrt, rpt, 1);
    } catch (FatalError const &) {
        return NAN;
    }

    switch (which) {
    case 0:
        return rpt.MeanAirTemp;
    case 1:
        return rpt.MeanAirHumRat;
    case 2:
        return rpt.OperativeTemp;
    case 3:
        return rpt.MeanAirDewPointTemp;
    case 4:
        return rpt.ThermOperativeTemp;
    default:
        return rpt.WetbulbGlobeTemp;
    }
}
}
