// Written by hand: Boilers::BoilerSpecs::CalcBoilerModel reads the boiler object (capacity,
// efficiency and its curve, part load limits, flow mode, outlet temperature limit, parasitics), its
// plant location (the loop's fluid and demand scheme, the loop side's flow lock, the branch for
// SetComponentFlowRate) and its inlet and outlet nodes, which gen_plain.py cannot set up. Plain
// extern "C" entry point over the real EnergyPlus Boilers code.
//
// Each call sets up one Boiler:HotWater as InitBoiler leaves it, alone on a branch of a water loop
// (node 1 inlet, node 2 outlet), with the loop side unlocked (lock 0) or locked (1), the flow mode
// constant (flowmode 0), not modulated (1) or leaving set point modulated (2), the loop's demand
// scheme single (dual 0) or dual set point dead band (1), the boiler's flow control active or series
// active (series 1), running or not (run). The efficiency curve is none (curve 0), a quadratic in
// the part load ratio (1) or a biquadratic in it and the water temperature (2), entering (tempmode
// 1) or leaving (2) or neither (0), with inputs limited to 0 to 1 and 5 to 95 C and coefficients c0
// to c5. The boiler is asked for load myload (W) with nominal capacity nomcap (W) and efficiency
// nomeff, part load limits minplr and maxplr, outlet temperature limit tlimit (C), design flow desmdot
// (kg/s), parasitic electric load parelec (W) and fuel capacity parfuel (W). The inlet node has
// temperature tin (C), mass flow mdotin, hardware maximum hwmax and available limits minav and maxav
// (kg/s); the outlet node has set points tset and tsetlo (C). The warm-up flag is on, so the curve
// warnings are skipped, and with them the resets of the curve output to 0.01 and 1.1. One
// EnergyPlusData for the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 10, lock, dual, series, run not 0 or 1,
// flowmode or curve or tempmode outside 0 to 2; any input that is not finite or lies outside what
// EnergyPlus meets: myload outside -1e8 to 1e8, nomcap outside 1 to 1e8, nomeff outside 0.1 to 1.2,
// minplr outside 0 to 1, maxplr outside minplr to 1.5, tlimit outside 20 to 120, desmdot, mdotin,
// hwmax, minav or maxav outside 0 to 1000, parelec or parfuel outside 0 to 1e5, tin outside 1 to 99
// (the water specific heat warns outside its table), tset or tsetlo outside -20 to 150, c0 to c5
// outside -5 to 5.
#include <cmath>
#include <EnergyPlus/Boilers.hh>
#include <EnergyPlus/CurveManager.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataBranchAirLoopPlant.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/FluidProperties.hh>
#include <EnergyPlus/Plant/DataPlant.hh>

using namespace EnergyPlus;

namespace {
DataPlant::PlantLoopData *loop = nullptr;
DataPlant::HalfLoopData *side = nullptr;
DataPlant::BranchData *branch = nullptr;
Curve::Curve *quad = nullptr;
Curve::Curve *biquad = nullptr;
Boilers::BoilerSpecs *boiler = nullptr;

EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataGlobal->WarmupFlag = true;
        Fluid::InitConstantFluidPropertiesData(*p);
        p->dataLoopNodes->Node.allocate(2);
        p->dataLoopNodes->NodeID.allocate(2);
        loop = new DataPlant::PlantLoopData;
        loop->glycol = Fluid::GetWater(*p);
        loop->MaxVolFlowRate = 0.01;
        side = new DataPlant::HalfLoopData;
        branch = new DataPlant::BranchData;
        branch->Comp.allocate(1);
        branch->TotalComponents = 1;
        quad = Curve::AddCurve(*p, "EFFQ");
        quad->curveType = Curve::CurveType::Quadratic;
        quad->numDims = 1;
        quad->inputLimits[0].min = 0.0;
        quad->inputLimits[0].max = 1.0;
        biquad = Curve::AddCurve(*p, "EFFBQ");
        biquad->curveType = Curve::CurveType::BiQuadratic;
        biquad->numDims = 2;
        biquad->inputLimits[0].min = 0.0;
        biquad->inputLimits[0].max = 1.0;
        biquad->inputLimits[1].min = 5.0;
        biquad->inputLimits[1].max = 95.0;
        boiler = new Boilers::BoilerSpecs;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

bool flag(int v)
{
    return v == 0 || v == 1;
}
} // namespace

