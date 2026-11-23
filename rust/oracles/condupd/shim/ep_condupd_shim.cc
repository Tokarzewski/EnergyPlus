// Written by hand: PlantUtilities::UpdateChillerComponentCondenserSide and
// UpdateComponentHeatRecoverySide read a component's condenser or heat recovery nodes, its plant
// loop's fluid and the loops connected to its loop side, which gen_plain.py cannot set up. Plain
// extern "C" entry point over the real EnergyPlus PlantUtilities code.
//
// Each call sets up a water loop (loop 1) whose demand side holds the component's inlet node 1 and
// outlet node 2, and two other loops: nconn (0 to 2) of them are connected to that side, the first
// to loop 2's supply side and the second to loop 3's demand side, each putting demand on the
// remote loop or not (dem1, dem2). It then calls the condenser-side update (hr 0) or the heat
// recovery one (hr 1) with the model's heat rate q (W), inlet and outlet temperatures tmin and
// tmout (C) and mass flow mmod (kg/s), on the first HVAC iteration or not (first). The inlet node
// has mass flow inm (kg/s) and temperature intemp (C), the outlet node mass flow outm and
// temperature outt. The component's loop side starts with its simulate flag sim0, the remote sides
// with theirs off. One EnergyPlusData for the process, built when the library loads, with console
// output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 3, hr, first, sim0, dem1 or dem2 not 0 or
// 1, nconn outside 0 to 2; any input that is not finite or lies outside what EnergyPlus meets: q
// outside -1e8 to 1e8, tmin outside 1 to 99 (the water specific heat warns outside its table),
// tmout, intemp or outt outside -20 to 150, mmod, inm or outm outside 0 to 1000.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/FluidProperties.hh>
#include <EnergyPlus/Plant/DataPlant.hh>
#include <EnergyPlus/PlantUtilities.hh>

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
        Fluid::InitConstantFluidPropertiesData(*p);
        p->dataLoopNodes->Node.allocate(2);
        p->dataLoopNodes->NodeID.allocate(2);
        p->dataPlnt->PlantLoop.allocate(3);
        p->dataPlnt->PlantLoop(1).glycol = Fluid::GetWater(*p);
        auto &side = p->dataPlnt->PlantLoop(1).LoopSide(Demand);
        side.Connected.allocate(2);
        side.Connected(1).LoopNum = 2;
        side.Connected(1).LoopSideNum = Supply;
        side.Connected(2).LoopNum = 3;
        side.Connected(2).LoopSideNum = Demand;
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
// One result: the outlet node's temperature (0, C), and the simulate flags (1 or 0) of the
// component's loop side (1), loop 2's supply side (2) and loop 3's demand side (3).
double ep_CondUpd(int which,
                  int hr,
                  int first,
                  int sim0,
                  int nconn,
                  int dem1,
                  int dem2,
                  double q,
                  double tmin,
                  double tmout,
                  double mmod,
                  double inm,
                  double intemp,
                  double outm,
                  double outt)
{
    if (which < 0 || which > 3 || !flag(hr) || !flag(first) || !flag(sim0) || nconn < 0 || nconn > 2 || !flag(dem1) || !flag(dem2) ||
        !within(q, -1.0e8, 1.0e8) || !within(tmin, 1.0, 99.0) || !within(tmout, -20.0, 150.0) || !within(mmod, 0.0, 1000.0) ||
        !within(inm, 0.0, 1000.0) || !within(intemp, -20.0, 150.0) || !within(outm, 0.0, 1000.0) || !within(outt, -20.0, 150.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &in = state.dataLoopNodes->Node(1);
    in.MassFlowRate = inm;
    in.Temp = intemp;
    auto &out = state.dataLoopNodes->Node(2);
    out.MassFlowRate = outm;
    out.Temp = outt;
    auto &side = state.dataPlnt->PlantLoop(1).LoopSide(Demand);
    side.SimLoopSideNeeded = sim0 == 1;
    side.TotalConnected = nconn;
    side.Connected(1).LoopDemandsOnRemote = dem1 == 1;
    side.Connected(2).LoopDemandsOnRemote = dem2 == 1;
    auto &r1 = state.dataPlnt->PlantLoop(2).LoopSide(Supply);
    auto &r2 = state.dataPlnt->PlantLoop(3).LoopSide(Demand);
    r1.SimLoopSideNeeded = false;
    r2.SimLoopSideNeeded = false;
    if (hr == 1) {
        PlantUtilities::UpdateComponentHeatRecoverySide(
            state, 1, Demand, DataPlant::PlantEquipmentType::Chiller_Electric, 1, 2, q, tmin, tmout, mmod, first == 1);
    } else {
        PlantUtilities::UpdateChillerComponentCondenserSide(
            state, 1, Demand, DataPlant::PlantEquipmentType::Chiller_Electric, 1, 2, q, tmin, tmout, mmod, first == 1);
    }
    double const r[4] = {out.Temp, side.SimLoopSideNeeded ? 1.0 : 0.0, r1.SimLoopSideNeeded ? 1.0 : 0.0, r2.SimLoopSideNeeded ? 1.0 : 0.0};
    return r[which];
}
}
