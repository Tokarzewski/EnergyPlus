// Written by hand: SetPointManager::SPMTempest::calculate reads the air loop's cooled and heated
// zones, their zone and zone inlet nodes and their loads, which gen_plain.py cannot set up. Plain
// extern "C" entry point over the real EnergyPlus SetPointManager code.
//
// Each call sets up one SetpointManager:Warmest (warmest 1) or :Coldest (warmest 0) with supply air
// limits mint and maxt (C) on an air loop cooling zones 1 to nc (1 to 4) and heating zones 1 to nh
// (0 to nc). Zone z has total load load<z> (W), zone temperature zt<z> (C), and its inlet node the
// maximum mass flow mmax<z> (kg/s) and humidity ratio w<z> (kg/kg). One EnergyPlusData for the
// process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: warmest not 0 or 1, nc outside 1 to 4, nh outside 0 to nc;
// any input of any zone that is not finite or lies outside what EnergyPlus meets: loads outside
// -1e7 to 1e7, temperatures outside -60 to 80, mmax outside 0 to 1000, w outside 0 to 0.1, and mint
// above maxt (std::clamp requires it).
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
        p->dataZoneEquip->ZoneEquipConfig.allocate(4);
        p->dataAirLoop->AirToZoneNodeInfo.allocate(1);
        auto &a = p->dataAirLoop->AirToZoneNodeInfo(1);
        a.CoolCtrlZoneNums.allocate(4);
        a.HeatCtrlZoneNums.allocate(4);
        a.CoolZoneInletNodes.allocate(4);
        a.HeatZoneInletNodes.allocate(4);
        for (int z = 1; z <= 4; ++z) {
            p->dataZoneEquip->ZoneEquipConfig(z).ZoneNode = z;
            a.CoolCtrlZoneNums(z) = z;
            a.HeatCtrlZoneNums(z) = z;
            a.CoolZoneInletNodes(z) = z + 4;
            a.HeatZoneInletNodes(z) = z + 4;
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
double ep_Tempest(int warmest,
                  int nc,
                  int nh,
                  double mint,
                  double maxt,
                  double load1,
                  double zt1,
                  double mmax1,
                  double w1,
                  double load2,
                  double zt2,
                  double mmax2,
                  double w2,
                  double load3,
                  double zt3,
                  double mmax3,
                  double w3,
                  double load4,
                  double zt4,
                  double mmax4,
                  double w4)
{
    double const zs[4][4] = {{load1, zt1, mmax1, w1}, {load2, zt2, mmax2, w2}, {load3, zt3, mmax3, w3}, {load4, zt4, mmax4, w4}};
    if ((warmest != 0 && warmest != 1) || nc < 1 || nc > 4 || nh < 0 || nh > nc || !within(mint, -60.0, 80.0) || !within(maxt, mint, 80.0)) {
        return NAN;
    }
    for (auto const &z : zs) {
        if (!within(z[0], -1.0e7, 1.0e7) || !within(z[1], -60.0, 80.0) || !within(z[2], 0.0, 1000.0) || !within(z[3], 0.0, 0.1)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    for (int z = 1; z <= 4; ++z) {
        auto const &v = zs[z - 1];
        state.dataZoneEnergyDemand->ZoneSysEnergyDemand(z).TotalOutputRequired = v[0];
        nodes(z).Temp = v[1];
        nodes(z + 4).MassFlowRateMax = v[2];
        nodes(z + 4).HumRat = v[3];
    }
    auto &a = state.dataAirLoop->AirToZoneNodeInfo(1);
    a.NumZonesCooled = nc;
    a.NumZonesHeated = nh;
    SetPointManager::SPMTempest spm;
    spm.type = warmest == 1 ? SetPointManager::SPMType::Warmest : SetPointManager::SPMType::Coldest;
    spm.airLoopNum = 1;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.calculate(state);
    return spm.setPt;
}
}
