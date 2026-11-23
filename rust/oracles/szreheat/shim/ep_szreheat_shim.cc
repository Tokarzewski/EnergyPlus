// Written by hand: SetPointManager::SPMSingleZoneReheat::calculate reads the control zone's load
// and dead band flag, the air loop's outdoor air fraction and the zone, zone inlet, mixer inlet,
// loop inlet and fan nodes, which gen_plain.py cannot set up. Plain extern "C" entry point over the
// real EnergyPlus SetPointManager code.
//
// Each call sets up one SetpointManager:SingleZone:Reheat with supply air limits mint and maxt
// (C): the zone node at zonet (C), the zone inlet node with mass flow zonem (kg/s) and humidity
// ratio inw (kg/kg), the zone's total load and loads to the cooling and heating set points load,
// tocool and toheat (W), in the dead band or not (deadband). With an outdoor air mixer (hasoa 1)
// the return node has retw and reth and the outdoor air node oaw and oah (kg/kg, J/kg), mixed at
// the air loop's outdoor air fraction oafrac; without one the loop inlet node is at loopt (C).
// With a fan (hasfan 1) its inlet and outlet nodes are at fanin and fanout (C). One EnergyPlusData
// for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: hasoa, hasfan or deadband not 0 or 1; any input that is not
// finite or lies outside what EnergyPlus meets: oafrac outside 0 to 1, zonem outside 0 to 1000,
// humidity ratios outside 0 to 0.1, enthalpies outside -2e4 to 4e5, loads outside -1e7 to 1e7,
// temperatures outside -60 to 80, and mint above maxt (std::clamp requires it).
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataAirLoop.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/SetPointManager.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(7);
        p->dataLoopNodes->NodeID.allocate(7);
        p->dataAirLoop->AirLoopFlow.allocate(1);
        p->dataZoneEnergyDemand->ZoneSysEnergyDemand.allocate(1);
        p->dataZoneEnergyDemand->DeadBandOrSetback.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

bool flag(int v)
{
    return v == 0 || v == 1;
}
} // namespace

extern "C" {
// The supply air temperature set point (C).
double ep_SZReheat(int hasoa,
                   int hasfan,
                   int deadband,
                   double oafrac,
                   double zonem,
                   double inw,
                   double load,
                   double tocool,
                   double toheat,
                   double zonet,
                   double retw,
                   double reth,
                   double oaw,
                   double oah,
                   double loopt,
                   double fanin,
                   double fanout,
                   double mint,
                   double maxt)
{
    if (!flag(hasoa) || !flag(hasfan) || !flag(deadband) || !within(oafrac, 0.0, 1.0) || !within(zonem, 0.0, 1000.0) || !within(inw, 0.0, 0.1) ||
        !within(retw, 0.0, 0.1) || !within(oaw, 0.0, 0.1) || !within(reth, -2.0e4, 4.0e5) || !within(oah, -2.0e4, 4.0e5) ||
        !within(load, -1.0e7, 1.0e7) || !within(tocool, -1.0e7, 1.0e7) || !within(toheat, -1.0e7, 1.0e7) || !within(zonet, -60.0, 80.0) ||
        !within(loopt, -60.0, 80.0) || !within(fanin, -60.0, 80.0) || !within(fanout, -60.0, 80.0) || !within(mint, -60.0, 80.0) ||
        !within(maxt, mint, 80.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    nodes(1).Temp = zonet;
    nodes(2).MassFlowRate = zonem;
    nodes(2).HumRat = inw;
    nodes(3).HumRat = oaw;
    nodes(3).Enthalpy = oah;
    nodes(4).HumRat = retw;
    nodes(4).Enthalpy = reth;
    nodes(5).Temp = loopt;
    nodes(6).Temp = fanin;
    nodes(7).Temp = fanout;
    state.dataAirLoop->AirLoopFlow(1).OAFrac = oafrac;
    auto &demand = state.dataZoneEnergyDemand->ZoneSysEnergyDemand(1);
    demand.TotalOutputRequired = load;
    demand.OutputRequiredToCoolingSP = tocool;
    demand.OutputRequiredToHeatingSP = toheat;
    state.dataZoneEnergyDemand->DeadBandOrSetback(1) = deadband == 1;
    SetPointManager::SPMSingleZoneReheat spm;
    spm.ctrlZoneNum = 1;
    spm.airLoopNum = 1;
    spm.zoneNodeNum = 1;
    spm.zoneInletNodeNum = 2;
    spm.oaInNodeNum = hasoa == 1 ? 3 : 0;
    spm.retNodeNum = 4;
    spm.loopInNodeNum = 5;
    spm.fanInNodeNum = hasfan == 1 ? 6 : 0;
    spm.fanOutNodeNum = hasfan == 1 ? 7 : 0;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.calculate(state);
    return spm.setPt;
}
}
