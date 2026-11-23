// Written by hand: the zone mixing part of ZoneEquipmentManager::CalcAirFlowSimple reads the
// mixing object, its schedules and both zones' air from the EnergyPlusData state, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// ZoneEquipmentManager code.
//
// Each call sets up zones 1 and 2 in a model with simple air flows and, when nmix is 1, one
// ZoneMixing object that brings air from zone 2 into zone 1, and nothing else (no infiltration,
// ventilation, cross mixing, earth tubes, cooling towers, thermal chimneys or zone air balance).
// The object's design flow rate (DesiredAirFlowRateSaved) is saved (m3/s), its delta temperature
// td (as a schedule when hastd is 1), and it has the minimum and maximum zone, source zone and
// outdoor temperature schedules at mint, maxt, mins, maxs, mino and maxo when their flags
// hmint to hmaxo are 1. hybrid 1 closes it (HybridCtrlType::Close). Zone i's air is at mat_i
// with humidity ratio hum_i, at the previous system time step xmpt_i and wprev_i, which the
// first system time step loop uses (systs is the loop index). Zone 1 sees outdoor dry bulb
// outdb; the barometric pressure is outbaro. Then the real CalcAirFlowSimple runs without zone
// mixing or infiltration flow adjustment. One EnergyPlusData for the process, built when the
// library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 6; a flag outside 0 or 1; a pair of
// minimum and maximum schedules with the minimum above the maximum (EnergyPlus warns and changes
// it, which needs the error file); and the fatal error EnergyPlus stops on, a negative air
// density.
#include <cmath>
#include <EnergyPlus/CoolTower.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalFanSys.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/EarthTube.hh>
#include <EnergyPlus/ScheduleManager.hh>
#include <EnergyPlus/ThermalChimney.hh>
#include <EnergyPlus/UtilityRoutines.hh>
#include <EnergyPlus/ZoneEquipmentManager.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
Sched::Schedule *tdSched;
Sched::Schedule *limSched[6];

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->NumOfZones = 2;
        p->dataHeatBal->Zone.allocate(2);
        p->dataHeatBal->Zone(1).Name = "ZONE 1";
        p->dataHeatBal->Zone(2).Name = "ZONE 2";
        p->dataHeatBal->Mixing.allocate(1);
        p->dataHeatBal->AirFlowFlag = true;
        p->dataHeatBalFanSys->ZoneMassBalanceFlag.dimension(2, false);
        p->dataEarthTube->GetInputFlag = false;
        p->dataCoolTower->GetInputFlag = false;
        p->dataThermalChimneys->ThermalChimneyGetInputFlag = false;
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(2);
        tdSched = Sched::AddScheduleConstant(*p, "MIXING DELTA TEMPERATURE");
        for (int i = 0; i < 6; ++i) {
            limSched[i] = Sched::AddScheduleConstant(*p, "MIXING LIMIT " + std::to_string(i + 1));
        }
        auto &m = p->dataHeatBal->Mixing(1);
        m.Name = "MIXING 1";
        m.ZonePtr = 1;
        m.spaceIndex = 1;
        m.FromZone = 2;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One value after CalcAirFlowSimple (see the file comment): which picks zone 1's MCPM (0), MCPTM
// (1), MixingMassFlowZone (2) and MixingMassFlowXHumRat (3), and the mixing object's
// DesiredAirFlowRate (4) and MixingMassFlowRate (5), both starting at 0, and ReportFlag (6, as 0
// or 1).
double ep_Mixing(int which,
                 int nmix,
                 int hybrid,
                 double saved,
                 double td,
                 int hastd,
                 int hmint,
                 double mint,
                 int hmaxt,
                 double maxt,
                 int hmins,
                 double mins,
                 int hmaxs,
                 double maxs,
                 int hmino,
                 double mino,
                 int hmaxo,
                 double maxo,
                 int systs,
                 double mat1,
                 double hum1,
                 double xmpt1,
                 double wprev1,
                 double mat2,
                 double hum2,
                 double xmpt2,
                 double wprev2,
                 double outdb,
                 double outbaro)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    for (int f : {nmix, hybrid, hastd, hmint, hmaxt, hmins, hmaxs, hmino, hmaxo}) {
        if (!flag(f)) {
            return NAN;
        }
    }
    if (which < 0 || which > 6 || (hmint == 1 && hmaxt == 1 && mint > maxt) || (hmins == 1 && hmaxs == 1 && mins > maxs) ||
        (hmino == 1 && hmaxo == 1 && mino > maxo)) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataHeatBal->TotMixing = nmix;
    auto &m = state.dataHeatBal->Mixing(1);
    m.HybridControlType = hybrid == 1 ? DataHeatBalance::HybridCtrlType::Close : DataHeatBalance::HybridCtrlType::Indiv;
    m.DesiredAirFlowRateSaved = saved;
    m.DesiredAirFlowRate = 0.0;
    m.MixingMassFlowRate = 0.0;
    m.ReportFlag = false;
    m.DeltaTemperature = td;
    tdSched->currentVal = td;
    m.deltaTempSched = hastd == 1 ? tdSched : nullptr;
    double const lim[6] = {mint, maxt, mins, maxs, mino, maxo};
    int const has[6] = {hmint, hmaxt, hmins, hmaxs, hmino, hmaxo};
    Sched::Schedule *s[6];
    for (int i = 0; i < 6; ++i) {
        limSched[i]->currentVal = lim[i];
        s[i] = has[i] == 1 ? limSched[i] : nullptr;
    }
    m.minIndoorTempSched = s[0];
    m.maxIndoorTempSched = s[1];
    m.minSourceTempSched = s[2];
    m.maxSourceTempSched = s[3];
    m.minOutdoorTempSched = s[4];
    m.maxOutdoorTempSched = s[5];
    state.dataHeatBal->Zone(1).OutDryBulbTemp = outdb;
    state.dataEnvrn->OutBaroPress = outbaro;
    auto &z1 = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    auto &z2 = state.dataZoneTempPredictorCorrector->zoneHeatBalance(2);
    z1.MAT = mat1;
    z1.airHumRat = hum1;
    z1.XMPT = xmpt1;
    z1.WTimeMinusP = wprev1;
    z2.MAT = mat2;
    z2.airHumRat = hum2;
    z2.XMPT = xmpt2;
    z2.WTimeMinusP = wprev2;

    try {
        ZoneEquipmentManager::CalcAirFlowSimple(state, systs, false, false);
    } catch (FatalError const &) {
        return NAN;
    }

    switch (which) {
    case 0:
        return z1.MCPM;
    case 1:
        return z1.MCPTM;
    case 2:
        return z1.MixingMassFlowZone;
    case 3:
        return z1.MixingMassFlowXHumRat;
    case 4:
        return m.DesiredAirFlowRate;
    case 5:
        return m.MixingMassFlowRate;
    default:
        return m.ReportFlag ? 1.0 : 0.0;
    }
}
}
