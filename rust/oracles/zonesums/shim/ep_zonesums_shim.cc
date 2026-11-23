// Written by hand: ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums reads the zone's internal gain
// devices, air flow terms, supply air inlets, plenum conditions, air terminal leaks and parallel PIU
// leakage from the EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry
// point over the real EnergyPlus ZoneTempPredictorCorrector code.
//
// Each call sets up zone 1 with one space (space 1, no heat transfer surfaces) that has ndev (0 to
// 3) internal gain devices, device i with convective gain cg_i and return air convective gain rg_i
// (W). The zone's high temperature radiant heater and pool convection are radsys and pool, and
// heat to return air goes to the zone when noheatret is 1. Its infiltration, ventilation, mixing,
// earth tube and cooling tower terms are mcpi and mcpti to mcpc and mcptc, its air balance outdoor
// air term mdotcpoa at outdoor dry bulb outdb, and its air humidity ratio airhumrat. kind makes
// the zone uncontrolled (0), controlled (1) with ninlets (0 to 3) supply air inlet nodes, a return
// plenum (2) with ninlets inlet nodes and, when hasadu is 1, one air distribution unit leaking to
// it (an upstream leak mup from its inlet node at tup when upleak is 1, a downstream leak mdn from
// its outlet node at tdn when dnleak is 1), or a supply plenum (3) fed by the first inlet node.
// Inlet node i carries mass flow m_i at temperature t_i. When haspiu is 1 the zone has one parallel
// PIU with backdraft damper leakage leak (kg/s) from its primary air inlet at tpri; hasnode gives
// the zone a system node. mult and listmult are the zone multipliers. Then the real
// calcZoneOrSpaceSums runs for zone 1 (no space), as the corrector when corrector is 1. One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 4; kind outside 0 to 3; a flag outside 0
// or 1; ndev or ninlets outside 0 to 3; mult or listmult outside 1 to 10000 (their int product must
// not overflow).
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataDefineEquip.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataZoneEquipment.hh>
#include <EnergyPlus/PoweredInductionUnits.hh>
#include <EnergyPlus/ZonePlenum.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        auto &hb = *p->dataHeatBal;
        hb.Zone.allocate(1);
        hb.Zone(1).Name = "ZONE 1";
        hb.Zone(1).spaceIndexes.push_back(1);
        hb.space.allocate(1);
        hb.space(1).HTSurfaceFirst = 1;
        hb.space(1).HTSurfaceLast = 0;
        hb.spaceIntGainDevices.allocate(1);
        hb.spaceIntGainDevices(1).device.allocate(3);
        p->dataHeatBalFanSys->SumConvHTRadSys.dimension(1, 0.0);
        p->dataHeatBalFanSys->SumConvPool.dimension(1, 0.0);
        p->dataZoneEquip->ZoneEquipConfig.allocate(1);
        auto &cfg = p->dataZoneEquip->ZoneEquipConfig(1);
        cfg.InletNode.allocate(3);
        p->dataZonePlenum->ZoneRetPlenCond.allocate(1);
        auto &ret = p->dataZonePlenum->ZoneRetPlenCond(1);
        ret.InletNode.allocate(3);
        ret.ADUIndex.allocate(1);
        ret.ADUIndex(1) = 1;
        p->dataZonePlenum->ZoneSupPlenCond.allocate(1);
        p->dataZonePlenum->ZoneSupPlenCond(1).InletNode = 2;
        for (int i = 1; i <= 3; ++i) {
            cfg.InletNode(i) = i + 1;
            ret.InletNode(i) = i + 1;
        }
        p->dataDefineEquipment->AirDistUnit.allocate(1);
        p->dataDefineEquipment->AirDistUnit(1).InletNodeNum = 5;
        p->dataDefineEquipment->AirDistUnit(1).OutletNodeNum = 6;
        p->dataPowerInductionUnits->PIU.allocate(1);
        p->dataPowerInductionUnits->PIU(1).PriAirInNode = 7;
        p->dataLoopNodes->Node.allocate(7);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        p->dataZoneTempPredictorCorrector->spaceHeatBalance.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One sum after calcZoneOrSpaceSums for zone 1 (see the file comment): which picks SumIntGain (0),
