// Written by hand: SetPointManager::SPMReturnWaterTemp and SPMSystemNode read plant and reference
// nodes and a schedule, which gen_plain.py cannot set up. Plain extern "C" entry points over the
// real EnergyPlus SetPointManager code; each builds the manager, calls calculate and returns the
// set point it computes.
//
// - RetWater(hot, ttype, mdot, tsup, tret, target, retsp, mint, maxt):
//   SetpointManager:ReturnTemperature:ChilledWater (hot 0) or :HotWater (1) on a plant loop already
//   found (so the node check has run), with supply node mass flow mdot (kg/s) and temperature tsup
//   and return node temperature tret (C). The target return temperature is scheduled (ttype 0, the
//   schedule's value target), constant (1, target) or the return node's set point (2, retsp), and
//   the supply set point is limited to mint to maxt (C). The result is currentSupplySetPt.
// - SysNode(ctrl, ref, lowref, highref, splow, sphigh): SetpointManager:SystemNodeReset on a
//   reference node whose temperature and humidity ratio are both ref, for the control variable
//   temperature, maximum or minimum temperature, humidity ratio, maximum or minimum humidity ratio
//   (ctrl 0 to 5) or mass flow (6, which reads nothing), interpolating from splow at lowref to
//   sphigh at highref.
// One EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: hot not 0 or 1, ttype outside 0 to 2, ctrl outside 0 to 6,
// mint above maxt (std::clamp requires it); any input that is not finite or lies outside what
// EnergyPlus meets: mdot outside 0 to 1000, temperatures, targets, set points and references outside
// -100 to 200 (so never the -999 sensed-node flag, whose fatal error is not reached).
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
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

bool temp(double v)
{
    return within(v, -100.0, 200.0);
}
} // namespace

extern "C" {
double ep_RetWater(int hot, int ttype, double mdot, double tsup, double tret, double target, double retsp, double mint, double maxt)
{
    if ((hot != 0 && hot != 1) || ttype < 0 || ttype > 2 || !within(mdot, 0.0, 1000.0) || !temp(tsup) || !temp(tret) || !temp(target) ||
        !temp(retsp) || !temp(mint) || !temp(maxt) || mint > maxt) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &supply = state.dataLoopNodes->Node(1);
    supply.MassFlowRate = mdot;
    supply.Temp = tsup;
    auto &ret = state.dataLoopNodes->Node(2);
    ret.Temp = tret;
    ret.TempSetPoint = retsp;
    sched->currentVal = target;
    SetPointManager::SPMReturnWaterTemp spm;
    spm.type = hot == 1 ? SetPointManager::SPMType::HotWaterReturnTemp : SetPointManager::SPMType::ChilledWaterReturnTemp;
    spm.supplyNodeNum = 1;
    spm.returnNodeNum = 2;
    spm.plantLoopNum = 1;
    spm.returnTempSched = ttype == 0 ? sched : nullptr;
    spm.returnTempType = ttype == 0   ? SetPointManager::ReturnTempType::Scheduled
                         : ttype == 1 ? SetPointManager::ReturnTempType::Constant
                                      : SetPointManager::ReturnTempType::Setpoint;
    spm.returnTempConstantTarget = target;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.calculate(state);
    return spm.currentSupplySetPt;
}

double ep_SysNode(int ctrl, double ref, double lowref, double highref, double splow, double sphigh)
{
    if (ctrl < 0 || ctrl > 6 || !temp(ref) || !temp(lowref) || !temp(highref) || !temp(splow) || !temp(sphigh)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &node = state.dataLoopNodes->Node(1);
    node.Temp = ref;
    node.HumRat = ref;
    HVAC::CtrlVarType const vars[7] = {HVAC::CtrlVarType::Temp,
                                       HVAC::CtrlVarType::MaxTemp,
                                       HVAC::CtrlVarType::MinTemp,
                                       HVAC::CtrlVarType::HumRat,
                                       HVAC::CtrlVarType::MaxHumRat,
                                       HVAC::CtrlVarType::MinHumRat,
                                       HVAC::CtrlVarType::MassFlowRate};
    SetPointManager::SPMSystemNode spm;
    spm.ctrlVar = vars[ctrl];
    spm.refNodeNum = 1;
    spm.lowRef = lowref;
    spm.highRef = highref;
    spm.lowRefSetPt = splow;
    spm.highRefSetPt = sphigh;
    spm.calculate(state);
    return spm.setPt;
}
}
