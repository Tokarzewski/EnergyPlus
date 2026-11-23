// Written by hand: the ventilation part of ZoneEquipmentManager::CalcAirFlowSimple reads the zone's
// ventilation object, its schedules and the zone and outdoor conditions from the EnergyPlusData
// state, which gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// ZoneEquipmentManager code.
//
// Each call sets up zone 1 in a model with simple air flows and, when nvent is 1, one ventilation
// object and nothing else (no infiltration, mixing, earth tubes, cooling towers, thermal chimneys
// or zone air balance). The object uses model mod (DataHeatBalance::VentilationModelType: 0 design
// flow rate, 1 wind and stack open area) and density basis den (InfVentDensityBasis: 0 outdoor, 1
// standard, 2 indoor) with availability schedule value avail. For the design flow rate model: its
// design level dl (m3/s), constant, temperature, velocity and velocity squared coefficients c0, ct,
// cv and cvv, its fan type fantype (VentilationType: 0 natural, 1 intake, 2 exhaust, 3 balanced),
// fan pressure rise fanp (Pa) and total efficiency fane. For the wind and stack model: its opening
// area openarea (m2) with opening area fraction schedule value openfrac, opening effectiveness
// openeff and discharge coefficient disc (-99999 to autocalculate), effective angle effangle (deg)
// and height difference dh (m). Its controls: minimum and maximum indoor temperatures mint and
// maxt, minimum and maximum outdoor temperatures minot and maxot, delta temperature delt (all as
// schedules when scheds is 1), maximum wind speed maxwind, hybrid control hybrid (HybridCtrlType:
// 0 individual, 1 close), and an EMS flow override emsflow (m3/s) when ems is 1. The zone air is at
// mat with humidity ratio hum, at the previous system time step xmpt and wprev, which the first
// system time step loop uses (systs is the loop index). Outdoors the zone sees outdb, wind speed
// wind from winddir (deg), the barometric pressure outbaro, the outdoor humidity ratio and
// enthalpy outhum and outenth (or linkedhum and linkedenth from the zone's outdoor air node when
// linked is 1), the standard air density stdrho; the system time step is tsyssec seconds. Then
// the real CalcAirFlowSimple runs without zone mixing or infiltration flow adjustment. One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 7; nvent outside 0 or 1; a model, density
// basis, fan type or hybrid control outside its range here; a flag outside 0 or 1; scheduled
// minimum temperatures above their maximums (EnergyPlus warns and changes them, which needs the
// error file); and the fatal error EnergyPlus stops on, a negative air density.
#include <cmath>
#include <EnergyPlus/CoolTower.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/EarthTube.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/ThermalChimney.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <EnergyPlus/ZoneEquipmentManager.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
Sched::Schedule *availSched;
Sched::Schedule *openSched;
Sched::Schedule *limSched[5];

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataHeatBal->ZnAirRpt.allocate(1);
        p->dataHeatBal->Ventilation.allocate(1);
        p->dataHeatBal->AirFlowFlag = true;
        p->dataEarthTube->GetInputFlag = false;
        p->dataCoolTower->GetInputFlag = false;
        p->dataThermalChimneys->ThermalChimneyGetInputFlag = false;
        p->dataLoopNodes->Node.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        availSched = Sched::AddScheduleConstant(*p, "VENTILATION AVAILABILITY");
        openSched = Sched::AddScheduleConstant(*p, "VENTILATION OPENING FRACTION");
        for (int i = 0; i < 5; ++i) {
            limSched[i] = Sched::AddScheduleConstant(*p, "VENTILATION LIMIT " + std::to_string(i + 1));
        }
        auto &v = p->dataHeatBal->Ventilation(1);
        v.Name = "VENTILATION 1";
        v.ZonePtr = 1;
        v.spaceIndex = 1;
        v.availSched = availSched;
        v.openAreaFracSched = openSched;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One value after CalcAirFlowSimple for zone 1 (see the file comment): which picks the zone's MCPV
