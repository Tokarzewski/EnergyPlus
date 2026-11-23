// Written by hand: ThermalComfort::CalcFangerPMV and CalcStandardEffectiveTemp take the
// EnergyPlusData state, keep their intermediate results in it and read the outdoor barometric
// pressure from it, which gen_plain.py cannot set up. Plain extern "C" entry points over the real
// EnergyPlus ThermalComfort code.
//
// FangerPMV calls CalcFangerPMV for air temperature at (C), mean radiant temperature rt (C),
// relative humidity rh (0 to 1), air velocity vel (m/s), activity level act (W/m2), clothing clo
// (clo) and work efficiency work (W/m2). StandardEffectiveTemp calls CalcStandardEffectiveTemp for
// the same inputs with the activity in met and the outdoor barometric pressure outbaro (Pa). One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which out of range; and for StandardEffectiveTemp, whose
// iterations need not end otherwise, any input that is not finite or lies outside what EnergyPlus
// can meet: at or rt outside -40 to 60 C, rh outside 0 to 1, vel outside 0 to 5 m/s, act outside
// 0.7 to 4 met, clo outside 0 to 3, work outside 0 to half the activity, outbaro outside 60000 to
// 110000 Pa.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/EPVector.hh>
#include <EnergyPlus/ThermalComfort.hh>

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
} // namespace

extern "C" {
// One result of CalcFangerPMV (see the file comment): which picks the PMV (0), the clothing surface
// temperature CloSurfTemp (1), the iteration count IterNum (2) and the convective heat transfer
// coefficient Hc (3).
double ep_FangerPMV(int which, double at, double rt, double rh, double vel, double act, double clo, double work)
{
    if (which < 0 || which > 3) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &tc = *state.dataThermalComforts;
    double const pmv = ThermalComfort::CalcFangerPMV(state, at, rt, rh, vel, act, clo, work);
    switch (which) {
    case 0:
        return pmv;
    case 1:
        return tc.CloSurfTemp;
    case 2:
        return tc.IterNum;
    default:
        return tc.Hc;
    }
}

// One result of CalcStandardEffectiveTemp (see the file comment): which picks SET (0), the
// effective temperature EffTemp (1), the skin and core temperatures SkinTemp (2) and CoreTemp (3),
// the skin wettedness SkinWetTot (4) and the dry heat loss at SET, DryHeatLossSET (5).
double ep_StandardEffectiveTemp(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work)
{
    if (which < 0 || which > 5 || !within(outbaro, 60000.0, 110000.0) || !within(at, -40.0, 60.0) || !within(rt, -40.0, 60.0) ||
        !within(rh, 0.0, 1.0) || !within(vel, 0.0, 5.0) || !within(act, 0.7, 4.0) || !within(clo, 0.0, 3.0) ||
        !within(work, 0.0, act * 58.2 / 2.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataEnvrn->OutBaroPress = outbaro;
    auto &tc = *state.dataThermalComforts;
    double const set = ThermalComfort::CalcStandardEffectiveTemp(state, at, rt, rh, vel, act, clo, work);
    switch (which) {
    case 0:
        return set;
    case 1:
        return tc.EffTemp;
    case 2:
        return tc.SkinTemp;
    case 3:
        return tc.CoreTemp;
    case 4:
        return tc.SkinWetTot;
    default:
        return tc.DryHeatLossSET;
    }
}
}
