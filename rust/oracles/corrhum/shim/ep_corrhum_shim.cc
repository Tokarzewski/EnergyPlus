// Written by hand: ZoneSpaceHeatBalanceData::correctHumRat reads the zone's supply air inlets,
// moisture balance terms and solution algorithm from the EnergyPlusData state, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// ZoneTempPredictorCorrector code.
//
// Each call sets up zone 1, controlled when controlled is 1 with ninlets (0 to 3) supply air inlet
// nodes carrying mass flows m1 to m3 at humidity ratios w1in to w3in, and with system node 1 when
// hasnode is 1. The zone air is at zt with humidity ratio airhumrat, outdoors at barometric
// pressure outbaro and humidity ratio outhumrat. Its moisture balance terms are the latent gains
// (latentgain, plus the high temperature radiant heater and pool gains latentradsys and
// latentpool), the outdoor, ventilation, cooling tower, earth tube, convection, mixing and outdoor
// air mass flows (oamfl, vamfl, ctmfl, eamfl and eamflxw, sumhmara and sumhmaraw, mixm and mixw,
// mdotoa), its volume and moisture capacity multiplier (volume, volcapmult), the system time step
// tsyssec seconds, the zone air solution algorithm algo (DataHeatBalance::SolutionAlgo, 0 to 2)
// with the humidity ratio history wprev0 to wprev2 and w1, and multipliers mult and listmult. Then
// the real correctHumRat runs for zone 1 (no space). One EnergyPlusData for the process, built when
// the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 2; a flag outside 0 or 1; ninlets outside
// 0 to 3; algo outside 0 to 2; mult or listmult outside 1 to 10000 (their int product must not
// overflow); and the fatal error EnergyPlus stops on, a negative air density.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataRoomAirModel.hh>
#include <EnergyPlus/DataZoneEquipment.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataZoneEquip->ZoneEquipConfig.allocate(1);
        auto &cfg = p->dataZoneEquip->ZoneEquipConfig(1);
        cfg.InletNode.allocate(3);
        cfg.InletNode(1) = 2;
        cfg.InletNode(2) = 3;
        cfg.InletNode(3) = 4;
        p->dataLoopNodes->Node.allocate(4);
        p->dataHeatBalFanSys->SumLatentHTRadSys.dimension(1, 0.0);
        p->dataHeatBalFanSys->SumLatentPool.dimension(1, 0.0);
        p->dataRoomAir->AirModel.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of correctHumRat for zone 1 (see the file comment): which picks the new zone air
// humidity ratio airHumRatTemp (0), and the system node's HumRat (1) and Enthalpy (2), both
// starting at 0.
double ep_CorrectHumRat(int which,
                        int controlled,
                        int ninlets,
                        double m1,
                        double w1in,
                        double m2,
                        double w2in,
                        double m3,
                        double w3in,
                        int hasnode,
                        int mult,
                        int listmult,
                        double zt,
                        double outbaro,
                        double airhumrat,
                        double latentgain,
                        double latentradsys,
                        double latentpool,
                        double tsyssec,
                        double oamfl,
                        double vamfl,
                        double ctmfl,
                        double eamfl,
                        double eamflxw,
                        double sumhmaraw,
                        double sumhmara,
                        double mixw,
                        double mixm,
                        double mdotoa,
                        double outhumrat,
                        double volume,
                        double volcapmult,
                        int algo,
                        double wprev0,
                        double wprev1,
                        double wprev2,
                        double w1)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 2 || !flag(controlled) || !flag(hasnode) || ninlets < 0 || ninlets > 3 || algo < 0 || algo > 2 || mult < 1 ||
        mult > 10000 || listmult < 1 || listmult > 10000) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &zone = state.dataHeatBal->Zone(1);
    zone.IsControlled = controlled == 1;
    zone.SystemZoneNodeNumber = hasnode == 1 ? 1 : 0;
    zone.Volume = volume;
    zone.ZoneVolCapMultpMoist = volcapmult;
    zone.Multiplier = mult;
    zone.ListMultiplier = listmult;
    state.dataZoneEquip->ZoneEquipConfig(1).NumInletNodes = ninlets;
    auto &nodes = state.dataLoopNodes->Node;
    nodes(1).HumRat = 0.0;
    nodes(1).Enthalpy = 0.0;
    nodes(2).MassFlowRate = m1;
    nodes(2).HumRat = w1in;
    nodes(3).MassFlowRate = m2;
    nodes(3).HumRat = w2in;
    nodes(4).MassFlowRate = m3;
    nodes(4).HumRat = w3in;
    state.dataEnvrn->OutBaroPress = outbaro;
    state.dataEnvrn->OutHumRat = outhumrat;
    state.dataHeatBalFanSys->SumLatentHTRadSys(1) = latentradsys;
    state.dataHeatBalFanSys->SumLatentPool(1) = latentpool;
    state.dataHVACGlobal->TimeStepSysSec = tsyssec;
    state.dataHeatBal->ZoneAirSolutionAlgo = static_cast<DataHeatBalance::SolutionAlgo>(algo);
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.ZT = zt;
    zhb.airHumRat = airhumrat;
    zhb.latentGain = latentgain;
    zhb.OAMFL = oamfl;
    zhb.VAMFL = vamfl;
    zhb.CTMFL = ctmfl;
    zhb.EAMFL = eamfl;
    zhb.EAMFLxHumRat = eamflxw;
    zhb.SumHmARaW = sumhmaraw;
    zhb.SumHmARa = sumhmara;
    zhb.MixingMassFlowXHumRat = mixw;
    zhb.MixingMassFlowZone = mixm;
    zhb.MDotOA = mdotoa;
    zhb.WPrevZoneTSTemp = {wprev0, wprev1, wprev2, 0.0};
    zhb.W1 = w1;
    zhb.airHumRatTemp = 0.0;

    try {
        zhb.correctHumRat(state, 1);
    } catch (FatalError const &) {
        return NAN;
    }

    switch (which) {
    case 0:
        return zhb.airHumRatTemp;
    case 1:
        return nodes(1).HumRat;
    default:
        return nodes(1).Enthalpy;
    }
}
}