extern "C" {
// One result: the boiler load (0, W), its mass flow (1, kg/s), outlet temperature (2, C), part load
// ratio (3), efficiency (4), fuel use (5, W), parasitic electric power (6, W) and fuel rate (7, W),
// the inlet and outlet nodes' mass flow (8 and 9, kg/s) and the outlet's maximum available (10).
double ep_Boiler(int which,
                 int lock,
                 int flowmode,
                 int dual,
                 int series,
                 int run,
                 int curve,
                 int tempmode,
                 double myload,
                 double nomcap,
                 double nomeff,
                 double minplr,
                 double maxplr,
                 double tlimit,
                 double desmdot,
                 double parelec,
                 double parfuel,
                 double tin,
                 double mdotin,
                 double hwmax,
                 double minav,
                 double maxav,
                 double tset,
                 double tsetlo,
                 double c0,
                 double c1,
                 double c2,
                 double c3,
                 double c4,
                 double c5)
{
    double const cs[6] = {c0, c1, c2, c3, c4, c5};
    if (which < 0 || which > 10 || !flag(lock) || !flag(dual) || !flag(series) || !flag(run) || flowmode < 0 || flowmode > 2 || curve < 0 ||
        curve > 2 || tempmode < 0 || tempmode > 2 || !within(myload, -1.0e8, 1.0e8) || !within(nomcap, 1.0, 1.0e8) ||
        !within(nomeff, 0.1, 1.2) || !within(minplr, 0.0, 1.0) || !within(maxplr, minplr, 1.5) || !within(tlimit, 20.0, 120.0) ||
        !within(desmdot, 0.0, 1000.0) || !within(parelec, 0.0, 1.0e5) || !within(parfuel, 0.0, 1.0e5) || !within(tin, 1.0, 99.0) ||
        !within(mdotin, 0.0, 1000.0) || !within(hwmax, 0.0, 1000.0) || !within(minav, 0.0, 1000.0) || !within(maxav, 0.0, 1000.0) ||
        !within(tset, -20.0, 150.0) || !within(tsetlo, -20.0, 150.0)) {
        return NAN;
    }
    for (double c : cs) {
        if (!within(c, -5.0, 5.0)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &in = state.dataLoopNodes->Node(1);
    in.Temp = tin;
    in.MassFlowRate = mdotin;
    in.MassFlowRateMin = 0.0;
    in.MassFlowRateMax = hwmax;
    in.MassFlowRateMinAvail = minav;
    in.MassFlowRateMaxAvail = maxav;
    in.MassFlowRateRequest = 0.0;
    auto &out = state.dataLoopNodes->Node(2);
    out = Node::NodeData();
    out.TempSetPoint = tset;
    out.TempSetPointLo = tsetlo;
    loop->LoopDemandCalcScheme = dual == 1 ? DataPlant::LoopDemandCalcScheme::DualSetPointDeadBand : DataPlant::LoopDemandCalcScheme::SingleSetPoint;
    side->FlowLock = lock == 1 ? DataPlant::FlowLock::Locked : DataPlant::FlowLock::Unlocked;
    auto const ctrl = series == 1 ? DataBranchAirLoopPlant::ControlType::SeriesActive : DataBranchAirLoopPlant::ControlType::Active;
    auto &comp = branch->Comp(1);
    comp.NodeNumIn = 1;
    comp.NodeNumOut = 2;
    comp.EMSLoadOverrideOn = false;
    comp.FlowCtrl = ctrl;
    comp.CurOpSchemeType = DataPlant::OpScheme::HeatingRB;
    for (int i = 0; i < 6; ++i) {
        quad->coeff[i] = cs[i];
        biquad->coeff[i] = cs[i];
    }
    auto &b = *boiler;
    b.BoilerInletNodeNum = 1;
    b.BoilerOutletNodeNum = 2;
    b.NomCap = nomcap;
    b.NomEffic = nomeff;
    b.MinPartLoadRat = minplr;
    b.MaxPartLoadRat = maxplr;
    b.TempUpLimitBoilerOut = tlimit;
    b.DesMassFlowRate = desmdot;
    b.ParasiticElecLoad = parelec;
    b.ParasiticFuelCapacity = parfuel;
    b.FlowMode = flowmode == 0 ? DataPlant::FlowMode::Constant
                 : flowmode == 1 ? DataPlant::FlowMode::NotModulated
                                 : DataPlant::FlowMode::LeavingSetpointModulated;
    b.EfficiencyCurve = curve == 0 ? nullptr : curve == 1 ? quad : biquad;
    b.CurveTempMode = static_cast<Boilers::TempMode>(tempmode);
    b.FaultyBoilerFoulingFlag = false;
    b.FuelUsed = 0.0;
    b.BoilerOutletTemp = 0.0;
    b.BoilerPLR = 0.0;
    b.BoilerEff = 0.0;
    b.ParasiticFuelRate = 0.0;
    b.plantLoc.loopNum = 1;
    b.plantLoc.loopSideNum = DataPlant::LoopSideLocation::Supply;
    b.plantLoc.branchNum = 1;
    b.plantLoc.compNum = 1;
    b.plantLoc.loop = loop;
    b.plantLoc.side = side;
    b.plantLoc.branch = branch;
    b.plantLoc.comp = &comp;
    b.CalcBoilerModel(state, myload, run == 1, ctrl);
    switch (which) {
    case 0:
        return b.BoilerLoad;
    case 1:
        return b.BoilerMassFlowRate;
    case 2:
        return b.BoilerOutletTemp;
    case 3:
        return b.BoilerPLR;
    case 4:
        return b.BoilerEff;
    case 5:
        return b.FuelUsed;
    case 6:
        return b.ParasiticElecPower;
    case 7:
        return b.ParasiticFuelRate;
    case 8:
        return in.MassFlowRate;
    case 9:
        return out.MassFlowRate;
    default:
        return out.MassFlowRateMaxAvail;
    }
}
}
