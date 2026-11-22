// Written by hand: GLHEVert::calcGFunctions needs a borehole field, the plant loop's fluid and
// the inlet node, which gen_plain.py cannot set up. Plain extern "C" entry points over the real
// EnergyPlus GLHEVert code. One EnergyPlusData for the process, built when the library loads: the
// built-in fluids initialised as init_constant_state does, one plant loop of water and one inlet
// node (as the Slinky unit test sets them up), WarmupFlag set so the out-of-range fluid warnings
// stay silent, console output off (calcUniformHeatFluxGFunctions prints its progress).
//
// The field is nx by ny identical boreholes `spacing` apart, built as GLHEResponseFactors and the
// GLHEVert constructor build one (see the vert and vertfluid shims), with the uniform heat flux
// method for the long time steps. The domain (ADR-009): 1 to 2 boreholes each way, at most 20000
// short time steps and 60 long ones, and combined vectors free of NaN of at least two values
// (interpGFunc's std::upper_bound needs a sorted range and indexes the two ends).
#include <cmath>
#include <memory>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/FluidProperties.hh>
#include <EnergyPlus/GroundHeatExchangers/BoreholeSingle.hh>
#include <EnergyPlus/GroundHeatExchangers/Properties.hh>
#include <EnergyPlus/GroundHeatExchangers/ResponseFactors.hh>
#include <EnergyPlus/GroundHeatExchangers/Vertical.hh>
#include <EnergyPlus/Plant/DataPlant.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::GroundHeatExchangers;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        Fluid::InitConstantFluidPropertiesData(*p);
        p->dataGlobal->WarmupFlag = true;
        p->dataPlnt->PlantLoop.allocate(1);
        p->dataPlnt->PlantLoop(1).FluidName = "WATER";
        p->dataPlnt->PlantLoop(1).glycol = Fluid::GetWater(*p);
        p->dataLoopNodes->Node.allocate(1);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

constexpr int max_short_steps = 20000;
constexpr int max_long = 60;

// The steps calcShortTimestepGFunctions takes, counted as it counts them; 0 past the cap.
int short_steps(double t_s)
{
    double const t_max = std::exp(-8.6) * t_s;
    double total = 0.0;
    int n = 0;
    while (total < t_max) {
        total += 120;
        if (++n > max_short_steps) {
            return 0;
        }
    }
    return n;
}

// The LNTTS values setupTimeVectors makes, counted as it counts them; 0 past the cap.
int long_count(double t_s, double maxSimYears)
{
    double lntts = -8.5;
    int n = 1;
    while (std::exp(lntts) * t_s < maxSimYears * 365 * Constant::rHoursInDay * Constant::rSecsInHour) {
        lntts += 0.5;
        if (++n > max_long) {
            return 0;
        }
    }
    return n;
}

struct Field
{
    GLHEVert v;
    std::shared_ptr<GLHEResponseFactors> rf = std::make_shared<GLHEResponseFactors>();
    std::shared_ptr<GLHEVertProps> props = std::make_shared<GLHEVertProps>();
    bool ok = false;

    Field(int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK,
          double groutRhoCp, double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow,
          double inletTemp, double maxSimYears)
    {
        double const diffusivity = soilK / soilRhoCp;
        double const t_s = (bhLength * bhLength) / (9 * diffusivity);
        if (nx < 1 || nx > 2 || ny < 1 || ny > 2 || short_steps(t_s) == 0 || long_count(t_s, maxSimYears) == 0) {
            return;
        }
        props->bhLength = bhLength;
        props->bhTopDepth = bhTopDepth;
        props->bhDiameter = bhDiameter;
        rf->props = props;
        for (int iy = 0; iy < ny; ++iy) {
            for (int ix = 0; ix < nx; ++ix) {
                auto bh = std::make_shared<GLHEVertSingle>();
                bh->xLoc = ix * spacing;
                bh->yLoc = iy * spacing;
                bh->props = props;
                rf->myBorholes.push_back(bh);
            }
        }
        rf->numBoreholes = rf->myBorholes.size();
        rf->SetupBHPointsForResponseFactorsObject();
        rf->maxSimYears = maxSimYears;
        v.myRespFactors = rf;
        v.gFuncCalcMethod = GFuncCalcMethod::UniformHeatFlux;

        v.plantLoc.loopNum = 1;
        v.inletNodeNum = 1;
        st().dataLoopNodes->Node(1).Temp = inletTemp;
        v.inletTemp = inletTemp;
        v.designMassFlow = massFlow;
        v.massFlowRate = massFlow;

        v.bhDiameter = bhDiameter;
        v.bhRadius = v.bhDiameter / 2.0;
        v.bhLength = bhLength;
        v.bhUTubeDist = uTubeDist;
        v.pipe.outDia = outDia;
        v.pipe.thickness = thickness;
        v.pipe.innerDia = outDia - 2 * thickness;
        v.pipe.outRadius = v.pipe.outDia / 2;
        v.pipe.innerRadius = v.pipe.innerDia / 2;
        v.pipe.k = kPipe;
        v.pipe.rhoCp = pipeRhoCp;
        v.grout.k = groutK;
        v.grout.rhoCp = groutRhoCp;
        v.soil.k = soilK;
        v.soil.rhoCp = soilRhoCp;
        rf->gRefRatio = v.bhRadius / v.bhLength;
        v.totalTubeLength = rf->numBoreholes * props->bhLength;
        v.soil.diffusivity = v.soil.k / v.soil.rhoCp;
        v.theta_1 = v.bhUTubeDist / (2 * v.bhRadius);
        v.theta_2 = v.bhRadius / v.pipe.outRadius;
        v.theta_3 = 1 / (2 * v.theta_1 * v.theta_2);
        v.sigma = (v.grout.k - v.soil.k) / (v.grout.k + v.soil.k);

        v.calcGFunctions(st());
        auto const &x = rf->LNTTS;
        ok = x.size() >= 2;
        for (auto const val : x) {
            ok = ok && !std::isnan(val);
        }
    }
};
} // namespace

#define FIELD_PARAMS                                                                                                                         \
    int nx, int ny, double spacing, double bhLength, double bhTopDepth, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, \
        double soilK, double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, double inletTemp,    \
        double maxSimYears
#define FIELD_ARGS                                                                                                                           \
    nx, ny, spacing, bhLength, bhTopDepth, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, \
        massFlow, inletTemp, maxSimYears

extern "C" {
int ep_VGCombinedLen(FIELD_PARAMS)
{
    Field f(FIELD_ARGS);
    return f.ok ? static_cast<int>(f.rf->LNTTS.size()) : 0;
}

double ep_VGCombinedLntts(int k, FIELD_PARAMS)
{
    Field f(FIELD_ARGS);
    return f.ok && k >= 0 && k < static_cast<int>(f.rf->LNTTS.size()) ? f.rf->LNTTS[k] : NAN;
}

double ep_VGCombinedGFunction(int k, FIELD_PARAMS)
{
    Field f(FIELD_ARGS);
    return f.ok && k >= 0 && k < static_cast<int>(f.rf->GFNC.size()) ? f.rf->GFNC[k] : NAN;
}

// getGFunc at `time`, with the response factors' reference ratio replaced by gRefRatio when it is
// positive (as a g-function set made for another borehole radius would have it).
double ep_VGGFunc(double time, double gRefRatio, FIELD_PARAMS)
{
    Field f(FIELD_ARGS);
    if (!f.ok) {
        return NAN;
    }
    if (gRefRatio > 0) {
        f.rf->gRefRatio = gRefRatio;
    }
    return f.v.getGFunc(time);
}
}
