// Written by hand: PlantUtilities::SetComponentFlowRate reads the component's plant location (its
// loop's size, the loop side's flow lock, the branch's components and their EMS overrides, the
// component's operation scheme and flow control) and the loop nodes, which gen_plain.py cannot set
// up. Plain extern "C" entry point over the real EnergyPlus PlantUtilities code.
//
// Each call sets up one plant loop whose branch holds ncomp components (1 to 3) in series on nodes
// 1 to ncomp + 1: component c runs from node c to node c + 1. Node n starts with mass flow
// mdot<n>, hardware limits min<n> and max<n>, available limits minav<n> and maxav<n>, and flow
// request req<n> (kg/s). Component k asks for flow (kg/s), with the loop side's flow lock unlocked
// (lock 0) or locked (1), the loop still to be sized (autosize 1, MaxVolFlowRate AutoSize) or not,
// component k's flow control series active (series 1) or active, its operation scheme demand
// (demand 1) or not. Bits 2c - 2 and 2c - 1 of ems turn on component c's EMS load override and make
// its value 0 (else 1 W). One EnergyPlusData for the process, built when the library loads, with
// console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 17, ncomp outside 1 to 3, k outside 1 to
// ncomp, lock, autosize, series or demand not 0 or 1, ems outside 0 to 63; any flow that is not
// finite or lies outside 0 to 1000 (a negative hardware maximum makes the C++ report a severe error
// once sizing is done).
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataBranchAirLoopPlant.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataSizing.hh>
#include <EnergyPlus/Plant/DataPlant.hh>
#include <EnergyPlus/PlantUtilities.hh>

using namespace EnergyPlus;

namespace {
DataPlant::PlantLoopData *loop = nullptr;
DataPlant::HalfLoopData *side = nullptr;
DataPlant::BranchData *branch = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(4);
        p->dataLoopNodes->NodeID.allocate(4);
        loop = new DataPlant::PlantLoopData;
        side = new DataPlant::HalfLoopData;
        branch = new DataPlant::BranchData;
        branch->Comp.allocate(3);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool flowOk(double v)
{
    return std::isfinite(v) && v >= 0.0 && v <= 1000.0;
}
} // namespace

extern "C" {
// One result: the flow the component gets (0, kg/s); node n's mass flow (n, from 1 to 4), minimum
// available (n + 4), maximum available (n + 8) and request (n + 12); whether the loop side must be
// simulated again (17, 0 or 1).
double ep_SetFlow(int which,
                  int ncomp,
                  int k,
                  int lock,
                  int autosize,
                  int series,
                  int demand,
                  int ems,
                  double flow,
                  double mdot1,
                  double min1,
                  double max1,
                  double minav1,
                  double maxav1,
                  double req1,
                  double mdot2,
                  double min2,
                  double max2,
                  double minav2,
                  double maxav2,
                  double req2,
                  double mdot3,
                  double min3,
                  double max3,
                  double minav3,
                  double maxav3,
                  double req3,
                  double mdot4,
                  double min4,
                  double max4,
                  double minav4,
                  double maxav4,
                  double req4)
{
    double const nodes[4][6] = {{mdot1, min1, max1, minav1, maxav1, req1},
                                {mdot2, min2, max2, minav2, maxav2, req2},
                                {mdot3, min3, max3, minav3, maxav3, req3},
                                {mdot4, min4, max4, minav4, maxav4, req4}};
    if (which < 0 || which > 17 || ncomp < 1 || ncomp > 3 || k < 1 || k > ncomp || (lock != 0 && lock != 1) || (autosize != 0 && autosize != 1) ||
        (series != 0 && series != 1) || (demand != 0 && demand != 1) || ems < 0 || ems > 63 || !flowOk(flow)) {
        return NAN;
    }
    for (auto const &n : nodes) {
        for (double v : n) {
            if (!flowOk(v)) {
                return NAN;
            }
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    for (int n = 1; n <= 4; ++n) {
        auto &node = state.dataLoopNodes->Node(n);
        node.MassFlowRate = nodes[n - 1][0];
        node.MassFlowRateMin = nodes[n - 1][1];
        node.MassFlowRateMax = nodes[n - 1][2];
        node.MassFlowRateMinAvail = nodes[n - 1][3];
        node.MassFlowRateMaxAvail = nodes[n - 1][4];
        node.MassFlowRateRequest = nodes[n - 1][5];
    }
    loop->MaxVolFlowRate = autosize == 1 ? DataSizing::AutoSize : 0.01;
    side->FlowLock = lock == 1 ? DataPlant::FlowLock::Locked : DataPlant::FlowLock::Unlocked;
    side->SimLoopSideNeeded = false;
    branch->TotalComponents = ncomp;
    for (int c = 1; c <= 3; ++c) {
        auto &comp = branch->Comp(c);
        comp.NodeNumIn = c;
        comp.NodeNumOut = c + 1;
        comp.EMSLoadOverrideOn = ((ems >> (2 * c - 2)) & 1) != 0;
        comp.EMSLoadOverrideValue = ((ems >> (2 * c - 1)) & 1) != 0 ? 0.0 : 1.0;
        comp.FlowCtrl = DataBranchAirLoopPlant::ControlType::Active;
        comp.CurOpSchemeType = DataPlant::OpScheme::Invalid;
    }
    auto &comp = branch->Comp(k);
    comp.FlowCtrl = series == 1 ? DataBranchAirLoopPlant::ControlType::SeriesActive : DataBranchAirLoopPlant::ControlType::Active;
    comp.CurOpSchemeType = demand == 1 ? DataPlant::OpScheme::Demand : DataPlant::OpScheme::HeatingRB;
    PlantLocation plantLoc;
    plantLoc.loopNum = 1;
    plantLoc.loopSideNum = DataPlant::LoopSideLocation::Supply;
    plantLoc.branchNum = 1;
    plantLoc.compNum = k;
    plantLoc.loop = loop;
    plantLoc.side = side;
    plantLoc.branch = branch;
    plantLoc.comp = &comp;
    double compFlow = flow;
    PlantUtilities::SetComponentFlowRate(state, compFlow, k, k + 1, plantLoc);
    if (which == 0) {
        return compFlow;
    }
    if (which == 17) {
        return side->SimLoopSideNeeded ? 1.0 : 0.0;
    }
    auto const &node = state.dataLoopNodes->Node((which - 1) % 4 + 1);
    switch ((which - 1) / 4) {
    case 0:
        return node.MassFlowRate;
    case 1:
        return node.MassFlowRateMinAvail;
    case 2:
        return node.MassFlowRateMaxAvail;
    default:
        return node.MassFlowRateRequest;
    }
}
}
