// Written by hand: HeatBalanceIntRadExchange::CalcInteriorRadExchange reads a radiant enclosure (its
// surfaces, areas, view factors, ScriptF or Carroll factors), the surfaces' constructions, window
// states and inside absorptances, which gen_plain.py cannot set up. Plain extern "C" entry point over
// the real EnergyPlus HeatBalanceIntRadExchange code, on the box zone of rust/oracles/intrad.
//
// The enclosure (as EnergyPlus orders its surfaces): the surfaces whose bit is set in `mask`, from
// bit 0: floor (L by W, tilt 180), roof (L by W, tilt 0), four walls (L by H facing rot, W by H
// facing rot + 90, L by H facing rot + 180, W by H facing rot + 270, tilt 90), and an internal mass
// of area `mass` (azimuth rot, tilt 90). The k-th included surface (k from 1) has inside thermal
// absorptance e0 + de (k - 1) and inside face temperature t0 + dt (k - 1) (C). Surface `win` (0 for
// none) is a bare single-glazed window whose inside glass face was twin (C) at the end of the last
// time step. As InitInteriorRadExchange does, the view factors come from CalcApproximateViewFactors
// and FixViewFactors, and for the Carroll method (carroll 1) the factors from CalcFMRT.
//
// Each call runs CalcInteriorRadExchange at the start of an environment for the first iteration of
// the inside heat balance (iteration 0), which sets ScriptF or the Carroll factors, and with iter 1
// once more for a later iteration, with every inside face temperature raised by 0.5 k (C). It gives
// the net long-wave radiation to surface i (which 0, W/m2) or the long-wave radiation from the zone
// incident on it (which 1, W/m2, windows only). One EnergyPlusData for the process, built when the
// library loads, with console output off.
//
// Guards (ADR-009), which return NaN: which outside 0 to 1, carroll or iter not 0 or 1, mask outside
// 1 to 127, i outside 1 to the number of surfaces, win outside 0 to it; any input that is not finite
// or lies outside what EnergyPlus meets: L, W or H outside 0.5 to 50 m, mass outside 0 to 500 m2
// (and 0.5 or more when the mass is included), rot outside 0 to 360, any absorptance outside 0.05
// to 0.99, t0 or twin outside -30 to 80 C, dt outside -10 to 10 C. FixViewFactors' ShowFatalError
// throws, and the shim returns NaN for it.
#include <cmath>
#include <ObjexxFCL/Array1D.hh>
#include <ObjexxFCL/Array2D.hh>
#include <EnergyPlus/Construction.hh>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobalConstants.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/DataHeatBalSurface.hh>
#include <EnergyPlus/DataHeatBalance.hh>
#include <EnergyPlus/DataSurfaces.hh>
#include <EnergyPlus/DataViewFactorInformation.hh>
#include <EnergyPlus/EnergyPlus.hh>
#include <EnergyPlus/HeatBalanceIntRadExchange.hh>
#include <EnergyPlus/UtilityRoutines.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::HeatBalanceIntRadExchange;
using DataSurfaces::SurfaceClass;

namespace {
int constexpr MaxSurf = 7;

EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto &sd = *p->dataSurface;
        sd.Surface.allocate(MaxSurf);
        sd.SurfaceWindow.allocate(MaxSurf);
        sd.SurfWinIRfromParentZone.dimension(MaxSurf, 0.0);
        sd.SurfWinShadingFlag.allocate(MaxSurf);
        sd.SurfWinExtIntShadePrevTS.allocate(MaxSurf);
        sd.SurfWinWindowModelType.dimension(MaxSurf, DataSurfaces::WindowModel::Detailed);
        p->dataConstruction->Construct.allocate(MaxSurf);
        p->dataHeatBalSurf->SurfAbsThermalInt.dimension(MaxSurf, 0.0);
        p->dataHeatBal->Zone.allocate(1);
        p->dataHeatBal->Zone(1).Name = "BOX";
        p->dataViewFactor->NumOfRadiantEnclosures = 1;
        p->dataViewFactor->EnclRadInfo.allocate(1);
        p->dataHeatBalIntRadExchg->MaxNumOfRadEnclosureSurfs = MaxSurf;
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}
} // namespace

