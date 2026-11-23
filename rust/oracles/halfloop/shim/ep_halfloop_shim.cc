// Written by hand: HVACInterfaceManager::UpdateHalfLoopInletTemp reads a plant loop's half-loop
// tank state, its fluid and the time, which gen_plain.py cannot set up. Plain extern "C" entry point
// over the real EnergyPlus HVACInterfaceManager code.
//
// Each call sets up a water loop of mass mass (kg) whose side `side` (0 demand, 1 supply) has just
// been simulated: its outlet node has temperature tin (C) and mass flow mdot (kg/s), and the other
// side (the tank's outlet) carries pump heat pump (W), the tank outlet temperature prev (C) from the
// last call and last (C) from the last time step. With newstep 1 the time has moved on since the
// last call, so the last time step's value becomes prev; with 0 it has not. The system time step is
// tssec seconds. Results: the tank outlet (average) temperature the routine returns (0), the tank's
// final temperature (1), the loop side inlet's m cp dT (2, W) and M cp dT/dt (3, W), and the last
// time step's tank outlet temperature it used (4). One EnergyPlusData for the process, built when
// the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 4, side or newstep not 0 or 1; any input
// that is not finite or lies outside what EnergyPlus meets: tin outside -20 to 150, mdot outside 0
// to 1000, pump outside 0 to 1e6, mass outside 0 to 1e7, last and prev outside 1 to 99 (the water
// specific heat warns outside its table), tssec outside 1 to 3600.
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
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->WarmupFlag = true;
        Fluid::InitConstantFluidPropertiesData(*p);
        p->dataLoopNodes->Node.allocate(2);
        p->dataLoopNodes->NodeID.allocate(2);
        p->dataPlnt->PlantLoop.allocate(1);
        auto &loop = p->dataPlnt->PlantLoop(1);
        loop.glycol = Fluid::GetWater(*p);
        loop.LoopSide(DataPlant::LoopSideLocation::Demand).NodeNumOut = 1;
        loop.LoopSide(DataPlant::LoopSideLocation::Supply).NodeNumOut = 2;
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
double ep_HalfLoop(int which, int side, int newstep, double tin, double mdot, double pump, double mass, double last, double prev, double tssec)
{
    if (which < 0 || which > 4 || (side != 0 && side != 1) || (newstep != 0 && newstep != 1) || !within(tin, -20.0, 150.0) ||
        !within(mdot, 0.0, 1000.0) || !within(pump, 0.0, 1.0e6) || !within(mass, 0.0, 1.0e7) || !within(last, 1.0, 99.0) ||
        !within(prev, 1.0, 99.0) || !within(tssec, 1.0, 3600.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto const inSide = side == 1 ? DataPlant::LoopSideLocation::Supply : DataPlant::LoopSideLocation::Demand;
    auto const outSide = side == 1 ? DataPlant::LoopSideLocation::Demand : DataPlant::LoopSideLocation::Supply;
    auto &loop = state.dataPlnt->PlantLoop(1);
    loop.Mass = mass;
    auto &node = state.dataLoopNodes->Node(loop.LoopSide(inSide).NodeNumOut);
    node.Temp = tin;
    node.MassFlowRate = mdot;
    auto &tank = loop.LoopSide(outSide);
    tank.TotalPumpHeat = pump;
    tank.TempInterfaceTankOutlet = prev;
    tank.LastTempInterfaceTankOutlet = last;
    Real64 const now = (state.dataGlobal->HourOfDay - 1) + state.dataGlobal->TimeStep * state.dataGlobal->TimeStepZone +
                       state.dataHVACGlobal->SysTimeElapsed;
    tank.TimeElapsed = newstep == 1 ? now - 1.0 : now;
    state.dataHVACGlobal->TimeStepSysSec = tssec;
    Real64 out = 0.0;
    HVACInterfaceManager::UpdateHalfLoopInletTemp(state, 1, inSide, out);
    double const r[5] = {out, tank.TempInterfaceTankOutlet, tank.LoopSideInlet_MdotCpDeltaT, tank.LoopSideInlet_McpDTdt,
                         tank.LastTempInterfaceTankOutlet};
    return r[which];
}
}
