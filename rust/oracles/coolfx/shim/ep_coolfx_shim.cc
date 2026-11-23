// Written by hand: ThermalComfort::CalcCoolingEffectAdjustedPMV reads the person's comfort inputs
// from the EnergyPlusData state (as GetThermalComfortInputsASHRAE leaves them) and the outdoor
// barometric pressure, which gen_plain.py cannot set up. Plain extern "C" entry point over the real
// EnergyPlus ThermalComfort code.
//
// Each call sets up one People object whose comfort inputs are air temperature at (C), mean radiant
// temperature rt (C), relative humidity rh (0 to 1), air velocity vel (m/s), activity level act
// (W/m2, the activity in met being act / 58.2), clothing clo (clo) and work efficiency work (W/m2),
// at outdoor barometric pressure outbaro (Pa). Then the real CalcCoolingEffectAdjustedPMV runs,
// and CalcFangerPPD on its adjusted PMV. One EnergyPlusData for the process, built when the library
// loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 2; and, as the standard effective
// temperature iterations need not end otherwise, any input that is not finite or lies outside
// what EnergyPlus meets: at or rt outside 10 to 40 C (the root search goes 50 C below them), rh
// outside 0 to 1, vel outside 0 to 5 m/s, act outside 0.7 to 4 met, clo outside 0 to 3, work
// outside 0 to half the activity, outbaro outside 60000 to 110000 Pa.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/EPVector.hh>
#include <EnergyPlus/ThermalComfort.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataHeatBal->TotPeople = 1;
        p->dataHeatBal->People.allocate(1);
        p->dataHeatBal->People(1).Name = "PEOPLE 1";
        p->dataThermalComforts->PeopleNum = 1;
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
// One result (see the file comment): which picks the cooling effect (0), the cooling effect
// adjusted PMV (1) and its PPD (2).
double ep_CoolingEffect(int which, double outbaro, double at, double rt, double rh, double vel, double act, double clo, double work)
{
    if (which < 0 || which > 2 || !within(outbaro, 60000.0, 110000.0) || !within(at, 10.0, 40.0) || !within(rt, 10.0, 40.0) ||
        !within(rh, 0.0, 1.0) || !within(vel, 0.0, 5.0) || !within(act, 0.7 * 58.2, 4.0 * 58.2) || !within(clo, 0.0, 3.0) ||
        !within(work, 0.0, act / 2.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataEnvrn->OutBaroPress = outbaro;
    auto &tc = *state.dataThermalComforts;
    tc.AirTemp = at;
    tc.RadTemp = rt;
    tc.RelHum = rh;
    tc.AirVel = vel;
    tc.ActLevel = act;
    tc.ActMet = act / ThermalComfort::ActLevelConv;
    tc.CloUnit = clo;
    tc.WorkEff = work;
    double ce = 0.0;
    double pmv = 0.0;
    ThermalComfort::CalcCoolingEffectAdjustedPMV(state, ce, pmv);
    switch (which) {
    case 0:
        return ce;
    case 1:
        return pmv;
    default:
        return ThermalComfort::CalcFangerPPD(pmv);
    }
}
}
