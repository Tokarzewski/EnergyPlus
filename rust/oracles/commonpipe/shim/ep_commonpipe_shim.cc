// Written by hand: HVACInterfaceManager::UpdateCommonPipe with a single common pipe (and through it
// ManageSingleCommonPipe) reads a primary-secondary plant loop's nodes, tank state, common pipe and
// fluid, which gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// HVACInterfaceManager code.
//
// Each call sets up a water loop of mass mass (kg) with a single common pipe, after the common pipes
// have been set up and outside the first environment call. The side `side` (0 demand, 1 supply)
// has just been simulated and its outlet feeds the tank: the demand (secondary) outlet node has mass
// flow msec (kg/s) and the supply (primary) outlet node mpri, and the simulated side's outlet is at
// tin (C) with pump heat pump (W). The other side's tank has outlet temperatures prev (C, last call)
// and last (C, last time step), newstep 1 meaning the time has moved on, and the simulated side's
// own tank temperature from its last update is other (C). The system time step is tssec seconds.
// Results: the mixed outlet temperature (0), the secondary (1) and primary (2) inlet node
// temperatures, the common pipe's flow (3), temperature (4) and direction (5), and the tank's final
// (6) and average (7) temperatures. One EnergyPlusData for the process, built when the library
// loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 7, side or newstep not 0 or 1; any input
// that is not finite or lies outside what EnergyPlus meets: tin and other outside -20 to 150, flows
// outside 0 to 1000, pump outside 0 to 1e6, mass outside 0 to 1e7, last and prev outside 1 to 99
// (the water specific heat warns outside its table), tssec outside 1 to 3600.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/FluidProperties.hh>
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
        p->dataGlobal->WarmupFlag = true;
        p->dataGlobal->BeginEnvrnFlag = false;
        Fluid::InitConstantFluidPropertiesData(*p);
        p->dataLoopNodes->Node.allocate(4);
        p->dataLoopNodes->NodeID.allocate(4);
        p->dataPlnt->PlantLoop.allocate(1);
        auto &loop = p->dataPlnt->PlantLoop(1);
        loop.glycol = Fluid::GetWater(*p);
        loop.LoopSide(Demand).NodeNumOut = 1;
        loop.LoopSide(Supply).NodeNumOut = 2;
        loop.LoopSide(Demand).NodeNumIn = 3;
        loop.LoopSide(Supply).NodeNumIn = 4;
        p->dataHVACInterfaceMgr->PlantCommonPipe.allocate(1);
        p->dataHVACInterfaceMgr->CommonPipeSetupFinished = true;
        p->dataGlobal->HourOfDay = 5;
        p->dataGlobal->TimeStep = 2;
        p->dataGlobal->TimeStepZone = 0.25;
        p->dataHVACGlobal->SysTimeElapsed = 0.0;
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
double ep_CommonPipe(int which,
                     int side,
                     int newstep,
                     double tin,
                     double msec,
                     double mpri,
                     double pump,
                     double mass,
                     double last,
                     double prev,
                     double other,
                     double tssec)
{
    if (which < 0 || which > 7 || (side != 0 && side != 1) || (newstep != 0 && newstep != 1) || !within(tin, -20.0, 150.0) ||
        !within(msec, 0.0, 1000.0) || !within(mpri, 0.0, 1000.0) || !within(pump, 0.0, 1.0e6) || !within(mass, 0.0, 1.0e7) ||
        !within(last, 1.0, 99.0) || !within(prev, 1.0, 99.0) || !within(other, -20.0, 150.0) || !within(tssec, 1.0, 3600.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto const inSide = side == 1 ? Supply : Demand;
    auto const outSide = side == 1 ? Demand : Supply;
    auto &loop = state.dataPlnt->PlantLoop(1);
    loop.Mass = mass;
    auto &nodes = state.dataLoopNodes->Node;
    for (int k = 1; k <= 4; ++k) {
        nodes(k) = Node::NodeData();
    }
    nodes(1).MassFlowRate = msec;
    nodes(2).MassFlowRate = mpri;
    nodes(loop.LoopSide(inSide).NodeNumOut).Temp = tin;
    loop.LoopSide(inSide).TotalPumpHeat = pump;
    loop.LoopSide(inSide).LoopSideInlet_TankTemp = other;
    auto &tank = loop.LoopSide(outSide);
    tank.TempInterfaceTankOutlet = prev;
    tank.LastTempInterfaceTankOutlet = last;
    Real64 const now = (state.dataGlobal->HourOfDay - 1) + state.dataGlobal->TimeStep * state.dataGlobal->TimeStepZone +
                       state.dataHVACGlobal->SysTimeElapsed;
    tank.TimeElapsed = newstep == 1 ? now - 1.0 : now;
    state.dataHVACGlobal->TimeStepSysSec = tssec;
    auto &pipe = state.dataHVACInterfaceMgr->PlantCommonPipe(1);
    pipe = HVACInterfaceManager::CommonPipeData();
    PlantLocation plantLoc;
    plantLoc.loopNum = 1;
    plantLoc.loopSideNum = inSide;
    Real64 mixed = 0.0;
    HVACInterfaceManager::UpdateCommonPipe(state, plantLoc, DataPlant::CommonPipeType::Single, mixed);
    double const r[8] = {
        mixed, nodes(3).Temp, nodes(4).Temp, pipe.Flow, pipe.Temp, static_cast<double>(pipe.FlowDir), tank.TempInterfaceTankOutlet,
        tank.LoopSideInlet_TankTemp};
    return r[which];
}
}
