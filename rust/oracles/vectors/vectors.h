// Written by hand alongside shim/ep_vectors_shim.cc.
#ifndef EP_VECTORS_H
#define EP_VECTORS_H

double VAreaPolygon(int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double VNewellArea(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double VNewellNormal(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double VAzimuthTilt(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double VCoPlanar(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
int VPointsInPlane(int which, int n, double x1, double y1, double z1, double x2, double y2, double z2, double x3, double y3, double z3, double x4, double y4, double z4, double x5, double y5, double z5);
double VRound(int which, double x, double y, double z, double roundto);
double VNormalize(int which, double x, double y, double z);

#endif
