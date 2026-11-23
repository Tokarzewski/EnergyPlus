// Written by hand: MixedAir::OAMixerProps::CalcOAMixer reads the mixer object's return, outdoor and
// relief streams as InitOAMixer copies them from the nodes, which gen_plain.py cannot set up. Plain
// extern "C" entry point over the real EnergyPlus MixedAir code.
//
// Each call sets up one OutdoorAir:Mixer as InitOAMixer leaves it: the return air at temperature
// rett (C), humidity ratio retw (kg/kg), enthalpy reth (J/kg), pressure retp (Pa) and mass flow
// retm (kg/s), the outdoor air at oat, oaw, oah, oap and oam, and the relief air mass flow relm
// (kg/s), then calls the real routine. One EnergyPlusData for the process, built when the library
// loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 5; any input that is not finite or lies
// outside what EnergyPlus meets: temperatures outside -60 to 80, humidity ratios outside 0 to 0.1,
// enthalpies outside -2e4 to 4e5 (the saturation temperature's table warns far outside), pressures
// outside 5e4 to 1.2e5, mass flows outside 0 to 1000.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/MixedAir.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

// One stream's temperature, humidity ratio, enthalpy, pressure and mass flow are in range.
bool streamOk(double t, double w, double h, double p, double m)
{
    return within(t, -60.0, 80.0) && within(w, 0.0, 0.1) && within(h, -2.0e4, 4.0e5) && within(p, 5.0e4, 1.2e5) && within(m, 0.0, 1000.0);
}
} // namespace

extern "C" {
// One result: the mixed air temperature (0, C), humidity ratio (1, kg/kg), enthalpy (2, J/kg),
// pressure (3, Pa) and mass flow (4, kg/s), and the relief air mass flow after the call (5, kg/s).
double ep_OAMixer(int which,
                  double rett,
                  double retw,
                  double reth,
                  double retp,
                  double retm,
                  double oat,
                  double oaw,
                  double oah,
                  double oap,
                  double oam,
                  double relm)
{
    if (which < 0 || which > 5 || !streamOk(rett, retw, reth, retp, retm) || !streamOk(oat, oaw, oah, oap, oam) || !within(relm, 0.0, 1000.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    MixedAir::OAMixerProps m;
    m.RetTemp = rett;
    m.RetHumRat = retw;
    m.RetEnthalpy = reth;
    m.RetPressure = retp;
    m.RetMassFlowRate = retm;
    m.OATemp = oat;
    m.OAHumRat = oaw;
    m.OAEnthalpy = oah;
    m.OAPressure = oap;
    m.OAMassFlowRate = oam;
    m.RelMassFlowRate = relm;
    m.CalcOAMixer(state);
    switch (which) {
    case 0:
        return m.MixTemp;
    case 1:
        return m.MixHumRat;
    case 2:
        return m.MixEnthalpy;
    case 3:
        return m.MixPressure;
    case 4:
        return m.MixMassFlowRate;
    default:
        return m.RelMassFlowRate;
    }
}
}
