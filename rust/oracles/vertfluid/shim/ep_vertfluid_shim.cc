// Written by hand: GLHEVert's borehole resistances and short-time-step g-functions read the
// plant loop's fluid and the inlet node, which gen_plain.py cannot set up. Plain extern "C" entry
// points over the real EnergyPlus GLHEVert code. One EnergyPlusData for the process, built when
// the library loads: the built-in fluids initialised as init_constant_state does, one plant loop
// of water and one inlet node (as the Slinky unit test sets them up), WarmupFlag set so the
// out-of-range fluid warnings stay silent (they never change a result), console output off.
//
// The borehole is built as the GLHEVert constructor builds it from its response factors:
// innerDia = outDia - 2 * thickness, radii half the diameters, the multipole constants theta_1,
// theta_2, theta_3 and sigma, and the soil diffusivity k / rhoCp.
#include <cmath>
#include <memory>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/FluidProperties.hh>
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

// The most short time steps a case may take (each solves 535 cells).
constexpr int max_short_steps = 20000;

// A GLHEVert on the process's plant loop and inlet node, with `numBH` boreholes.
struct Bore
{
    GLHEVert v;
    std::shared_ptr<GLHEResponseFactors> rf = std::make_shared<GLHEResponseFactors>();

    Bore(double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp)
    {
        v.plantLoc.loopNum = 1;
        v.inletNodeNum = 1;
        st().dataLoopNodes->Node(1).Temp = inletTemp;
        v.inletTemp = inletTemp;
        v.pipe.outDia = outDia;
        v.pipe.thickness = thickness;
        v.pipe.innerDia = outDia - 2 * thickness;
        v.pipe.outRadius = v.pipe.outDia / 2;
        v.pipe.innerRadius = v.pipe.innerDia / 2;
        v.pipe.k = kPipe;
        v.massFlowRate = massFlow;
        v.designMassFlow = massFlow;
        rf->numBoreholes = numBH;
        v.myRespFactors = rf;
    }

    void borehole(double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK)
    {
        v.bhLength = bhLength;
        v.bhDiameter = bhDiameter;
        v.bhRadius = v.bhDiameter / 2.0;
        v.bhUTubeDist = uTubeDist;
        v.grout.k = groutK;
        v.soil.k = soilK;
        v.theta_1 = v.bhUTubeDist / (2 * v.bhRadius);
        v.theta_2 = v.bhRadius / v.pipe.outRadius;
        v.theta_3 = 1 / (2 * v.theta_1 * v.theta_2);
        v.sigma = (v.grout.k - v.soil.k) / (v.grout.k + v.soil.k);
    }
};

// The number of steps calcShortTimestepGFunctions takes, counted as it counts them; 0 when it
// would take more than max_short_steps (or never stop).
int short_steps(double bhLength, double diffusivity)
{
    double const t_s = (bhLength * bhLength) / (9.0 * diffusivity);
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

// Runs calcShortTimestepGFunctions and returns its k-th g-function (lntts false) or LNTTS (true).
double short_step(int k, bool lntts, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK,
                  double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH, double inletTemp)
{
    double const diffusivity = soilK / soilRhoCp;
    if (numBH < 1 || k < 0 || k >= short_steps(bhLength, diffusivity)) {
        return NAN;
    }
    Bore b(outDia, thickness, kPipe, massFlow, numBH, inletTemp);
    b.borehole(bhLength, bhDiameter, uTubeDist, groutK, soilK);
    b.v.grout.rhoCp = groutRhoCp;
    b.v.soil.rhoCp = soilRhoCp;
    b.v.soil.diffusivity = diffusivity;
    b.v.pipe.rhoCp = pipeRhoCp;
    b.v.calcShortTimestepGFunctions(st());
    return lntts ? b.v.LNTTS_shortTimestep[k] : b.v.GFNC_shortTimestep[k];
}
} // namespace

extern "C" {
double ep_VFConvectionResistance(double outDia, double thickness, double massFlow, int numBH, double inletTemp)
{
    if (numBH < 1) {
        return NAN;
    }
    Bore b(outDia, thickness, 0.0, massFlow, numBH, inletTemp);
    return b.v.calcPipeConvectionResistance(st());
}

double ep_VFPipeResistance(double outDia, double thickness, double kPipe, double massFlow, int numBH, double inletTemp)
{
    if (numBH < 1) {
        return NAN;
    }
    Bore b(outDia, thickness, kPipe, massFlow, numBH, inletTemp);
    return b.v.calcPipeResistance(st());
}

// which: 0 average, 1 total internal, 2 grout, 3 effective (calcHXResistance).
double ep_VFBoreholeResistance(int which, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double outDia,
                               double thickness, double kPipe, double massFlow, int numBH, double inletTemp)
{
    if (numBH < 1 || which < 0 || which > 3) {
        return NAN;
    }
    Bore b(outDia, thickness, kPipe, massFlow, numBH, inletTemp);
    b.borehole(bhLength, bhDiameter, uTubeDist, groutK, soilK);
    switch (which) {
    case 0:
        return b.v.calcBHAverageResistance(st());
    case 1:
        return b.v.calcBHTotalInternalResistance(st());
    case 2:
        return b.v.calcBHGroutResistance(st());
    default:
        return b.v.calcHXResistance(st());
    }
}

int ep_VFShortSteps(double bhLength, double soilK, double soilRhoCp)
{
    return short_steps(bhLength, soilK / soilRhoCp);
}

double ep_VFShortGFunction(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK,
                           double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH,
                           double inletTemp)
{
    return short_step(
        k, false, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp);
}

double ep_VFShortLntts(int k, double bhLength, double bhDiameter, double uTubeDist, double groutK, double groutRhoCp, double soilK,
                       double soilRhoCp, double outDia, double thickness, double kPipe, double pipeRhoCp, double massFlow, int numBH,
                       double inletTemp)
{
    return short_step(
        k, true, bhLength, bhDiameter, uTubeDist, groutK, groutRhoCp, soilK, soilRhoCp, outDia, thickness, kPipe, pipeRhoCp, massFlow, numBH, inletTemp);
}
}
