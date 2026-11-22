// Written by hand: the ground temperature models are objects whose parameters come from input
// objects, which gen_plain.py cannot build. Plain extern "C" entry points over the real EnergyPlus
// models in GroundTemperatureModeling/: Kusuda-Achenbach, Xing, and the four monthly site tables
// (deep, shallow, building surface, F/C factor), each built from plain numbers with the year's
// length (dataWeather->NumDaysInYear) given. A site table is t0, t0 + dt, ..., t0 + 11 dt.
//
// The domain (ADR-009): a site table's month (the given one, or ceil(seconds / seconds a month))
// must fit an int and index the table after the C++'s `month % 12` fold; months 0, -1, ..., 12k
// index outside it and return NaN. Kusuda's month must not be INT_MIN (`_month - 1` overflows).
#include <cmath>
#include <climits>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/GroundTemperatureModeling/KusudaAchenbachGroundTemperatureModel.hh>
#include <EnergyPlus/GroundTemperatureModeling/SiteBuildingSurfaceGroundTemperatures.hh>
#include <EnergyPlus/GroundTemperatureModeling/SiteDeepGroundTemperatures.hh>
#include <EnergyPlus/GroundTemperatureModeling/SiteFCFactorMethodGroundTemperatures.hh>
#include <EnergyPlus/GroundTemperatureModeling/SiteShallowGroundTemperatures.hh>
#include <EnergyPlus/GroundTemperatureModeling/XingGroundTemperatureModel.hh>
#include <EnergyPlus/WeatherManager.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::GroundTemp;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = new EnergyPlusData;
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

bool site_month_ok(int month)
{
    int const m = month >= 1 && month <= 12 ? month : month % 12;
    return m >= 1 && m <= 12;
}

// The site table's month for `seconds`, or 0 when it does not fit an int.
int site_month_of(double seconds, int numDaysInYear)
{
    double const secPerMonth = numDaysInYear * Constant::rSecsInDay / 12;
    double const m = std::ceil(seconds / secPerMonth);
    return m >= INT_MIN && m <= INT_MAX ? static_cast<int>(m) : 0;
}

template <class T> double site(T &&model, std::array<Real64, 12> &temps, int byMonth, double when, int numDaysInYear, double t0, double dt)
{
    for (int i = 0; i < 12; ++i) {
        temps[i] = t0 + dt * i;
    }
    st().dataWeather->NumDaysInYear = numDaysInYear;
    if (byMonth != 0) {
        double const m = when;
        if (!(m >= INT_MIN && m <= INT_MAX) || m != std::trunc(m) || !site_month_ok(static_cast<int>(m))) {
            return NAN;
        }
        return model.getGroundTempAtTimeInMonths(st(), 0.0, static_cast<int>(m));
    }
    if (!site_month_ok(site_month_of(when, numDaysInYear))) {
        return NAN;
    }
    return model.getGroundTempAtTimeInSeconds(st(), 0.0, when);
}
} // namespace

extern "C" {
double ep_GTKusudaSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity)
{
    st().dataWeather->NumDaysInYear = numDaysInYear;
    KusudaGroundTempsModel m;
    m.aveGroundTemp = ave;
    m.aveGroundTempAmplitude = amp;
    m.phaseShiftInSecs = phaseSecs;
    m.groundThermalDiffusivity = diffusivity;
    return m.getGroundTempAtTimeInSeconds(st(), depth, seconds);
}

double ep_GTKusudaMonth(double depth, int month, int numDaysInYear, double ave, double amp, double phaseSecs, double diffusivity)
{
    if (month == INT_MIN) { // `_month - 1` would overflow
        return NAN;
    }
    st().dataWeather->NumDaysInYear = numDaysInYear;
    KusudaGroundTempsModel m;
    m.aveGroundTemp = ave;
    m.aveGroundTempAmplitude = amp;
    m.phaseShiftInSecs = phaseSecs;
    m.groundThermalDiffusivity = diffusivity;
    return m.getGroundTempAtTimeInMonths(st(), depth, month);
}

double ep_GTXingSeconds(double depth, double seconds, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2,
                        double diffusivity)
{
    st().dataWeather->NumDaysInYear = numDaysInYear;
    XingGroundTempsModel m;
    m.aveGroundTemp = ave;
    m.surfTempAmplitude_1 = amp1;
    m.phaseShift_1 = phase1;
    m.surfTempAmplitude_2 = amp2;
    m.phaseShift_2 = phase2;
    m.groundThermalDiffusivity = diffusivity;
    return m.getGroundTempAtTimeInSeconds(st(), depth, seconds);
}

double ep_GTXingMonth(double depth, int month, int numDaysInYear, double ave, double amp1, double phase1, double amp2, double phase2,
                      double diffusivity)
{
    st().dataWeather->NumDaysInYear = numDaysInYear;
    XingGroundTempsModel m;
    m.aveGroundTemp = ave;
    m.surfTempAmplitude_1 = amp1;
    m.phaseShift_1 = phase1;
    m.surfTempAmplitude_2 = amp2;
    m.phaseShift_2 = phase2;
    m.groundThermalDiffusivity = diffusivity;
    return m.getGroundTempAtTimeInMonths(st(), depth, month);
}

// kind: 0 deep, 1 shallow, 2 building surface, 3 F/C factor. byMonth: `when` is a month number
// (it must be a whole number) rather than seconds.
double ep_GTSite(int kind, int byMonth, double when, int numDaysInYear, double t0, double dt)
{
    switch (kind) {
    case 0: {
        SiteDeepGroundTemps m;
        return site(m, m.deepGroundTemps, byMonth, when, numDaysInYear, t0, dt);
    }
    case 1: {
        SiteShallowGroundTemps m;
        return site(m, m.surfaceGroundTemps, byMonth, when, numDaysInYear, t0, dt);
    }
    case 2: {
        SiteBuildingSurfaceGroundTemps m;
        return site(m, m.buildingSurfaceGroundTemps, byMonth, when, numDaysInYear, t0, dt);
    }
    case 3: {
        SiteFCFactorMethodGroundTemps m;
        return site(m, m.fcFactorGroundTemps, byMonth, when, numDaysInYear, t0, dt);
    }
    default:
        return NAN;
    }
}
}
