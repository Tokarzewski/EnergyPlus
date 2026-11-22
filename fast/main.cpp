// fast-anything harness for EnergyPlus: an annual run of the per-timestep psychrometric and
// exterior-convection work for a five-zone building like testfiles/5ZoneAirCooled.idf, driven by
// the Chicago O'Hare TMY3 weather file in weather/.
//
//     g++ -O2 -std=c++17 main.cpp simulation.cpp ep_kernels.cpp -o ep_harness
//
// Each zone timestep (6 per hour) does the outdoor-air state from the EPW record, then three HVAC
// iterations over the zones: the zone air state (humidity ratio, wet bulb, enthalpy, density),
// a coil leaving-air state, and an exterior convection coefficient for every exterior surface.
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
double SimulateYear(const std::vector<double> &tdb, const std::vector<double> &tdp, const std::vector<double> &pb,
                    const std::vector<double> &wspd, const std::vector<double> &wdir);

struct Hour {
    double tdb, tdp, pb, wspd, wdir;
};

static std::vector<Hour> read_epw(const char *path)
{
    std::vector<Hour> hours;
    std::ifstream in(path);
    if (!in) return hours;
    std::string line;
    for (int i = 0; i < 8 && std::getline(in, line); ++i) {
    }
    while (std::getline(in, line)) {
        std::vector<std::string> f;
        std::stringstream ss(line);
        std::string cell;
        while (std::getline(ss, cell, ',')) f.push_back(cell);
        if (f.size() < 22) continue;
        hours.push_back({std::atof(f[6].c_str()), std::atof(f[7].c_str()), std::atof(f[9].c_str()), std::atof(f[21].c_str()),
                         std::atof(f[20].c_str())});
    }
    return hours;
}

int main()
{
    const char *candidates[] = {"weather/USA_IL_Chicago-OHare.Intl.AP.725300_TMY3.epw",
                                "../weather/USA_IL_Chicago-OHare.Intl.AP.725300_TMY3.epw",
                                "../../weather/USA_IL_Chicago-OHare.Intl.AP.725300_TMY3.epw"};
    std::vector<Hour> hours;
    for (auto c : candidates) {
        hours = read_epw(c);
        if (!hours.empty()) break;
    }
    if (hours.empty()) {
        std::fprintf(stderr, "weather file not found\n");
        return 1;
    }
    std::vector<double> tdb, tdp, pb, wspd, wdir;
    for (const Hour &h : hours) {
        tdb.push_back(h.tdb);
        tdp.push_back(h.tdp);
        pb.push_back(h.pb);
        wspd.push_back(h.wspd);
        wdir.push_back(h.wdir);
    }
    double checksum = SimulateYear(tdb, tdp, pb, wspd, wdir);
    std::printf("%zu hours, checksum %.9f\n", hours.size(), checksum);
    return 0;
}
