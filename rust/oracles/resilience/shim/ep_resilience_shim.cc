// Written by hand: HeatBalanceSurfaceManager::CalcThermalResilience reads the zone air state, the
// heat index method and the reporting flags from the EnergyPlusData state, which gen_plain.py
// cannot set up. Plain extern "C" entry point over the real EnergyPlus HeatBalanceSurfaceManager
// code.
//
// Each call sets up zone 1 with air averaged over the zone time step at ztav and humidity ratio
// airhumratavg, outdoor barometric pressure outbaro, and heat index method method
// (DataHeatBalance::HeatIndexMethod: 0 simplified, 1 extended), with the thermal resilience summary
// on so that both indices are calculated, and past the first call so that no output variables are
// set up. Then the real CalcThermalResilience runs. One EnergyPlusData for the process, built when
// the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 1; method outside 0 to 1; outbaro outside
// 10000 to 200000 Pa.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/HeatBalanceSurfaceManager.hh>
#include <EnergyPlus/OutputReportTabular.hh>
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
        p->dataHeatBal->Resilience.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        p->dataHeatBalSurfMgr->ManageSurfaceHeatBalancefirstTime = false;
        p->dataOutRptTab->displayThermalResilienceSummary = true;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of CalcThermalResilience for zone 1 (see the file comment): which picks the zone heat
// index ZoneHeatIndex (0, C) or humidex ZoneHumidex (1).
double ep_ThermalResilience(int which, int method, double ztav, double airhumratavg, double outbaro)
{
    if (which < 0 || which > 1 || method < 0 || method > 1 || !(outbaro >= 10000.0 && outbaro <= 200000.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataHeatBal->heatIndexMethod = static_cast<DataHeatBalance::HeatIndexMethod>(method);
    state.dataEnvrn->OutBaroPress = outbaro;
    auto &z = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    z.ZTAV = ztav;
    z.airHumRatAvg = airhumratavg;
    auto &r = state.dataHeatBal->Resilience(1);
    r.ZoneHeatIndex = 0.0;
    r.ZoneHumidex = 0.0;

    HeatBalanceSurfaceManager::CalcThermalResilience(state);

    return which == 0 ? r.ZoneHeatIndex : r.ZoneHumidex;
}
}
