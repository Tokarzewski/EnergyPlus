// Written by hand: five SetPointManager calculations read zone demands, nodes and the outdoor
// conditions, which gen_plain.py cannot set up. Plain extern "C" entry points over the real
// EnergyPlus SetPointManager code; each builds the manager, calls calculate and returns setPt.
//
// - OneStage(heating, stage, on, off): SPMSingleZoneOneStageCooling (heating 0) or Heating (1) for a
//   zone at stage number stage, with on and off set points (C).
// - SZHum(maxhum, zonem, zonew, hum, dehum): SPMSingleZoneHum, minimum (maxhum 0) or maximum (1),
//   for a zone node with mass flow zonem (kg/s) and humidity ratio zonew, and moisture loads to
//   the humidifying and dehumidifying set points hum and dehum (kg/s).
// - Pretreat(ctrl, refsp, retv, mixm, oam, minsp, maxsp): SPMOutsideAirPretreat for temperature
//   (ctrl 0), maximum, minimum or plain humidity ratio (1 to 3), with reference set point refsp, the
//   return inlet's temperature or humidity ratio retv, mixed outlet and outdoor inlet mass flows mixm
//   and oam (kg/s) and set point limits minsp to maxsp. The missing set point check has run already.
// - FollowOA(wet, oadb, oawb, offset, mint, maxt): SPMFollowOutsideAirTemp on the outdoor dry bulb
//   (wet 0) or wet bulb (1) with offset (K) and limits mint to maxt (C).
// - FollowNode(wet, more, nodet, nodewb, offset, mint, maxt): SPMFollowSysNodeTemp on a reference
//   node's dry bulb nodet (wet 0) or wet bulb nodewb (wet 1, only when the extra node data are
//   allocated, more 1; 0 otherwise), with offset and limits.
//
// One EnergyPlusData for the process, built when the library loads, with console output off.
// Guards (ADR-009), which return NaN: a flag not 0 or 1, ctrl outside 0 to 3, stage outside -10 to
// 10, a lower limit above its upper one; any input that is not finite or lies outside what
// EnergyPlus meets: temperatures and offsets outside -100 to 100, humidity ratios outside 0 to 0.1,
// set points and return values outside -100 to 100, flows outside 0 to 1000, moisture loads
// outside -10 to 10.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/OutputProcessor.hh> // before SetPointManager.hh, which uses OutputProcessor::VariableType
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/SetPointManager.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(4);
        p->dataLoopNodes->NodeID.allocate(4);
        p->dataZoneEnergyDemand->ZoneSysEnergyDemand.allocate(1);
        p->dataZoneEnergyDemand->ZoneSysMoistureDemand.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

bool flag(int v)
{
    return v == 0 || v == 1;
}

bool temp(double v)
{
    return within(v, -100.0, 100.0);
}

bool limits(double lo, double hi)
{
    return temp(lo) && temp(hi) && lo <= hi;
}
} // namespace

