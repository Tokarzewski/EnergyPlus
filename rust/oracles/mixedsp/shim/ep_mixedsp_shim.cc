// Written by hand: SetPointManager::SPMMixedAir::calculate reads the reference, fan and cooling
// coil nodes and the outdoor temperature, which gen_plain.py cannot set up. Plain extern "C" entry
// point over the real EnergyPlus SetPointManager code.
//
// Each call sets up one SetpointManager:MixedAir: the fan's inlet and outlet nodes at fanin and
// fanout (C) and the reference node with set point refsp and temperature reft (C). With layout 0
// it names no cooling coil; otherwise the coil's inlet and outlet nodes are at coilin and coilout
// (C) with minimum coil outlet temperature mincoil (C), and the reference node is its own node
// (layout 1 and 2) or the coil's outlet node (layout 3, which then has refsp and coilout). The
// outdoor dry bulb is oat (C). The missing set point check has already run. One EnergyPlusData for
// the process, built when the library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 1, layout outside 0 to 3; any temperature
// that is not finite or lies outside -60 to 80 C.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataEnvironment.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataLoopNode.hh>
#include <EnergyPlus/OutputProcessor.hh> // before SetPointManager.hh, which uses OutputProcessor::VariableType
#include <EnergyPlus/SetPointManager.hh>

using namespace EnergyPlus;

namespace {
EnergyPlus::EnergyPlusData &st()
{
    static EnergyPlus::EnergyPlusData *s = [] {
        auto *p = new EnergyPlus::EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        p->dataLoopNodes->Node.allocate(5);
        p->dataLoopNodes->NodeID.allocate(5);
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlus::EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}
} // namespace

extern "C" {
// One result: the mixed air temperature set point (0, C), or whether the freeze check is on (1).
double ep_MixedSP(int which, int layout, double refsp, double reft, double fanin, double fanout, double coilin, double coilout, double mincoil, double oat)
{
    double const ts[8] = {refsp, reft, fanin, fanout, coilin, coilout, mincoil, oat};
    if (which < 0 || which > 1 || layout < 0 || layout > 3) {
        return NAN;
    }
    for (double t : ts) {
        if (!within(t, -60.0, 80.0)) {
            return NAN;
        }
    }
    EnergyPlus::EnergyPlusData &state = st();
    auto &nodes = state.dataLoopNodes->Node;
    nodes(1).Temp = fanin;
    nodes(2).Temp = fanout;
    nodes(3).TempSetPoint = refsp;
    nodes(3).Temp = reft;
    nodes(4).Temp = coilin;
    nodes(5).Temp = coilout;
    nodes(5).TempSetPoint = layout == 3 ? refsp : 0.0;
    state.dataEnvrn->OutDryBulbTemp = oat;
    SetPointManager::SPMMixedAir spm;
    spm.fanInNodeNum = 1;
    spm.fanOutNodeNum = 2;
    spm.refNodeNum = layout == 3 ? 5 : 3;
    spm.coolCoilInNodeNum = layout == 0 ? 0 : 4;
    spm.coolCoilOutNodeNum = layout == 0 ? 0 : 5;
    spm.minCoolCoilOutTemp = mincoil;
    spm.mySetPointCheckFlag = false;
    spm.calculate(state);
    return which == 0 ? spm.setPt : (spm.freezeCheckEnable ? 1.0 : 0.0);
}
}
