// Written by hand: BlindOpticsDiffuse and BlindOpticsBeam read the blind's slat geometry from a
// Material::MaterialBlind in EnergyPlusData and take 15- and 16-element property arrays, which
// gen_plain.py cannot build. Plain extern "C" entry points over the real EnergyPlus WindowManager
// code; blind material 1 is set up per call.
//
// c[1] and c[2] are the slat width and separation, as CalcWindowBlindProperties fills them; the
// slat properties c[3] to c[14] are passed in. One EnergyPlusData for the process, built when the
// library loads, with console output off.
#include <array>
#include <cmath>
#include <EnergyPlus/Data/EnergyPlusData.hh>
#include <EnergyPlus/DataGlobals.hh>
#include <EnergyPlus/Material.hh>
#include <EnergyPlus/WindowManager.hh>

using namespace EnergyPlus;

namespace {
EnergyPlusData &st()
{
    static EnergyPlusData *s = [] {
        auto *p = new EnergyPlusData;
        p->dataGlobal->printConsoleOutput = false;
        auto *b = new Material::MaterialBlind;
        b->Name = "BLIND";
        p->dataMaterial->materials.push_back(b);
        b->Num = p->dataMaterial->materials.isize();
        return p;
    }();
    return *s;
}
[[maybe_unused]] EnergyPlusData &loaded = st();

Material::MaterialBlind &blind(double width, double sep, double thick)
{
    auto *b = dynamic_cast<Material::MaterialBlind *>(st().dataMaterial->materials(1));
    b->SlatWidth = width;
    b->SlatSeparation = sep;
    b->SlatThickness = thick;
    return *b;
}
} // namespace

extern "C" {
// BlindOpticsDiffuse (isolvis 1: solar and IR, 2: visible) at slat angle b_el (radians) with slat
// properties c6..c14 (c3 to c5 are 0, as CalcWindowBlindProperties sets them): p[which].
double ep_BlindDiffuse(int which, int isolvis, double width, double sep, double thick, double b_el, double c6, double c7, double c8,
                       double c9, double c10, double c11, double c12, double c13, double c14)
{
    if (which < 0 || which > 15) {
        return NAN;
    }
    blind(width, sep, thick);
    std::array<Real64, 15> const c = {0.0, width, sep, 0.0, 0.0, 0.0, c6, c7, c8, c9, c10, c11, c12, c13, c14};
    std::array<Real64, 16> p{};
    Window::BlindOpticsDiffuse(st(), 1, isolvis, c, b_el, p);
    return p[which];
}

// BlindOpticsBeam at slat angle b_el and profile angle s_el (radians) with slat properties
// c3..c11: p[which].
double ep_BlindBeam(int which, double width, double sep, double thick, double b_el, double s_el, double c3, double c4, double c5, double c6,
                    double c7, double c8, double c9, double c10, double c11)
{
    if (which < 0 || which > 15) {
        return NAN;
    }
    blind(width, sep, thick);
    std::array<Real64, 15> const c = {0.0, width, sep, c3, c4, c5, c6, c7, c8, c9, c10, c11, 0.0, 0.0, 0.0};
    std::array<Real64, 16> p{};
    Window::BlindOpticsBeam(st(), 1, c, b_el, s_el, p);
    return p[which];
}

// MaterialBlind::BeamBeamTrans at profile angle and slat angle (radians).
double ep_BlindBeamBeam(double width, double sep, double thick, double profAng, double slatAng)
{
    return blind(width, sep, thick).BeamBeamTrans(profAng, slatAng);
}
}
