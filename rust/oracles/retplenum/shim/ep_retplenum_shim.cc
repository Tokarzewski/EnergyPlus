// Written by hand: ZonePlenum::CalcAirZoneReturnPlenum reads a return plenum's inlet, induced-air
// and zone conditions and the air distribution units that leak into it, which gen_plain.py cannot
// set up. Plain extern "C" entry point over the real EnergyPlus ZonePlenum code.
//
// Each call sets up one AirLoopHVAC:ReturnPlenum as InitAirZoneReturnPlenum leaves it: nin inlets
// (1 to 4) with mass flow m, maximum and minimum available flows x and n (kg/s) and pressure p (Pa);
// nadu air distribution units (0 to 2) that can leak into it, each with an upstream and a
// downstream leak flag (u, d), leak flows lu, ld and parallel PIU leak lp (kg/s) and changes to the
// available flows dx and dn (kg/s); nind induced-air outlets (0 to 2) with mass flows i1 and i2
// (kg/s); and the plenum zone at temperature tz (C), humidity ratio wz and enthalpy hz (J/kg). One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 6, nin outside 1 to 4, nadu or nind
// outside 0 to 2, a leak flag not 0 or 1; any input that is not finite or lies outside what
// EnergyPlus meets: a flow outside 0 to 1000, an available-flow change outside -1000 to 1000, a
// pressure outside 0 to 2e5, tz outside -60 to 80, wz outside 0 to 0.1, hz outside -1e5 to 1e6.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataDefineEquip.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/ZonePlenum.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataZonePlenum->ZoneRetPlenCond.allocate(1);
        auto &c = p->dataZonePlenum->ZoneRetPlenCond(1);
        c.InletMassFlowRate.allocate(4);
        c.InletMassFlowRateMaxAvail.allocate(4);
        c.InletMassFlowRateMinAvail.allocate(4);
        c.InletPressure.allocate(4);
        c.InducedMassFlowRate.allocate(2);
        c.ADUIndex.allocate(2);
        c.ADUIndex(1) = 1;
        c.ADUIndex(2) = 2;
        p->dataDefineEquipment->AirDistUnit.allocate(2);
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
// One result: the outlet's mass flow (0), maximum available (1) and minimum available (2) flows
// (kg/s), temperature (3, C), humidity ratio (4), pressure (5, Pa) and enthalpy (6, J/kg).
double ep_RetPlenum(int which,
                    int nin,
                    int nadu,
                    int nind,
                    double m1,
                    double x1,
                    double n1,
                    double p1,
                    double m2,
                    double x2,
                    double n2,
                    double p2,
                    double m3,
                    double x3,
                    double n3,
                    double p3,
                    double m4,
                    double x4,
                    double n4,
                    double p4,
                    int u1,
                    int d1,
                    double lu1,
                    double ld1,
                    double lp1,
                    double dx1,
                    double dn1,
                    int u2,
                    int d2,
                    double lu2,
                    double ld2,
                    double lp2,
                    double dx2,
                    double dn2,
                    double i1,
                    double i2,
                    double tz,
                    double wz,
                    double hz)
{
    double const ms[4] = {m1, m2, m3, m4};
    double const xs[4] = {x1, x2, x3, x4};
    double const ns[4] = {n1, n2, n3, n4};
    double const ps[4] = {p1, p2, p3, p4};
    int const us[2] = {u1, u2};
    int const ds[2] = {d1, d2};
    double const lus[2] = {lu1, lu2};
    double const lds[2] = {ld1, ld2};
    double const lps[2] = {lp1, lp2};
    double const dxs[2] = {dx1, dx2};
    double const dns[2] = {dn1, dn2};
    double const is[2] = {i1, i2};
    if (which < 0 || which > 6 || nin < 1 || nin > 4 || nadu < 0 || nadu > 2 || nind < 0 || nind > 2 || !within(tz, -60.0, 80.0) ||
        !within(wz, 0.0, 0.1) || !within(hz, -1.0e5, 1.0e6)) {
        return NAN;
    }
    for (int k = 0; k < 4; ++k) {
        if (!within(ms[k], 0.0, 1000.0) || !within(xs[k], 0.0, 1000.0) || !within(ns[k], 0.0, 1000.0) || !within(ps[k], 0.0, 2.0e5)) {
            return NAN;
        }
    }
    for (int k = 0; k < 2; ++k) {
        if (!flag(us[k]) || !flag(ds[k]) || !within(lus[k], 0.0, 1000.0) || !within(lds[k], 0.0, 1000.0) || !within(lps[k], 0.0, 1000.0) ||
            !within(dxs[k], -1000.0, 1000.0) || !within(dns[k], -1000.0, 1000.0) || !within(is[k], 0.0, 1000.0)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &c = state.dataZonePlenum->ZoneRetPlenCond(1);
    c.NumInletNodes = nin;
    c.NumADUs = nadu;
    c.NumInducedNodes = nind;
    for (int k = 0; k < 4; ++k) {
        c.InletMassFlowRate(k + 1) = ms[k];
        c.InletMassFlowRateMaxAvail(k + 1) = xs[k];
        c.InletMassFlowRateMinAvail(k + 1) = ns[k];
        c.InletPressure(k + 1) = ps[k];
    }
    for (int k = 0; k < 2; ++k) {
        auto &adu = state.dataDefineEquipment->AirDistUnit(k + 1);
        adu.UpStreamLeak = us[k] == 1;
        adu.DownStreamLeak = ds[k] == 1;
        adu.MassFlowRateUpStrLk = lus[k];
        adu.MassFlowRateDnStrLk = lds[k];
        adu.massFlowRateParallelPIULk = lps[k];
        adu.MaxAvailDelta = dxs[k];
        adu.MinAvailDelta = dns[k];
        c.InducedMassFlowRate(k + 1) = is[k];
    }
    c.ZoneTemp = tz;
    c.ZoneHumRat = wz;
    c.ZoneEnthalpy = hz;
    ZonePlenum::CalcAirZoneReturnPlenum(state, 1);
    double const r[7] = {
        c.OutletMassFlowRate, c.OutletMassFlowRateMaxAvail, c.OutletMassFlowRateMinAvail, c.OutletTemp, c.OutletHumRat, c.OutletPressure,
        c.OutletEnthalpy};
    return r[which];
}
}
