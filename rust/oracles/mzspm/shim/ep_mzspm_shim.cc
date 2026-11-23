// Written by hand: SetPointManager::SPMMultiZoneTemp and SPMMultiZoneHum read an air loop's cooled
// zones, their zone and zone inlet nodes and their loads, which gen_plain.py cannot set up. Plain
// extern "C" entry points over the real EnergyPlus SetPointManager code; each builds the manager,
// calls calculate and returns setPt.
//
// Each call uses an air loop cooling zones 1 to nc (1 to 4). Zone z's inlet node has mass flow m<z>
// (kg/s) and, for MZTemp, humidity ratio iw<z>; its zone node humidity ratio zw<z> and, for MZTemp,
// temperature zt<z> (C).
// - MZTemp(heating, nc, mint, maxt, ...): SetpointManager:MultiZone:Cooling:Average (heating 0) or
//   :Heating:Average (1) with supply limits mint to maxt (C); zone z's total load is load<z> (W).
// - MZHum(kind, nc, minh, maxh, ...): SetpointManager:MultiZone:MinimumHumidity:Average (kind 0),
//   :MaximumHumidity:Average (1), :Humidity:Minimum (2) or :Humidity:Maximum (3) with limits minh to
//   maxh; zone z's moisture loads to its humidifying and dehumidifying set points are hum<z> and
//   dehum<z> (kg/s).
// One EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: heating not 0 or 1, kind outside 0 to 3, nc outside 1 to 4, a
// lower limit above its upper one (std::clamp requires it); any input that is not finite or lies
// outside what EnergyPlus meets: temperatures outside -60 to 80, humidity ratios outside 0 to 0.1,
// flows outside 0 to 1000, loads outside -1e7 to 1e7, moisture loads outside -10 to 10.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataAirLoop.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/DataZoneEquipment.hh>
#include <EnergyPlus/OutputProcessor.hh> // before SetPointManager.hh, which uses OutputProcessor::VariableType
#include <EnergyPlus/SetPointManager.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(8);
        p->dataLoopNodes->NodeID.allocate(8);
        p->dataZoneEnergyDemand->ZoneSysEnergyDemand.allocate(4);
        p->dataZoneEnergyDemand->ZoneSysMoistureDemand.allocate(4);
        p->dataZoneEquip->ZoneEquipConfig.allocate(4);
        p->dataAirLoop->AirToZoneNodeInfo.allocate(1);
        auto &a = p->dataAirLoop->AirToZoneNodeInfo(1);
        a.CoolCtrlZoneNums.allocate(4);
        a.CoolZoneInletNodes.allocate(4);
        for (int z = 1; z <= 4; ++z) {
            p->dataZoneEquip->ZoneEquipConfig(z).ZoneNode = z;
            a.CoolCtrlZoneNums(z) = z;
            a.CoolZoneInletNodes(z) = z + 4;
        }
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
// The supply air temperature set point (C).
double ep_MZTemp(int heating,
                 int nc,
                 double mint,
                 double maxt,
                 double m1,
                 double load1,
                 double zt1,
                 double zw1,
                 double iw1,
                 double m2,
                 double load2,
                 double zt2,
                 double zw2,
                 double iw2,
                 double m3,
                 double load3,
                 double zt3,
                 double zw3,
                 double iw3,
                 double m4,
                 double load4,
                 double zt4,
                 double zw4,
                 double iw4)
{
    double const zs[4][5] = {
        {m1, load1, zt1, zw1, iw1}, {m2, load2, zt2, zw2, iw2}, {m3, load3, zt3, zw3, iw3}, {m4, load4, zt4, zw4, iw4}};
    if ((heating != 0 && heating != 1) || nc < 1 || nc > 4 || !within(mint, -60.0, 80.0) || !within(maxt, mint, 80.0)) {
        return NAN;
    }
    for (auto const &z : zs) {
        if (!within(z[0], 0.0, 1000.0) || !within(z[1], -1.0e7, 1.0e7) || !within(z[2], -60.0, 80.0) || !within(z[3], 0.0, 0.1) ||
            !within(z[4], 0.0, 0.1)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    for (int z = 1; z <= 4; ++z) {
        auto const &v = zs[z - 1];
        nodes(z + 4).MassFlowRate = v[0];
        state.dataZoneEnergyDemand->ZoneSysEnergyDemand(z).TotalOutputRequired = v[1];
        nodes(z).Temp = v[2];
        nodes(z).HumRat = v[3];
        nodes(z + 4).HumRat = v[4];
    }
    state.dataAirLoop->AirToZoneNodeInfo(1).NumZonesCooled = nc;
    SetPointManager::SPMMultiZoneTemp spm;
    spm.type = heating == 1 ? SetPointManager::SPMType::MZHeatingAverage : SetPointManager::SPMType::MZCoolingAverage;
    spm.airLoopNum = 1;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.calculate(state);
    return spm.setPt;
}

// The supply air humidity ratio set point (kg/kg).
double ep_MZHum(int kind,
                int nc,
                double minh,
                double maxh,
                double m1,
                double hum1,
                double dehum1,
                double zw1,
                double m2,
                double hum2,
                double dehum2,
                double zw2,
                double m3,
                double hum3,
                double dehum3,
                double zw3,
                double m4,
                double hum4,
                double dehum4,
                double zw4)
{
    double const zs[4][4] = {{m1, hum1, dehum1, zw1}, {m2, hum2, dehum2, zw2}, {m3, hum3, dehum3, zw3}, {m4, hum4, dehum4, zw4}};
    if (kind < 0 || kind > 3 || nc < 1 || nc > 4 || !within(minh, 0.0, 0.1) || !within(maxh, minh, 0.1)) {
        return NAN;
    }
    for (auto const &z : zs) {
        if (!within(z[0], 0.0, 1000.0) || !within(z[1], -10.0, 10.0) || !within(z[2], -10.0, 10.0) || !within(z[3], 0.0, 0.1)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    for (int z = 1; z <= 4; ++z) {
        auto const &v = zs[z - 1];
        nodes(z + 4).MassFlowRate = v[0];
        auto &demand = state.dataZoneEnergyDemand->ZoneSysMoistureDemand(z);
        demand.OutputRequiredToHumidifyingSP = v[1];
        demand.OutputRequiredToDehumidifyingSP = v[2];
        nodes(z).HumRat = v[3];
    }
    state.dataAirLoop->AirToZoneNodeInfo(1).NumZonesCooled = nc;
    SetPointManager::SPMMultiZoneHum spm;
    spm.type = kind == 0   ? SetPointManager::SPMType::MZMinHumAverage
               : kind == 1 ? SetPointManager::SPMType::MZMaxHumAverage
               : kind == 2 ? SetPointManager::SPMType::MZMinHum
                           : SetPointManager::SPMType::MZMaxHum;
    spm.airLoopNum = 1;
    spm.minSetHum = minh;
    spm.maxSetHum = maxh;
    spm.calculate(state);
    return spm.setPt;
}
}