// SumMCp (1), SumMCpT (2), SumSysMCp (3) and SumSysMCpT (4).
double ep_ZoneSums(int which,
                   int corrector,
                   int kind,
                   int ndev,
                   double cg1,
                   double rg1,
                   double cg2,
                   double rg2,
                   double cg3,
                   double rg3,
                   double radsys,
                   double pool,
                   int noheatret,
                   double mcpi,
                   double mcpti,
                   double mcpv,
                   double mcptv,
                   double mcpm,
                   double mcptm,
                   double mcpe,
                   double mcpte,
                   double mcpc,
                   double mcptc,
                   double mdotcpoa,
                   double outdb,
                   double airhumrat,
                   int ninlets,
                   double m1,
                   double t1,
                   double m2,
                   double t2,
                   double m3,
                   double t3,
                   int hasadu,
                   int upleak,
                   double mup,
                   double tup,
                   int dnleak,
                   double mdn,
                   double tdn,
                   int haspiu,
                   double leak,
                   double tpri,
                   int hasnode,
                   int mult,
                   int listmult)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    for (int f : {corrector, noheatret, hasadu, upleak, dnleak, haspiu, hasnode}) {
        if (!flag(f)) {
            return NAN;
        }
    }
    if (which < 0 || which > 4 || kind < 0 || kind > 3 || ndev < 0 || ndev > 3 || ninlets < 0 || ninlets > 3 || mult < 1 || mult > 10000 ||
        listmult < 1 || listmult > 10000) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &hb = *state.dataHeatBal;
    auto &zone = hb.Zone(1);
    zone.NoHeatToReturnAir = noheatret == 1;
    zone.OutDryBulbTemp = outdb;
    zone.IsControlled = kind == 1;
    zone.IsReturnPlenum = kind == 2;
    zone.IsSupplyPlenum = kind == 3;
    zone.PlenumCondNum = kind >= 2 ? 1 : 0;
    zone.SystemZoneNodeNumber = hasnode == 1 ? 1 : 0;
    zone.Multiplier = mult;
    zone.ListMultiplier = listmult;
    zone.leakageParallelPIUNums.clear();
    if (haspiu == 1) {
        zone.leakageParallelPIUNums.push_back(1.0);
    }
    auto &gains = hb.spaceIntGainDevices(1);
    gains.numberOfDevices = ndev;
    double const cg[3] = {cg1, cg2, cg3};
    double const rg[3] = {rg1, rg2, rg3};
    for (int i = 0; i < 3; ++i) {
        gains.device(i + 1).ConvectGainRate = cg[i];
        gains.device(i + 1).ReturnAirConvGainRate = rg[i];
    }
    state.dataHeatBalFanSys->SumConvHTRadSys(1) = radsys;
    state.dataHeatBalFanSys->SumConvPool(1) = pool;
    state.dataZoneEquip->ZoneEquipConfig(1).NumInletNodes = ninlets;
    auto &ret = state.dataZonePlenum->ZoneRetPlenCond(1);
    ret.NumInletNodes = ninlets;
    ret.NumADUs = hasadu;
    auto &adu = state.dataDefineEquipment->AirDistUnit(1);
    adu.UpStreamLeak = upleak == 1;
    adu.MassFlowRateUpStrLk = mup;
    adu.DownStreamLeak = dnleak == 1;
    adu.MassFlowRateDnStrLk = mdn;
    state.dataPowerInductionUnits->PIU(1).leakFlow = leak;
    auto &nodes = state.dataLoopNodes->Node;
    double const m[3] = {m1, m2, m3};
    double const t[3] = {t1, t2, t3};
    for (int i = 0; i < 3; ++i) {
        nodes(i + 2).MassFlowRate = m[i];
        nodes(i + 2).Temp = t[i];
    }
    nodes(5).Temp = tup;
    nodes(6).Temp = tdn;
    nodes(7).Temp = tpri;
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.MCPI = mcpi;
    zhb.MCPTI = mcpti;
    zhb.MCPV = mcpv;
    zhb.MCPTV = mcptv;
    zhb.MCPM = mcpm;
    zhb.MCPTM = mcptm;
    zhb.MCPE = mcpe;
    zhb.MCPTE = mcpte;
    zhb.MCPC = mcpc;
    zhb.MCPTC = mcptc;
    zhb.MDotCPOA = mdotcpoa;
    zhb.airHumRat = airhumrat;
    zhb.SumIntGain = -1.0;
    zhb.SumMCp = -1.0;
    zhb.SumMCpT = -1.0;

    zhb.calcZoneOrSpaceSums(state, corrector == 1, 1);

    switch (which) {
    case 0:
        return zhb.SumIntGain;
    case 1:
        return zhb.SumMCp;
    case 2:
        return zhb.SumMCpT;
    case 3:
        return zhb.SumSysMCp;
    default:
        return zhb.SumSysMCpT;
    }
}
}
