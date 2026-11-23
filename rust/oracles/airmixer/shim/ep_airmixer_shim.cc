// Written by hand: MixerComponent::CalcAirMixer reads the mixer object's inlet streams as
// InitAirMixer copies them from the nodes, and its inlet flow fractions from the previous call,
// which gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// MixerComponent code.
//
// Each call sets up one AirLoopHVAC:ZoneMixer with n inlets (1 to 4) as InitAirMixer leaves it:
// inlet i has mass flow m<i>, maximum and minimum available flows maxav<i> and minav<i> (kg/s),
// temperature t<i> (C), humidity ratio w<i> (kg/kg), enthalpy h<i> (J/kg), pressure p<i> (Pa) and
// the flow fraction f<i> the last call left, then calls the real routine. There are no parallel
// PIUs, so no leakage is added. One EnergyPlusData for the process, built when the library loads,
// with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 10, n outside 1 to 4; any input of any
// inlet that is not finite or lies outside what EnergyPlus meets: flows outside 0 to 1000,
// temperatures outside -60 to 80, humidity ratios outside 0 to 0.1, enthalpies outside -2e4 to
// 4e5, pressures outside 5e4 to 1.2e5, flow fractions outside 0 to 1.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/MixerComponent.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataMixerComponent->MixerCond.allocate(1);
        auto &m = p->dataMixerComponent->MixerCond(1);
        m.MixerName = "MIXER";
        m.InletNode.allocate(4);
        m.InletMassFlowRate.allocate(4);
        m.InletMassFlowRateMaxAvail.allocate(4);
        m.InletMassFlowRateMinAvail.allocate(4);
        m.InletTemp.allocate(4);
        m.InletHumRat.allocate(4);
        m.InletEnthalpy.allocate(4);
        m.InletPressure.allocate(4);
        m.InletFlowFraction.allocate(4);
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
// One result: the outlet temperature (0, C), humidity ratio (1, kg/kg), enthalpy (2, J/kg),
// pressure (3, Pa), mass flow (4, kg/s), maximum and minimum available flows (5 and 6, kg/s), and
// inlet i's flow fraction after the call (6 + i).
double ep_AirMixer(int which,
                   int n,
                   double m1,
                   double maxav1,
                   double minav1,
                   double t1,
                   double w1,
                   double h1,
                   double p1,
                   double f1,
                   double m2,
                   double maxav2,
                   double minav2,
                   double t2,
                   double w2,
                   double h2,
                   double p2,
                   double f2,
                   double m3,
                   double maxav3,
                   double minav3,
                   double t3,
                   double w3,
                   double h3,
                   double p3,
                   double f3,
                   double m4,
                   double maxav4,
                   double minav4,
                   double t4,
                   double w4,
                   double h4,
                   double p4,
                   double f4)
{
    double const in[4][8] = {{m1, maxav1, minav1, t1, w1, h1, p1, f1},
                             {m2, maxav2, minav2, t2, w2, h2, p2, f2},
                             {m3, maxav3, minav3, t3, w3, h3, p3, f3},
                             {m4, maxav4, minav4, t4, w4, h4, p4, f4}};
    if (which < 0 || which > 10 || n < 1 || n > 4) {
        return NAN;
    }
    for (auto const &s : in) {
        if (!within(s[0], 0.0, 1000.0) || !within(s[1], 0.0, 1000.0) || !within(s[2], 0.0, 1000.0) || !within(s[3], -60.0, 80.0) ||
            !within(s[4], 0.0, 0.1) || !within(s[5], -2.0e4, 4.0e5) || !within(s[6], 5.0e4, 1.2e5) || !within(s[7], 0.0, 1.0)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &m = state.dataMixerComponent->MixerCond(1);
    m.NumInletNodes = n;
    for (int i = 1; i <= 4; ++i) {
        auto const &s = in[i - 1];
        m.InletNode(i) = i;
        m.InletMassFlowRate(i) = s[0];
        m.InletMassFlowRateMaxAvail(i) = s[1];
        m.InletMassFlowRateMinAvail(i) = s[2];
        m.InletTemp(i) = s[3];
        m.InletHumRat(i) = s[4];
        m.InletEnthalpy(i) = s[5];
        m.InletPressure(i) = s[6];
        m.InletFlowFraction(i) = s[7];
    }
    int mixerNum = 1;
    MixerComponent::CalcAirMixer(state, mixerNum);
    switch (which) {
    case 0:
        return m.OutletTemp;
    case 1:
        return m.OutletHumRat;
    case 2:
        return m.OutletEnthalpy;
    case 3:
        return m.OutletPressure;
    case 4:
        return m.OutletMassFlowRate;
    case 5:
        return m.OutletMassFlowRateMaxAvail;
    case 6:
        return m.OutletMassFlowRateMinAvail;
    default:
        return m.InletFlowFraction(which - 6);
    }
}
}
