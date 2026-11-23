// Written by hand: SetPointManager::SPMWarmestTempFlow and SPMReturnAirBypassFlow read an air
// loop's cooled zones, their nodes and loads, or a return air bypass's mixer nodes and schedule,
// which gen_plain.py cannot set up. Plain extern "C" entry points over the real EnergyPlus
// SetPointManager code; each builds the manager, calls calculate and returns what it sets.
//
// - WarmFlow(which, flowfirst, nc, mint, maxt, minturn, ...): SetpointManager:WarmestTemperatureFlow
//   with the temperature first (flowfirst 0) or flow first (1) strategy, supply limits mint to maxt
//   (C) and minimum turndown minturn, ready to simulate, on an air loop cooling zones 1 to nc (1 to
//   4). Zone z has total load load<z> (W), zone temperature zt<z> (C), and its inlet node the
//   maximum mass flow mmax<z> (kg/s) and humidity ratio w<z>. Results: the supply set point (0, C),
//   the turndown (1) and the critical zone's number (2).
// - RABypass(spt, tloop, tmix, msup, tsup, mtot, trab): SetpointManager:ReturnAirBypassFlow with the
//   scheduled set point spt (C), the loop outlet at tloop and the mixer outlet at tmix (C) with
//   mass flow mtot (kg/s), the mixer's supply inlet with mass flow msup at tsup and its return air
//   bypass inlet at trab. The result is the bypass flow set point (kg/s).
// One EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 2, flowfirst not 0 or 1, nc outside 1 to
// 4, mint above maxt or minturn outside 0 to 1 (std::clamp requires ordered limits), mtot negative;
// any input that is not finite or lies outside what EnergyPlus meets: loads outside -1e7 to 1e7,
// temperatures outside -60 to 80, flows outside 0 to 1000, humidity ratios outside 0 to 0.1.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataAirLoop.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/DataZoneEquipment.hh>
#include <EnergyPlus/OutputProcessor.hh> // before SetPointManager.hh, which uses OutputProcessor::VariableType
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/SetPointManager.hh>

using namespace EnergyPlus;

namespace {
Sched::ScheduleConstant *sched = nullptr;

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
        a.CoolZoneInletNodes.allocate(4);
        for (int z = 1; z <= 4; ++z) {
            p->dataZoneEquip->ZoneEquipConfig(z).ZoneNode = z;
            a.CoolCtrlZoneNums(z) = z;
            a.CoolZoneInletNodes(z) = z + 4;
        }
        sched = new Sched::ScheduleConstant;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

bool temp(double v)
{
    return within(v, -60.0, 80.0);
}
} // namespace

extern "C" {
double ep_WarmFlow(int which,
                   int flowfirst,
                   int nc,
                   double mint,
                   double maxt,
                   double minturn,
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
    if (which < 0 || which > 2 || (flowfirst != 0 && flowfirst != 1) || nc < 1 || nc > 4 || !temp(mint) || !temp(maxt) || mint > maxt ||
        !within(minturn, 0.0, 1.0)) {
        return NAN;
    }
    for (auto const &z : zs) {
        if (!within(z[0], -1.0e7, 1.0e7) || !temp(z[1]) || !within(z[2], 0.0, 1000.0) || !within(z[3], 0.0, 0.1)) {
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
    state.dataAirLoop->AirToZoneNodeInfo(1).NumZonesCooled = nc;
    SetPointManager::SPMWarmestTempFlow spm;
    spm.strategy = flowfirst == 1 ? SetPointManager::ControlStrategy::FlowFirst : SetPointManager::ControlStrategy::TempFirst;
    spm.airLoopNum = 1;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.minTurndown = minturn;
    spm.simReady = true;
    spm.calculate(state);
    double const r[3] = {spm.setPt, spm.turndown, static_cast<double>(spm.critZoneNum)};
    return r[which];
}

double ep_RABypass(double spt, double tloop, double tmix, double msup, double tsup, double mtot, double trab)
{
    if (!temp(spt) || !temp(tloop) || !temp(tmix) || !within(msup, 0.0, 1000.0) || !temp(tsup) || !within(mtot, 0.0, 1000.0) ||
        !temp(trab)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    nodes(1).Temp = trab;
    nodes(2).MassFlowRate = msup;
    nodes(2).Temp = tsup;
    nodes(3).MassFlowRate = mtot;
    nodes(3).Temp = tmix;
    nodes(4).Temp = tloop;
    sched->currentVal = spt;
    SetPointManager::SPMReturnAirBypassFlow spm;
    spm.sched = sched;
    spm.rabMixInNodeNum = 1;
    spm.supMixInNodeNum = 2;
    spm.mixOutNodeNum = 3;
    spm.sysOutNodeNum = 4;
    spm.calculate(state);
    return spm.FlowSetPt;
}
}
