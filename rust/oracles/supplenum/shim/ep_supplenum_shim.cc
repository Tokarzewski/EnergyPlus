// Written by hand: ZonePlenum::CalcAirZoneSupplyPlenum and UpdateAirZoneSupplyPlenum read a supply
// plenum's outlet flows and zone conditions and write its inlet, zone and outlet nodes, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus ZonePlenum code.
//
// Each call sets up one AirLoopHVAC:SupplyPlenum as InitAirZoneSupplyPlenum leaves it, with inlet
// node 1, zone node 2 and nout outlet nodes (1 to 4) from node 3 on, and runs Calc then Update for
// the first pass (first 1: the zone's air passed to the outlets) or the second (first 0: the
// outlets' flows summed back to the inlet). The inlet node has mass flow inm and available flows
// inmx and inmn (kg/s); outlet k asks for mass flow mk with available flows xk and nk (kg/s); the
// plenum zone is at temperature tz (C), humidity ratio wz and enthalpy hz (J/kg). The zone and
// outlet nodes start at rest (flows, temperature, humidity ratio and enthalpy 0) and the contaminant
// models are off. One EnergyPlusData for the process, built when the library loads, with console
// output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 7, first not 0 or 1, nout outside 1 to 4;
// any input that is not finite or lies outside what EnergyPlus meets: a flow outside 0 to 1000, tz
// outside -60 to 80, wz outside 0 to 0.1, hz outside -1e5 to 1e6.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/ZonePlenum.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(6);
        p->dataLoopNodes->NodeID.allocate(6);
        p->dataZonePlenum->ZoneSupPlenCond.allocate(1);
        auto &c = p->dataZonePlenum->ZoneSupPlenCond(1);
        c.InletNode = 1;
        c.ZoneNodeNum = 2;
        c.OutletNode.allocate(4);
        c.OutletMassFlowRate.allocate(4);
        c.OutletMassFlowRateMaxAvail.allocate(4);
        c.OutletMassFlowRateMinAvail.allocate(4);
        c.OutletTemp.allocate(4);
        c.OutletHumRat.allocate(4);
        c.OutletEnthalpy.allocate(4);
        c.OutletPressure.allocate(4);
        for (int k = 1; k <= 4; ++k) {
            c.OutletNode(k) = k + 2;
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
// One result: the inlet node's mass flow (0), maximum available (1) and minimum available (2) flows
// (kg/s), whether the inlet flow changed by more than 0.01 kg/s (3, 1 or 0), the zone node's mass
// flow (4, kg/s), and the last outlet node's temperature (5, C), humidity ratio (6) and enthalpy
// (7, J/kg).
double ep_SupPlenum(int which,
                    int first,
                    int nout,
                    double inm,
                    double inmx,
                    double inmn,
                    double m1,
                    double x1,
                    double n1,
                    double m2,
                    double x2,
                    double n2,
                    double m3,
                    double x3,
                    double n3,
                    double m4,
                    double x4,
                    double n4,
                    double tz,
                    double wz,
                    double hz)
{
    double const ms[4] = {m1, m2, m3, m4};
    double const xs[4] = {x1, x2, x3, x4};
    double const ns[4] = {n1, n2, n3, n4};
    if (which < 0 || which > 7 || (first != 0 && first != 1) || nout < 1 || nout > 4 || !within(inm, 0.0, 1000.0) ||
        !within(inmx, 0.0, 1000.0) || !within(inmn, 0.0, 1000.0) || !within(tz, -60.0, 80.0) || !within(wz, 0.0, 0.1) ||
        !within(hz, -1.0e5, 1.0e6)) {
        return NAN;
    }
    for (int k = 0; k < 4; ++k) {
        if (!within(ms[k], 0.0, 1000.0) || !within(xs[k], 0.0, 1000.0) || !within(ns[k], 0.0, 1000.0)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    for (int k = 2; k <= 6; ++k) {
        state.dataLoopNodes->Node(k) = Node::NodeData();
    }
    auto &in = state.dataLoopNodes->Node(1);
    in.MassFlowRate = inm;
    in.MassFlowRateMaxAvail = inmx;
    in.MassFlowRateMinAvail = inmn;
    auto &c = state.dataZonePlenum->ZoneSupPlenCond(1);
    c.NumOutletNodes = nout;
    for (int k = 0; k < 4; ++k) {
        c.OutletMassFlowRate(k + 1) = ms[k];
        c.OutletMassFlowRateMaxAvail(k + 1) = xs[k];
        c.OutletMassFlowRateMinAvail(k + 1) = ns[k];
        c.OutletTemp(k + 1) = 0.0;
        c.OutletHumRat(k + 1) = 0.0;
        c.OutletEnthalpy(k + 1) = 0.0;
    }
    c.InletMassFlowRate = 0.0;
    c.InletMassFlowRateMaxAvail = 0.0;
    c.InletMassFlowRateMinAvail = 0.0;
    c.ZoneTemp = tz;
    c.ZoneHumRat = wz;
    c.ZoneEnthalpy = hz;
    bool changed = false;
    ZonePlenum::CalcAirZoneSupplyPlenum(state, 1, first == 1);
    ZonePlenum::UpdateAirZoneSupplyPlenum(state, 1, changed, first == 1);
    auto const &zone = state.dataLoopNodes->Node(2);
    auto const &last = state.dataLoopNodes->Node(nout + 2);
    double const r[8] = {
        in.MassFlowRate, in.MassFlowRateMaxAvail, in.MassFlowRateMinAvail, changed ? 1.0 : 0.0, zone.MassFlowRate, last.Temp, last.HumRat,
        last.Enthalpy};
    return r[which];
}
}
