// Written by hand: SplitterComponent's InitAirLoopSplitter, CalcAirLoopSplitter and UpdateSplitter
// read and write the splitter object and its inlet and outlet nodes, which gen_plain.py cannot set
// up. Plain extern "C" entry point over the real EnergyPlus SplitterComponent code.
//
// Each call sets up one AirLoopHVAC:ZoneSplitter with n outlets (1 to 4): node 1 is its inlet,
// nodes 2 to n + 1 its outlets. Node 1 has mass flow m0, maximum and minimum available flows maxav0
// and minav0 (kg/s), temperature t0 (C), humidity ratio w0 (kg/kg), enthalpy h0 (J/kg) and pressure
// p0 (Pa); outlet i's node has m<i>, maxav<i>, minav<i>, t<i>, w<i>, h<i> and p<i>. Then it runs
// what SimAirLoopSplitter runs, init, calc and update, for the first HVAC iteration or not (fhvac)
// and the first call (first 1, before the zone equipment) or the second (first 0, passing the
// flows back), not at the beginning of an environment and without contaminants. One EnergyPlusData
// for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 31, n outside 1 to 4, fhvac or first not 0
// or 1; any node input that is not finite or lies outside what EnergyPlus meets: flows outside 0 to
// 1000, temperatures outside -60 to 80, humidity ratios outside 0 to 0.1, enthalpies outside -2e4
// to 4e5, pressures outside 5e4 to 1.2e5.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/SplitterComponent.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->BeginEnvrnFlag = false;
        p->dataLoopNodes->Node.allocate(5);
        p->dataLoopNodes->NodeID.allocate(5);
        p->dataSplitterComponent->SplitterCond.allocate(1);
        auto &s = p->dataSplitterComponent->SplitterCond(1);
        s.SplitterName = "SPLITTER";
        s.InletNode = 1;
        s.OutletNode.allocate(4);
        s.OutletMassFlowRate.allocate(4);
        s.OutletMassFlowRateMaxAvail.allocate(4);
        s.OutletMassFlowRateMinAvail.allocate(4);
        s.OutletTemp.allocate(4);
        s.OutletHumRat.allocate(4);
        s.OutletEnthalpy.allocate(4);
        s.OutletPressure.allocate(4);
        for (int i = 1; i <= 4; ++i) {
            s.OutletNode(i) = i + 1;
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
// One result: outlet i's node (i from 1 to 4) mass flow ((i - 1) * 7), maximum available (+ 1),
// minimum available (+ 2), temperature (+ 3), humidity ratio (+ 4), enthalpy (+ 5) and pressure
// (+ 6); the inlet node's mass flow (28), maximum and minimum available flows (29 and 30); whether
// the splitter's inlet flow changed (31, 0 or 1).
double ep_AirSplit(int which,
                   int n,
                   int fhvac,
                   int first,
                   double m0,
                   double maxav0,
                   double minav0,
                   double t0,
                   double w0,
                   double h0,
                   double p0,
                   double m1,
                   double maxav1,
                   double minav1,
                   double t1,
                   double w1,
                   double h1,
                   double p1,
                   double m2,
                   double maxav2,
                   double minav2,
                   double t2,
                   double w2,
                   double h2,
                   double p2,
                   double m3,
                   double maxav3,
                   double minav3,
                   double t3,
                   double w3,
                   double h3,
                   double p3,
                   double m4,
                   double maxav4,
                   double minav4,
                   double t4,
                   double w4,
                   double h4,
                   double p4)
{
    double const nodes[5][7] = {{m0, maxav0, minav0, t0, w0, h0, p0},
                                {m1, maxav1, minav1, t1, w1, h1, p1},
                                {m2, maxav2, minav2, t2, w2, h2, p2},
                                {m3, maxav3, minav3, t3, w3, h3, p3},
                                {m4, maxav4, minav4, t4, w4, h4, p4}};
    if (which < 0 || which > 31 || n < 1 || n > 4 || (fhvac != 0 && fhvac != 1) || (first != 0 && first != 1)) {
        return NAN;
    }
    for (auto const &v : nodes) {
        if (!within(v[0], 0.0, 1000.0) || !within(v[1], 0.0, 1000.0) || !within(v[2], 0.0, 1000.0) || !within(v[3], -60.0, 80.0) ||
            !within(v[4], 0.0, 0.1) || !within(v[5], -2.0e4, 4.0e5) || !within(v[6], 5.0e4, 1.2e5)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    for (int i = 1; i <= 5; ++i) {
        auto &node = state.dataLoopNodes->Node(i);
        auto const &v = nodes[i - 1];
        node = Node::NodeData();
        node.MassFlowRate = v[0];
        node.MassFlowRateMaxAvail = v[1];
        node.MassFlowRateMinAvail = v[2];
        node.Temp = v[3];
        node.HumRat = v[4];
        node.Enthalpy = v[5];
        node.Press = v[6];
    }
    auto &s = state.dataSplitterComponent->SplitterCond(1);
    s.NumOutletNodes = n;
    bool const firstCall = first == 1;
    bool changed = false;
    SplitterComponent::InitAirLoopSplitter(state, 1, fhvac == 1, firstCall);
    SplitterComponent::CalcAirLoopSplitter(state, 1, firstCall);
    SplitterComponent::UpdateSplitter(state, 1, changed, firstCall);
    if (which == 31) {
        return changed ? 1.0 : 0.0;
    }
    if (which >= 28) {
        auto const &in = state.dataLoopNodes->Node(1);
        return which == 28 ? in.MassFlowRate : which == 29 ? in.MassFlowRateMaxAvail : in.MassFlowRateMinAvail;
    }
    auto const &o = state.dataLoopNodes->Node(which / 7 + 2);
    switch (which % 7) {
    case 0:
        return o.MassFlowRate;
    case 1:
        return o.MassFlowRateMaxAvail;
    case 2:
        return o.MassFlowRateMinAvail;
    case 3:
        return o.Temp;
    case 4:
        return o.HumRat;
    case 5:
        return o.Enthalpy;
    default:
        return o.Press;
    }
}
}
