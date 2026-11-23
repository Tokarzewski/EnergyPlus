// Written by hand: ZoneTempPredictorCorrector::DetectOscillatingZoneTemp reads the zone air
// temperature histories, occupancy and dead band flags from the EnergyPlusData state and sets up
// output variables on its first call, which gen_plain.py cannot set up. Plain extern "C" entry
// point over the real EnergyPlus ZoneTempPredictorCorrector code.
//
// Each call sets up nzones (1 to 3) zones with the oscillation output variables already set up
// and reported. Zone i has the zone air temperature history ha_i, hb_i and hc_i (ZoneTempHist rows
// 1 to 3, row 4 at 0) and the new zone air temperature zt_i; it is occupied (in the ASH55 comfort
// results, kept when comfort is 1) when occ_i is 1 and in its dead band or set back when db_i is
// 1. The system time step is tsys hours, and the run period sums start at sum1 to sum3. Then the
// real DetectOscillatingZoneTemp runs. One EnergyPlusData for the process, built when the library
// loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 26; nzones outside 1 to 3; a flag outside
// 0 or 1; or which naming a zone past nzones.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataZoneEnergyDemands.hh>
#include <EnergyPlus/ThermalComfort.hh>
#include <EnergyPlus/ZoneTempPredictorCorrector.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto &z = *p->dataZoneTempPredictorCorrector;
        z.SetupOscillationOutputFlag = false;
        z.OscillationVariablesNeeded = true;
        z.ZoneTempHist.allocate(4, 3);
        z.ZoneTempOscillate.dimension(3, 0.0);
        z.ZoneTempOscillateDuringOccupancy.dimension(3, 0.0);
        z.ZoneTempOscillateInDeadband.dimension(3, 0.0);
        z.zoneHeatBalance.allocate(3);
        p->dataZoneEnergyDemand->CurDeadBandOrSetback.dimension(3, false);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One result after DetectOscillatingZoneTemp (see the file comment): for zone i (1 to 3) at
// 7 * (i - 1), which picks ZoneTempHist rows 1 to 4 (+0 to +3), ZoneTempOscillate (+4),
// ZoneTempOscillateDuringOccupancy (+5) and ZoneTempOscillateInDeadband (+6); then
// AnyZoneTempOscillate (21), ...DuringOccupancy (22) and ...InDeadband (23), and the run period
// sums of the three (24 to 26).
double ep_Oscillate(int which,
                    int nzones,
                    double tsys,
                    int comfort,
                    double ha1,
                    double hb1,
                    double hc1,
                    double zt1,
                    int occ1,
                    int db1,
                    double ha2,
                    double hb2,
                    double hc2,
                    double zt2,
                    int occ2,
                    int db2,
                    double ha3,
                    double hb3,
                    double hc3,
                    double zt3,
                    int occ3,
                    int db3,
                    double sum1,
                    double sum2,
                    double sum3)
{
    auto flag = [](int v) { return v == 0 || v == 1; };
    if (which < 0 || which > 26 || nzones < 1 || nzones > 3) {
        return NAN;
    }
    for (int f : {comfort, occ1, db1, occ2, db2, occ3, db3}) {
        if (!flag(f)) {
            return NAN;
        }
    }
    int const slot = which % 7;
    int const zone = which / 7 + 1;
    if (which < 21 && zone > nzones) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataGlobal->NumOfZones = nzones;
    state.dataHVACGlobal->TimeStepSys = tsys;
    auto &z = *state.dataZoneTempPredictorCorrector;
    z.ZoneTempHist = 0.0;
    z.ZoneTempOscillate = 0.0;
    z.ZoneTempOscillateDuringOccupancy = 0.0;
    z.ZoneTempOscillateInDeadband = 0.0;
    auto &comf = state.dataThermalComforts->ThermalComfortInASH55;
    if (comf.allocated()) {
        comf.deallocate();
    }
    if (comfort == 1) {
        comf.allocate(nzones);
    }
    double const h[3][4] = {{ha1, hb1, hc1, zt1}, {ha2, hb2, hc2, zt2}, {ha3, hb3, hc3, zt3}};
    int const occ[3] = {occ1, occ2, occ3};
    int const db[3] = {db1, db2, db3};
    for (int i = 1; i <= nzones; ++i) {
        z.ZoneTempHist(1, i) = h[i - 1][0];
        z.ZoneTempHist(2, i) = h[i - 1][1];
        z.ZoneTempHist(3, i) = h[i - 1][2];
        z.ZoneTempHist(4, i) = 0.0;
        z.zoneHeatBalance(i).ZT = h[i - 1][3];
        if (comfort == 1) {
            comf(i).ZoneIsOccupied = occ[i - 1] == 1;
        }
        state.dataZoneEnergyDemand->CurDeadBandOrSetback(i) = db[i - 1] == 1;
    }
    z.AnnualAnyZoneTempOscillate = sum1;
    z.AnnualAnyZoneTempOscillateDuringOccupancy = sum2;
    z.AnnualAnyZoneTempOscillateInDeadband = sum3;

    ZoneTempPredictorCorrector::DetectOscillatingZoneTemp(state);

    switch (which) {
    case 21:
        return z.AnyZoneTempOscillate;
    case 22:
        return z.AnyZoneTempOscillateDuringOccupancy;
    case 23:
        return z.AnyZoneTempOscillateInDeadband;
    case 24:
        return z.AnnualAnyZoneTempOscillate;
    case 25:
        return z.AnnualAnyZoneTempOscillateDuringOccupancy;
    case 26:
        return z.AnnualAnyZoneTempOscillateInDeadband;
    default:
        break;
    }
    if (slot < 4) {
        return z.ZoneTempHist(slot + 1, zone);
    }
    if (slot == 4) {
        return z.ZoneTempOscillate(zone);
    }
    if (slot == 5) {
        return z.ZoneTempOscillateDuringOccupancy(zone);
    }
    return z.ZoneTempOscillateInDeadband(zone);
}
}
