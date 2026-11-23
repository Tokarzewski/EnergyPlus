// Written by hand alongside shim/ep_predload_shim.cc.
#ifndef EP_PREDLOAD_H
#define EP_PREDLOAD_H

double PredictedSystemLoad(int which, int ctrl, int algo, double rafnfrac, double tempdep, double tempind, double aircap, double t1, double setpt, double setptlo, double setpthi, int hasite, double itetemp, int beginsim, int hasnode, double nodetemp, int staged, int stagenum, double setptlast, double lcf, int mult, int listmult);

#endif
