// Written by hand: the infiltration part of ZoneEquipmentManager::CalcAirFlowSimple reads the
// zone's infiltration objects, their schedules and the zone and outdoor conditions from the
// EnergyPlusData state, which gen_plain.py cannot set up. Plain extern "C" entry point over the
// real EnergyPlus ZoneEquipmentManager code.
//
// Each call sets up zone 1 in a model with simple air flows and ninf (0 to 2) infiltration objects
// and nothing else (no ventilation, mixing, earth tubes, cooling towers, thermal chimneys or zone
// air balance). Object i uses model mod_i (DataHeatBalance::InfiltrationModelType: 0 design flow
// rate, 1 Sherman-Grimsrud, 2 AIM-2) and density basis den_i (InfVentDensityBasis: 0 outdoor, 1
// standard, 2 indoor) with schedule value frac_i; for the design flow rate model its design level
// dl_i (m3/s) and constant, temperature, velocity and velocity squared coefficients c0_i, ct_i,
// cv_i and cvv_i; for Sherman-Grimsrud its leakage area la_i (cm2), stack and wind coefficients
// cs_i and cw_i; for AIM-2 its flow coefficient fc_i, stack and wind coefficients as_i and aw_i,
// pressure exponent n_i and shelter factor sf_i. When ems_i is 1, EMS overrides its flow with
// emsv_i (m3/s). The zone air is at mat with humidity ratio hum, at the previous system time step
// xmpt and wprev, and the first system time step loop uses those (systs is the system time step
// loop index). Outdoors the zone sees outdb and wind speed wind, the weather station wind speed
// is metwind, the barometric pressure outbaro, the outdoor humidity ratio outhum (or linkedhum
// from the zone's outdoor air node when linked is 1), and the standard air density stdrho. Then
// the real CalcAirFlowSimple runs without zone mixing or infiltration flow adjustment. One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 8; ninf outside 0 to 2; a model or density
// basis outside its enum; a flag outside 0 or 1; and the fatal error EnergyPlus stops on, a
// negative air density.
#include <cmath>
#include <EnergyPlus/CoolTower.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
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
Sched::Schedule *infSched[2];

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 1;
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataHeatBal->Infiltration.allocate(2);
        p->dataHeatBal->AirFlowFlag = true;
        p->dataEarthTube->GetInputFlag = false;
        p->dataCoolTower->GetInputFlag = false;
        p->dataThermalChimneys->ThermalChimneyGetInputFlag = false;
        p->dataLoopNodes->Node.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        for (int i = 0; i < 2; ++i) {
            infSched[i] = Sched::AddScheduleConstant(*p, "INFILTRATION " + std::to_string(i + 1));
            auto &inf = p->dataHeatBal->Infiltration(i + 1);
            inf.Name = "INFILTRATION " + std::to_string(i + 1);
            inf.ZonePtr = 1;
            inf.spaceIndex = 1;
            inf.sched = infSched[i];
        }
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

struct Obj
{
    int mod, den;
    double frac, dl, c0, ct, cv, cvv, la, cs, cw, fc, as, aw, n, sf;
    int ems;
    double emsv;
};

void set(EnergyPlus::EnergyPlusData &state, int i, Obj const &o)
{
    auto &inf = state.dataHeatBal->Infiltration(i);
    inf.ModelType = static_cast<DataHeatBalance::InfiltrationModelType>(o.mod);
    inf.densityBasis = static_cast<DataHeatBalance::InfVentDensityBasis>(o.den);
    infSched[i - 1]->currentVal = o.frac;
    inf.DesignLevel = o.dl;
    inf.ConstantTermCoef = o.c0;
    inf.TemperatureTermCoef = o.ct;
    inf.VelocityTermCoef = o.cv;
    inf.VelocitySQTermCoef = o.cvv;
    inf.LeakageArea = o.la;
    inf.BasicStackCoefficient = o.cs;
    inf.BasicWindCoefficient = o.cw;
    inf.FlowCoefficient = o.fc;
    inf.AIM2StackCoefficient = o.as;
    inf.AIM2WindCoefficient = o.aw;
    inf.PressureExponent = o.n;
    inf.ShelterFactor = o.sf;
    inf.EMSOverrideOn = o.ems == 1;
    inf.EMSAirFlowRateValue = o.emsv;
    inf.VolumeFlowRate = 0.0;
    inf.MassFlowRate = 0.0;
    inf.MCpI_temp = 0.0;
}
} // namespace

