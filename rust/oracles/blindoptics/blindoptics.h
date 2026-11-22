// Written by hand alongside shim/ep_blindoptics_shim.cc.
#ifndef EP_BLINDOPTICS_H
#define EP_BLINDOPTICS_H

double BlindDiffuse(int which, int isolvis, double width, double sep, double thick, double b_el, double c6, double c7, double c8, double c9, double c10, double c11, double c12, double c13, double c14);
double BlindBeam(int which, double width, double sep, double thick, double b_el, double s_el, double c3, double c4, double c5, double c6, double c7, double c8, double c9, double c10, double c11);
double BlindBeamBeam(double width, double sep, double thick, double profAng, double slatAng);

#endif
