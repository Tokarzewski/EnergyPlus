// Written by hand alongside shim/ep_solarpos_shim.cc.
#ifndef EP_SOLARPOS_H
#define EP_INTRAD_H

double SkyEmissivity(int model, double osky, double drybulb, double dewpoint, double relhum);
double TauModel(int which, int tau2017, double etr, double cosZen, double taub, double taud);
double DailySolar(int which, int doy, double lat);
double SunCosines(int which, double time, double eqt, double sind, double cosd, double lat, double lon, double tzm);
double SunUpDown(int which, int tsInHour, double curTime, double ts1off, double eqt, double sind, double cosd, double lat, double lon, double tzm);
double WaterMains(int doy, double lat, double annualAvg, double maxDiff, double mult, double offset);
double LumEff(int which, int month, double cosZen, double dif, double beam, double elev, double dewpoint);

#endif
