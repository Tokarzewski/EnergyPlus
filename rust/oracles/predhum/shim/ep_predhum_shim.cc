// Written by hand: ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio reads the zone's
// humidistat, moisture balance terms and solution algorithm from the EnergyPlusData state, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// ZoneTempPredictorCorrector code.
//
// Each call sets up zone 1. With ctrltype -1 it has no humidistat; with 0 (dew point) or 1
// (relative humidity, DataZoneControls::HumidityCtrlVarType) its humidistat has humidifying and
// dehumidifying set points humsp and dehumsp (dew points in C, or RH in %), with EMS overrides
// emshumval and emsdehumval when emshumon and emsdehumon are 1. The zone air is at zt with humidity
// ratio airhumrat, outdoors at barometric pressure outbaro and humidity ratio outhumrat. Its
// moisture balance terms are the latent gains (latentgain, plus the high temperature radiant
// heater and pool gains latentradsys and latentpool), the outdoor, ventilation, cooling tower,
// earth tube, convection, mixing and outdoor air mass flows (oamfl, vamfl, ctmfl, eamfl and
// eamflxw, sumhmara and sumhmaraw, mixm and mixw, mdotoa), its volume and moisture capacity
// multiplier (volume, volcapmult), the system time step tsyssec seconds, the zone air solution
// algorithm algo (DataHeatBalance::SolutionAlgo, 0 to 2) with the humidity ratio history wprev0
// to wprev2 and w1, and multipliers mult and listmult. Then the real calcPredictedHumidityRatio
// runs for zone 1 (no space) with RoomAirflowNetwork fraction rafnfrac. One EnergyPlusData for
// the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 5; a flag outside 0 or 1; ctrltype
// outside -1 to 1; algo outside 0 to 2; mult or listmult outside 1 to 10000 (their int product must
// not overflow); outbaro outside 10000 to 200000 Pa, and dew point set points that are not finite,
// for which PsyWFnTdpPb's search never ends; and the fatal errors EnergyPlus stops on, a negative
// air density and loads that compare as none of humidify, dehumidify or deadband (NaN).
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataRoomAirModel.hh>
#include <EnergyPlus/DataZoneControls.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
Sched::Schedule *humSched = nullptr;
Sched::Schedule *dehumSched = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataZoneCtrls->HumidityControlZone.allocate(1);
        auto &h = p->dataZoneCtrls->HumidityControlZone(1);
        h.ControlName = "HUMIDISTAT 1";
        h.ActualZoneNum = 1;
        humSched = Sched::AddScheduleConstant(*p, "HUMIDIFYING");
        dehumSched = Sched::AddScheduleConstant(*p, "DEHUMIDIFYING");
        h.humidifyingSched = humSched;
        h.dehumidifyingSched = dehumSched;
        p->dataHeatBalFanSys->SumLatentHTRadSys.dimension(1, 0.0);
        p->dataHeatBalFanSys->SumLatentPool.dimension(1, 0.0);
        p->dataRoomAir->AirModel.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        p->dataZoneEnergyDemand->ZoneSysMoistureDemand.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result of calcPredictedHumidityRatio for zone 1 (see the file comment): which picks the
// zone system moisture demand's predictedRate (0), predictedHumSPRate (1), predictedDehumSPRate
// (2), TotalOutputRequired (3), OutputRequiredToHumidifyingSP (4) and
// OutputRequiredToDehumidifyingSP (5), each starting at 0.
double ep_PredictedHumidityRatio(int which,
                                 int ctrltype,
                                 double humsp,
                                 double dehumsp,
                                 int emshumon,
                                 double emshumval,
                                 int emsdehumon,
                                 double emsdehumval,
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
                                 double w1,
                                 double rafnfrac,
                                 int mult,
                                 int listmult)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 5 || !flag(emshumon) || !flag(emsdehumon) || ctrltype < -1 || ctrltype > 1 || algo < 0 || algo > 2 || mult < 1 ||
        mult > 10000 || listmult < 1 || listmult > 10000 || !(outbaro >= 10000.0 && outbaro <= 200000.0) ||
        (ctrltype == 0 && !(std::isfinite(humsp) && std::isfinite(dehumsp)))) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &zone = state.dataHeatBal->Zone(1);
    zone.humidityControlZoneIndex = ctrltype == -1 ? 0 : 1;
    zone.Volume = volume;
    zone.ZoneVolCapMultpMoist = volcapmult;
    zone.Multiplier = mult;
    zone.ListMultiplier = listmult;
    auto &h = state.dataZoneCtrls->HumidityControlZone(1);
    h.humidityControlVariableType = ctrltype == 1 ? DataZoneControls::HumidityCtrlVarType::RelativeHumidity : DataZoneControls::HumidityCtrlVarType::DewPoint;
    humSched->currentVal = humsp;
    dehumSched->currentVal = dehumsp;
    h.EMSOverrideHumidifySetPointOn = emshumon == 1;
    h.EMSOverrideHumidifySetPointValue = emshumval;
    h.EMSOverrideDehumidifySetPointOn = emsdehumon == 1;
    h.EMSOverrideDehumidifySetPointValue = emsdehumval;
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
    auto &d = state.dataZoneEnergyDemand->ZoneSysMoistureDemand(1);
    d.predictedRate = 0.0;
    d.predictedHumSPRate = 0.0;
    d.predictedDehumSPRate = 0.0;
    d.TotalOutputRequired = 0.0;
    d.OutputRequiredToHumidifyingSP = 0.0;
    d.OutputRequiredToDehumidifyingSP = 0.0;

    try {
        zhb.calcPredictedHumidityRatio(state, rafnfrac, 1);
    } catch (FatalError const &) {
        return NAN;
    }

    switch (which) {
    case 0:
        return d.predictedRate;
    case 1:
        return d.predictedHumSPRate;
    case 2:
        return d.predictedDehumSPRate;
    case 3:
        return d.TotalOutputRequired;
    case 4:
        return d.OutputRequiredToHumidifyingSP;
    default:
        return d.OutputRequiredToDehumidifyingSP;
    }
}
}
