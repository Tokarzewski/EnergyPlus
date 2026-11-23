// Written by hand: SetPointManager's SPMSingleZoneTemp::calculate and SPMOutsideAir::calculate read
// the control zone's loads, the zone and zone inlet nodes, the outdoor temperature and the
// optional schedule, which gen_plain.py cannot set up. Plain extern "C" entry points over the real
// EnergyPlus SetPointManager code.
//
// SZTemp sets up one SetpointManager:SingleZone:Heating (heating 1) or :Cooling (heating 0) with
// supply air limits mint and maxt (C): the zone node at zonet (C), the zone inlet node with mass
// flow zonem (kg/s) and humidity ratio inw (kg/kg), and the zone's loads to the heating and cooling
// set points toheat and tocool (W). OAReset sets up one SetpointManager:OutdoorAirReset with the
// outdoor dry bulb oat (C), the first reset line from setpoint lsp1 at outdoor low1 to hsp1 at
// high1 and the second from lsp2 at low2 to hsp2 at high2 (C), with no schedule (sched 0) or a
// schedule at 1 or 2 choosing the line. One EnergyPlusData for the process, built when the library
// loads, with console output off.
//
// Guards (ADR-009), which return NaN: heating not 0 or 1, sched outside 0 to 2 (a schedule value
// other than 1 or 2 is a severe error); any input that is not finite or lies outside what
// EnergyPlus meets: zonem outside 0 to 1000, inw outside 0 to 0.1, loads outside -1e7 to 1e7,
// temperatures outside -60 to 80, and mint above maxt (std::clamp requires it).
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/OutputProcessor.hh> // before SetPointManager.hh, which uses OutputProcessor::VariableType
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/SetPointManager.hh>

using namespace EnergyPlus;

namespace {
Sched::ScheduleConstant *sched = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(2);
        p->dataLoopNodes->NodeID.allocate(2);
        p->dataZoneEnergyDemand->ZoneSysEnergyDemand.allocate(1);
        sched = new Sched::ScheduleConstant;
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
// The supply air temperature set point of a single zone heating or cooling manager (C).
double ep_SZTemp(int heating, double zonem, double inw, double toheat, double tocool, double zonet, double mint, double maxt)
{
    if ((heating != 0 && heating != 1) || !within(zonem, 0.0, 1000.0) || !within(inw, 0.0, 0.1) || !within(toheat, -1.0e7, 1.0e7) ||
        !within(tocool, -1.0e7, 1.0e7) || !within(zonet, -60.0, 80.0) || !within(mint, -60.0, 80.0) || !within(maxt, mint, 80.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    nodes(1).Temp = zonet;
    nodes(2).MassFlowRate = zonem;
    nodes(2).HumRat = inw;
    auto &demand = state.dataZoneEnergyDemand->ZoneSysEnergyDemand(1);
    demand.OutputRequiredToHeatingSP = toheat;
    demand.OutputRequiredToCoolingSP = tocool;
    SetPointManager::SPMSingleZoneTemp spm;
    spm.type = heating == 1 ? SetPointManager::SPMType::SZHeating : SetPointManager::SPMType::SZCooling;
    spm.ctrlZoneNum = 1;
    spm.zoneNodeNum = 1;
    spm.zoneInletNodeNum = 2;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.calculate(state);
    return spm.setPt;
}

// The set point of an outdoor air reset manager (C).
double ep_OAReset(
    int schedv, double oat, double low1, double high1, double lsp1, double hsp1, double low2, double high2, double lsp2, double hsp2)
{
    double const ts[9] = {oat, low1, high1, lsp1, hsp1, low2, high2, lsp2, hsp2};
    if (schedv < 0 || schedv > 2) {
        return NAN;
    }
    for (double t : ts) {
        if (!within(t, -60.0, 80.0)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataEnvrn->OutDryBulbTemp = oat;
    sched->currentVal = schedv;
    SetPointManager::SPMOutsideAir spm;
    spm.sched = schedv == 0 ? nullptr : sched;
    spm.low1 = low1;
    spm.high1 = high1;
    spm.lowSetPt1 = lsp1;
    spm.highSetPt1 = hsp1;
    spm.low2 = low2;
    spm.high2 = high2;
    spm.lowSetPt2 = lsp2;
    spm.highSetPt2 = hsp2;
    spm.calculate(state);
    return spm.setPt;
}
}
