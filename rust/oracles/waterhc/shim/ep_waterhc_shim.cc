// Written by hand: WaterCoils::CalcSimpleHeatingCoil reads the coil object (UA, availability
// schedule, its plant loop's fluid), its inlet air and water states, which gen_plain.py cannot set
// up. Plain extern "C" entry point over the real EnergyPlus WaterCoils code.
//
// Each call sets up one Coil:Heating:Water (simple, UA and flow) as InitWaterCoil leaves it, on a
// plant loop of water: UA ua (W/K), availability schedule value avail, inlet air mass flow mdota
// (kg/s), temperature tain (C) and humidity ratio win (kg/kg), inlet water mass flow mdotw and its
// maximum mdotwmax (kg/s), temperature twin (C) and enthalpy hwin (J/kg). Then the real routine runs
// with the fan cycling (cycling 1, part load ratio plr) or continuous, as a design (design 1) or
// simulation calculation. One EnergyPlusData for the process, built when the library loads, with
// console output off and the built-in fluid properties.
//
// Guards (ADR-009), which return NaN: which outside 0 to 5, cycling or design not 0 or 1; any input
// that is not finite or lies outside what EnergyPlus meets: ua outside 1 to 1e7 (ShowFatalError at
// 0), avail outside 0 to 1, mdota outside 0 to 50, tain outside -40 to 60, win outside 0 to 0.04,
// mdotw outside 0 to mdotwmax, mdotwmax outside 0 to 100, twin outside 1 to 99 (the water specific
// heat warns outside its table), hwin outside -1e6 to 1e6, plr outside 0 to 1.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/FluidProperties.hh>
#include <EnergyPlus/Plant/DataPlant.hh>
#include <EnergyPlus/ScheduleManager.hh> // before WaterCoils.hh, which uses Sched::Schedule
#include <EnergyPlus/WaterCoils.hh>

using namespace EnergyPlus;

namespace {
Sched::ScheduleConstant *avail = nullptr;
DataPlant::PlantLoopData *loop = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        Fluid::InitConstantFluidPropertiesData(*p);
        avail = new Sched::ScheduleConstant;
        loop = new DataPlant::PlantLoopData;
        loop->glycol = Fluid::GetWater(*p);
        auto &wc = *p->dataWaterCoils;
        wc.WaterCoil.allocate(1);
        wc.MySizeFlag.dimension(1, false);
        wc.MyUAAndFlowCalcFlag.dimension(1, false);
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
// One result: the heating rate (0, W), outlet air temperature (1, C), outlet water temperature
// (2, C), outlet water enthalpy (3, J/kg) and mass flow (4, kg/s), outlet air enthalpy (5, J/kg).
double ep_WaterHeatCoil(int which,
                        int cycling,
                        int design,
                        double ua,
                        double availVal,
                        double mdota,
                        double tain,
                        double win,
                        double mdotw,
                        double mdotwmax,
                        double twin,
                        double hwin,
                        double plr)
{
    if (which < 0 || which > 5 || (cycling != 0 && cycling != 1) || (design != 0 && design != 1) || !within(ua, 1.0, 1.0e7) ||
        !within(availVal, 0.0, 1.0) || !within(mdota, 0.0, 50.0) || !within(tain, -40.0, 60.0) || !within(win, 0.0, 0.04) ||
        !within(mdotwmax, 0.0, 100.0) || !within(mdotw, 0.0, mdotwmax) || !within(twin, 1.0, 99.0) || !within(hwin, -1.0e6, 1.0e6) ||
        !within(plr, 0.0, 1.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    avail->currentVal = availVal;
    auto &coil = state.dataWaterCoils->WaterCoil(1);
    coil.Name = "COIL";
    coil.availSched = avail;
    coil.WaterPlantLoc.loop = loop;
    coil.UACoilVariable = ua;
    coil.InletAirMassFlowRate = mdota;
    coil.InletAirTemp = tain;
    coil.InletAirHumRat = win;
    coil.InletWaterMassFlowRate = mdotw;
    coil.MaxWaterMassFlowRate = mdotwmax;
    coil.InletWaterTemp = twin;
    coil.InletWaterEnthalpy = hwin;
    int const calcMode = design == 1 ? state.dataWaterCoils->DesignCalc : state.dataWaterCoils->SimCalc;
    WaterCoils::CalcSimpleHeatingCoil(state, 1, cycling == 1 ? HVAC::FanOp::Cycling : HVAC::FanOp::Continuous, plr, calcMode);
    switch (which) {
    case 0:
        return coil.TotWaterHeatingCoilRate;
    case 1:
        return coil.OutletAirTemp;
    case 2:
        return coil.OutletWaterTemp;
    case 3:
        return coil.OutletWaterEnthalpy;
    case 4:
        return coil.OutletWaterMassFlowRate;
    default:
        return coil.OutletAirEnthalpy;
    }
}
}
