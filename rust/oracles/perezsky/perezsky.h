// Written by hand alongside shim/ep_perezsky_shim.cc.
#ifndef EP_PEREZSKY_H
#define EP_INTRAD_H

double PerezSky(int which, int detailed, double sx, double sy, double sz, double elev, double beam, double dif, double nx, double ny, double nz, double vfsky, double tilt, double iso, double sunlit, double horiz);
double ClipLine(int which, double x0, double x1, double y0, double y1, double maxX, double minX, double maxY, double minY);

#endif
