// Written by hand: PlantUtilities' node helpers (InitComponentNodes, SafeCopyPlantNode,
// BoundValueToNodeMinMaxAvail, TightenNodeMinMaxAvails, BoundValueToWithinTwoValues) read and write
// loop nodes, which gen_plain.py cannot set up. Plain extern "C" entry points over the real
// EnergyPlus PlantUtilities code.
//
// Each call uses inlet node 1 and outlet node 2, filled from the arguments (every other field at
// its default), and one plant loop whose pressure simulation is none or pump power correction
// (lp 1 or 2 in SafeCopy; 0 passes no loop). One EnergyPlusData for the process, built when the
// library loads, with console output off.
//
// - InitNodes(which, minm, maxm): InitComponentNodes with component flow limits minm and maxm
//   (kg/s); results: the outlet's mass flow (0), the inlet's minimum (1), minimum available (2),
//   maximum (3) and maximum available (4) flows, mass flow (5) and request (6).
// - SafeCopy(which, lp, t, m, q, h, tmin, tmax, mmin, mminav, mmax, mmaxav, w, p, op): copies an
//   inlet at temperature t (C), mass flow m (kg/s), quality q, enthalpy h (J/kg), temperature limits
//   tmin and tmax, flow limits mmin, mminav, mmax, mmaxav (kg/s), humidity ratio w and pressure p
//   (Pa) to an outlet at pressure op; results: the outlet's temperature (0), mass flow (1), quality
//   (2), enthalpy (3), temperature limits (4, 5), minimum and maximum available flows (6, 7),
//   humidity ratio (8) and pressure (9).
// - BoundNode(v, minav, maxav): BoundValueToNodeMinMaxAvail on a node with those available flows.
// - Tighten(which, oldmin, oldmax, newmin, newmax): TightenNodeMinMaxAvails on a node with
//   available flows oldmin and oldmax; results: its minimum (0) and maximum (1) available flows.
// - BoundTwo(v, lo, hi): BoundValueToWithinTwoValues.
//
// Guards (ADR-009), which return NaN: which outside its range, lp outside 0 to 2; any input that
// is not finite or lies outside what EnergyPlus meets: flows outside -1000 to 1000, temperatures
// outside -100 to 500, quality outside 0 to 1, enthalpy outside -1e7 to 1e7, humidity ratio outside
// 0 to 0.1, pressures outside 0 to 1e7.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/Plant/DataPlant.hh>
#include <EnergyPlus/PlantUtilities.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(2);
        p->dataLoopNodes->NodeID.allocate(2);
        p->dataPlnt->PlantLoop.allocate(2);
        p->dataPlnt->PlantLoop(1).PressureSimType = DataPlant::PressSimType::NoPressure;
        p->dataPlnt->PlantLoop(2).PressureSimType = DataPlant::PressSimType::PumpPowerCorrection;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

bool flow(double v)
{
    return within(v, -1000.0, 1000.0);
}

Node::NodeData &fresh(EnergyPlus::EnergyPlusData &state, int n)
{
    auto &node = state.dataLoopNodes->Node(n);
    node = Node::NodeData();
    return node;
}
} // namespace

extern "C" {
double ep_InitNodes(int which, double minm, double maxm)
{
    if (which < 0 || which > 6 || !flow(minm) || !flow(maxm)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &in = fresh(state, 1);
    auto &out = fresh(state, 2);
    in.MassFlowRate = 1.0;
    in.MassFlowRateRequest = 1.0;
    out.MassFlowRate = 1.0;
    PlantUtilities::InitComponentNodes(state, minm, maxm, 1, 2);
    double const r[7] = {
        out.MassFlowRate, in.MassFlowRateMin, in.MassFlowRateMinAvail, in.MassFlowRateMax, in.MassFlowRateMaxAvail, in.MassFlowRate,
        in.MassFlowRateRequest};
    return r[which];
}

double ep_SafeCopy(int which,
                   int lp,
                   double t,
                   double m,
                   double q,
                   double h,
                   double tmin,
                   double tmax,
                   double mmin,
                   double mminav,
                   double mmax,
                   double mmaxav,
                   double w,
                   double p,
                   double op)
{
    double const flows[5] = {m, mmin, mminav, mmax, mmaxav};
    if (which < 0 || which > 9 || lp < 0 || lp > 2 || !within(t, -100.0, 500.0) || !within(q, 0.0, 1.0) || !within(h, -1.0e7, 1.0e7) ||
        !within(tmin, -100.0, 500.0) || !within(tmax, -100.0, 500.0) || !within(w, 0.0, 0.1) || !within(p, 0.0, 1.0e7) ||
        !within(op, 0.0, 1.0e7)) {
        return NAN;
    }
    for (double f : flows) {
        if (!flow(f)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &in = fresh(state, 1);
    auto &out = fresh(state, 2);
    in.Temp = t;
    in.MassFlowRate = m;
    in.Quality = q;
    in.Enthalpy = h;
    in.TempMin = tmin;
    in.TempMax = tmax;
    in.MassFlowRateMin = mmin;
    in.MassFlowRateMinAvail = mminav;
    in.MassFlowRateMax = mmax;
    in.MassFlowRateMaxAvail = mmaxav;
    in.HumRat = w;
    in.Press = p;
    out.Press = op;
    if (lp == 0) {
        PlantUtilities::SafeCopyPlantNode(state, 1, 2);
    } else {
        PlantUtilities::SafeCopyPlantNode(state, 1, 2, lp);
    }
    double const r[10] = {
        out.Temp, out.MassFlowRate, out.Quality, out.Enthalpy, out.TempMin, out.TempMax, out.MassFlowRateMinAvail, out.MassFlowRateMaxAvail,
        out.HumRat, out.Press};
    return r[which];
}

double ep_BoundNode(double v, double minav, double maxav)
{
    if (!flow(v) || !flow(minav) || !flow(maxav)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &node = fresh(state, 1);
    node.MassFlowRateMinAvail = minav;
    node.MassFlowRateMaxAvail = maxav;
    return PlantUtilities::BoundValueToNodeMinMaxAvail(state, v, 1);
}

double ep_Tighten(int which, double oldmin, double oldmax, double newmin, double newmax)
{
    if (which < 0 || which > 1 || !flow(oldmin) || !flow(oldmax) || !flow(newmin) || !flow(newmax)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &node = fresh(state, 1);
    node.MassFlowRateMinAvail = oldmin;
    node.MassFlowRateMaxAvail = oldmax;
    PlantUtilities::TightenNodeMinMaxAvails(state, 1, newmin, newmax);
    return which == 0 ? node.MassFlowRateMinAvail : node.MassFlowRateMaxAvail;
}

double ep_BoundTwo(double v, double lo, double hi)
{
    if (!flow(v) || !flow(lo) || !flow(hi)) {
        return NAN;
    }
    return PlantUtilities::BoundValueToWithinTwoValues(v, lo, hi);
}
}
