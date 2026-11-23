// Written by hand: GeneralRoutines' CalcComponentSensibleLatentOutput and
// CalcZoneSensibleLatentOutput return their results through references, which gen_plain.py cannot
// pass back. Plain extern "C" entry points over the real EnergyPlus GeneralRoutines code.
//
// CompOutput is CalcComponentSensibleLatentOutput for air of mass flow m (kg/s) going from state 1
// (t1 C, w1 kg/kg) to state 2 (t2, w2); ZoneOutput is CalcZoneSensibleLatentOutput for equipment
// supply air at te and we entering a zone at tz and wz; ZoneSensOutput is calcZoneSensibleOutput
// for the same with only the zone's humidity ratio.
//
// Guards (ADR-009), which return NaN: which outside 0 to 2; any input that is not finite or lies
// outside what EnergyPlus meets: m outside 0 to 1000, temperatures outside -60 to 80, humidity
// ratios outside 0 to 0.1.
#include <cmath>
#include <EnergyPlus/GeneralRoutines.hh>

using namespace EnergyPlus;

namespace {
bool within(double v, double lo, double hi)
{
    return std::isfinite(v) && v >= lo && v <= hi;
}

bool airOk(double m, double ta, double wa, double tb, double wb)
{
    return within(m, 0.0, 1000.0) && within(ta, -60.0, 80.0) && within(wa, 0.0, 0.1) && within(tb, -60.0, 80.0) && within(wb, 0.0, 0.1);
}

double pick(int which, double sens, double lat, double tot)
{
    return which == 0 ? sens : which == 1 ? lat : tot;
}
} // namespace

extern "C" {
// One result: the sensible (0), latent (1) or total (2) rate from state 1 to state 2 (W).
double ep_CompOutput(int which, double m, double t2, double w2, double t1, double w1)
{
    if (which < 0 || which > 2 || !airOk(m, t2, w2, t1, w1)) {
        return NAN;
    }
    double sens = 0.0;
    double lat = 0.0;
    double tot = 0.0;
    CalcComponentSensibleLatentOutput(m, t2, w2, t1, w1, sens, lat, tot);
    return pick(which, sens, lat, tot);
}

// One result: the sensible (0), latent (1) or total (2) rate the equipment adds to the zone (W).
double ep_ZoneOutput(int which, double m, double te, double we, double tz, double wz)
{
    if (which < 0 || which > 2 || !airOk(m, te, we, tz, wz)) {
        return NAN;
    }
    double sens = 0.0;
    double lat = 0.0;
    double tot = 0.0;
    CalcZoneSensibleLatentOutput(m, te, we, tz, wz, sens, lat, tot);
    return pick(which, sens, lat, tot);
}

// The sensible rate the equipment adds to the zone (W).
double ep_ZoneSensOutput(double m, double te, double tz, double wz)
{
    if (!airOk(m, te, 0.0, tz, wz)) {
        return NAN;
    }
    return calcZoneSensibleOutput(m, te, tz, wz);
}
}
