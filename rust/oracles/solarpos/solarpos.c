// The solarpos unit as rust-anything sees it: each function forwards to the real EnergyPlus
// WeatherManager code through libep_solarpos_shim (shim/ep_solarpos_shim.cc), which
// sets the site and state from plain numbers (see the shim).
#include "solarpos.h"

double ep_SkyEmissivity(int model, double osky, double drybulb, double dewpoint, double relhum);
double ep_TauModel(int which, int tau2017, double etr, double cosZen, double taub, double taud);
double ep_DailySolar(int which, int doy, double lat);
double ep_SunCosines(int which, double time, double eqt, double sind, double cosd, double lat, double lon, double tzm);
double ep_SunUpDown(int which, int tsInHour, double curTime, double ts1off, double eqt, double sind, double cosd, double lat, double lon, double tzm);
double ep_WaterMains(int doy, double lat, double annualAvg, double maxDiff, double mult, double offset);
double ep_LumEff(int which, int month, double cosZen, double dif, double beam, double elev, double dewpoint);

double SkyEmissivity(int model, double osky, double drybulb, double dewpoint, double relhum)
{
    return ep_SkyEmissivity(model, osky, drybulb, dewpoint, relhum);
}

double TauModel(int which, int tau2017, double etr, double cosZen, double taub, double taud)
{
    return ep_TauModel(which, tau2017, etr, cosZen, taub, taud);
}

double DailySolar(int which, int doy, double lat)
{
    return ep_DailySolar(which, doy, lat);
}

double SunCosines(int which, double time, double eqt, double sind, double cosd, double lat, double lon, double tzm)
{
    return ep_SunCosines(which, time, eqt, sind, cosd, lat, lon, tzm);
}

double SunUpDown(int which, int tsInHour, double curTime, double ts1off, double eqt, double sind, double cosd, double lat, double lon, double tzm)
{
    return ep_SunUpDown(which, tsInHour, curTime, ts1off, eqt, sind, cosd, lat, lon, tzm);
}

double WaterMains(int doy, double lat, double annualAvg, double maxDiff, double mult, double offset)
{
    return ep_WaterMains(doy, lat, annualAvg, maxDiff, mult, offset);
}

double LumEff(int which, int month, double cosZen, double dif, double beam, double elev, double dewpoint)
{
    return ep_LumEff(which, month, cosZen, dif, beam, elev, dewpoint);
}

