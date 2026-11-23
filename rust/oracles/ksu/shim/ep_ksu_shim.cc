// Written by hand: ThermalComfort::CalcThermalComfortKSU reads a People object (its schedules,
// clothing type and MRT calculation type), the zone's comfort air temperature and humidity ratio,
// the enclosure MRT, the radiant HVAC gains to people and the outdoor barometric pressure, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus ThermalComfort
// code.
//
// Each call sets up one People object in one zone, flagged for the KSU two-node model, with
// constant schedules: activity level act (W per person), work efficiency eff (0 to 1), clothing
// insulation clo (clo, clothing type InsulationSchedule) and air velocity vel (m/s); and the
// enclosure averaged MRT mrt (C). The zone's comfort air temperature is ztav (C) and humidity ratio
// w (kg/kg), the radiant gain from high temperature radiant heaters to people qrad (W), the
// outdoor barometric pressure outbaro (Pa). Then the real CalcThermalComfortKSU runs. One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 2; any input that is not finite or lies
// outside what EnergyPlus meets: outbaro outside 60000 to 110000 Pa, ztav or mrt outside -10 to
// 60 C, w outside 0 to 0.05, qrad outside 0 to 2000 W, act outside 50 to 900 W, eff outside 0 to
// 0.5, clo outside 0 to 3, vel outside 0 to 5 m/s.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataRoomAirModel.hh>
#include <EnergyPlus/DataViewFactorInformation.hh>
#include <EnergyPlus/EPVector.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/ThermalComfort.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
Sched::ScheduleConstant *activity = nullptr;
Sched::ScheduleConstant *workEff = nullptr;
Sched::ScheduleConstant *clothing = nullptr;
Sched::ScheduleConstant *airVel = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto &hb = *p->dataHeatBal;
        hb.TotPeople = 1;
        hb.People.allocate(1);
        hb.space.allocate(1);
        hb.space(1).radiantEnclosureNum = 1;
        auto &people = hb.People(1);
        people.Name = "PEOPLE 1";
        people.ZonePtr = 1;
        people.spaceIndex = 1;
        people.KSU = true;
        people.MRTCalcType = DataHeatBalance::CalcMRT::EnclosureAveraged;
        people.clothingType = DataHeatBalance::ClothingType::InsulationSchedule;
        activity = new Sched::ScheduleConstant;
        workEff = new Sched::ScheduleConstant;
        clothing = new Sched::ScheduleConstant;
        airVel = new Sched::ScheduleConstant;
        people.activityLevelSched = activity;
        people.workEffSched = workEff;
        people.clothingSched = clothing;
        people.airVelocitySched = airVel;
        p->dataViewFactor->EnclRadInfo.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        auto &fs = *p->dataHeatBalFanSys;
        for (auto *a : {&fs.ZoneQdotRadHVACToPerson,
                        &fs.ZoneQHTRadSysToPerson,
                        &fs.ZoneQHWBaseboardToPerson,
                        &fs.ZoneQSteamBaseboardToPerson,
                        &fs.ZoneQElecBaseboardToPerson,
                        &fs.ZoneQCoolingPanelToPerson}) {
            a->dimension(1, 0.0);
        }
        p->dataThermalComforts->ThermalComfortData.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}
} // namespace

extern "C" {
// One result (see the file comment): which picks the KSU thermal sensation vote (0), the mean
// radiant temperature it used (1) and the operative temperature it reports (2).
double ep_KsuTsv(int which, double outbaro, double ztav, double w, double mrt, double qrad, double act, double eff, double clo, double vel)
{
    if (which < 0 || which > 2 || !within(outbaro, 60000.0, 110000.0) || !within(ztav, -10.0, 60.0) || !within(w, 0.0, 0.05) ||
        !within(mrt, -10.0, 60.0) || !within(qrad, 0.0, 2000.0) || !within(act, 50.0, 900.0) || !within(eff, 0.0, 0.5) ||
        !within(clo, 0.0, 3.0) || !within(vel, 0.0, 5.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataEnvrn->OutBaroPress = outbaro;
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.ZTAVComf = ztav;
    zhb.airHumRatAvgComf = w;
    state.dataViewFactor->EnclRadInfo(1).MRT = mrt;
    state.dataHeatBalFanSys->ZoneQHTRadSysToPerson(1) = qrad;
    activity->currentVal = act;
    workEff->currentVal = eff;
    clothing->currentVal = clo;
    airVel->currentVal = vel;
    ThermalComfort::CalcThermalComfortKSU(state);
    auto const &comfort = state.dataThermalComforts->ThermalComfortData(1);
    switch (which) {
    case 0:
        return comfort.KsuTSV;
    case 1:
        return comfort.ThermalComfortMRT;
    default:
        return comfort.ThermalComfortOpTemp;
    }
}
}
