// Written by hand: GLHEBase::calcGroundHeatExchanger is a time-stepping model that reads the
// simulation clock, the plant loop's fluid and the inlet node, and keeps its load history between
// calls. Plain extern "C" entry points that run it for a number of time steps on the real
// EnergyPlus code. One EnergyPlusData for the process, built when the library loads: the built-in
// fluids initialised as init_constant_state does, one plant loop of water and one inlet node (as
// the Slinky unit test sets them up), WarmupFlag set (so the out-of-range fluid warnings stay
// silent and the design-day reset never fires), console output off.
//
// The borehole is set up as the GLHEVert constructor sets it up (aggregation sizes 15 and 192,
// the history arrays for one simulation year). Its g-function curve is given rather than
// computed (units 14 to 16 port that): LNTTS -15 to 3 in steps of 0.5, g_k = gA k + gB k^2. Step s
// (from 0) is time step s % tsPerHour + 1 of hour s / tsPerHour; the inlet temperature is
// tIn0 + tInAmp ((7 s) % 13 - 6) / 6 and the flow is flow0, or 0 every offEvery-th step.
//
// The domain (ADR-009): 1 to 6000 steps, 1 to 60 time steps an hour (the history arrays hold an
// hour of 60), at least one borehole.
#include <cmath>
#include <memory>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHVACGlobals.hh>
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
        p->dataHVACGlobal->SysTimeElapsed = 0.0;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();
} // namespace

extern "C" {
// which: 0 outlet temperature, 1 average fluid temperature, 2 borehole wall temperature, 3 heat
// transfer rate QGLHE, 4 HX resistance, after the last step.
double ep_GSRun(int which, int steps, int tsPerHour, double tGround, double tIn0, double tInAmp, double flow0, int offEvery, double gA,
                double gB, double gRefRatio, double bhLength, double bhDiameter, double uTubeDist, double groutK, double soilK, double soilRhoCp,
                double outDia, double thickness, double kPipe, int numBH)
{
    if (which < 0 || which > 4 || steps < 1 || steps > 6000 || tsPerHour < 1 || tsPerHour > 60 || numBH < 1) {
        return NAN;
    }
    auto &state = st();
    GLHEVert v;
    auto rf = std::make_shared<GLHEResponseFactors>();
    v.myRespFactors = rf;
    rf->numBoreholes = numBH;
    rf->maxSimYears = 1;
    for (int k = 0; k <= 36; ++k) {
        rf->LNTTS.push_back(-15 + 0.5 * k);
        rf->GFNC.push_back(gA * k + gB * k * k);
    }
    v.gFunctionsExist = true;

    v.plantLoc.loopNum = 1;
    v.inletNodeNum = 1;
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
    v.grout.k = groutK;
    v.soil.k = soilK;
    v.soil.rhoCp = soilRhoCp;
    rf->gRefRatio = gRefRatio > 0 ? gRefRatio : v.bhRadius / v.bhLength;
    v.totalTubeLength = rf->numBoreholes * bhLength;
    v.soil.diffusivity = v.soil.k / v.soil.rhoCp;
    v.theta_1 = v.bhUTubeDist / (2 * v.bhRadius);
    v.theta_2 = v.bhRadius / v.pipe.outRadius;
    v.theta_3 = 1 / (2 * v.theta_1 * v.theta_2);
    v.sigma = (v.grout.k - v.soil.k) / (v.grout.k + v.soil.k);
    v.SubAGG = 15;
    v.AGG = 192;
    v.QnMonthlyAgg.dimension(static_cast<int>(rf->maxSimYears * 12), 0.0);
    v.QnHr.dimension(730 + v.AGG + v.SubAGG, 0.0);
    v.QnSubHr.dimension(static_cast<int>((v.SubAGG + 1) * maxTSinHr + 1), 0.0);
    v.LastHourN.dimension(v.SubAGG + 1, 0);
    v.prevTimeSteps.allocate(static_cast<int>((v.SubAGG + 1) * maxTSinHr + 1));
    v.prevTimeSteps = 0.0;
    v.tempGround = tGround;

    state.dataGlobal->TimeStepZone = 1.0 / tsPerHour;
    for (int s = 0; s < steps; ++s) {
        int const hour = s / tsPerHour;
        state.dataGlobal->TimeStep = s % tsPerHour + 1;
        state.dataGlobal->HourOfDay = hour % 24 + 1;
        state.dataGlobal->DayOfSim = hour / 24 + 1;
        state.dataLoopNodes->Node(1).Temp = tIn0 + tInAmp * ((7 * s) % 13 - 6) / 6.0;
        v.massFlowRate = offEvery > 0 && s % offEvery == offEvery - 1 ? 0.0 : flow0;
        v.calcGroundHeatExchanger(state);
    }
    switch (which) {
    case 0:
        return v.outletTemp;
    case 1:
        return v.aveFluidTemp;
    case 2:
        return v.bhTemp;
    case 3:
        return v.QGLHE;
    default:
        return v.HXResistance;
    }
}
}
