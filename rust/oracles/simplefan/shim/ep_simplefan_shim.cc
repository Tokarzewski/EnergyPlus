// Written by hand: Fans::FanComponent's simulate methods read the fan object (its availability
// schedule, design pressure rise, efficiencies, flow limits and part load coefficients), its inlet
// air state and the HVAC globals, which gen_plain.py cannot set up. Plain extern "C" entry point over
// the real EnergyPlus Fans code.
//
// Each call sets up one fan as FanComponent::init leaves it: availability schedule value avail,
// inlet mass flow mdot (kg/s), maximum and minimum mass flow mdotmax and mdotmin (kg/s), maximum and
// minimum volume flow volmax and volmin (m3/s), design pressure rise dp (Pa), total efficiency eff,
// motor efficiency moteff, motor in air fraction motinair, standard air density rho (kg/m3), inlet
// air temperature tin (C) and humidity ratio win (kg/kg) with its enthalpy from PsyHFnTdbW, and the
// part load coefficients c0 to c4. kind picks the model: Fan:ConstantVolume (0), Fan:VariableVolume
// (1), Fan:VariableVolume with the pressure rise pr passed in (2), Fan:OnOff (3, with the coil's
// part load fraction plf). turn sets TurnFansOn (1) or TurnFansOff (2); bits 1, 2 and 4 of ems turn
// on the EMS overrides of mass flow (emsflow), pressure rise (emspress) and total efficiency
// (emseff). The control flow maximum starts at mdotmax. One EnergyPlusData for the process, built
// when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 8, kind outside 0 to 3, turn outside 0 to
// 2, ems outside 0 to 7; any input that is not finite or lies outside what EnergyPlus meets: avail
// outside 0 to 1, mdot outside 0 to 100, mdotmax outside 0.01 to 100, mdotmin outside 0 to mdotmax,
// volmax outside 0.01 to 100, volmin outside 0 to volmax, dp, pr or emspress outside 0 to 5000,
// eff or emseff outside 0.05 to 1, moteff or motinair outside 0 to 1, rho outside 0.5 to 1.5, tin
// outside -40 to 60, win outside 0 to 0.04, plf outside 0.05 to 1, c0 to c4 outside -10 to 10,
// emsflow outside 0 to 100.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/ScheduleManager.hh> // before Fans.hh, which uses Sched::Schedule
#include <EnergyPlus/Fans.hh>
#include <EnergyPlus/Psychrometrics.hh>

using namespace EnergyPlus;

namespace {
Sched::ScheduleConstant *avail = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        avail = new Sched::ScheduleConstant;
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
// One result (see the file comment): total power (0, W), power lost to the air (1, W), outlet air
// temperature (2, C), enthalpy (3, J/kg), mass flow (4, kg/s) and humidity ratio (5, kg/kg), the
// runtime fraction (6), the control flow maximum (7, kg/s) and the on/off fan part load fraction
// global after the call (8).
double ep_SimpleFan(int which,
                    int kind,
                    int turn,
                    int ems,
                    double availVal,
                    double mdot,
                    double mdotmax,
                    double mdotmin,
                    double volmax,
                    double volmin,
                    double dp,
                    double eff,
                    double moteff,
                    double motinair,
                    double rho,
                    double tin,
                    double win,
                    double plf,
                    double c0,
                    double c1,
                    double c2,
                    double c3,
                    double c4,
                    double pr,
                    double emsflow,
                    double emspress,
                    double emseff)
{
    if (which < 0 || which > 8 || kind < 0 || kind > 3 || turn < 0 || turn > 2 || ems < 0 || ems > 7 || !within(availVal, 0.0, 1.0) ||
        !within(mdot, 0.0, 100.0) || !within(mdotmax, 0.01, 100.0) || !within(mdotmin, 0.0, mdotmax) || !within(volmax, 0.01, 100.0) ||
        !within(volmin, 0.0, volmax) || !within(dp, 0.0, 5000.0) || !within(eff, 0.05, 1.0) || !within(moteff, 0.0, 1.0) ||
        !within(motinair, 0.0, 1.0) || !within(rho, 0.5, 1.5) || !within(tin, -40.0, 60.0) || !within(win, 0.0, 0.04) ||
        !within(plf, 0.05, 1.0) || !within(c0, -10.0, 10.0) || !within(c1, -10.0, 10.0) || !within(c2, -10.0, 10.0) ||
        !within(c3, -10.0, 10.0) || !within(c4, -10.0, 10.0) || !within(pr, 0.0, 5000.0) || !within(emsflow, 0.0, 100.0) ||
        !within(emspress, 0.0, 5000.0) || !within(emseff, 0.05, 1.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &hvac = *state.dataHVACGlobal;
    hvac.TurnFansOn = (turn == 1);
    hvac.TurnFansOff = (turn == 2);
    hvac.NightVentOn = false;
    hvac.OnOffFanPartLoadFraction = plf;
    avail->currentVal = availVal;
    Fans::FanComponent fan;
    fan.Name = "FAN";
    fan.availSched = avail;
    fan.inletAirMassFlowRate = mdot;
    fan.maxAirMassFlowRate = mdotmax;
    fan.minAirMassFlowRate = mdotmin;
    fan.maxAirFlowRate = volmax;
    fan.minAirFlowRate = volmin;
    fan.deltaPress = dp;
    fan.totalEff = eff;
    fan.motorEff = moteff;
    fan.motorInAirFrac = motinair;
    fan.rhoAirStdInit = rho;
    fan.inletAirTemp = tin;
    fan.inletAirHumRat = win;
    fan.inletAirEnthalpy = Psychrometrics::PsyHFnTdbW(tin, win);
    fan.coeffs = {c0, c1, c2, c3, c4};
    fan.massFlowRateMaxAvail = mdotmax;
    fan.massFlowRateMinAvail = mdotmin;
    fan.EMSMaxMassFlowOverrideOn = (ems & 1) != 0;
    fan.EMSAirMassFlowValue = emsflow;
    fan.EMSPressureOverrideOn = (ems & 2) != 0;
    fan.EMSPressureValue = emspress;
    fan.EMSTotalEffOverrideOn = (ems & 4) != 0;
    fan.EMSTotalEffValue = emseff;
    switch (kind) {
    case 0:
        fan.simulateConstant(state);
        break;
    case 1:
        fan.simulateVAV(state);
        break;
    case 2:
        fan.simulateVAV(state, pr);
        break;
    default:
        fan.simulateOnOff(state);
    }
    switch (which) {
    case 0:
        return fan.totalPower;
    case 1:
        return fan.powerLossToAir;
    case 2:
        return fan.outletAirTemp;
    case 3:
        return fan.outletAirEnthalpy;
    case 4:
        return fan.outletAirMassFlowRate;
    case 5:
        return fan.outletAirHumRat;
    case 6:
        return fan.runtimeFrac;
    case 7:
        return fan.massFlowRateMaxAvail;
    default:
        return hvac.OnOffFanPartLoadFraction;
    }
}
}
