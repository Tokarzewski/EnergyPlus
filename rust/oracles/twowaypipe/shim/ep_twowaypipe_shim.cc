// Written by hand: HVACInterfaceManager::ManageTwoWayCommonPipe reads a primary-secondary plant
// loop's inlet set points, nodes, half-loop tank temperatures and two-way common pipe, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// HVACInterfaceManager code.
//
// Each call sets up a loop with a two-way common pipe and a constant speed primary pump, after the
// common pipes have been set up and outside the first environment call. The side `side` (0 demand,
// 1 supply) has just been simulated and its tank outlet is at tank (C); the other side's tank
// temperature from its last update is other (C). The supply (primary) and demand (secondary) inlet
// nodes carry set points or not (psp, ssp) with values tpsp and tssp (C) and last temperatures
// tpin and tsin (C); the demand outlet has mass flow msec and the supply outlet mpri (kg/s). The
// common pipe last carried pts from primary to secondary and recirculated prc on the primary leg and
// src on the secondary leg (kg/s). Results: the primary-to-secondary (0) and secondary-to-primary (1)
// flows, the primary (2) and secondary (3) leg flows, and the secondary (4) and primary (5) inlet
// temperatures. One EnergyPlusData for the process, built when the library loads, with console
// output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 5, side, psp or ssp not 0 or 1; any input
// that is not finite or lies outside what EnergyPlus meets: temperatures outside -20 to 150, flows
// outside 0 to 1000.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/HVACInterfaceManager.hh>
#include <EnergyPlus/Plant/DataPlant.hh>

using namespace EnergyPlus;

namespace {
auto constexpr Demand = DataPlant::LoopSideLocation::Demand;
auto constexpr Supply = DataPlant::LoopSideLocation::Supply;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->BeginEnvrnFlag = false;
        p->dataLoopNodes->Node.allocate(4);
        p->dataLoopNodes->NodeID.allocate(4);
        p->dataPlnt->PlantLoop.allocate(1);
        auto &loop = p->dataPlnt->PlantLoop(1);
        loop.LoopSide(Demand).NodeNumOut = 1;
        loop.LoopSide(Supply).NodeNumOut = 2;
        loop.LoopSide(Demand).NodeNumIn = 3;
        loop.LoopSide(Supply).NodeNumIn = 4;
        p->dataHVACInterfaceMgr->PlantCommonPipe.allocate(1);
        p->dataHVACInterfaceMgr->CommonPipeSetupFinished = true;
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
    return within(v, -20.0, 150.0);
}

bool flow(double v)
{
    return within(v, 0.0, 1000.0);
}

bool flag(int v)
{
    return v == 0 || v == 1;
}
} // namespace

extern "C" {
double ep_TwoWayPipe(int which,
                     int side,
                     int psp,
                     int ssp,
                     double tank,
                     double other,
                     double tpsp,
                     double tssp,
                     double tpin,
                     double tsin,
                     double msec,
                     double mpri,
                     double pts,
                     double prc,
                     double src)
{
    if (which < 0 || which > 5 || !flag(side) || !flag(psp) || !flag(ssp) || !temp(tank) || !temp(other) || !temp(tpsp) || !temp(tssp) ||
        !temp(tpin) || !temp(tsin) || !flow(msec) || !flow(mpri) || !flow(pts) || !flow(prc) || !flow(src)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto const thisSide = side == 1 ? Supply : Demand;
    auto const otherSide = side == 1 ? Demand : Supply;
    auto &loop = state.dataPlnt->PlantLoop(1);
    loop.LoopSide(Supply).InletNodeSetPt = psp == 1;
    loop.LoopSide(Demand).InletNodeSetPt = ssp == 1;
    loop.LoopSide(otherSide).LoopSideInlet_TankTemp = other;
    auto &nodes = state.dataLoopNodes->Node;
    nodes(1).MassFlowRate = msec;
    nodes(2).MassFlowRate = mpri;
    nodes(3).TempSetPoint = tssp;
    nodes(3).Temp = tsin;
    nodes(4).TempSetPoint = tpsp;
    nodes(4).Temp = tpin;
    auto &pipe = state.dataHVACInterfaceMgr->PlantCommonPipe(1);
    pipe.SupplySideInletPumpType = HVACInterfaceManager::FlowType::Constant;
    pipe.PriToSecFlow = pts;
    pipe.SecToPriFlow = pts;
    pipe.PriCPLegFlow = prc;
    pipe.SecCPLegFlow = src;
    PlantLocation plantLoc;
    plantLoc.loopNum = 1;
    plantLoc.loopSideNum = thisSide;
    HVACInterfaceManager::ManageTwoWayCommonPipe(state, plantLoc, tank);
    double const r[6] = {pipe.PriToSecFlow, pipe.SecToPriFlow, pipe.PriCPLegFlow, pipe.SecCPLegFlow, nodes(3).Temp, nodes(4).Temp};
    return r[which];
}
}
