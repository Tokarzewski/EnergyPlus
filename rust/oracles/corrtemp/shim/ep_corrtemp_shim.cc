// Written by hand: ZoneSpaceHeatBalanceData::correctAirTemp reads the zone's air and humidity
// histories, internal gains, air flow terms, supply air inlets and system node from the
// EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry point over the
// real EnergyPlus ZoneTempPredictorCorrector code.
//
// Each call sets up zone 1, fully mixed, with one space (space 1, no heat transfer surfaces) that
// has ndev (0 to 2) internal gain devices, device i with convective gain cg_i and return air
// convective gain rg_i (W). The zone's high temperature radiant heater convection is radsys, and
// heat to return air goes to the zone when noheatret is 1. Its infiltration and ventilation terms
// are mcpi, mcpti, mcpv and mcptv, its air balance outdoor air term mdotcpoa at outdoor dry bulb
// outdb. It is controlled when controlled is 1, with ninlets (0 to 2) supply air inlet nodes, inlet
// i carrying mass flow m_i at temperature tin_i and humidity ratio win_i, and has system node 1
// with mass flow nodemflow when hasnode is 1. Its volume, sensible and moisture capacity
// multipliers are volume, capsens and capmoist, the system time step tsyssec seconds, the outdoor
// barometric pressure and humidity ratio outbaro and outhumrat. The zone air starts at mat with
// humidity ratio airhumrat; xmat0 to xmat2 and wprev0 to wprev2 are its zone time step histories,
// dsxmat0 to dsxmat2 and dswprev0 to dswprev2 its system time step ones (usezonehist picks), and t1
// and w1 the analytical and Euler starting values. nonair and sysdeplag are the non-air system
// response and the lagged system-dependent loads, latentgain, oamfl, vamfl and mdotoa the latent
// gain and the outdoor, ventilation and outdoor air mass flows of the moisture balance, and algo
// the zone air solution algorithm (DataHeatBalance::SolutionAlgo, 0 to 2); mult and listmult are
// the zone multipliers. Then the real correctAirTemp runs for zone 1 (no space). One EnergyPlusData
// for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 13; a flag outside 0 or 1; algo outside 0
// to 2; ndev or ninlets outside 0 to 2; mult or listmult outside 1 to 10000 (their int product
// must not overflow); and the fatal error EnergyPlus stops on, a negative air density.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/DataRoomAirModel.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
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
        auto &hb = *p->dataHeatBal;
        hb.Zone.allocate(1);
        hb.Zone(1).Name = "ZONE 1";
        hb.Zone(1).spaceIndexes.push_back(1);
        hb.space.allocate(1);
        hb.space(1).HTSurfaceFirst = 1;
        hb.space(1).HTSurfaceLast = 0;
        hb.spaceIntGainDevices.allocate(1);
        hb.spaceIntGainDevices(1).device.allocate(2);
        p->dataHeatBalFanSys->SumConvHTRadSys.dimension(1, 0.0);
        p->dataHeatBalFanSys->SumConvPool.dimension(1, 0.0);
        p->dataHeatBalFanSys->SumLatentHTRadSys.dimension(1, 0.0);
        p->dataHeatBalFanSys->SumLatentPool.dimension(1, 0.0);
        p->dataHeatBalFanSys->TempTstatAir.dimension(1, 0.0);
        p->dataHeatBalFanSys->LoadCorrectionFactor.dimension(1, 0.0);
        p->dataZoneEquip->ZoneEquipConfig.allocate(1);
        auto &cfg = p->dataZoneEquip->ZoneEquipConfig(1);
        cfg.InletNode.allocate(2);
        cfg.InletNode(1) = 2;
        cfg.InletNode(2) = 3;
        p->dataLoopNodes->Node.allocate(3);
        p->dataRoomAir->GetAirModelData = false;
        p->dataRoomAir->anyNonMixingRoomAirModel = false;
        p->dataRoomAir->AirModel.allocate(1);
        p->dataZoneEnergyDemand->ZoneSysEnergyDemand.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        p->dataZoneTempPredictorCorrector->spaceHeatBalance.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of correctAirTemp for zone 1 (see the file comment): which picks the new zone air
