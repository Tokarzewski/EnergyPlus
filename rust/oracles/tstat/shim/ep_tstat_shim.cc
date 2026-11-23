// Written by hand: ZoneTempPredictorCorrector::CalcZoneAirTempSetPoints reads the thermostats,
// their schedules, the operative temperature and overcool controls and the EMS overrides from the
// EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry point over the
// real EnergyPlus ZoneTempPredictorCorrector code.
//
// Each call sets up zone 1 with one thermostat whose control type schedule is at ctype
// (HVAC::SetptType, 0 to 4). Its single heating set point schedule is at heatsp (used when
// heatused is 1), single cooling at coolsp, single heating and cooling at hcsp, and dual set point
// heating and cooling at dhsp and dcsp. The zone's set points start at setpt0, hi0 and lo0. When
// anyop is 1 the model has operative temperature control and this thermostat uses it as opctrl
// says (DataZoneControls::TempCtrl, 0 none, 1 constant, 2 scheduled) with radiative fraction frac
// against the zone mean radiant temperature mrt. When anyhum is 1 the model has temperature and
// humidity control and this thermostat overcools as ovctrl says (0 to 2) by up to ovrange with
// control ratio ovratio, the zone air at relative humidity relhum against the dehumidifying set
// point dehum. EMS overrides the heating set point with emsheatval when emsheat is 1 and the
// cooling one with emscoolval when emscool is 1. No adaptive comfort, optimum start, faulty
// thermostats or comfort control. Then the real CalcZoneAirTempSetPoints runs. One EnergyPlusData
// for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 5; ctype outside 0 to 4 (EnergyPlus
// reports an illegal control type, which needs the error file); opctrl or ovctrl outside 0 to 2;
// a flag outside 0 or 1.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataZoneControls.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
// The control type, single heating, single cooling, single heating and cooling, dual heating,
// dual cooling, radiative fraction, overcool range and dehumidifying schedules.
Sched::Schedule *sched[9];

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataHeatBal->Zone(1).TempControlledZoneIndex = 1;
        p->dataHeatBalFanSys->zoneTstatSetpts.allocate(1);
        p->dataHeatBalFanSys->TempControlType.allocate(1);
        p->dataHeatBalFanSys->TempControlTypeRpt.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        auto &zc = *p->dataZoneCtrls;
        zc.NumTempControlledZones = 1;
        zc.TempControlledZone.allocate(1);
        auto &tz = zc.TempControlledZone(1);
        tz.Name = "TSTAT 1";
        tz.ActualZoneNum = 1;
        char const *names[9] = {"CONTROL TYPE", "HEAT", "COOL", "HEAT COOL", "DUAL HEAT", "DUAL COOL", "RADIATIVE FRACTION", "OVERCOOL", "DEHUM"};
        for (int i = 0; i < 9; ++i) {
            sched[i] = Sched::AddScheduleConstant(*p, names[i]);
        }
        tz.setptTypeSched = sched[0];
        tz.setpts[(int)HVAC::SetptType::SingleHeat].heatSetptSched = sched[1];
        tz.setpts[(int)HVAC::SetptType::SingleCool].coolSetptSched = sched[2];
        tz.setpts[(int)HVAC::SetptType::SingleHeatCool].heatSetptSched = sched[3];
        tz.setpts[(int)HVAC::SetptType::DualHeatCool].heatSetptSched = sched[4];
        tz.setpts[(int)HVAC::SetptType::DualHeatCool].coolSetptSched = sched[5];
        tz.opTempRadiativeFractionSched = sched[6];
        tz.zoneOvercoolRangeSched = sched[7];
        tz.dehumidifyingSched = sched[8];
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result for zone 1 after CalcZoneAirTempSetPoints (see the file comment): which picks the
// zone's set point (0), high (1) and low (2) set points, the thermostat's high (3) and low (4) set
// points, both starting at 0, and the reported control type (5).
double ep_TstatSetPoints(int which,
                         int ctype,
                         int heatused,
                         double heatsp,
                         double coolsp,
                         double hcsp,
                         double dhsp,
                         double dcsp,
                         double setpt0,
                         double hi0,
                         double lo0,
                         int anyop,
                         int opctrl,
                         double frac,
                         double mrt,
                         int anyhum,
                         int ovctrl,
                         double ovrange,
                         double ovratio,
                         double relhum,
                         double dehum,
                         int emsheat,
                         double emsheatval,
                         int emscool,
                         double emscoolval)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 5 || ctype < 0 || ctype > 4 || opctrl < 0 || opctrl > 2 || ovctrl < 0 || ovctrl > 2 || !flag(heatused) ||
        !flag(anyop) || !flag(anyhum) || !flag(emsheat) || !flag(emscool)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    double const vals[9] = {double(ctype), heatsp, coolsp, hcsp, dhsp, dcsp, frac, ovrange, dehum};
    for (int i = 0; i < 9; ++i) {
        sched[i]->currentVal = vals[i];
    }
    auto &zc = *state.dataZoneCtrls;
    zc.AnyOpTempControl = anyop == 1;
    zc.AnyZoneTempAndHumidityControl = anyhum == 1;
    auto &tz = zc.TempControlledZone(1);
    tz.setpts[(int)HVAC::SetptType::SingleHeat].isUsed = heatused == 1;
    tz.OpTempCtrl = static_cast<DataZoneControls::TempCtrl>(opctrl);
    tz.FixedRadiativeFraction = frac;
    tz.OvercoolCtrl = static_cast<DataZoneControls::TempCtrl>(ovctrl);
    tz.ZoneOvercoolConstRange = ovrange;
    tz.ZoneOvercoolControlRatio = ovratio;
    tz.EMSOverrideHeatingSetPointOn = emsheat == 1;
    tz.EMSOverrideHeatingSetPointValue = emsheatval;
    tz.EMSOverrideCoolingSetPointOn = emscool == 1;
    tz.EMSOverrideCoolingSetPointValue = emscoolval;
    tz.ZoneThermostatSetPointHi = 0.0;
    tz.ZoneThermostatSetPointLo = 0.0;
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.MRT = mrt;
    zhb.airRelHum = relhum;
    auto &sp = state.dataHeatBalFanSys->zoneTstatSetpts(1);
    sp.setpt = setpt0;
    sp.setptHi = hi0;
    sp.setptLo = lo0;

    ZoneTempPredictorCorrector::CalcZoneAirTempSetPoints(state);

    switch (which) {
    case 0:
        return sp.setpt;
    case 1:
        return sp.setptHi;
    case 2:
        return sp.setptLo;
    case 3:
        return tz.ZoneThermostatSetPointHi;
    case 4:
        return tz.ZoneThermostatSetPointLo;
    default:
        return state.dataHeatBalFanSys->TempControlTypeRpt(1);
    }
}
}
