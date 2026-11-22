// The solveroot unit as rust-anything sees it: each function forwards to General::SolveRoot or
// General::SolveRoot2 through libep_solveroot_shim (shim/ep_solveroot_shim.cc). SolveRootX and
// SolveRootFlag are the root and exit flag SolveRoot returns for root algorithm `algo` on test
// function `fn`; SolveRoot2Algo and SolveRoot2X are the algorithm SolveRoot2's stats settle on
// and the last root after `calls` calls.
#include "solveroot.h"

double ep_SolveRootX(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1);
int ep_SolveRootFlag(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1);
int ep_SolveRoot2Algo(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1);
double ep_SolveRoot2X(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1);

double SolveRootX(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1)
{
    return ep_SolveRootX(algo, numOfIter, Eps, MaxIte, func, a, b, c, X_0, X_1);
}

int SolveRootFlag(int algo, int numOfIter, double Eps, int MaxIte, int func, double a, double b, double c, double X_0, double X_1)
{
    return ep_SolveRootFlag(algo, numOfIter, Eps, MaxIte, func, a, b, c, X_0, X_1);
}

int SolveRoot2Algo(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1)
{
    return ep_SolveRoot2Algo(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1);
}

double SolveRoot2X(int start, int calls, double Eps, int maxIters, int func, double a, double b, double c, double X_0, double X_1)
{
    return ep_SolveRoot2X(start, calls, Eps, maxIters, func, a, b, c, X_0, X_1);
}
