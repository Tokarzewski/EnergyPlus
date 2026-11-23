// Written by hand: HeatingCoils::CalcElectricHeatingCoil and CalcFuelHeatingCoil read the coil
// object (availability schedule, efficiency, capacity, parasitics, part load fraction curve), its
// inlet air state and set point, which gen_plain.py cannot set up. Plain extern "C" entry point over
// the real EnergyPlus HeatingCoils code.
//
// Each call sets up one Coil:Heating:Electric (fuel 0) or Coil:Heating:Fuel (fuel 1) as
// InitHeatingCoil leaves it: availability schedule value avail, inlet air mass flow mdot (kg/s),
// temperature tin (C) and humidity ratio win (kg/kg), desired outlet temperature tset (C),
// efficiency eff, nominal capacity cap (W), and for fuel coils the parasitic electric load parelec
// (W) and parasitic fuel capacity parfuel (W), and with plf 1 a quadratic part load fraction curve
// c0 + c1 x + c2 x^2 over x from 0 to 1. The coil is asked for load qreq (W), or with sensed 1 for
// its set point (QCoilReq = SensedLoadFlagValue), with the fan cycling (cycling 1, part load ratio
// plr) or continuous. One EnergyPlusData for the process, built when the library loads, with
// console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 9, fuel, cycling, sensed or plf not 0 or
// 1; any input that is not finite or lies outside what EnergyPlus meets: avail outside 0 to 1, mdot
// outside 0 to 50, tin or tset outside -40 to 80, win outside 0 to 0.04, qreq outside -1e6 to 1e7,
// plr outside 0 to 1, eff outside 0.05 to 1.2, cap outside 0 to 1e7, parelec or parfuel outside 0
// to 1e4, c0 to c2 outside -2 to 2; and, as the fuel coil warns once and then on every call
// otherwise, a part load fraction curve under 0.7005 or under x / 1.0005 anywhere on 0 to 1
// (checked at 0, 1 and the vertex).
#include <cmath>
#include <EnergyPlus/CurveManager.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/ScheduleManager.hh> // before HeatingCoils.hh, which uses Sched::Schedule
#include <EnergyPlus/HeatingCoils.hh>

using namespace EnergyPlus;

namespace {
Sched::ScheduleConstant *avail = nullptr;
Curve::Curve *plfCurve = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        avail = new Sched::ScheduleConstant;
        plfCurve = Curve::AddCurve(*p, "PLF");
        plfCurve->curveType = Curve::CurveType::Quadratic;
        plfCurve->numDims = 1;
        plfCurve->inputLimits[0].min = 0.0;
        plfCurve->inputLimits[0].max = 1.0;
        p->dataHeatingCoils->HeatingCoil.allocate(1);
        p->dataLoopNodes->Node.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

// c0 + c1 x + c2 x^2 - k x is at least m at 0, 1 and, when it lies between them, the vertex.
bool quadAtLeast(double c0, double c1, double c2, double k, double m)
{
    double const b = c1 - k;
    auto f = [&](double x) { return c0 + x * (b + x * c2); };
    if (f(0.0) < m || f(1.0) < m) {
        return false;
    }
    if (c2 > 0.0) {
        double const xv = -b / (2.0 * c2);
        if (xv > 0.0 && xv < 1.0 && f(xv) < m) {
            return false;
        }
    }
    return true;
}
} // namespace

extern "C" {
// One result: the load delivered (0, W), outlet air temperature (1, C) and enthalpy (2, J/kg), the
// electric use (3, W), fuel use (4, W), runtime fraction (5), parasitic fuel rate (6, W), the on/off
// fan part load fraction global (7, from 1), the electric heating coil power global (8, from 0) and
// QCoilReq after the call (9, W).
double ep_HeatCoil(int which,
                   int fuel,
                   int cycling,
                   int sensed,
                   int plf,
                   double availVal,
                   double mdot,
                   double tin,
                   double win,
                   double tset,
                   double qreq,
                   double plr,
                   double eff,
                   double cap,
                   double parelec,
                   double parfuel,
                   double c0,
                   double c1,
                   double c2)
{
    if (which < 0 || which > 9 || (fuel != 0 && fuel != 1) || (cycling != 0 && cycling != 1) || (sensed != 0 && sensed != 1) ||
        (plf != 0 && plf != 1) || !within(availVal, 0.0, 1.0) || !within(mdot, 0.0, 50.0) || !within(tin, -40.0, 80.0) ||
        !within(win, 0.0, 0.04) || !within(tset, -40.0, 80.0) || !within(qreq, -1.0e6, 1.0e7) || !within(plr, 0.0, 1.0) ||
        !within(eff, 0.05, 1.2) || !within(cap, 0.0, 1.0e7) || !within(parelec, 0.0, 1.0e4) || !within(parfuel, 0.0, 1.0e4) ||
        !within(c0, -2.0, 2.0) || !within(c1, -2.0, 2.0) || !within(c2, -2.0, 2.0) ||
        (plf == 1 && (!quadAtLeast(c0, c1, c2, 0.0, 0.7005) || !quadAtLeast(c0, c1, c2, 1.0 / 1.0005, 0.0)))) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataHVACGlobal->OnOffFanPartLoadFraction = 1.0;
    state.dataHVACGlobal->ElecHeatingCoilPower = 0.0;
    avail->currentVal = availVal;
    plfCurve->coeff[0] = c0;
    plfCurve->coeff[1] = c1;
    plfCurve->coeff[2] = c2;
    auto &coil = state.dataHeatingCoils->HeatingCoil(1);
    coil = HeatingCoils::HeatingCoilEquipConditions();
    coil.Name = "COIL";
    coil.availSched = avail;
    coil.InletAirMassFlowRate = mdot;
    coil.InletAirTemp = tin;
    coil.InletAirHumRat = win;
    coil.DesiredOutletTemp = tset;
    coil.Efficiency = eff;
    coil.NominalCapacity = cap;
    coil.ParasiticElecLoad = parelec;
    coil.ParasiticFuelCapacity = parfuel;
    coil.PLFCurveIndex = plf == 1 ? plfCurve->Num : 0;
    coil.AirOutletNodeNum = 1;
    double q = sensed == 1 ? Node::SensedLoadFlagValue : qreq;
    double actual = 0.0;
    HVAC::FanOp const fanOp = cycling == 1 ? HVAC::FanOp::Cycling : HVAC::FanOp::Continuous;
    if (fuel == 0) {
        HeatingCoils::CalcElectricHeatingCoil(state, 1, q, actual, fanOp, plr);
    } else {
        HeatingCoils::CalcFuelHeatingCoil(state, 1, q, actual, fanOp, plr);
    }
    switch (which) {
    case 0:
        return actual;
    case 1:
        return coil.OutletAirTemp;
    case 2:
        return coil.OutletAirEnthalpy;
    case 3:
        return coil.ElecUseLoad;
    case 4:
        return coil.FuelUseLoad;
    case 5:
        return coil.RTF;
    case 6:
        return coil.ParasiticFuelRate;
    case 7:
        return state.dataHVACGlobal->OnOffFanPartLoadFraction;
    case 8:
        return state.dataHVACGlobal->ElecHeatingCoilPower;
    default:
        return q;
    }
}
}