// temperature ZT (0), TempDepCoef (1), TempIndCoef (2), AirPowerCap (3), the zone's air system
// sensible heating and cooling rates (4, 5) and energies (6, 7), the new humidity ratio airHumRat
// (8) and relative humidity airRelHum (9), the system node's Temp (10), HumRat (11) and Enthalpy
// (12), all three starting at 0, and the returned temperature change (13).
double ep_CorrectAirTemp(int which,
                         int usezonehist,
                         int algo,
                         int hasnode,
                         int controlled,
                         int mult,
                         int listmult,
                         double volume,
                         double capsens,
                         double capmoist,
                         double tsyssec,
                         double outbaro,
                         double outhumrat,
                         double mat,
                         double airhumrat,
                         double xmat0,
                         double xmat1,
                         double xmat2,
                         double dsxmat0,
                         double dsxmat1,
                         double dsxmat2,
                         double wprev0,
                         double wprev1,
                         double wprev2,
                         double dswprev0,
                         double dswprev1,
                         double dswprev2,
                         double t1,
                         double w1,
                         int ndev,
                         double cg1,
                         double rg1,
                         double cg2,
                         double rg2,
                         int noheatret,
                         double radsys,
                         double mcpi,
                         double mcpti,
                         double mcpv,
                         double mcptv,
                         double mdotcpoa,
                         double outdb,
                         int ninlets,
                         double m1,
                         double tin1,
                         double win1,
                         double m2,
                         double tin2,
                         double win2,
                         double nodemflow,
                         double nonair,
                         double sysdeplag,
                         double latentgain,
                         double oamfl,
                         double vamfl,
                         double mdotoa)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 13 || !flag(usezonehist) || !flag(hasnode) || !flag(controlled) || !flag(noheatret) || algo < 0 || algo > 2 ||
        ndev < 0 || ndev > 2 || ninlets < 0 || ninlets > 2 || mult < 1 || mult > 10000 || listmult < 1 || listmult > 10000) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &hb = *state.dataHeatBal;
    auto &zone = hb.Zone(1);
    zone.IsControlled = controlled == 1;
    zone.SystemZoneNodeNumber = hasnode == 1 ? 1 : 0;
    zone.Multiplier = mult;
    zone.ListMultiplier = listmult;
    zone.Volume = volume;
    zone.ZoneVolCapMultpSens = capsens;
    zone.ZoneVolCapMultpMoist = capmoist;
    zone.NoHeatToReturnAir = noheatret == 1;
    zone.OutDryBulbTemp = outdb;
    hb.ZoneAirSolutionAlgo = static_cast<DataHeatBalance::SolutionAlgo>(algo);
    auto &gains = hb.spaceIntGainDevices(1);
    gains.numberOfDevices = ndev;
    gains.device(1).ConvectGainRate = cg1;
    gains.device(1).ReturnAirConvGainRate = rg1;
    gains.device(2).ConvectGainRate = cg2;
    gains.device(2).ReturnAirConvGainRate = rg2;
    state.dataHeatBalFanSys->SumConvHTRadSys(1) = radsys;
    state.dataZoneEquip->ZoneEquipConfig(1).NumInletNodes = ninlets;
    auto &nodes = state.dataLoopNodes->Node;
    nodes(1).MassFlowRate = nodemflow;
    nodes(1).Temp = 0.0;
    nodes(1).HumRat = 0.0;
    nodes(1).Enthalpy = 0.0;
    nodes(2).MassFlowRate = m1;
    nodes(2).Temp = tin1;
    nodes(2).HumRat = win1;
    nodes(3).MassFlowRate = m2;
    nodes(3).Temp = tin2;
    nodes(3).HumRat = win2;
    state.dataEnvrn->OutBaroPress = outbaro;
    state.dataEnvrn->OutHumRat = outhumrat;
    state.dataHVACGlobal->TimeStepSysSec = tsyssec;
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.MAT = mat;
    zhb.airHumRat = airhumrat;
    zhb.XMAT = {xmat0, xmat1, xmat2, 0.0};
    zhb.DSXMAT = {dsxmat0, dsxmat1, dsxmat2, 0.0};
    zhb.WPrevZoneTS = {wprev0, wprev1, wprev2, 0.0};
    zhb.DSWPrevZoneTS = {dswprev0, dswprev1, dswprev2, 0.0};
    zhb.T1 = t1;
    zhb.W1 = w1;
    zhb.MCPI = mcpi;
    zhb.MCPTI = mcpti;
    zhb.MCPV = mcpv;
    zhb.MCPTV = mcptv;
    zhb.MDotCPOA = mdotcpoa;
    zhb.NonAirSystemResponse = nonair;
    zhb.SysDepZoneLoadsLagged = sysdeplag;
    zhb.latentGain = latentgain;
    zhb.OAMFL = oamfl;
    zhb.VAMFL = vamfl;
    zhb.MDotOA = mdotoa;
    auto &demand = state.dataZoneEnergyDemand->ZoneSysEnergyDemand(1);

    double tempChange = 0.0;
    try {
        tempChange = zhb.correctAirTemp(state, usezonehist == 1, 1);
    } catch (FatalError const &) {
        return NAN;
    }

    switch (which) {
    case 0:
        return zhb.ZT;
    case 1:
        return zhb.TempDepCoef;
    case 2:
        return zhb.TempIndCoef;
    case 3:
        return zhb.AirPowerCap;
    case 4:
        return demand.airSysHeatRate;
    case 5:
        return demand.airSysCoolRate;
    case 6:
        return demand.airSysHeatEnergy;
    case 7:
        return demand.airSysCoolEnergy;
    case 8:
        return zhb.airHumRat;
    case 9:
        return zhb.airRelHum;
    case 10:
        return nodes(1).Temp;
    case 11:
        return nodes(1).HumRat;
    case 12:
        return nodes(1).Enthalpy;
    default:
        return tempChange;
    }
}
}
