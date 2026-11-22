// Written by hand (SolveRoot takes a std::function, which gen_plain.py cannot wrap).
// Plain extern "C" entry points over the real General::SolveRoot and General::SolveRoot2, run on
// a fixed family of test functions picked by `fn`. One EnergyPlusData for the process, built when
// the library loads, whose root-finding settings each call sets and then restores.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/General.hh>
#include <EnergyPlus/HVACSystemRootFindingAlgorithm.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] { return new EnergyPlusData; }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

// The test functions; solveroot-rs's `test_fn` must compute the same.
double fx(int fn, double a, double b, double c, double x)
{
    switch (fn) {
    case 0:
        return a * x * x * x + b * x + c;
    case 1:
        return std::exp(a * x) - b + c * x;
    case 2:
        return std::sin(a * x) + b * x + c;
    default:
        return a;
    }
}

// Inputs where the C++ is undefined (signed overflow of its iteration counters) or would run
// for too long; the exports return NaN or 0 for them (ADR-009).
bool defined(int numOfIter, int maxIte)
{
    return maxIte <= 1000 && numOfIter >= -1000 && numOfIter <= 1000;
}

struct Out
{
    int flag;
    double x;
};

Out solve(int algo, int numOfIter, double Eps, int MaxIte, int fn, double a, double b, double c, double X_0, double X_1)
{
    auto &rf = *st().dataRootFinder;
    RootAlgo const algoBackup = rf.rootAlgo;
    int const iterBackup = rf.NumOfIter;
    rf.rootAlgo = static_cast<RootAlgo>(algo);
    rf.NumOfIter = numOfIter;
    Out o{0, 0.0};
    General::SolveRoot(st(), Eps, MaxIte, o.flag, o.x, [&](Real64 X) { return fx(fn, a, b, c, X); }, X_0, X_1);
    rf.rootAlgo = algoBackup;
    rf.NumOfIter = iterBackup;
    return o;
}

// `calls` SolveRoot2 calls from fresh stats starting at `start`, the k-th on fn with c + 0.01 k;
// stops early if the stats pick no algorithm (the next call would index its arrays at -1).
struct Out2
{
    int algo;
    double x;
};

Out2 solve2(int start, int calls, double Eps, int maxIters, int fn, double a, double b, double c, double X_0, double X_1)
{
    General::SolveRootStats stats;
    stats.algo = static_cast<RootAlgo>(start);
    Out2 o{start, 0.0};
    for (int k = 0; k < calls && stats.algo != RootAlgo::Invalid; ++k) {
        double const ck = c + 0.01 * k;
        int flag = 0;
        o.x = General::SolveRoot2(st(), Eps, maxIters, flag, [&](Real64 X) { return fx(fn, a, b, ck, X); }, X_0, X_1, stats);
    }
    o.algo = static_cast<int>(stats.algo);
    return o;
}

bool defined2(int start, int calls, int maxIters)
{
    return start >= 0 && start < static_cast<int>(RootAlgo::Num) && calls >= 0 && calls <= 100 && defined(0, maxIters) &&
           maxIters >= -1000;
}
} // namespace

extern "C" {
double ep_SolveRootX(int algo, int numOfIter, double Eps, int MaxIte, int fn, double a, double b, double c, double X_0, double X_1)
{
    return defined(numOfIter, MaxIte) ? solve(algo, numOfIter, Eps, MaxIte, fn, a, b, c, X_0, X_1).x : NAN;
}

int ep_SolveRootFlag(int algo, int numOfIter, double Eps, int MaxIte, int fn, double a, double b, double c, double X_0, double X_1)
{
    return defined(numOfIter, MaxIte) ? solve(algo, numOfIter, Eps, MaxIte, fn, a, b, c, X_0, X_1).flag : 0;
}

int ep_SolveRoot2Algo(int start, int calls, double Eps, int maxIters, int fn, double a, double b, double c, double X_0, double X_1)
{
    return defined2(start, calls, maxIters) ? solve2(start, calls, Eps, maxIters, fn, a, b, c, X_0, X_1).algo : 0;
}

double ep_SolveRoot2X(int start, int calls, double Eps, int maxIters, int fn, double a, double b, double c, double X_0, double X_1)
{
    return defined2(start, calls, maxIters) ? solve2(start, calls, Eps, maxIters, fn, a, b, c, X_0, X_1).x : NAN;
}
}
