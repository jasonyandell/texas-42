"""Compiled sparse edge expansion for NumPy's random-policy frontier.

Float64, no fastmath; ascending tile order and reference RNG/CDF arithmetic.
"""
import numpy as np
from numba import njit

@njit(cache=True)
def prepare(tn,td,tw,deals,played,leader,trick,tlen,suit,lead,me,delta):
    n=len(tn)
    masks=np.empty(n,np.int64)
    counts=np.empty(n,np.int64)
    mine=np.empty(n,np.bool_)
    sampled=np.empty(n,np.bool_)
    for i in range(n):
        node=tn[i]
        seat=(leader[node]+tlen[node])%4
        hand=deals[td[i],seat]&~played[node]
        if tlen[node]:
            follow=hand&suit[lead[trick[node,0]]]
            if follow: hand=follow
        masks[i]=hand
        count=0
        scan=hand
        while scan:
            count+=1
            scan&=scan-1
        counts[i]=count
        mine[i]=seat==me
        sampled[i]=seat!=me and tw[i]*(1.0/count)<delta
    return masks,counts,mine,sampled

@njit(cache=True)
def expand(tn,td,tw,masks,counts,mine,sampled,uniform):
    size=0
    for i in range(len(tn)):
        size+=1 if sampled[i] else counts[i]
    keys=np.empty(size,np.int64)
    deals=np.empty(size,np.int64)
    weights=np.empty(size,np.float64)
    at=0
    random_index=0
    for i in range(len(tn)):
        p=1.0/counts[i]
        chosen=-1
        if sampled[i]:
            total=0.0
            for _ in range(counts[i]): total+=p
            r=uniform[random_index]*total
            random_index+=1
            cumulative=0.0
            for t in range(28):
                if masks[i] & (1<<t):
                    cumulative+=p
                    if cumulative>r:
                        chosen=t
                        break
        for t in range(28):
            if masks[i] & (1<<t) and (not sampled[i] or t==chosen):
                keys[at]=tn[i]*28+t
                deals[at]=td[i]
                weights[at]=tw[i] if mine[i] or sampled[i] else tw[i]*p
                at+=1
    return keys,deals,weights
