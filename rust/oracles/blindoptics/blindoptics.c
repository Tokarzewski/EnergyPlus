// The blindoptics unit as rust-anything sees it: each function forwards to the real EnergyPlus
// WindowManager and Material code through libep_blindoptics_shim (shim/ep_blindoptics_shim.cc),
// which sets up a blind material from plain numbers (see the shim).
#include "blindoptics.h"

double ep_BlindDiffuse(int which, int isolvis, double width, double sep, double thick, double b_el, double c6, double c7, double c8, double c9, double c10, double c11, double c12, double c13, double c14);
double ep_BlindBeam(int which, double width, double sep, double thick, double b_el, double s_el, double c3, double c4, double c5, double c6, double c7, double c8, double c9, double c10, double c11);
double ep_BlindBeamBeam(double width, double sep, double thick, double profAng, double slatAng);

double BlindDiffuse(int which, int isolvis, double width, double sep, double thick, double b_el, double c6, double c7, double c8, double c9, double c10, double c11, double c12, double c13, double c14)
{
    return ep_BlindDiffuse(which, isolvis, width, sep, thick, b_el, c6, c7, c8, c9, c10, c11, c12, c13, c14);
}

double BlindBeam(int which, double width, double sep, double thick, double b_el, double s_el, double c3, double c4, double c5, double c6, double c7, double c8, double c9, double c10, double c11)
{
    return ep_BlindBeam(which, width, sep, thick, b_el, s_el, c3, c4, c5, c6, c7, c8, c9, c10, c11);
}

double BlindBeamBeam(double width, double sep, double thick, double profAng, double slatAng)
{
    return ep_BlindBeamBeam(width, sep, thick, profAng, slatAng);
}

