// Written by hand: HVACInterfaceManager::UpdatePlantLoopInterface reads a plant loop's half-loop
// nodes, tank state, convergence history and fluid, which gen_plain.py cannot set up. Plain extern
// "C" entry point over the real EnergyPlus HVACInterfaceManager code.
//
// Each call sets up a water loop of mass mass (kg) without a common pipe, whose side `side` (0
// demand, 1 supply) has just been simulated: that side's outlet node (node 1 for the demand side,
// 2 for the supply side) has temperature tin (C), mass flow mdot, available flows minav and maxav
// (kg/s), quality q and pressure p (Pa); the other side's inlet node (node 3) last had temperature
// otemp (C) and mass flow omdot (kg/s). The other side carries pump heat pump (W) and tank outlet
// temperatures prev (C, last call) and last (C, last time step); newstep 1 means the time has moved
// on since the last call. The loop has pressure components or not (haspress). The system time step
// is tssec seconds. Every slot of the four convergence histories holds hist, and the out-of-tolerance
// flag starts as oot0. Results: the out-of-tolerance flag (0), the mass flow (1) and temperature (2)
// not-converged flags (1 or 0), the other inlet node's temperature (3), mass flow (4), minimum (5)
// and maximum (6) available flows, quality (7), pressure (8) and enthalpy (9), and the side's flow
// and temperature histories' newest (10, 11) and next (12, 13) entries. One EnergyPlusData for the
// process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 13, side, newstep, haspress or oot0 not 0
// or 1; any input that is not finite or lies outside what EnergyPlus meets: tin outside -20 to 150,
// flows outside 0 to 1000, q outside 0 to 1, p outside 0 to 1e7, otemp, last and prev outside 1 to
// 99 (the water specific heat warns outside its table), pump outside 0 to 1e6, mass outside 0 to
// 1e7, tssec outside 1 to 3600, hist outside -1000 to 1000.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataConvergParams.hh>
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
        p->dataLoopNodes->Node.allocate(3);
        p->dataLoopNodes->NodeID.allocate(3);
        p->dataPlnt->PlantLoop.allocate(1);
        auto &loop = p->dataPlnt->PlantLoop(1);
        loop.glycol = Fluid::GetWater(*p);
        loop.LoopSide(DataPlant::LoopSideLocation::Demand).NodeNumOut = 1;
        loop.LoopSide(DataPlant::LoopSideLocation::Supply).NodeNumOut = 2;
        p->dataConvergeParams->PlantConvergence.allocate(1);
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

bool flag(int v)
{
    return v == 0 || v == 1;
}
} // namespace

extern "C" {
double ep_PlantIface(int which,
                     int side,
                     int newstep,
                     int haspress,
                     int oot0,
                     double tin,
                     double mdot,
                     double minav,
                     double maxav,
                     double q,
                     double p,
                     double otemp,
                     double omdot,
                     double pump,
                     double mass,
                     double last,
                     double prev,
                     double tssec,
                     double hist)
{
    if (which < 0 || which > 13 || !flag(side) || !flag(newstep) || !flag(haspress) || !flag(oot0) || !within(tin, -20.0, 150.0) ||
        !within(mdot, 0.0, 1000.0) || !within(minav, 0.0, 1000.0) || !within(maxav, 0.0, 1000.0) || !within(q, 0.0, 1.0) ||
        !within(p, 0.0, 1.0e7) || !within(otemp, 1.0, 99.0) || !within(omdot, 0.0, 1000.0) || !within(pump, 0.0, 1.0e6) ||
        !within(mass, 0.0, 1.0e7) || !within(last, 1.0, 99.0) || !within(prev, 1.0, 99.0) || !within(tssec, 1.0, 3600.0) ||
        !within(hist, -1000.0, 1000.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto const thisSide = side == 1 ? DataPlant::LoopSideLocation::Supply : DataPlant::LoopSideLocation::Demand;
    auto const otherSide = side == 1 ? DataPlant::LoopSideLocation::Demand : DataPlant::LoopSideLocation::Supply;
    auto &loop = state.dataPlnt->PlantLoop(1);
    loop.Mass = mass;
    loop.HasPressureComponents = haspress == 1;
    int const outNode = loop.LoopSide(thisSide).NodeNumOut;
    auto &out = state.dataLoopNodes->Node(outNode);
    out.Temp = tin;
    out.MassFlowRate = mdot;
    out.MassFlowRateMinAvail = minav;
    out.MassFlowRateMaxAvail = maxav;
    out.Quality = q;
    out.Press = p;
    auto &other = state.dataLoopNodes->Node(3);
    other = Node::NodeData();
    other.Temp = otemp;
    other.MassFlowRate = omdot;
    auto &tank = loop.LoopSide(otherSide);
    tank.TotalPumpHeat = pump;
    tank.TempInterfaceTankOutlet = prev;
    tank.LastTempInterfaceTankOutlet = last;
    Real64 const now = (state.dataGlobal->HourOfDay - 1) + state.dataGlobal->TimeStep * state.dataGlobal->TimeStepZone +
                       state.dataHVACGlobal->SysTimeElapsed;
    tank.TimeElapsed = newstep == 1 ? now - 1.0 : now;
    state.dataHVACGlobal->TimeStepSysSec = tssec;
    auto &conv = state.dataConvergeParams->PlantConvergence(1);
    conv.PlantFlowDemandToSupplyTolValue.fill(hist);
    conv.PlantFlowSupplyToDemandTolValue.fill(hist);
    conv.PlantTempDemandToSupplyTolValue.fill(hist);
    conv.PlantTempSupplyToDemandTolValue.fill(hist);
    PlantLocation plantLoc;
    plantLoc.loopNum = 1;
    plantLoc.loopSideNum = thisSide;
    bool outOfTolerance = oot0 == 1;
    HVACInterfaceManager::UpdatePlantLoopInterface(state, plantLoc, outNode, 3, outOfTolerance, DataPlant::CommonPipeType::No);
    auto const &flowTol = side == 1 ? conv.PlantFlowSupplyToDemandTolValue : conv.PlantFlowDemandToSupplyTolValue;
    auto const &tempTol = side == 1 ? conv.PlantTempSupplyToDemandTolValue : conv.PlantTempDemandToSupplyTolValue;
    double const r[14] = {outOfTolerance ? 1.0 : 0.0,
                          conv.PlantMassFlowNotConverged ? 1.0 : 0.0,
                          conv.PlantTempNotConverged ? 1.0 : 0.0,
                          other.Temp,
                          other.MassFlowRate,
                          other.MassFlowRateMinAvail,
                          other.MassFlowRateMaxAvail,
                          other.Quality,
                          other.Press,
                          other.Enthalpy,
                          flowTol[0],
                          tempTol[0],
                          flowTol[1],
                          tempTol[1]};
    return r[which];
}
}
