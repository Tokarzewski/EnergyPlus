// Written by hand: Pumps::CalcPumps reads the pump object (type, flow limits, nominal flow and
// power, part load coefficients, efficiencies, bank size, EMS pressure override), its plant
// location (the loop in the plant loop array, its fluid and pressure simulation, the loop side's
// flow lock, the branch for SetComponentFlowRate) and its inlet and outlet nodes, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus Pumps code.
//
// Each call sets up one pump as InitializePumps leaves it, alone on the supply side's branch of a
// water plant loop (node 1 inlet, node 2 outlet): variable speed (ptype 0), constant speed (1),
// condensate (2), variable speed bank (3) or constant speed bank (4) of nbank pumps, with the loop
// side unlocked (lock 0) or locked (1), its flow control active or series active (series 1), the
// loop's pressure simulation on (press 1, loop pressure drop pdrop Pa) or off, and the EMS pressure
// override on (ems 1, emsp Pa) or off. The pump is asked for flowreq (kg/s) with flow limits mmin
// and mmax (kg/s), nominal flow nomvol (m3/s) and power nompow (W), part load coefficients c0 to
// c3, motor and pump efficiencies motor and peff, and fraction of motor losses to the fluid frac.
// The inlet node has temperature tin (C), pressure pin (Pa), mass flow mdotin, hardware maximum
// hwmax and available limits minav and maxav (kg/s). The warm-up flag is on, as in sizing, so the
// low flow constant speed warning stays quiet. A negative power still warns (to no file) and is
// set to 0. One EnergyPlusData for the process, built when the library loads, with console output
// off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 10, ptype outside 0 to 4, lock, series,
// press or ems not 0 or 1, nbank outside 1 to 20; any input that is not finite or lies outside what
// EnergyPlus meets: flowreq, mmax, mdotin, hwmax, minav or maxav outside 0 to 1000, mmin outside 0
// to mmax, nomvol outside 1e-6 to 10, nompow outside 0 to 1e7, c0 to c3 outside -5 to 5, motor or
// peff outside 0.1 to 1, frac outside 0 to 1, pdrop, emsp or pin outside 0 to 1e7, tin outside 1
// to 99 (the water density warns outside its table).
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataBranchAirLoopPlant.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/FluidProperties.hh>
#include <EnergyPlus/Plant/DataPlant.hh>
#include <EnergyPlus/Pumps.hh>

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
        loop.MaxVolFlowRate = 0.01;
        auto &side = loop.LoopSide(DataPlant::LoopSideLocation::Supply);
        side.Branch.allocate(1);
        side.Branch(1).TotalComponents = 1;
        side.Branch(1).Comp.allocate(1);
        p->dataPumps->PumpEquip.allocate(1);
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
// One result: the pump mass flow (0, kg/s), electric power (1, W), shaft power (2, W), heat to the
// fluid (3, W), pumps running in the bank (4), whether the pump runs (5, 0 or 1), the pump object's
// power (6, W), the outlet node's temperature (7, C) and pressure (8, Pa), the inlet and outlet
// nodes' mass flow (9 and 10, kg/s).
double ep_Pump(int which,
               int ptype,
               int lock,
               int series,
               int press,
               int ems,
               int nbank,
               double flowreq,
               double mmax,
               double mmin,
               double nomvol,
               double nompow,
               double c0,
               double c1,
               double c2,
               double c3,
               double motor,
               double peff,
               double frac,
               double pdrop,
               double emsp,
               double tin,
               double pin,
               double mdotin,
               double hwmax,
               double minav,
               double maxav)
{
    if (which < 0 || which > 10 || ptype < 0 || ptype > 4 || !flag(lock) || !flag(series) || !flag(press) || !flag(ems) || nbank < 1 ||
        nbank > 20 || !within(flowreq, 0.0, 1000.0) || !within(mmax, 0.0, 1000.0) || !within(mmin, 0.0, mmax) || !within(nomvol, 1.0e-6, 10.0) ||
        !within(nompow, 0.0, 1.0e7) || !within(c0, -5.0, 5.0) || !within(c1, -5.0, 5.0) || !within(c2, -5.0, 5.0) || !within(c3, -5.0, 5.0) ||
        !within(motor, 0.1, 1.0) || !within(peff, 0.1, 1.0) || !within(frac, 0.0, 1.0) || !within(pdrop, 0.0, 1.0e7) ||
        !within(emsp, 0.0, 1.0e7) || !within(tin, 1.0, 99.0) || !within(pin, 0.0, 1.0e7) || !within(mdotin, 0.0, 1000.0) ||
        !within(hwmax, 0.0, 1000.0) || !within(minav, 0.0, 1000.0) || !within(maxav, 0.0, 1000.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &in = state.dataLoopNodes->Node(1);
    in = Node::NodeData();
    in.Temp = tin;
    in.Press = pin;
    in.MassFlowRate = mdotin;
    in.MassFlowRateMin = 0.0;
    in.MassFlowRateMax = hwmax;
    in.MassFlowRateMinAvail = minav;
    in.MassFlowRateMaxAvail = maxav;
    in.MassFlowRateRequest = 0.0;
    auto &out = state.dataLoopNodes->Node(2);
    out = Node::NodeData();
    auto &loop = state.dataPlnt->PlantLoop(1);
    loop.UsePressureForPumpCalcs = press == 1;
    loop.PressureDrop = pdrop;
    auto &side = loop.LoopSide(DataPlant::LoopSideLocation::Supply);
    side.FlowLock = lock == 1 ? DataPlant::FlowLock::Locked : DataPlant::FlowLock::Unlocked;
    auto &branch = side.Branch(1);
    auto &comp = branch.Comp(1);
    comp.NodeNumIn = 1;
    comp.NodeNumOut = 2;
    comp.EMSLoadOverrideOn = false;
    comp.FlowCtrl = series == 1 ? DataBranchAirLoopPlant::ControlType::SeriesActive : DataBranchAirLoopPlant::ControlType::Active;
    comp.CurOpSchemeType = DataPlant::OpScheme::Pump;
    auto &pump = state.dataPumps->PumpEquip(1);
    pump.pumpType = static_cast<Pumps::PumpType>(ptype);
    pump.InletNodeNum = 1;
    pump.OutletNodeNum = 2;
    pump.NumPumpsInBank = nbank;
    pump.MassFlowRateMax = mmax;
    pump.MassFlowRateMin = mmin;
    pump.NomVolFlowRate = nomvol;
    pump.NomPowerUse = nompow;
    pump.PartLoadCoef = {c0, c1, c2, c3};
    pump.MotorEffic = motor;
    pump.PumpEffic = peff;
    pump.FracMotorLossToFluid = frac;
    pump.EMSPressureOverrideOn = ems == 1;
    pump.EMSPressureOverrideValue = emsp;
    pump.HasVFD = false;
    pump.Power = 0.0;
    pump.plantLoc.loopNum = 1;
    pump.plantLoc.loopSideNum = DataPlant::LoopSideLocation::Supply;
    pump.plantLoc.branchNum = 1;
    pump.plantLoc.compNum = 1;
    pump.plantLoc.loop = &loop;
    pump.plantLoc.side = &side;
    pump.plantLoc.branch = &branch;
    pump.plantLoc.comp = &comp;
    auto &dp = *state.dataPumps;
    dp.NumPumpsRunning = 0;
    dp.NumPumpsFullLoad = 0;
    dp.PumpMassFlowRate = 0.0;
    dp.Power = 0.0;
    dp.ShaftPower = 0.0;
    dp.PumpHeattoFluid = 0.0;
    bool running = false;
    Pumps::CalcPumps(state, 1, flowreq, running);
    switch (which) {
    case 0:
        return dp.PumpMassFlowRate;
    case 1:
        return dp.Power;
    case 2:
        return dp.ShaftPower;
    case 3:
        return dp.PumpHeattoFluid;
    case 4:
        return dp.NumPumpsRunning;
    case 5:
        return running ? 1.0 : 0.0;
    case 6:
        return pump.Power;
    case 7:
        return out.Temp;
    case 8:
        return out.Press;
    case 9:
        return in.MassFlowRate;
    default:
        return out.MassFlowRate;
    }
}
}
