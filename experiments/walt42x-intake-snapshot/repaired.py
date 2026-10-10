"""Pure, void-aware NumPy Walt ladder. L0=random, L1=BR(L0), L2=BR(L1).

Uniform samples from the public-void support, float64 weights, full public
history keys. Resource limits abort a decision, never silently reduce its rung.
"""
from dataclasses import dataclass
import hashlib
import json
import time
from collections import Counter
from types import MappingProxyType
import numpy as np
import walt42x as ref
import fast_numpy
from compiled_edges import expand
from nested_kernels import (support_counts,unrank_deals,prepare,deterministic_edges,
                            prepare_support_batch,unrank_batch)

class Limit(RuntimeError): pass
class FrontierLimit(Limit): pass

class Rules(ref.Rules):
    def __init__(self,trump):
        if trump not in range(7): raise ValueError('pip trump must be 0..6')
        super().__init__(trump)
        self.trump=trump

class Pub(ref.Pub):
    def __init__(self,played,leader,trick,tlen,t1,t0,voids,history,depth):
        super().__init__(played,leader,trick,tlen,t1,t0)
        self.voids=voids
        # Four uint64 words, nine 7-bit (seat,tile) events per word.
        self.history=history
        self.depth=depth

    @classmethod
    def initial(cls):
        return cls(np.array([0]),np.array([1]),np.zeros((1,4),np.int64),
            np.array([0]),np.array([0]),np.array([0]),np.zeros((1,4),np.int64),
            np.zeros((1,4),np.uint64),0)

    def take(self,i):
        return Pub(*(getattr(self,k)[i] for k in
            ('played','leader','trick','tlen','t1','t0','voids','history')),self.depth)

    @classmethod
    def concat(cls,items):
        if len({p.depth for p in items})!=1: raise ValueError('batch roots must share a physical ply')
        return cls(*(np.concatenate([getattr(p,k) for p in items]) for k in
            ('played','leader','trick','tlen','t1','t0','voids','history')),items[0].depth)

    def play(self,rules,t):
        if self.depth>=28: raise ValueError('cannot play past the end of the hand')
        seat=self.turn()
        voids=self.voids.copy()
        if self.tlen[0]:
            suit=rules.suit[rules.lead[self.trick[:,0]]]
            fail=((suit>>t)&1)==0
            rows=np.flatnonzero(fail)
            voids[rows,seat[rows]]|=suit[rows]
        hist=self.history.copy()
        hist[:,self.depth//9]|=((seat*32+t).astype(np.uint64)<<np.uint64(7*(self.depth%9)))
        out=fast_numpy.play(self,rules,t)
        return Pub(*(getattr(out,k) for k in ('played','leader','trick','tlen','t1','t0')),
                   voids,hist,self.depth+1)

    def key(self,hand,index=0):
        # Equality uses every field, not a hash digest. Redundant derived public
        # fields also make accidental inconsistent state reuse fail closed.
        return (int(hand),self.depth,int(self.played[index]),int(self.leader[index]),
            int(self.tlen[index]),int(self.t1[index]),int(self.t0[index]),
            *map(int,self.trick[index]),*map(int,self.voids[index]),*map(int,self.history[index]))

def sample(pub,hand,n,rng):
    if len(pub.played)!=1: raise ValueError('sample takes one information state')
    seat=int(pub.turn()[0]);played=int(pub.played[0]);hand=int(hand)
    base=7-(pub.depth-int(pub.tlen[0]))//4
    sizes=[base]*4
    for i in range(int(pub.tlen[0])): sizes[(int(pub.leader[0])+i)%4]-=1
    if hand&played or hand.bit_count()!=sizes[seat] or hand&int(pub.voids[0,seat]):
        raise ValueError('own hand contradicts the public state')
    tiles=np.array([t for t in range(28) if not ((played|hand)>>t&1)],np.int64)
    seats=np.array([s for s in range(4) if s!=seat],np.int64)
    caps=np.array([sizes[s] for s in seats],np.int64)
    allowed=np.array([sum(1<<j for j,s in enumerate(seats) if not (int(pub.voids[0,s])>>int(t)&1))
                      for t in tiles],np.int64)
    dp=support_counts(allowed,caps)
    total=int(dp[0,*caps])
    if total<=0: raise ValueError('public information has no consistent deals')
    ranks=rng.integers(total,size=n,dtype=np.int64)
    return unrank_deals(tiles,allowed,caps,seats,seat,hand,dp,ranks)

@dataclass(frozen=True)
class Spec:
    samples: tuple=(8,30)
    delta: float=1.0
    seed: int=42
    def __post_init__(self):
        if not self.samples or any(n<=0 or int(n)!=n for n in self.samples): raise ValueError('positive integer sample schedule required')
        if not 0<=self.delta<=1: raise ValueError('delta must be in [0,1]')
        object.__setattr__(self,'samples',tuple(map(int,self.samples)))
        object.__setattr__(self,'delta',float(self.delta))
        object.__setattr__(self,'seed',int(self.seed))

class Field:
    def __init__(self,rules,spec=Spec(),batch=32,cache=True,seconds=60,fiber_cap=1_000_000,cache_limit=50_000,engine='numpy'):
        if batch<1: raise ValueError('batch must be positive')
        if engine not in ('numpy','native'): raise ValueError('unknown engine')
        self.engine=engine
        self._rules=Rules(rules.trump);self._spec=spec;self.batch=batch;self.use_cache=cache
        for name in ('suit','lead','strength'): getattr(self._rules,name).setflags(write=False)
        self.cache_limit=cache_limit
        self._cache={};self.stats=Counter();self.fiber_cap=fiber_cap
        self.deadline=time.monotonic()+seconds
        self._identity=json.dumps(['walt42x-pure-v2','support-rank-v1',rules.trump,30,1,
            spec.delta,spec.seed,'float64','least-tile-exact-float-tie'],separators=(',',':')).encode()

    @property
    def rules(self): return self._rules
    @property
    def spec(self): return self._spec
    @property
    def identity(self): return self._identity
    @property
    def cache(self): return MappingProxyType(self._cache)

    def check(self):
        if time.monotonic()>self.deadline: raise Limit('whole-query/game deadline')

    def rng(self,level,key):
        # An enclosing solver's sample budget cannot change the lower field.
        payload=self.identity+json.dumps([level,self.spec.samples[:level],key],separators=(',',':')).encode()
        digest=hashlib.blake2b(payload,digest_size=16,person=b'walt42x-state-v1').digest()
        return np.random.default_rng(int.from_bytes(digest,'little'))

    def actions(self,level,pub,hands,return_values=False):
        if not 1<=level<=len(self.spec.samples): raise ValueError('level outside sample schedule')
        self.check()
        legal=self.rules.legal(hands,pub.trick,pub.tlen)
        legal_counts=legal.sum(1);first=legal.argmax(1)
        if np.any(legal_counts==0): raise ValueError('nonterminal query needs a legal move')
        made,sett=pub.outcome()
        if np.any(made|sett): raise ValueError('terminal state has no policy decision')
        turns=pub.turn()
        result=np.empty(len(hands),np.int64)
        values=[None]*len(hands)
        missing={}
        for i,hand in enumerate(hands):
            self.stats[f'L{level}_rows']+=1
            # Forced actions do not need a policy sample. State-local RNG means
            # this cannot perturb a later decision's sample or continuation.
            if legal_counts[i]==1 and not return_values:
                result[i]=first[i];self.stats[f'L{level}_forced']+=1;continue
            key=(level,pub.key(hand,i))
            if self.use_cache and key in self._cache:
                result[i],frozen=self._cache[key];values[i]=dict(frozen)
                self.stats[f'L{level}_cache_hits']+=1
            else:
                if key not in missing: missing[key]=[]
                missing[key].append(i)
        items=list(missing.items())
        self.stats[f'L{level}_unique_misses']+=len(items)
        pending=[items[start:start+self.batch] for start in range(0,len(items),self.batch)][::-1]
        while pending:
            self.check()
            chunk=pending.pop()
            rows=np.array([indexes[0] for _,indexes in chunk])
            roots=pub.take(rows)
            rngs=[self.rng(level,key[1]) for key,_ in chunk]
            root_hands=hands[rows]
            tiles,allowed,seats,caps,acting,dp,totals=prepare_support_batch(
                roots.played,roots.leader,roots.tlen,roots.voids,root_hands,roots.depth)
            if np.any(totals<=0): raise ValueError('public information has no consistent deals')
            n=self.spec.samples[level-1]
            ranks=np.array([rng.integers(int(total),size=n,dtype=np.int64) for rng,total in zip(rngs,totals)])
            sampled=unrank_batch(tiles,allowed,seats,caps,acting,dp,root_hands,ranks)
            deals=list(sampled.reshape(len(rows),n,4))
            try:
                scored=solve(self,level,roots,deals,rngs)
            except FrontierLimit:
                if len(chunk)==1: raise
                mid=len(chunk)//2
                pending.extend([chunk[mid:],chunk[:mid]])
                self.stats['batch_splits']+=1
                continue
            for ((key,indexes),scores) in zip(chunk,scored):
                seat=int(turns[indexes[0]])
                tile=(max if seat%2 else min)(scores,key=scores.get)
                if self.use_cache and len(self._cache)<self.cache_limit: self._cache[key]=(tile,tuple(scores.items()))
                for i in indexes: result[i]=tile;values[i]=scores
        return (result,values) if return_values else result

def solve(field,level,node,deal_batches,rngs):
    """Batched full best responses; each root has its own deals, RNG and focal seat."""
    if np.any(node.outcome()[0]|node.outcome()[1]): raise ValueError('terminal search root')
    if field.engine=='native' and level==1:
        from native_frontier import solve as compiled_solve
        return compiled_solve(field,node,deal_batches,rngs)
    # L0 uses only legal moves; internal L1 nodes never query another modeled
    # mind. The full history/void state is needed at the root, not copied inside.
    if level==1:
        node=ref.Pub(*(getattr(node,k) for k in ('played','leader','trick','tlen','t1','t0')))
    me=node.turn().copy()
    counts=np.array([len(d) for d in deal_batches])
    deals=np.concatenate(deal_batches)
    tn=np.repeat(np.arange(len(me)),counts)
    td=np.arange(len(deals));tw=np.ones(len(deals))
    owner=np.arange(len(me))
    plies=[];parent=tile=None
    while True:
        field.check()
        if len(tn)>field.fiber_cap: raise FrontierLimit('frontier fiber cap')
        field.stats[f'L{level}_fiber_visits']+=len(tn)
        field.stats[f'L{level}_peak_fibers']=max(field.stats[f'L{level}_peak_fibers'],len(tn))
        made,sett=node.outcome();term=made|sett
        m=made[tn]
        leaf=np.bincount(tn[m],weights=tw[m],minlength=len(term)).astype(float,copy=False)
        plies.append((parent,tile,node.turn()==me[owner],me[owner]%2==1,leaf))
        if term.all(): break
        keep=~term[tn];tn,td,tw=tn[keep],td[keep],tw[keep]
        masks,counts,mine,small=prepare(tn,td,tw,deals,node.played,node.leader,node.trick,
            node.tlen,field.rules.suit,field.rules.lead,me[owner],field.spec.delta)
        if np.any(counts==0): raise ValueError('a live fiber has no legal move')
        if level==1:
            # Keep each root's random stream independent of batching and order.
            roots=owner[tn[small]]
            order=np.argsort(roots,kind='stable')
            tally=np.bincount(roots,minlength=len(me))
            uniform=np.empty(len(roots));at=0
            for q in np.flatnonzero(tally):
                n=int(tally[q]);uniform[order[at:at+n]]=rngs[q].random(n);at+=n
            keys,next_deal,weights=expand(tn,td,tw,masks,counts,mine,small,uniform)
        else:
            chosen=np.full(len(tn),-1,np.int64)
            other=np.flatnonzero(~mine)
            if len(other):
                qpub=node.take(tn[other])
                hands=deals[td[other],qpub.turn()]&~qpub.played
                chosen[other]=field.actions(level-1,qpub,hands)
            keys,next_deal,weights=deterministic_edges(tn,td,tw,masks,counts,mine,chosen)
        if len(keys)>field.fiber_cap: raise FrontierLimit('expanded frontier fiber cap')
        if field.engine=='native':
            from native_frontier import group_edges
            key,inv=group_edges(keys,len(node.played))
        else:
            key,inv=np.unique(keys,return_inverse=True)
        parent,tile=key//28,key%28
        node=(fast_numpy.play(node.take(parent),field.rules,tile) if level==1 else
              node.take(parent).play(field.rules,tile))
        owner=owner[parent]
        tn,td,tw=inv,next_deal,weights
    values=plies[-1][4]
    for depth in range(len(plies)-1,1,-1):
        parent=plies[depth][0]
        _,_,pme,podd,leaf=plies[depth-1]
        starts=np.r_[0,np.flatnonzero(parent[1:]!=parent[:-1])+1]
        parents=parent[starts]
        sums=np.bincount(parent,weights=values,minlength=len(leaf))[parents]
        maxi=np.maximum.reduceat(values,starts);mini=np.minimum.reduceat(values,starts)
        out=leaf.copy()
        out[parents]=np.where(pme[parents],np.where(podd[parents],maxi,mini),sums)
        values=out
    parent,tiles=plies[1][:2]
    result=[{} for _ in me]
    for p,t,v in zip(parent,tiles,values): result[int(p)][int(t)]=float(v)
    return result

def deal(seed):
    rng=np.random.default_rng(seed);order=list(range(28));rng.shuffle(order)
    hands=[sum(1<<t for t in order[7*i:7*i+7]) for i in range(4)]
    tiles=lambda m:[t for t in range(28) if m>>t&1]
    _,bidder,trump=max(((sum(p in ref.TILES[t] for t in tiles(hands[s])),
        hands[s]>>ref.TILES.index((p,p))&1,sum(ref.TILES[t][0]==ref.TILES[t][1] for t in tiles(hands[s]))),s,p)
        for s in range(4) for p in range(7))
    return Rules(trump),[hands[(bidder+i+3)%4] for i in range(4)]

def game(seed=1,spec=Spec(),level=2,batch=32,seconds=60,cache=True,progress=None,fiber_cap=1_000_000,engine='numpy'):
    started=time.perf_counter()
    rules,hands=deal(seed)
    field=Field(rules,spec,batch=batch,seconds=seconds,cache=cache,fiber_cap=fiber_cap,engine=engine)
    pub=Pub.initial();moves=[];status='complete';reason=None
    try:
        while not any(bool(v[0]) for v in pub.outcome()):
            seat=int(pub.turn()[0]);hand=hands[seat]&~int(pub.played[0])
            t0=time.perf_counter()
            # Forced roots require no search; a report marks their value absent.
            if rules.legal(np.array([hand]),pub.trick,pub.tlen).sum()>1:
                action,scored=field.actions(level,pub,np.array([hand]),return_values=True)
                values=scored[0]
            else:
                action=field.actions(level,pub,np.array([hand]));values=None
            tile=int(action[0]);pub=pub.play(rules,action)
            row={'seat':seat,'tile':tile,'seconds':time.perf_counter()-t0,
                 't1':int(pub.t1[0]),'t0':int(pub.t0[0]),'values':values}
            moves.append(row)
            if progress: progress(row)
    except Limit as e:
        status='incomplete';reason=str(e)
    return {'seed':seed,'spec':{'samples':spec.samples,'delta':spec.delta,'seed':spec.seed},
        'policy_version':'walt42x-pure-v2','cache_limit':field.cache_limit,'fiber_cap':field.fiber_cap,
        'level':level,'batch':batch,'cache':cache,'engine':engine,'status':status,'reason':reason,
        'seconds':time.perf_counter()-started,'trump':rules.trump,'hands':hands,
        'moves':moves,'stats':dict(field.stats),'cache_entries':len(field.cache),
        'outcome':('made' if pub.t1[0]>=30 else 'set') if status=='complete' else None}
