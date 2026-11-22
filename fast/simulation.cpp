// The annual simulation loop of the fast-anything harness: per zone timestep, the outdoor-air
// state, three HVAC iterations of zone air and coil states, and exterior convection for every
// exterior surface of a five-zone building. One call does the whole year, so fast-anything's
// JSON bridge pays once for the weather arrays instead of once per kernel call.
#include <algorithm>
#include <cmath>
#include <cstddef>
#include <vector>


double PsyPsatFnTemp(double T);
double PsyTsatFnPb(double Press);
double PsyTwbFnTdbWPb(double TDB, double dW, double Patm);
std::vector<double> PsyTwbFnTdbWPbBatch(const std::vector<double> &tdb, const std::vector<double> &w, const std::vector<double> &pb);
double PsyWFnTdpPb(double TDP, double PB);
double PsyHFnTdbW(double TDB, double dW);
double PsyTdbFnHW(double H, double dW);
double PsyCpAirFnW(double dw);
double PsyRhoAirFnPbTdbW(double pb, double tdb, double dw);
double CalcASHRAETARPNatural(double Tsurf, double Tamb, double cosTilt);
double CalcASHRAESimpExtConvCoeff(int Roughness, double SurfWindSpeed);
double CalcMoWITTWindward(double DeltaTemp, double WindAtZ);
double CalcMoWITTLeeward(double DeltaTemp, double WindAtZ);
double CalcDOE2Windward(double SurfaceTemp, double AirTemp, double CosineTilt, double WindAtZ, int RoughnessIndex);
double CalcDOE2Leeward(double SurfaceTemp, double AirTemp, double CosineTilt, double WindAtZ, int RoughnessIndex);
double CalcSparrowWindward(int RoughnessIndex, double FacePerimeter, double FaceArea, double WindAtZ);

namespace {
struct Surface {
    double cosTilt, azimuth, area, perimeter;
    int roughness, model; // model: 0 DOE-2, 1 MoWITT, 2 TARP natural + Sparrow, 3 simple
};
} // namespace

