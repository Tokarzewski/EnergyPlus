// Written by hand alongside shim/ep_solveroot_shim.cc.
#ifndef EP_SOLVEROOT_H
#define EP_SOLVEROOT_H

double SolveRootX(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1);
int SolveRootFlag(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1);
int SolveRoot2Algo(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1);
double SolveRoot2X(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1);

#endif
