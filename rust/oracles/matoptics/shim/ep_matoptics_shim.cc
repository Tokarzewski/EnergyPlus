// Written by hand: these Material routines take screen and glass material objects, a Vector3D and
// return several results through references, which gen_plain.py cannot build. Plain extern "C"
// entry points over the real EnergyPlus Material code.
//
// Guards keep the routines in the inputs they are defined for: CalcScreenTransmittance asserts
// phi and theta in [0, pi]; NormalizePhiTheta loops once per 2 pi, so angles past 1000 rad are
// refused; the index helpers convert to int, so values whose index would not fit an int are
// refused. Refused inputs return NaN (ADR-009). One EnergyPlusData for the process, built when the
// library loads, with console output off. ShowFatalError (a simple glazing with a conductivity of
// 0 or less) throws, and the shim returns NaN for it.
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/EnergyPlus.hh>
#include <EnergyPlus/Geometry/Vector3D.hh>
#include <EnergyPlus/Material.hh>

using namespace EnergyPlus;
using namespace EnergyPlus::Material;

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

bool fits(double x)
{
    return std::abs(x) < 2.0e9;
}
} // namespace

extern "C" {
// CalcScreenTransmittance for a screen (model 0 DoNotModel, 1 DirectBeam, 2 Diffuse) with
// diameter-to-spacing ratio gamma and cylinder reflectances refl and reflVis, at relative sun
// angles phi and theta (radians, 0 to pi): which picks BmTrans (0), BmTransBack, BmTransVis,
// DfTrans, DfTransBack, DfTransVis, RefSolFront, RefVisFront, RefSolBack, RefVisBack, AbsSolFront
// or AbsSolBack (11).
double ep_ScreenTrans(int which, int model, double gamma, double refl, double reflVis, double phi, double theta)
{
    if (model < 0 || model > 2 || !(phi >= 0.0 && phi <= Constant::Pi) || !(theta >= 0.0 && theta <= Constant::Pi)) {
        return NAN;
    }
    MaterialScreen screen;
    screen.diameterToSpacingRatio = gamma;
    screen.CylinderRef = refl;
    screen.CylinderRefVis = reflVis;
    screen.bmRefModel = static_cast<ScreenBeamReflectanceModel>(model);
    ScreenBmTransAbsRef t;
    CalcScreenTransmittance(st(), &screen, phi, theta, t);
    double const r[12] = {t.BmTrans,     t.BmTransBack, t.BmTransVis, t.DfTrans,    t.DfTransBack, t.DfTransVis,
                          t.RefSolFront, t.RefVisFront, t.RefSolBack, t.RefVisBack, t.AbsSolFront, t.AbsSolBack};
    return which >= 0 && which < 12 ? r[which] : NAN;
}

// GetRelativePhiTheta for a window at phiWin, thetaWin and sun direction (sx, sy, sz): phi (0) or
// theta (1).
double ep_RelPhiTheta(int which, double phiWin, double thetaWin, double sx, double sy, double sz)
{
    if (!(std::abs(phiWin) <= 1000.0) || !(std::abs(thetaWin) <= 1000.0)) {
        return NAN;
    }
    Real64 phi = 0.0, theta = 0.0;
    GetRelativePhiTheta(phiWin, thetaWin, Vector3D(sx, sy, sz), phi, theta);
    return which == 0 ? phi : which == 1 ? theta : NAN;
}

// NormalizePhiTheta: phi (0) or theta (1).
double ep_NormPhiTheta(int which, double phi, double theta)
{
    if (!(std::abs(phi) <= 1000.0) || !(std::abs(theta) <= 1000.0)) {
        return NAN;
    }
    NormalizePhiTheta(phi, theta);
    return which == 0 ? phi : which == 1 ? theta : NAN;
}

// GetPhiThetaIndices: iPhi1 (0), iPhi2 (1), iTheta1 (2) or iTheta2 (3).
double ep_PhiThetaIdx(int which, double phi, double theta, double dPhi, double dTheta)
{
    if (!fits(phi / dPhi) || !fits(theta / dTheta)) {
        return NAN;
    }
    int r[4] = {0, 0, 0, 0};
    GetPhiThetaIndices(phi, theta, dPhi, dTheta, r[0], r[1], r[2], r[3]);
    return which >= 0 && which < 4 ? r[which] : NAN;
}

// GetProfIndices: idxLo (0) or idxHi (1).
double ep_ProfIdx(int which, double profAng)
{
    if (!fits((profAng + Constant::PiOvr2) / dProfAng)) {
        return NAN;
    }
    int lo = 0, hi = 0;
    GetProfIndices(profAng, lo, hi);
    return which == 0 ? lo : which == 1 ? hi : NAN;
}

// GetSlatIndicesInterpFac: idxLo (0), idxHi (1) or interpFac (2).
double ep_SlatIdx(int which, double slatAng)
{
    if (!fits(slatAng / dSlatAng)) {
        return NAN;
    }
    int lo = 0, hi = 0;
    Real64 f = 0.0;
    GetSlatIndicesInterpFac(slatAng, lo, hi, f);
    return which == 0 ? lo : which == 1 ? hi : which == 2 ? f : NAN;
}

// MaterialGlass::SetupSimpleWindowGlazingSystem for U-factor u and SHGC shgc (and the visible
// transmittance vt when vtByUser is 1): which picks Trans (0), ReflectSolBeamBack,
// ReflectSolBeamFront, TransVis, ReflectVisBeamBack, ReflectVisBeamFront, Thickness, Conductivity
// or Resistance (8).
double ep_SimpleGlazing(int which, double u, double shgc, int vtByUser, double vt)
{
    MaterialGlass g;
    g.Name = "SIMPLE";
    g.SimpleWindowUfactor = u;
    g.SimpleWindowSHGC = shgc;
    g.SimpleWindowVTinputByUser = vtByUser != 0;
    g.SimpleWindowVisTran = vt;
    try {
        g.SetupSimpleWindowGlazingSystem(st());
    } catch (FatalError const &) {
        return NAN;
    }
    double const r[9] = {g.Trans,
                         g.ReflectSolBeamBack,
                         g.ReflectSolBeamFront,
                         g.TransVis,
                         g.ReflectVisBeamBack,
                         g.ReflectVisBeamFront,
                         g.Thickness,
                         g.Conductivity,
                         g.Resistance};
    return which >= 0 && which < 9 ? r[which] : NAN;
}
}