// Hourly weather columns from an EPW file; returns the run's checksum.
double SimulateYear(const std::vector<double> &tdb, const std::vector<double> &tdp, const std::vector<double> &pb,
                    const std::vector<double> &wspd, const std::vector<double> &wdir)
{
    size_t const n = tdb.size();
    if (n == 0 || tdp.size() < n || pb.size() < n || wspd.size() < n || wdir.size() < n) return 0.0;
    constexpr int nZones = 5, timestepsPerHour = 6, hvacIters = 3, surfsPerZone = 12;
    std::vector<Surface> surfs;
    for (int z = 0; z < nZones; ++z) {
        for (int s = 0; s < surfsPerZone; ++s) {
            double cosTilt = s == 0 ? 1.0 : (s == 1 ? -1.0 : 0.0); // roof, floor, walls
            surfs.push_back({cosTilt, 90.0 * (s % 4), 10.0 + 3.0 * s, 14.0 + s, s % 6, (z + s) % 4});
        }
    }
    std::vector<double> zoneT(nZones, 22.0), zoneW(nZones, 0.008), surfT(surfs.size(), 20.0);

    // Wet-bulb requests are collected for a day and solved in one batched call (the results only
    // feed reporting, not the next state), the way EnergyPlus's weather manager fills a day's
    // timestep arrays at once.
    std::vector<double> qTdb, qW, qPb, qWeight;
    double checksum = 0.0;
    auto wetBulb = [&](double qtdb, double qw, double qpb, double weight) {
        qTdb.push_back(qtdb);
        qW.push_back(qw);
        qPb.push_back(qpb);
        qWeight.push_back(weight);
    };
    auto flush = [&]() {
        if (qTdb.empty()) return;
        std::vector<double> twb = PsyTwbFnTdbWPbBatch(qTdb, qW, qPb);
        for (size_t i = 0; i < twb.size(); ++i) checksum += twb[i] * qWeight[i];
        qTdb.clear();
        qW.clear();
        qPb.clear();
        qWeight.clear();
    };
    for (size_t h = 0; h < n; ++h) {
        size_t c = h;
        size_t x = (h + 1) % n;
        for (int ts = 0; ts < timestepsPerHour; ++ts) {
            double frac = (ts + 1.0) / timestepsPerHour;
            double tout = tdb[c] + (tdb[x] - tdb[c]) * frac;
            double tdpNow = std::min(tout, tdp[c] + (tdp[x] - tdp[c]) * frac);
            double pbNow = pb[c] + (pb[x] - pb[c]) * frac;
            double wind = wspd[c] + (wspd[x] - wspd[c]) * frac;

            // outdoor air state
            double wOut = PsyWFnTdpPb(tdpNow, pbNow);
            wetBulb(tout, wOut, pbNow, 1e-3);
            double hOut = PsyHFnTdbW(tout, wOut);
            checksum += hOut * 1e-9;

            for (int it = 0; it < hvacIters; ++it) {
                for (int z = 0; z < nZones; ++z) {
                    // zone air: drift towards set point with outdoor coupling
                    double tz = zoneT[z] + 0.05 * (tout - zoneT[z]) / (1 + it);
                    double wz = zoneW[z] + 0.02 * (wOut - zoneW[z]);
                    wetBulb(tz, wz, pbNow, 1e-3);
                    double hz = PsyHFnTdbW(tz, wz);
                    double rho = PsyRhoAirFnPbTdbW(pbNow, tz, wz);
                    double cp = PsyCpAirFnW(wz);
                    // cooling/heating coil leaving air: remove or add 10% of the enthalpy difference
                    double hCoil = hz - 0.1 * (hz - PsyHFnTdbW(tz > 24.0 ? 13.0 : 30.0, wz));
                    double tCoil = PsyTdbFnHW(hCoil, wz);
                    wetBulb(tCoil, wz, pbNow, 1e-4);
                    zoneT[z] = tz + (21.0 + (tz > 24.0 ? 3.0 : 0.0) - tz) * 0.3;
                    zoneW[z] = wz;
                    checksum += rho * cp * 1e-6;

                    // exterior convection for the zone's surfaces
                    for (int s = 0; s < surfsPerZone; ++s) {
                        size_t i = (size_t)z * surfsPerZone + s;
                        const Surface &sf = surfs[i];
                        double ts_ = surfT[i];
                        double windward = std::cos((wdir[c] - sf.azimuth) * 3.14159265358979 / 180.0) > 0.0;
                        double hc;
                        switch (sf.model) {
                        case 0:
                            hc = windward ? CalcDOE2Windward(ts_, tout, sf.cosTilt, wind, sf.roughness)
                                          : CalcDOE2Leeward(ts_, tout, sf.cosTilt, wind, sf.roughness);
                            break;
                        case 1:
                            hc = windward ? CalcMoWITTWindward(ts_ - tout, wind) : CalcMoWITTLeeward(ts_ - tout, wind);
                            break;
                        case 2:
                            hc = CalcASHRAETARPNatural(ts_, tout, sf.cosTilt) +
                                 CalcSparrowWindward(sf.roughness, sf.perimeter, sf.area, wind);
                            break;
                        default:
                            hc = CalcASHRAESimpExtConvCoeff(sf.roughness, wind);
                        }
                        // crude surface heat balance: 0.4 m2K/W wall between zone and outside film
                        double U = 1.0 / (0.4 + 1.0 / (hc + 1e-9));
                        surfT[i] = tout + (zoneT[z] - tout) * U * 0.4;
                        checksum += hc * 1e-5;
                    }
                }
            }
        }
        if (h % 24 == 23) flush();
    }
    flush();
    // the saturation temperature at the site pressure, once per run (sizing)
    checksum += PsyTsatFnPb(pb[0]) * 1e-3 + PsyPsatFnTemp(20.0) * 1e-6;
    return checksum;
}