extern "C" {
double ep_OneStage(int heating, int stage, double on, double off)
{
    if (!flag(heating) || stage < -10 || stage > 10 || !temp(on) || !temp(off)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataZoneEnergyDemand->ZoneSysEnergyDemand(1).StageNum = stage;
    if (heating == 1) {
        SetPointManager::SPMSingleZoneOneStageHeating spm;
        spm.ctrlZoneNum = 1;
        spm.heatingOnSetPt = on;
        spm.heatingOffSetPt = off;
        spm.calculate(state);
        return spm.setPt;
    }
    SetPointManager::SPMSingleZoneOneStageCooling spm;
    spm.ctrlZoneNum = 1;
    spm.coolingOnSetPt = on;
    spm.coolingOffSetPt = off;
    spm.calculate(state);
    return spm.setPt;
}

double ep_SZHum(int maxhum, double zonem, double zonew, double hum, double dehum)
{
    if (!flag(maxhum) || !within(zonem, 0.0, 1000.0) || !within(zonew, 0.0, 0.1) || !within(hum, -10.0, 10.0) || !within(dehum, -10.0, 10.0)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &zone = state.dataLoopNodes->Node(1);
    zone.MassFlowRate = zonem;
    zone.HumRat = zonew;
    auto &demand = state.dataZoneEnergyDemand->ZoneSysMoistureDemand(1);
    demand.OutputRequiredToHumidifyingSP = hum;
    demand.OutputRequiredToDehumidifyingSP = dehum;
    SetPointManager::SPMSingleZoneHum spm;
    spm.type = maxhum == 1 ? SetPointManager::SPMType::SZMaxHum : SetPointManager::SPMType::SZMinHum;
    spm.zoneNodeNum = 1;
    spm.ctrlZoneNum = 1;
    spm.calculate(state);
    return spm.setPt;
}

double ep_Pretreat(int ctrl, double refsp, double retv, double mixm, double oam, double minsp, double maxsp)
{
    if (ctrl < 0 || ctrl > 3 || !temp(refsp) || !temp(retv) || !within(mixm, 0.0, 1000.0) || !within(oam, 0.0, 1000.0) ||
        !limits(minsp, maxsp)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    for (int k = 1; k <= 4; ++k) {
        nodes(k) = Node::NodeData();
    }
    nodes(1).TempSetPoint = refsp;
    nodes(1).HumRatMax = refsp;
    nodes(1).HumRatMin = refsp;
    nodes(1).HumRatSetPoint = refsp;
    nodes(2).MassFlowRate = mixm;
    nodes(3).MassFlowRate = oam;
    nodes(4).Temp = retv;
    nodes(4).HumRat = retv;
    SetPointManager::SPMOutsideAirPretreat spm;
    spm.ctrlVar = ctrl == 0   ? HVAC::CtrlVarType::Temp
                  : ctrl == 1 ? HVAC::CtrlVarType::MaxHumRat
                  : ctrl == 2 ? HVAC::CtrlVarType::MinHumRat
                              : HVAC::CtrlVarType::HumRat;
    spm.refNodeNum = 1;
    spm.mixedOutNodeNum = 2;
    spm.oaInNodeNum = 3;
    spm.returnInNodeNum = 4;
    spm.minSetTemp = minsp;
    spm.maxSetTemp = maxsp;
    spm.minSetHum = minsp;
    spm.maxSetHum = maxsp;
    spm.mySetPointCheckFlag = false;
    spm.calculate(state);
    return spm.setPt;
}

double ep_FollowOA(int wet, double oadb, double oawb, double offset, double mint, double maxt)
{
    if (!flag(wet) || !temp(oadb) || !temp(oawb) || !temp(offset) || !limits(mint, maxt)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataEnvrn->OutDryBulbTemp = oadb;
    state.dataEnvrn->OutWetBulbTemp = oawb;
    SetPointManager::SPMFollowOutsideAirTemp spm;
    spm.refTempType = wet == 1 ? SetPointManager::AirTempType::WetBulb : SetPointManager::AirTempType::DryBulb;
    spm.offset = offset;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.calculate(state);
    return spm.setPt;
}

double ep_FollowNode(int wet, int more, double nodet, double nodewb, double offset, double mint, double maxt)
{
    if (!flag(wet) || !flag(more) || !temp(nodet) || !temp(nodewb) || !temp(offset) || !limits(mint, maxt)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataLoopNodes->Node(1).Temp = nodet;
    auto &extra = state.dataLoopNodes->MoreNodeInfo;
    if (more == 1) {
        if (!allocated(extra)) {
            extra.allocate(4);
        }
        extra(1).WetBulbTemp = nodewb;
    } else if (allocated(extra)) {
        extra.deallocate();
    }
    SetPointManager::SPMFollowSysNodeTemp spm;
    spm.refTempType = wet == 1 ? SetPointManager::AirTempType::WetBulb : SetPointManager::AirTempType::DryBulb;
    spm.refNodeNum = 1;
    spm.offset = offset;
    spm.minSetTemp = mint;
    spm.maxSetTemp = maxt;
    spm.calculate(state);
    return spm.setPt;
}
}
