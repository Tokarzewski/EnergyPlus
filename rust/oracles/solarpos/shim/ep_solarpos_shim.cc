// Written by hand: these WeatherManager routines read the site, the clock and the day's solar
// variables from EnergyPlusData, or return several results through references, which
// gen_plain.py cannot build. Plain extern "C" entry points over the real EnergyPlus code; the site
// (latitude, longitude, time zone meridian) and the other state they read are set per call, and
// SinLatitude and CosLatitude are computed as CheckLocationValidity computes them.
//
// One EnergyPlusData for the process, built when the library loads, with console output off.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/Psychrometrics.hh>
#include <EnergyPlus/WeatherManager.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::Weather;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

void site(double lat, double lon, double tzm)
{
    auto &e = *st().dataEnvrn;
    e.Latitude = lat;
    e.Longitude = lon;
    e.TimeZoneMeridian = tzm;
    e.SinLatitude = std::sin(Constant::DegToRad * lat);
    e.CosLatitude = std::cos(Constant::DegToRad * lat);
}
} // namespace

extern "C" {
// CalcSkyEmissivity: model 0 Clark and Allen, 1 Brunt, 2 Idso, 3 Berdahl and Martin.
double ep_SkyEmissivity(int model, double osky, double drybulb, double dewpoint, double relhum)
{
    SkyTempModel const m[4] = {SkyTempModel::ClarkAllen, SkyTempModel::Brunt, SkyTempModel::Idso, SkyTempModel::BerdahlMartin};
    if (model < 0 || model > 3) {
        return NAN;
    }
    return CalcSkyEmissivity(st(), m[model], osky, drybulb, dewpoint, relhum);
}

// ASHRAETauModel (tau2017 0: the 2009 coefficients, 1: the 2013/2017 ones): which picks IDirN (0),
// IDifH (1) or IGlbH (2).
double ep_TauModel(int which, int tau2017, double etr, double cosZen, double taub, double taud)
{
    Real64 dirN = 0.0, difH = 0.0, glbH = 0.0;
    ASHRAETauModel(st(), tau2017 != 0 ? DesDaySolarModel::ASHRAE_Tau2017 : DesDaySolarModel::ASHRAE_Tau, etr, cosZen, taub, taud, dirN, difH, glbH);
    return which == 0 ? dirN : which == 1 ? difH : which == 2 ? glbH : NAN;
}

// CalculateDailySolarCoeffs: which picks A (0), B (1), C (2), AnnVarSolConstant (3),
// EquationOfTime (4), SineSolarDeclination (5) or CosineSolarDeclination (6).
double ep_DailySolar(int which, int doy, double lat)
{
    site(lat, 0.0, 0.0);
    Real64 r[7] = {0, 0, 0, 0, 0, 0, 0};
    CalculateDailySolarCoeffs(st(), doy, r[0], r[1], r[2], r[3], r[4], r[5], r[6]);
    return which >= 0 && which < 7 ? r[which] : NAN;
}

// CalculateSunDirectionCosines: which picks SUNCOS x (0), y (1) or z (2).
double ep_SunCosines(int which, double time, double eqt, double sind, double cosd, double lat, double lon, double tzm)
{
    site(lat, lon, tzm);
    Vector3D c = {-1.0, -1.0, -1.0};
    CalculateSunDirectionCosines(st(), time, eqt, sind, cosd, c);
    return which == 0 ? c.x : which == 1 ? c.y : which == 2 ? c.z : NAN;
}

// DetermineSunUpDown with the clock (time steps per hour, CurrentTime, TS1TimeOffset) and the
// day's equation of time and declination: which picks SunCOS x (0), y (1), z (2), HrAngle (3),
// SolarAltitudeAngle (4), SolarAzimuthAngle (5) or SunIsUp (6, 0 or 1).
double ep_SunUpDown(int which, int tsInHour, double curTime, double ts1off, double eqt, double sind, double cosd, double lat, double lon, double tzm)
{
    auto &s = st();
    site(lat, lon, tzm);
    s.dataGlobal->TimeStepsInHour = tsInHour;
    s.dataGlobal->CurrentTime = curTime;
    s.dataEnvrn->TS1TimeOffset = ts1off;
    s.dataWeather->TodayVariables.EquationOfTime = eqt;
    s.dataWeather->TodayVariables.SinSolarDeclinAngle = sind;
    s.dataWeather->TodayVariables.CosSolarDeclinAngle = cosd;
    Vector3D c = {-1.0, -1.0, -1.0};
    DetermineSunUpDown(s, c);
    switch (which) {
    case 0:
        return c.x;
    case 1:
        return c.y;
    case 2:
        return c.z;
    case 3:
        return s.dataWeather->HrAngle;
    case 4:
        return s.dataWeather->SolarAltitudeAngle;
    case 5:
        return s.dataWeather->SolarAzimuthAngle;
    case 6:
        return s.dataEnvrn->SunIsUp ? 1.0 : 0.0;
    default:
        return NAN;
    }
}

// WaterMainsTempFromCorrelation on day doy at latitude lat.
double ep_WaterMains(int doy, double lat, double annualAvg, double maxDiff, double mult, double offset)
{
    site(lat, 0.0, 0.0);
    st().dataEnvrn->DayOfYear = doy;
    return WaterMainsTempFromCorrelation(st(), annualAvg, maxDiff, mult, offset);
}

// DayltgLuminousEfficacy for month (1 to 12, else NaN): which picks DiffLumEff (0), DirLumEff
// (1), SkyClearness (2) or SkyBrightness (3).
double ep_LumEff(int which, int month, double cosZen, double dif, double beam, double elev, double dewpoint)
{
    if (month < 1 || month > 12) {
        return NAN;
    }
    auto &e = *st().dataEnvrn;
    e.SOLCOS = {0.0, 0.0, cosZen};
    e.DifSolarRad = dif;
    e.BeamSolarRad = beam;
    e.Elevation = elev;
    e.Month = month;
    e.OutDewPointTemp = dewpoint;
    Real64 diff = 0.0, dir = 0.0;
    DayltgLuminousEfficacy(st(), diff, dir);
    return which == 0 ? diff : which == 1 ? dir : which == 2 ? e.SkyClearness : which == 3 ? e.SkyBrightness : NAN;
}
}
