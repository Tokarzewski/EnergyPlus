// EnergyPlus kernels used by the fast-anything harness.
//
// Copied from EnergyPlus develop (7ab256b5a):
//   src/EnergyPlus/General.cc                  Iterate
//   src/EnergyPlus/Psychrometrics.cc/.hh       PsyPsatFnTemp, PsyTsatFnPb, PsyTwbFnTdbWPb,
//                                              PsyWFnTdpPb, PsyWFnTdbTwbPb, PsyHFnTdbW, PsyTdbFnHW,
//                                              PsyCpAirFnW, PsyRhoAirFnPbTdbW
//   src/EnergyPlus/ConvectionCoefficients.cc/.hh  ASHRAE/TARP natural, MoWITT and DOE-2 exterior
//                                              correlations
//
// The arithmetic is unchanged. What differs from EnergyPlus: the EnergyPlusData& and CalledFrom
// arguments are dropped (they only carry the warning machinery, compiled out unless
// EP_psych_errors is set, and the per-pressure caches), PsyPsatFnTemp is the uncached _raw form,
// PsyTsatFnPb iterates every call instead of returning the last-pressure cache, the roughness enum
// is a plain int, and the functions are free functions with plain-data arguments so that
// fast-anything can record and replay them.
#include <algorithm>
#include <array>
#include <cmath>
#include <vector>

namespace {
constexpr double Kelvin = 273.15;
constexpr double TriplePointOfWaterTempKelvin = 273.16;
constexpr double iconvTol = 0.0001;
constexpr std::array<double, 6> RoughnessMultiplier{2.17, 1.67, 1.52, 1.13, 1.11, 1.0};
inline double pow_2(double x) { return x * x; }

// General::Iterate
void Iterate(double &ResultX, double const Tol, double const X0, double const Y0, double &X1, double &Y1, int const Iter, int &Cnvg)
{
    constexpr double small(1.e-9);
    constexpr double Perturb(0.1);
    if (Iter != 1) {
        if (std::abs(X0 - X1) < Tol || Y0 == 0.0) {
            ResultX = X0;
            Cnvg = 1;
            return;
        }
    }
    Cnvg = 0;
    if (Iter == 1) {
        if (std::abs(X0) > small) {
            ResultX = X0 * (1.0 + Perturb);
        } else {
            ResultX = Perturb;
        }
    } else {
        double DY = Y0 - Y1;
        if (std::abs(DY) < small) {
            DY = small;
        }
        ResultX = (Y0 * X1 - Y1 * X0) / DY;
    }
    X1 = X0;
    Y1 = Y0;
}
} // namespace

// ---------------------------------------------------------------- psychrometrics

double PsyPsatFnTemp(double T)
{
    double Pascal;
    double const Tkel(T + Kelvin);
    if (Tkel < 173.15) {
        Pascal = 0.001405102123874164;
    } else if (Tkel < TriplePointOfWaterTempKelvin) {
        constexpr double C1(-5674.5359);
        constexpr double C2(6.3925247);
        constexpr double C3(-0.9677843e-2);
        constexpr double C4(0.62215701e-6);
        constexpr double C5(0.20747825e-8);
        constexpr double C6(-0.9484024e-12);
        constexpr double C7(4.1635019);
        Pascal = std::exp(C1 / Tkel + C2 + Tkel * (C3 + Tkel * (C4 + Tkel * (C5 + C6 * Tkel))) + C7 * std::log(Tkel));
    } else if (Tkel <= 473.15) {
        constexpr double C8(-5800.2206);
        constexpr double C9(1.3914993);
        constexpr double C10(-0.048640239);
        constexpr double C11(0.41764768e-4);
        constexpr double C12(-0.14452093e-7);
        constexpr double C13(6.5459673);
        Pascal = std::exp(C8 / Tkel + C9 + Tkel * (C10 + Tkel * (C11 + Tkel * C12)) + C13 * std::log(Tkel));
    } else {
        Pascal = 1555073.745636215;
    }
    return Pascal;
}

