"""NumPy frontier optimization; float64, same random-policy sampling stream.

The generic assumed-policy path retains the intake's semantics (including its
known nested-field limitations). This is a throughput experiment, not that fix.
"""
import numpy as np
import walt42x as ref

def play(node, rules, tile):
    trick=node.trick.copy()
    trick[np.arange(len(tile)),node.tlen]=tile
    played=node.played | (1 << tile)
    # All nodes at a breadth-first ply have the same trick length.
    if node.tlen[0] != 3:
        return ref.Pub(played,node.leader,trick,node.tlen+1,node.t1,node.t0)
    pos=rules.strength[rules.lead[trick[:,0]][:,None],trick].argmax(1)
    winner=(node.leader+pos)%4
    points=1+ref.POINTS[trick].sum(1)
    return ref.Pub(played,winner,np.zeros_like(trick),np.zeros_like(node.tlen),
                   node.t1+np.where(winner%2==1,points,0),
                   node.t0+np.where(winner%2==0,points,0))

def search(rules,me,pub,deals,assume,rng,delta=0.0,compiled=False):
    n=len(deals)
    node=pub
    tn=np.zeros(n,dtype=np.int64)
    td=np.arange(n)
    tw=np.ones(n)
    plies=[]
    parent=tile=None
    while True:
        made,sett=node.outcome()
        term=made|sett
        m=made[tn]
        # NumPy returns integer bins when the weighted input is empty. Keep
        # nonterminal leaves floating too: they receive fractional backups.
        leaf=np.bincount(tn[m],weights=tw[m],minlength=len(term)).astype(float,copy=False)
        plies.append((parent,tile,term,node.turn()==me,leaf))
        if term.all(): break
        keep=~term[tn]
        tn,td,tw=tn[keep],td[keep],tw[keep]
        if compiled and assume is ref.random_policy:
            from compiled_edges import prepare,expand
            masks,counts,mine,sampled=prepare(tn,td,tw,deals,node.played,node.leader,
                node.trick,node.tlen,rules.suit,rules.lead,me,delta)
            keys,next_deal,cw=expand(tn,td,tw,masks,counts,mine,sampled,
                                   rng.random(int(sampled.sum())))
            key,inv=np.unique(keys,return_inverse=True)
            parent,tile=key//28,key%28
            node=play(node.take(parent),rules,tile)
            tn,td,tw=inv,next_deal,cw
            continue
        pubrows=node.take(tn)
        seat=pubrows.turn()
        hand=deals[td,seat]&~pubrows.played
        mine=seat==me
        P=rules.legal(hand,pubrows.trick,pubrows.tlen).astype(float)
        other=~mine
        if other.any():
            if assume is ref.random_policy:
                Po=P[other]
                counts=Po.sum(1)
                Po/=counts[:,None]
                small=tw[other]*(1/counts)<delta
            else:
                Po=assume(rules,seat[other],hand[other],pubrows.take(other),rng)
                small=tw[other]*np.where(Po>0,Po,np.inf).min(1)<delta
            if small.any(): Po[small]=ref.flip(Po[small],rng)
            P[other]=Po
        ti,t=np.nonzero(P)
        cw=tw[ti]*np.where(mine[ti],1.0,P[ti,t])
        key,inv=np.unique(tn[ti]*28+t,return_inverse=True)
        parent,tile=key//28,key%28
        node=play(node.take(parent),rules,tile)
        tn,td,tw=inv,td[ti],cw
    if len(plies)==1:
        return {}
    V=plies[-1][4]
    for depth in range(len(plies)-1,1,-1):
        parent=plies[depth][0]
        _,_,_,pme,pleaf=plies[depth-1]
        # Sorted public child keys make each parent's children contiguous.
        starts=np.r_[0,np.flatnonzero(parent[1:]!=parent[:-1])+1]
        parents=parent[starts]
        # Fractional values can be tied within roundoff. Match reference's
        # sequential additions so performance comparisons keep the same plays.
        sums=(np.add.reduceat(V,starts) if delta>=1 else
              np.bincount(parent,weights=V,minlength=len(pleaf))[parents])
        extrema=(np.maximum if me%2 else np.minimum).reduceat(V,starts)
        V=pleaf.copy()
        V[parents]=np.where(pme[parents],extrema,sums)
    return dict(zip(plies[1][1].tolist(),V.tolist()))

def compiled_search(rules,me,pub,deals,assume,rng,delta=0.0):
    return search(rules,me,pub,deals,assume,rng,delta,compiled=True)