extern "C" {
// One result (see the file comment) for surface i.
double ep_IRExchange(int which,
                     int i,
                     int mask,
                     double L,
                     double W,
                     double H,
                     double mass,
                     double rot,
                     double e0,
                     double de,
                     int carroll,
                     int win,
                     double t0,
                     double dt,
                     double twin,
                     int iter)
{
    if (which < 0 || which > 1 || (carroll != 0 && carroll != 1) || (iter != 0 && iter != 1) || mask < 1 || mask > 127 ||
        !within(L, 0.5, 50.0) || !within(W, 0.5, 50.0) || !within(H, 0.5, 50.0) || !within(mass, 0.0, 500.0) ||
        (((mask >> 6) & 1) == 1 && mass < 0.5) || !within(rot, 0.0, 360.0) || !within(e0, 0.05, 0.99) || !std::isfinite(de) ||
        !within(t0, -30.0, 80.0) || !within(dt, -10.0, 10.0) || !within(twin, -30.0, 80.0)) {
        return NAN;
    }
    int n = 0;
    for (int k = 0; k < MaxSurf; ++k) {
        n += (mask >> k) & 1;
    }
    if (i < 1 || i > n || win < 0 || win > n || !within(e0 + de * (n - 1), 0.05, 0.99)) {
        return NAN;
    }
    EnergyPlusData &state = st();
    double const a[MaxSurf] = {L * W, L * W, L * H, W * H, L * H, W * H, mass};
    double const az[MaxSurf] = {rot, rot, rot, rot + 90, rot + 180, rot + 270, rot};
    double const tl[MaxSurf] = {180, 0, 90, 90, 90, 90, 90};
    SurfaceClass const cls[MaxSurf] = {
        SurfaceClass::Floor, SurfaceClass::Roof, SurfaceClass::Wall, SurfaceClass::Wall, SurfaceClass::Wall, SurfaceClass::Wall, SurfaceClass::IntMass};
    auto &encl = state.dataViewFactor->EnclRadInfo(1);
    encl.NumOfSurfaces = n;
    encl.Area.dimension(n, 0.0);
    encl.Emissivity.dimension(n, 0.0);
    encl.FMRT.dimension(n, 0.0);
    encl.Fp.dimension(n, 0.0);
    encl.SurfacePtr.dimension(n, 0);
    encl.F.dimension(n, n, 0.0);
    encl.ScriptF.dimension(n, n, 0.0);
    Array1D<Real64> azimuth(n), tilt(n);
    Array1D<Real64> surfaceTemp(MaxSurf, 0.0), netLW(MaxSurf, 0.0);
    int s = 0;
    for (int k = 0; k < MaxSurf; ++k) {
        if (((mask >> k) & 1) == 0) {
            continue;
        }
        ++s;
        auto &surf = state.dataSurface->Surface(s);
        surf.Class = cls[k];
        surf.Zone = 1;
        surf.Name = "S" + std::to_string(k);
        surf.Construction = s;
        auto &c = state.dataConstruction->Construct(s);
        c.TypeIsWindow = (s == win);
        c.TotGlassLayers = (s == win) ? 1 : 0;
        c.InsideAbsorpThermal = e0 + de * (s - 1);
        state.dataHeatBalSurf->SurfAbsThermalInt(s) = c.InsideAbsorpThermal;
        state.dataSurface->SurfaceWindow(s).thetaFace[2] = twin + Constant::Kelvin;
        encl.Area(s) = a[k];
        encl.SurfacePtr(s) = s;
        azimuth(s) = az[k];
        tilt(s) = tl[k];
        surfaceTemp(s) = t0 + dt * (s - 1);
    }
    state.dataSurface->TotSurfaces = n;
    state.dataHeatBalIntRadExchg->LargestSurf = 0;
    state.dataHeatBalIntRadExchg->CarrollMethod = (carroll == 1);
    CalcApproximateViewFactors(state, n, encl.Area, azimuth, tilt, encl.F, encl.SurfacePtr);
    std::string name = "BOX";
    std::vector<int> const spaces;
    Real64 original = 0.0, fixedCheck = 0.0, finalCheck = 0.0, rowSum = 0.0;
    int iterations = 0;
    try {
        FixViewFactors(state, n, encl.Area, encl.F, name, spaces, original, fixedCheck, finalCheck, iterations, rowSum, false);
    } catch (FatalError const &) {
        return NAN;
    }
    if (carroll == 1) {
        CalcFMRT(state, n, encl.Area, encl.FMRT);
    }
    state.dataGlobal->BeginEnvrnFlag = true;
    CalcInteriorRadExchange(state, surfaceTemp, 0, netLW, ObjexxFCL::Optional_int_const(), "Inside");
    state.dataGlobal->BeginEnvrnFlag = false;
    if (iter == 1) {
        for (int k = 1; k <= n; ++k) {
            surfaceTemp(k) += 0.5 * k;
        }
        CalcInteriorRadExchange(state, surfaceTemp, 1, netLW, ObjexxFCL::Optional_int_const(), "Inside");
    }
    return which == 0 ? netLW(i) : state.dataSurface->SurfWinIRfromParentZone(i);
}
}