double PsyTsatFnPb(double Press)
{
    constexpr int itmax(50);
    constexpr double convTol(0.0001);
    double tSat = 100.0;
    if (Press >= 1555000.0) {
        tSat = 200.0;
    } else if (Press <= 0.0017) {
        tSat = -100.0;
    } else if ((Press > 611.000) && (Press < 611.25)) {
        tSat = 0.0;
    } else {
        double pSat;
        double error;
        double X1 = 0.0;
        double Y1 = 0.0;
        double ResultX = 0.0;
        int icvg = 0;
        for (int iter = 1; iter <= itmax; ++iter) {
            pSat = PsyPsatFnTemp(tSat);
            error = Press - pSat;
            Iterate(ResultX, convTol, tSat, error, X1, Y1, iter, icvg);
            tSat = ResultX;
            if (icvg == 1) {
                break;
            }
        }
    }
    return tSat;
}

double PsyTwbFnTdbWPb(double TDB, double dW, double Patm)
{
    constexpr int itmax(100);
    double newW;
    double W;
    double ResultX = 0.0;
    double WBT;
    double error;
    double X1 = 0.0;
    double Y1 = 0.0;
    double Wstar;
    double PSatstar;
    int icvg = 0;

    W = dW;
    if (W < 0.0) {
        W = 1.0e-5;
    }
    double const tBoil = PsyTsatFnPb(Patm);
    WBT = TDB;
    for (int iter = 1; iter <= itmax; ++iter) {
        if (WBT >= (tBoil - 0.09)) {
            WBT = tBoil - 0.1;
        }
        PSatstar = PsyPsatFnTemp(WBT);
        Wstar = 0.62198 * PSatstar / (Patm - PSatstar);
        if (WBT >= 0.0) {
            newW = ((2501.0 - 2.326 * WBT) * Wstar - 1.006 * (TDB - WBT)) / (2501.0 + 1.86 * TDB - 4.186 * WBT);
        } else {
            newW = ((2830.0 - 0.24 * WBT) * Wstar - 1.006 * (TDB - WBT)) / (2830.0 + 1.86 * TDB - 2.1 * WBT);
        }
        error = W - newW;
        Iterate(ResultX, iconvTol, WBT, error, X1, Y1, iter, icvg);
        WBT = ResultX;
        if (icvg == 1) {
            break;
        }
    }
    double TWB = WBT;
    if (TWB > TDB) {
        TWB = TDB;
    }
    return TWB;
}

// Batched form for the harness: the wet bulb of every (TDB, W, PB) state in one call.
// fast-anything's C++ bridge passes JSON per call, so a sub-microsecond kernel only pays off in Rust
// when one call carries a lot of work.
std::vector<double> PsyTwbFnTdbWPbBatch(const std::vector<double> &tdb, const std::vector<double> &w, const std::vector<double> &pb)
{
    std::vector<double> out(tdb.size());
    for (size_t i = 0; i < tdb.size(); ++i) {
        out[i] = PsyTwbFnTdbWPb(tdb[i], w[i], pb[i]);
    }
    return out;
}

double PsyWFnTdpPb(double TDP, double PB)
{
    double const PDEW(PsyPsatFnTemp(TDP));
    double const W(PDEW * 0.62198 / (PB - PDEW));
    if (W < 0.0) {
        double DeltaT = 0.0;
        double PDEW1 = PDEW;
        while (PDEW1 >= PB) {
            DeltaT++;
            PDEW1 = PsyPsatFnTemp(TDP - DeltaT);
        }
        return PDEW1 * 0.62198 / (PB - PDEW1);
    }
    return W;
}

double PsyHFnTdbW(double TDB, double dW)
{
    return 1.00484e3 * TDB + std::max(dW, 1.0e-5) * (2.50094e6 + 1.85895e3 * TDB);
}

double PsyTdbFnHW(double H, double dW)
{
    double const W(std::max(dW, 1.0e-5));
    return (H - 2.50094e6 * W) / (1.00484e3 + 1.85895e3 * W);
}

double PsyCpAirFnW(double dw)
{
    double const w(std::max(dw, 1.0e-5));
    return (1.00484e3 + w * 1.85895e3);
}

double PsyRhoAirFnPbTdbW(double pb, double tdb, double dw)
{
    // Psychrometrics.hh PsyRhoAirFnPbTdbW (non-_fast branch)
    return pb / (287.0 * (tdb + Kelvin) * (1.0 + 1.6077687 * std::max(dw, 1.0e-5)));
}

// ---------------------------------------------------------------- exterior convection

double CalcASHRAEVerticalWall(double DeltaTemp)
{
    return 1.31 * std::pow(std::abs(DeltaTemp), 1.0 / 3.0);
}