// (0), VAMFL (1) and MCPTV (2), the ventilation object's MCP (3), FanPower (4) and AirTemp (5,
// starting at 0), the zone's ventilation fan electricity VentilFanElec (6, J), and the object's
// MinIndoorTemperature after the controls (7).
double ep_Ventilation(int which,
                      int nvent,
                      int mod,
                      int den,
                      double avail,
                      double dl,
                      double c0,
                      double ct,
                      double cv,
                      double cvv,
                      int fantype,
                      double fanp,
                      double fane,
                      double openarea,
                      double openfrac,
                      double openeff,
                      double disc,
                      double effangle,
                      double dh,
                      int scheds,
                      double mint,
                      double maxt,
                      double minot,
                      double maxot,
                      double delt,
                      double maxwind,
                      int hybrid,
                      int ems,
                      double emsflow,
                      int systs,
                      double mat,
                      double hum,
                      double xmpt,
                      double wprev,
                      double outdb,
                      double wind,
                      double winddir,
                      double outbaro,
                      double outhum,
                      double outenth,
                      int linked,
                      double linkedhum,
                      double linkedenth,
                      double stdrho,
                      double tsyssec)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 7 || !flag(nvent) || !flag(mod) || den < 0 || den > 2 || fantype < 0 || fantype > 3 || !flag(hybrid) ||
        !flag(scheds) || !flag(ems) || !flag(linked)) {
        return NAN;
    }
    if (scheds == 1 && (mint > maxt || minot > maxot)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataHeatBal->TotVentilation = nvent;
    auto &v = state.dataHeatBal->Ventilation(1);
    v.ModelType = static_cast<DataHeatBalance::VentilationModelType>(mod);
    v.densityBasis = static_cast<DataHeatBalance::InfVentDensityBasis>(den);
    availSched->currentVal = avail;
    v.DesignLevel = dl;
    v.ConstantTermCoef = c0;
    v.TemperatureTermCoef = ct;
    v.VelocityTermCoef = cv;
    v.VelocitySQTermCoef = cvv;
    v.FanType = static_cast<DataHeatBalance::VentilationType>(fantype);
    v.FanPressure = fanp;
    v.FanEfficiency = fane;
    v.OpenArea = openarea;
    openSched->currentVal = openfrac;
    v.OpenEff = openeff;
    v.DiscCoef = disc;
    v.EffAngle = effangle;
    v.DH = dh;
    double const lim[5] = {mint, maxt, minot, maxot, delt};
    for (int i = 0; i < 5; ++i) {
        limSched[i]->currentVal = lim[i];
    }
    v.minIndoorTempSched = scheds == 1 ? limSched[0] : nullptr;
    v.maxIndoorTempSched = scheds == 1 ? limSched[1] : nullptr;
    v.minOutdoorTempSched = scheds == 1 ? limSched[2] : nullptr;
    v.maxOutdoorTempSched = scheds == 1 ? limSched[3] : nullptr;
    v.deltaTempSched = scheds == 1 ? limSched[4] : nullptr;
    // Without schedules the limits are the object's constant values.
    v.MinIndoorTemperature = scheds == 1 ? 0.0 : mint;
    v.MaxIndoorTemperature = scheds == 1 ? 0.0 : maxt;
    v.MinOutdoorTemperature = scheds == 1 ? 0.0 : minot;
    v.MaxOutdoorTemperature = scheds == 1 ? 0.0 : maxot;
    v.DelTemperature = scheds == 1 ? 0.0 : delt;
    v.MaxWindSpeed = maxwind;
    v.HybridControlType = static_cast<DataHeatBalance::HybridCtrlType>(hybrid);
    v.EMSSimpleVentOn = ems == 1;
    v.EMSimpleVentFlowRate = emsflow;
    v.MCP = 0.0;
    v.FanPower = 0.0;
    v.AirTemp = 0.0;
    auto &zone = state.dataHeatBal->Zone(1);
    zone.OutDryBulbTemp = outdb;
    zone.WindSpeed = wind;
    zone.WindDir = winddir;
    zone.LinkedOutAirNode = linked == 1 ? 1 : 0;
    state.dataLoopNodes->Node(1).HumRat = linkedhum;
    state.dataLoopNodes->Node(1).Enthalpy = linkedenth;
    state.dataEnvrn->OutBaroPress = outbaro;
    state.dataEnvrn->OutHumRat = outhum;
    state.dataEnvrn->OutEnthalpy = outenth;
    state.dataEnvrn->StdRhoAir = stdrho;
    state.dataHVACGlobal->TimeStepSysSec = tsyssec;
    state.dataHeatBal->ZnAirRpt(1).VentilFanElec = 0.0;
    auto &zhb = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    zhb.MAT = mat;
    zhb.airHumRat = hum;
    zhb.XMPT = xmpt;
    zhb.WTimeMinusP = wprev;

    try {
        ZoneEquipmentManager::CalcAirFlowSimple(state, systs, false, false);
    } catch (FatalError const &) {
        return NAN;
    }

    switch (which) {
    case 0:
        return zhb.MCPV;
    case 1:
        return zhb.VAMFL;
    case 2:
        return zhb.MCPTV;
    case 3:
        return v.MCP;
    case 4:
        return v.FanPower;
    case 5:
        return v.AirTemp;
    case 6:
        return state.dataHeatBal->ZnAirRpt(1).VentilFanElec;
    default:
        return v.MinIndoorTemperature;
    }
}
}