extern "C" {
// One value after CalcAirFlowSimple for zone 1 (see the file comment): which picks the zone's MCPI
// (0), OAMFL (1) and MCPTI (2), and infiltration object 1's VolumeFlowRate (3), MassFlowRate (4)
// and MCpI_temp (5), then object 2's (6 to 8).
double ep_Infiltration(int which,
                       int ninf,
                       int mod1,
                       int den1,
                       double frac1,
                       double dl1,
                       double c01,
                       double ct1,
                       double cv1,
                       double cvv1,
                       double la1,
                       double cs1,
                       double cw1,
                       double fc1,
                       double as1,
                       double aw1,
                       double n1,
                       double sf1,
                       int ems1,
                       double emsv1,
                       int mod2,
                       int den2,
                       double frac2,
                       double dl2,
                       double c02,
                       double ct2,
                       double cv2,
                       double cvv2,
                       double la2,
                       double cs2,
                       double cw2,
                       double fc2,
                       double as2,
                       double aw2,
                       double n2,
                       double sf2,
                       int ems2,
                       double emsv2,
                       int systs,
                       double mat,
                       double hum,
                       double xmpt,
                       double wprev,
                       double outdb,
                       double wind,
                       double metwind,
                       double outbaro,
                       double outhum,
                       int linked,
                       double linkedhum,
                       double stdrho)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    auto enum3 = [](int v) { return v >= 0 && v <= 2; };
    if (which < 0 || which > 8 || ninf < 0 || ninf > 2 || !enum3(mod1) || !enum3(den1) || !enum3(mod2) || !enum3(den2) || !flag(ems1) ||
        !flag(ems2) || !flag(linked)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataHeatBal->TotInfiltration = ninf;
    set(state, 1, {mod1, den1, frac1, dl1, c01, ct1, cv1, cvv1, la1, cs1, cw1, fc1, as1, aw1, n1, sf1, ems1, emsv1});
    set(state, 2, {mod2, den2, frac2, dl2, c02, ct2, cv2, cvv2, la2, cs2, cw2, fc2, as2, aw2, n2, sf2, ems2, emsv2});
    auto &zone = state.dataHeatBal->Zone(1);
    zone.OutDryBulbTemp = outdb;
    zone.WindSpeed = wind;
    zone.LinkedOutAirNode = linked == 1 ? 1 : 0;
    state.dataLoopNodes->Node(1).HumRat = linkedhum;
    state.dataEnvrn->WindSpeed = metwind;
    state.dataEnvrn->OutBaroPress = outbaro;
    state.dataEnvrn->OutHumRat = outhum;
    state.dataEnvrn->StdRhoAir = stdrho;
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

    auto const &i1 = state.dataHeatBal->Infiltration(1);
    auto const &i2 = state.dataHeatBal->Infiltration(2);
    switch (which) {
    case 0:
        return zhb.MCPI;
    case 1:
        return zhb.OAMFL;
    case 2:
        return zhb.MCPTI;
    case 3:
        return i1.VolumeFlowRate;
    case 4:
        return i1.MassFlowRate;
    case 5:
        return i1.MCpI_temp;
    case 6:
        return i2.VolumeFlowRate;
    case 7:
        return i2.MassFlowRate;
    default:
        return i2.MCpI_temp;
    }
}
}
