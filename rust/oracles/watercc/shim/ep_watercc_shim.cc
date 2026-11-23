// Written by hand: WaterCoils::CoolingCoil reads the coil object (UAs, heat exchanger type,
// availability schedule, its plant loop's fluid), its inlet air and water states and the outdoor
// pressure, which gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// WaterCoils code.
//
// Each call sets up one Coil:Cooling:Water with the simple analysis as InitWaterCoil leaves it, on
// a plant loop of water: heat exchanger hex (1 counter flow, 2 cross flow), availability schedule
// value avail, dry UA uatot, internal and external UAs uaint and uaext (W/K), design air volume
// flow desvol (m3/s), inlet air mass flow mdota (kg/s), temperature tain (C) and humidity ratio
// win (kg/kg), inlet water mass flow mdotw and its maximum mdotwmax (kg/s), temperature twin (C)
// and enthalpy hwin (J/kg), and outdoor pressure pb (Pa). The coil's cooling rates are zeroed
// first (a coil that does not run keeps its last ones). Then the real routine runs with the fan
// cycling (cycling 1, part load ratio plr) or continuous, as a design (design 1) or simulation
// calculation. One EnergyPlusData for the process, built when the library loads, with console
// output off and the built-in fluid properties.
//
// Results: outlet air temperature (0, C) and humidity ratio (1), outlet water temperature (2, C),
// total (3) and sensible (4) cooling rates (W), wet surface fraction (5), outlet water enthalpy
// (6, J/kg) and outlet air enthalpy (7, J/kg).
//
// Guards (ADR-009), which return NaN: which outside 0 to 7, cycling or design not 0 or 1, hex not
// 1 or 2; any input that is not finite or lies outside what EnergyPlus meets: avail outside 0 to 1,
// the UAs outside 0 to 1e7, desvol outside 0 to 100, mdota outside 0 to 50, tain outside -40 to 60,
// win outside 0 to 0.04, mdotw outside 0 to mdotwmax, mdotwmax outside 0 to 100, twin outside 1 to
// 99 (the water specific heat warns outside its table), hwin outside -1e6 to 1e6, plr outside 0 to
// 1, pb outside 50000 to 120000.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
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
double ep_WaterCoolCoil(int which,
                        int cycling,
                        int design,
                        int hex,
                        double availVal,
                        double uatot,
                        double uaint,
                        double uaext,
                        double desvol,
                        double mdota,
                        double tain,
                        double win,
                        double mdotw,
                        double mdotwmax,
                        double twin,
                        double hwin,
                        double plr,
                        double pb)
{
    if (which < 0 || which > 7 || (cycling != 0 && cycling != 1) || (design != 0 && design != 1) || (hex != 1 && hex != 2) ||
        !within(availVal, 0.0, 1.0) || !within(uatot, 0.0, 1.0e7) || !within(uaint, 0.0, 1.0e7) || !within(uaext, 0.0, 1.0e7) ||
        !within(desvol, 0.0, 100.0) || !within(mdota, 0.0, 50.0) || !within(tain, -40.0, 60.0) || !within(win, 0.0, 0.04) ||
        !within(mdotwmax, 0.0, 100.0) || !within(mdotw, 0.0, mdotwmax) || !within(twin, 1.0, 99.0) || !within(hwin, -1.0e6, 1.0e6) ||
        !within(plr, 0.0, 1.0) || !within(pb, 50000.0, 120000.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataEnvrn->OutBaroPress = pb;
    avail->currentVal = availVal;
    auto &coil = state.dataWaterCoils->WaterCoil(1);
    coil.Name = "COIL";
    coil.availSched = avail;
    coil.WaterPlantLoc.loop = loop;
    coil.CoolingCoilAnalysisMode = state.dataWaterCoils->SimpleAnalysis;
    coil.HeatExchType = hex;
    coil.UACoilTotal = uatot;
    coil.UACoilInternal = uaint;
    coil.UACoilExternal = uaext;
    coil.DesAirVolFlowRate = desvol;
    coil.InletAirMassFlowRate = mdota;
    coil.InletAirTemp = tain;
    coil.InletAirHumRat = win;
    coil.InletWaterMassFlowRate = mdotw;
    coil.MaxWaterMassFlowRate = mdotwmax;
    coil.InletWaterTemp = twin;
    coil.InletWaterEnthalpy = hwin;
    coil.TotWaterCoolingCoilRate = 0.0;
    coil.SenWaterCoolingCoilRate = 0.0;
    int const calcMode = design == 1 ? state.dataWaterCoils->DesignCalc : state.dataWaterCoils->SimCalc;
    WaterCoils::CoolingCoil(state, 1, false, calcMode, cycling == 1 ? HVAC::FanOp::Cycling : HVAC::FanOp::Continuous, plr);
    double const r[8] = {coil.OutletAirTemp,
                         coil.OutletAirHumRat,
                         coil.OutletWaterTemp,
                         coil.TotWaterCoolingCoilRate,
                         coil.SenWaterCoolingCoilRate,
                         coil.SurfAreaWetFraction,
                         coil.OutletWaterEnthalpy,
                         coil.OutletAirEnthalpy};
    return r[which];
}
}
