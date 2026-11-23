// Written by hand: the zone air history pushes are ZoneSpaceHeatBalanceData members that read the
// zone's room air model and the solution algorithm from the EnergyPlusData state, which
// gen_plain.py cannot set up. Plain extern "C" entry point over the real EnergyPlus
// ZoneTempPredictorCorrector code.
//
// Each call sets zone 1's air histories: the zone time step temperatures and humidity ratios
// x0 to x3 and w0 to w3 (XMAT, WPrevZoneTS), the down-stepped system time step ones dx0 to dx3 and
// dw0 to dw3 (DSXMAT, DSWPrevZoneTS), the averaged and current air temperatures ztav, zt and mat,
// the humidity ratios airhumrat, airhumratavg and airhumrattemp, the relative humidity airrelhum,
// the previous system time step values xmpt and wtimeminusp, and the Exact and Euler method terms
// tm2, tmx, wm2 and wmx, under the zone air solution algorithm algo (DataHeatBalance::SolutionAlgo,
// 0 to 2) with outdoor barometric pressure outbaro. The zone air is fully mixed. Then op picks
// what runs: pushZoneTimestepHistory (0), pushSystemTimestepHistory (1) or
// revertZoneTimestepHistory (2) for zone 1 (no space), or DownInterpolate4HistoryValues from
// time step oldts to newts on the old values x0 to x2 (3, the five-value form) or x0 to x3 (4, the
// array form). One EnergyPlusData for the process, built when the library loads, with console
// output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 29; op outside 0 to 4; algo outside 0
// to 2.
#include <array>
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataRoomAirModel.hh>
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
        p->dataRoomAir->AirModel.allocate(1);
        p->dataZoneTempPredictorCorrector->zoneHeatBalance.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// One value after the call for zone 1 (see the file comment): which picks XMAT[0..3] (0 to 3),
// WPrevZoneTS[0..3] (4 to 7), DSXMAT[0..3] (8 to 11), DSWPrevZoneTS[0..3] (12 to 15), XMPT (16),
// airHumRat (17), WTimeMinusP (18), airRelHum (19), TM2 (20), TMX (21), WM2 (22), WMX (23), the
// interpolated new values 0 to 4 (24 to 28, starting at 0; the array form fills 0 to 3), and the
// array form's return value (29, 0 for the other ops).
double ep_ZoneHistory(int which,
                      int op,
                      int algo,
                      double x0,
                      double x1,
                      double x2,
                      double x3,
                      double w0,
                      double w1,
                      double w2,
                      double w3,
                      double dx0,
                      double dx1,
                      double dx2,
                      double dx3,
                      double dw0,
                      double dw1,
                      double dw2,
                      double dw3,
                      double ztav,
                      double zt,
                      double mat,
                      double airhumrat,
                      double airhumratavg,
                      double airhumrattemp,
                      double airrelhum,
                      double xmpt,
                      double wtimeminusp,
                      double tm2,
                      double tmx,
                      double wm2,
                      double wmx,
                      double outbaro,
                      double oldts,
                      double newts)
{
    if (which < 0 || which > 29 || op < 0 || op > 4 || algo < 0 || algo > 2) {
        return NAN;
    }
    EnergyPlus::EnergyPlusData &state = st();
    state.dataHeatBal->ZoneAirSolutionAlgo = static_cast<DataHeatBalance::SolutionAlgo>(algo);
    state.dataEnvrn->OutBaroPress = outbaro;
    auto &z = state.dataZoneTempPredictorCorrector->zoneHeatBalance(1);
    z.XMAT = {x0, x1, x2, x3};
    z.WPrevZoneTS = {w0, w1, w2, w3};
    z.DSXMAT = {dx0, dx1, dx2, dx3};
    z.DSWPrevZoneTS = {dw0, dw1, dw2, dw3};
    z.ZTAV = ztav;
    z.ZT = zt;
    z.MAT = mat;
    z.airHumRat = airhumrat;
    z.airHumRatAvg = airhumratavg;
    z.airHumRatTemp = airhumrattemp;
    z.airRelHum = airrelhum;
    z.XMPT = xmpt;
    z.WTimeMinusP = wtimeminusp;
    z.TM2 = tm2;
    z.TMX = tmx;
    z.WM2 = wm2;
    z.WMX = wmx;
    std::array<Real64, 5> nv = {0.0, 0.0, 0.0, 0.0, 0.0};
    Real64 ret = 0.0;

    switch (op) {
    case 0:
        z.pushZoneTimestepHistory(state, 1);
        break;
    case 1:
        z.pushSystemTimestepHistory(state, 1);
        break;
    case 2:
        z.revertZoneTimestepHistory(state, 1);
        break;
    case 3:
        ZoneTempPredictorCorrector::DownInterpolate4HistoryValues(oldts, newts, x0, x1, x2, nv[0], nv[1], nv[2], nv[3], nv[4]);
        break;
    default: {
        std::array<Real64, 4> const oldVals = {x0, x1, x2, x3};
        std::array<Real64, 4> newVals = {0.0, 0.0, 0.0, 0.0};
        ret = ZoneTempPredictorCorrector::DownInterpolate4HistoryValues(oldts, newts, oldVals, newVals);
        for (int i = 0; i < 4; ++i) {
            nv[i] = newVals[i];
        }
    } break;
    }

    if (which < 4) {
        return z.XMAT[which];
    }
    if (which < 8) {
        return z.WPrevZoneTS[which - 4];
    }
    if (which < 12) {
        return z.DSXMAT[which - 8];
    }
    if (which < 16) {
        return z.DSWPrevZoneTS[which - 12];
    }
    switch (which) {
    case 16:
        return z.XMPT;
    case 17:
        return z.airHumRat;
    case 18:
        return z.WTimeMinusP;
    case 19:
        return z.airRelHum;
    case 20:
        return z.TM2;
    case 21:
        return z.TMX;
    case 22:
        return z.WM2;
    case 23:
        return z.WMX;
    case 29:
        return ret;
    default:
        return nv[which - 24];
    }
}
}