double CalcWaltonUnstableHorizontalOrTilt(double DeltaTemp, double CosineTilt)
{
    return 9.482 * std::pow(std::abs(DeltaTemp), 1.0 / 3.0) / (7.238 - std::abs(CosineTilt));
}

double CalcWaltonStableHorizontalOrTilt(double DeltaTemp, double CosineTilt)
{
    return 1.810 * std::pow(std::abs(DeltaTemp), 1.0 / 3.0) / (1.382 + std::abs(CosineTilt));
}

double CalcASHRAETARPNatural(double Tsurf, double Tamb, double cosTilt)
{
    double DeltaTemp = Tsurf - Tamb;
    if ((DeltaTemp == 0.0) || (cosTilt == 0.0)) {
        return CalcASHRAEVerticalWall(DeltaTemp);
    }
    if (((DeltaTemp < 0.0) && (cosTilt < 0.0)) || ((DeltaTemp > 0.0) && (cosTilt > 0.0))) {
        return CalcWaltonUnstableHorizontalOrTilt(DeltaTemp, cosTilt);
    }
    return CalcWaltonStableHorizontalOrTilt(DeltaTemp, cosTilt);
}

double CalcASHRAESimpExtConvCoeff(int Roughness, double SurfWindSpeed)
{
    constexpr static std::array<double, 6> D = {11.58, 12.49, 10.79, 8.23, 10.22, 8.23};
    constexpr static std::array<double, 6> E = {5.894, 4.065, 4.192, 4.00, 3.100, 3.33};
    constexpr static std::array<double, 6> F = {0.0, 0.028, 0.0, -0.057, 0.0, -0.036};
    return D[Roughness] + E[Roughness] * SurfWindSpeed + F[Roughness] * pow_2(SurfWindSpeed);
}

double CalcMoWITTNatural(double DeltaTemp)
{
    constexpr double temp_fac(0.84);
    return temp_fac * std::pow(std::abs(DeltaTemp), 1.0 / 3.0);
}

double CalcMoWITTForcedWindward(double WindAtZ)
{
    return 3.26 * std::pow(WindAtZ, 0.89);
}

double CalcMoWITTForcedLeeward(double WindAtZ)
{
    return 3.55 * std::pow(WindAtZ, 0.617);
}

double CalcMoWITTWindward(double DeltaTemp, double WindAtZ)
{
    double Hn = CalcMoWITTNatural(DeltaTemp);
    double Hf = CalcMoWITTForcedWindward(WindAtZ);
    return std::sqrt(pow_2(Hn) + pow_2(Hf));
}

double CalcMoWITTLeeward(double DeltaTemp, double WindAtZ)
{
    double Hn = CalcMoWITTNatural(DeltaTemp);
    double Hf = CalcMoWITTForcedLeeward(WindAtZ);
    return std::sqrt(pow_2(Hn) + pow_2(Hf));
}

double CalcDOE2Forced(double SurfaceTemp, double AirTemp, double CosineTilt, double HfSmooth, int RoughnessIndex)
{
    double Hn = CalcASHRAETARPNatural(SurfaceTemp, AirTemp, CosineTilt);
    double HcSmooth = std::sqrt(pow_2(Hn) + pow_2(HfSmooth));
    return RoughnessMultiplier[RoughnessIndex] * (HcSmooth - Hn);
}

double CalcDOE2Windward(double SurfaceTemp, double AirTemp, double CosineTilt, double WindAtZ, int RoughnessIndex)
{
    double HfSmooth = CalcMoWITTForcedWindward(WindAtZ);
    return CalcDOE2Forced(SurfaceTemp, AirTemp, CosineTilt, HfSmooth, RoughnessIndex);
}

double CalcDOE2Leeward(double SurfaceTemp, double AirTemp, double CosineTilt, double WindAtZ, int RoughnessIndex)
{
    double HfSmooth = CalcMoWITTForcedLeeward(WindAtZ);
    return CalcDOE2Forced(SurfaceTemp, AirTemp, CosineTilt, HfSmooth, RoughnessIndex);
}

double CalcSparrowWindward(int RoughnessIndex, double FacePerimeter, double FaceArea, double WindAtZ)
{
    return 2.537 * RoughnessMultiplier[RoughnessIndex] * std::sqrt(FacePerimeter * WindAtZ / FaceArea);
}
