import json, itertools
from functools import lru_cache
from fractions import Fraction
d=json.load(open('data.json'))
unseen=d['unseen']; OWN=('6-3','6-0','5-5')
def pv(t): return tuple(map(int,t.split('-')))
def trump(t): return 5 in pv(t)
def ledsuit(L): a,b=pv(L); return 't' if trump(L) else max(a,b)
def follows(t,q):
    a,b=pv(t)
    if q=='t': return trump(t)
    return (not trump(t)) and q in (a,b)
def legal(hand,led):
    if led is None: return tuple(hand)
    f=tuple(t for t in hand if follows(t,ledsuit(led))); return f or tuple(hand)
def strength(t,led):
    q=ledsuit(led); a,b=pv(t)
    if trump(t): return 100+(7 if a==b else a+b-5)
    if q!='t' and q in (a,b): return 7 if a==b else a+b-q
    return 0
def count(t): a,b=pv(t); s=a+b; return 5 if s==5 else 10 if s==10 else 0
# deals
deals=[]
for s0 in itertools.combinations(unseen,3):
    rest=[t for t in unseen if t not in s0]
    for s1 in itertools.combinations(rest,2):
        s3=tuple(t for t in rest if t not in s1)
        deals.append({0:s0,1:s1,3:s3})
print('deals',len(deals))
# full tree per deal: every legal play by everyone (perfect info)
def full_tree_count(deal):
    hands={0:set(deal[0]),1:set(deal[1]),3:set(deal[3]),2:set(OWN)}
    leaves=0; nodes=0
    def rec(leader, plays, bid, dfn):
        nonlocal leaves,nodes
        nodes+=1
        if bid>=30 or dfn>=13 or all(len(h)==0 for h in hands.values()):
            leaves+=1; return
        if len(plays)==4:
            win=max(plays,key=lambda p:strength(p[1],plays[0][1])); pts=sum(count(t) for _,t in plays)+1
            if win[0]%2: bid+=pts
            else: dfn+=pts
            if bid>=30 or dfn>=13 or all(len(h)==0 for h in hands.values()):
                leaves+=1; return
            leader=win[0]; plays=[]
        seat=(leader+len(plays))%4
        for t in legal(sorted(hands[seat]), plays[0][1] if plays else None):
            hands[seat].remove(t); rec(leader, plays+[(seat,t)], bid, dfn); hands[seat].add(t)
    rec(1,[(1,'6-4')],d['t1'],d['t0'])
    return leaves,nodes
tl=tn=0
for dl in deals:
    l,n=full_tree_count(dl); tl+=l; tn+=n
print('perfect-info leaves',tl,'nodes',tn)
# information tree from seat 2's view: distinct histories, with exact expectimax (random others, seat 2 chooses per history)
# state: history = tuple of (seat,tile) since start. consistent deals = those where each seat's played tiles ⊆ its hand.
from collections import defaultdict
hist_nodes=0
def consistent(hist):
    out=[]
    for dl in deals:
        ok=all(t in dl[s] for s,t in hist if s!=2)
        if ok: out.append(dl)
    return out
# recursive over histories; returns dict deal_index -> probability-weighted value? simpler: value(hist, set of deals with weights) 
def resolve(plays):
    win=max(plays,key=lambda p:strength(p[1],plays[0][1])); pts=sum(count(t) for _,t in plays)+1
    return win[0],pts
memo={}
def value(hist, leader, plays, bid, dfn, weights):
    # weights: dict deal_idx -> probability mass (unnormalized) of this history given deal
    global hist_nodes
    hist_nodes+=1
    if bid>=30: return sum(weights.values())   # mass where bid made
    if dfn>=13: return 0
    if len(plays)==4:
        w,pts=resolve(plays)
        if w%2: bid+=pts
        else: dfn+=pts
        if bid>=30: return sum(weights.values())
        if dfn>=13: return 0
        leader=w; plays=[]
    seat=(leader+len(plays))%4
    led=plays[0][1] if plays else None
    played={s:[t for ss,t in hist if ss==s] for s in (0,1,2,3)}
    if seat==2:
        hand=[t for t in OWN if t not in played[2]]
        best=None
        for t in legal(hand,led):
            v=value(hist+((2,t),),leader,plays+[(2,t)],bid,dfn,weights)
            if best is None or v<best: best=v   # seat 2 defends: minimize made-mass
        return best
    # chance: others play uniformly among legal; split weights by tile
    by_tile=defaultdict(dict)
    for i,wt in weights.items():
        hand=[t for t in deals[i][seat] if t not in played[seat]]
        L=legal(hand,led)
        for t in L: by_tile[t][i]=wt/len(L)
    tot=0
    for t,ws in by_tile.items():
        tot+=value(hist+((seat,t),),leader,plays+[(seat,t)],bid,dfn,ws)
    return tot
w0={i:Fraction(1,len(deals)) for i in range(len(deals))}
res={}
for t in ('6-3','6-0'):
    hist_nodes=0
    v=value(((1,'6-4'),(2,t)),1,[(1,'6-4'),(2,t)],d['t1'],d['t0'],w0)
    res[t]=(v,hist_nodes)
    print(t,'P(bid made | seat 2 plays it, others random, seat 2 best-responds) =',float(v),'=',v,' info-tree nodes',hist_nodes)
