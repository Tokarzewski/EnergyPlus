// Written by hand: GLHEVert's g-function code works on boreholes and response factors, which
// gen_plain.py cannot build. Plain extern "C" entry points over the real EnergyPlus GLHEVert
// code, on a rectangular field of identical boreholes `spacing` apart, built as
// GLHEResponseFactors builds one. One EnergyPlusData for the process, built when the library
// loads (with console output off, so the g-function progress messages stay off stdout).
#include <cmath>
#include <memory>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/GroundHeatExchangers/BoreholeSingle.hh>
#include <EnergyPlus/GroundHeatExchangers/Properties.hh>
#include <EnergyPlus/GroundHeatExchangers/ResponseFactors.hh>
#include <EnergyPlus/GroundHeatExchangers/Vertical.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::GroundHeatExchangers;

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

// The number of LNTTS values setupTimeVectors makes, counted as it counts them; 0 when it would
// make more than 1000 (or never stop, for a zero or negative time scale).
int num_lntts(double bhLength, double diffusivity, double maxSimYears)
{
    double const t_s = (bhLength * bhLength) / (9 * diffusivity);
    double lntts = -8.5;
    int n = 1;
    while (std::exp(lntts) * t_s < maxSimYears * 365 * Constant::rHoursInDay * Constant::rSecsInHour) {
        lntts += 0.5;
        if (++n > 1000) {
            return 0;
        }
    }
    return n;
}

// A GLHEVert whose response factors hold the boreholes `add` places.
struct Field
{
    GLHEVert v;
    std::shared_ptr<GLHEResponseFactors> rf = std::make_shared<GLHEResponseFactors>();
    std::shared_ptr<GLHEVertProps> props = std::make_shared<GLHEVertProps>();

    Field(double bhLength, double bhTopDepth, double bhDiameter, double diffusivity)
    {
        props->bhLength = bhLength;
        props->bhTopDepth = bhTopDepth;
        props->bhDiameter = bhDiameter;
        v.bhLength = bhLength;
        v.soil.diffusivity = diffusivity;
        rf->props = props;
        v.myRespFactors = rf;
    }

    std::shared_ptr<GLHEVertSingle> add(double x, double y)
    {
        auto bh = std::make_shared<GLHEVertSingle>();
        bh->xLoc = x;
        bh->yLoc = y;
        bh->props = props;
        rf->myBorholes.push_back(bh);
        return bh;
    }

    // As GLHEResponseFactors and the GLHEVert constructor finish a field.
    void finish()
    {
        rf->numBoreholes = rf->myBorholes.size();
        rf->SetupBHPointsForResponseFactorsObject();
        v.totalTubeLength = rf->numBoreholes * props->bhLength;
    }
};
} // namespace

extern "C" {
double ep_VertDistance(double xi, double yi, double zi, double xj, double yj, double zj, int which)
{
    if (which != 0 && which != 1) {
        return NAN;
    }
    return GLHEVert::distances({xi, yi, zi}, {xj, yj, zj})[which];
}

double ep_VertResponse(double d0, double d1, double diffusivity, double currTime)
{
    GLHEVert v;
    v.soil.diffusivity = diffusivity;
    return v.calcResponse({d0, d1}, currTime);
}

double ep_VertIntegral(double xi, double yi, double zi, double xj, double yj, double bhLength, double bhTopDepth, double bhDiameter,
                       double diffusivity, double currTime)
{
    Field f(bhLength, bhTopDepth, bhDiameter, diffusivity);
    auto bh_j = f.add(xj, yj);
    f.finish();
    return f.v.integral({xi, yi, zi}, bh_j, currTime);
}

double ep_VertDoubleIntegral(double xi, double yi, double xj, double yj, int same, double bhLength, double bhTopDepth, double bhDiameter,
                             double diffusivity, double currTime)
{
    Field f(bhLength, bhTopDepth, bhDiameter, diffusivity);
    auto bh_i = f.add(xi, yi);
    auto bh_j = same != 0 ? bh_i : f.add(xj, yj);
    f.finish();
    return f.v.doubleIntegral(bh_i, bh_j, currTime);
}

int ep_VertNumLntts(double bhLength, double diffusivity, double maxSimYears)
{
    return num_lntts(bhLength, diffusivity, maxSimYears);
}

double ep_VertLntts(double bhLength, double diffusivity, double maxSimYears, int k)
{
    if (k < 0 || k >= num_lntts(bhLength, diffusivity, maxSimYears)) {
        return NAN;
    }
    Field f(bhLength, 0.0, 0.0, diffusivity);
    f.rf->maxSimYears = maxSimYears;
    f.v.setupTimeVectors();
    return f.rf->LNTTS[k];
}

double ep_VertGFunction(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double diffusivity,
                        double maxSimYears, int k)
{
    if (nx < 1 || nx > 3 || ny < 1 || ny > 3 || k < 0 || k >= num_lntts(bhLength, diffusivity, maxSimYears)) {
        return NAN;
    }
    Field f(bhLength, bhTopDepth, bhDiameter, diffusivity);
    for (int iy = 0; iy < ny; ++iy) {
        for (int ix = 0; ix < nx; ++ix) {
            f.add(ix * spacing, iy * spacing);
        }
    }
    f.finish();
    f.rf->maxSimYears = maxSimYears;
    f.v.setupTimeVectors();
    f.v.calcUniformHeatFluxGFunctions(st());
    return f.rf->GFNC[k];
}

double ep_VertFrictionFactor(double reynoldsNum)
{
    return GLHEVert::frictionFactor(reynoldsNum);
}

double ep_VertPipeConductionResistance(double outDia, double innerDia, double k)
{
    GLHEVert v;
    v.pipe.outDia = outDia;
    v.pipe.innerDia = innerDia;
    v.pipe.k = k;
    return v.calcPipeConductionResistance();
}

double ep_VertTimeSS(double bhLength, double diffusivity)
{
    GLHEVert v;
    v.bhLength = bhLength;
    v.soil.diffusivity = diffusivity;
    v.getAnnualTimeConstant();
    return v.timeSS;
}
}
