"""Independent scalar algorithm, support enumeration, mechanics and purity gates."""
import json
from pathlib import Path
import struct
import time
import numpy as np
import repaired as r
import turbo
from verify_repaired import position, exhaustive_support
from verify import legal,play,oracle

MASK=(1<<64)-1

def mix(z):
    z=((z^(z>>30))*0xbf58476d1ce4e5b9)&MASK
    z=((z^(z>>27))*0x94d049bb133111eb)&MASK
    return z^(z>>31)

class Random:
    def __init__(self,state): self.state=state
    def next(self):
        self.state=(self.state+0x9e3779b97f4a7c15)&MASK
        return mix(self.state)
    def bounded(self,n):
        threshold=((1<<64)-n)%n
        while True:
            x=self.next()
            if x>=threshold: return x%n
    def integers(self,n,size,dtype):
        return np.array([self.bounded(n) for _ in range(size)],dtype)

def key(pub,hand,level,support=False):
    a=list(map(int,turbo.pack(pub)))
    k=[hand,a[0],a[1]|a[2]<<3|a[5]<<6,0,*a[10:14],0,0,0,0,0]
    if not support:
        k[2]|=a[3]<<12|a[4]<<18
        k[3]=sum(a[6+i]<<(5*i) for i in range(4))
        k[8:12]=a[14:18];k[12]=level
    return k

def salted(field,k,domain):
    h=0x243f6a8885a308d3
    for x in k: h=mix(h^x)
    delta=struct.unpack('<Q',struct.pack('<d',field.spec.delta))[0]
    return mix(h^mix(field.spec.seed)^mix(field.rules.trump)^mix(delta)^domain)

def sample(field,level,pub,hand):
    k=key(pub,hand,level,field.reuse_deals)
    salt=0x73616d706c652d31
    if not field.reuse_deals:
        for n in field.spec.samples[:level]: salt=mix(salt^n)
    return r.sample(pub,hand,field.spec.samples[level-1],Random(salted(field,k,salt)))

def scalar(field,level,pub,hand):
    worlds=sample(field,level,pub,hand)
    salt=0x726f6c6c6f757431
    for n in field.spec.samples[:level]: salt=mix(salt^n)
    rng=Random(salted(field,key(pub,hand,level),salt))
    me=int(pub.turn()[0])
    initial_path=rng.state
    def visit(p, fibers, root=False,path=0):
        made,sett=p.outcome()
        if made[0] or sett[0]: return sum(w for _,w in fibers) if made[0] else 0.0
        actor=int(p.turn()[0]);mine=actor==me;groups={}
        for d,w in fibers:
            hand=int(worlds[d,actor])&~int(p.played[0])
            moves=legal(hand,tuple(map(int,p.trick[0,:p.tlen[0]])),field.rules.trump)
            if not mine:
                if level>1:
                    if len(moves)>1:
                        vals=scalar(field,level-1,p,hand)
                        best=(max if actor%2 else min)(vals.values())
                        moves=[min(t for t,v in vals.items() if v==best)]
                elif w/len(moves)<field.spec.delta:
                    draw=Random(mix(path^mix((d+0xd1b54a32d192ed03)&MASK))) if field.addressed else rng
                    moves=[moves[draw.bounded(len(moves))]]
                else: w/=len(moves)
            for t in moves: groups.setdefault(t,[]).append((d,w))
        vals={t:visit(p.play(field.rules,np.array([t])),groups[t],path=mix(path^(((t+1)*0x9e3779b97f4a7c15)&MASK)) if field.addressed else 0) for t in sorted(groups)}
        if root: return vals
        return (max if me%2 else min)(vals.values()) if mine else sum(vals.values())
    return visit(pub,[(i,1.0) for i in range(len(worlds))],True,initial_path)

def fixed_values(field,pub,worlds):
    out=np.empty(28)
    assert turbo.LIB.walt_evaluate_l1(field.handle,turbo.pack(pub),np.array(worlds,np.uint32),len(worlds),17,out)==0
    return {int(t):float(out[t]) for t in np.flatnonzero(~np.isnan(out))}

def fusion_witness():
    # Seek a public-history branch where choosing a continuation separately in
    # each hidden world changes a root value. Then check the native joint solve
    # against the independent Fraction oracle, never that clairvoyant value.
    for seed in range(1,101):
        rules,hands,p=position(seed,16)
        if any(bool(v[0]) for v in p.outcome()): continue
        me=int(p.turn()[0]);hand=hands[me]&~int(p.played[0])
        f=turbo.Field(rules,r.Spec((8,),0.,73),seconds=120)
        worlds=f.sample(1,p,hand)
        lawful=fixed_values(f,p,worlds)
        clairvoyant={t:0. for t in lawful}
        for world in worlds:
            for t,v in fixed_values(f,p,[world]).items(): clairvoyant[t]+=v
        if any(abs(lawful[t]-clairvoyant[t])>1e-8 for t in lawful):
            exact=oracle(rules.trump,me,p,worlds)
            assert max(abs(lawful[t]-float(exact[t])) for t in exact)<1e-10
            f.close()
            return {'seed':seed,'depth':16,'trump':rules.trump,'actor':me,
                'worlds':worlds.tolist(),'lawful':lawful,'per_world_optimization':clairvoyant,
                'fraction_oracle':{t:str(v) for t,v in exact.items()}}
        f.close()
    raise AssertionError('no distinguishing strategy-fusion witness found')

def run(output='results/turbo-verification.json'):
    start=time.perf_counter();cases=[];positions=[];mechanics=0;rank_cases=0
    for seed in range(1,16):
        for depth in (12,16,20,21,22,23,24,27,28):
            rules,hands,p=position(seed,depth)
            if depth<28:
                actor=int(p.turn()[0]);hand=hands[actor]&~int(p.played[0])
                for t in legal(hand,tuple(map(int,p.trick[0,:p.tlen[0]])),rules.trump):
                    out=np.empty(18,np.uint64)
                    assert turbo.LIB.walt_play(rules.trump,turbo.pack(p),t,out)==0
                    assert np.array_equal(out,turbo.pack(p.play(rules,np.array([t]))))
                    before=(int(p.played[0]),int(p.leader[0]),tuple(map(int,p.trick[0,:p.tlen[0]])),int(p.t1[0]),int(p.t0[0]))
                    after=(int(out[0]),int(out[1]),tuple(map(int,out[6:6+int(out[2])])),int(out[3]),int(out[4]))
                    assert after==play(before,t,rules.trump)
                    mechanics+=1
                if not any(bool(v[0]) for v in p.outcome()): positions.append((rules,p,hand))
                if 20<=depth<=23:
                    count=exhaustive_support(p,hand)
                    # Native 2-dimensional DP must rank every support world in
                    # exactly the reference sampler's lexicographic order.
                    out=np.empty(4,np.uint32);total=np.empty(1,np.uint64)
                    support=[]
                    for rank in range(count):
                        assert turbo.LIB.walt_rank(turbo.pack(p),hand,rank,out,total)==0
                        assert total[0]==count;support.append(tuple(map(int,out)))
                    assert len(set(support))==count
                    class RankTape:
                        def integers(self,total,size,dtype):
                            assert total==size==count
                            return np.arange(count,dtype=dtype)
                    expected_ranked=r.sample(p,hand,count,RankTape())
                    assert support==list(map(tuple,expected_ranked.tolist()))
                    rank_cases+=count
    for rules,p,hand in positions:
        if p.depth<20 or p.depth>23: continue
        for reuse in (False,True):
            for delta in (0.,0.125,1.):
                spec=r.Spec((3,4,3,2),delta,57)
                f=turbo.Field(rules,spec,seconds=120,reuse_deals=reuse)
                for level in (1,2,3,4):
                    expected_sample=sample(f,level,p,hand)
                    assert np.array_equal(f.sample(level,p,hand),expected_sample)
                    expected=scalar(f,level,p,hand)
                    action,vs=f.actions(level,p,np.array([hand]),return_values=True)
                    assert set(expected)==set(vs[0])
                    err=max(abs(expected[t]-vs[0][t]) for t in expected)
                    assert err<1e-10,(level,delta,reuse,p.depth,expected,vs)
                    best=(max if int(p.turn()[0])%2 else min)(expected.values())
                    assert action[0]==min(t for t,v in expected.items() if v==best)
                    if delta==0 and level==1:
                        exact=oracle(rules.trump,int(p.turn()[0]),p,expected_sample)
                        assert max(abs(float(exact[t])-vs[0][t]) for t in exact)<1e-10
                    cases.append({'depth':p.depth,'level':level,'delta':delta,'reuse':reuse,'error':err})
                f.close()
        if len(cases)>=192: break
    # Mixed-history/actor batches; cache and query order cannot change values.
    group=[x for x in positions if x[0].trump==4 and x[1].depth>=16][:8]
    assert len(group)>=3
    spec=r.Spec((4,8,4),1.,42)
    for reuse in (False,True):
        expected=[]
        for rules,p,h in group:
            f=turbo.Field(rules,spec,cache=False,reuse_deals=reuse)
            expected.append(f.actions(2,p,np.array([h]),True));f.close()
        f=turbo.Field(group[0][0],spec,reuse_deals=reuse)
        for i in list(range(len(group)))[::-1]+list(range(len(group))):
            _,p,h=group[i];a,v=f.actions(2,p,np.array([h]),True)
            assert np.array_equal(a,expected[i][0]) and v==expected[i][1]
        f.close()
    # Deal reuse is prefix-coupled through public support and own hand. Larger
    # enclosing budgets and unrelated queries cannot alter the lower sample.
    rules,p,hand=positions[0]
    f=turbo.Field(rules,r.Spec((4,8,16)),reuse_deals=True)
    a=f.sample(1,p,hand);b=f.sample(3,p,hand);assert np.array_equal(a,b[:4])
    try: f.spec=r.Spec((1,))
    except AttributeError: pass
    else: raise AssertionError('native spec can be replaced after buffer sizing')
    f2=turbo.Field(rules,r.Spec((4,99)),reuse_deals=True)
    assert np.array_equal(a,f2.sample(1,p,hand));f2.close()
    f.close()
    for kwargs in ({'seconds':0},{'fiber_cap':1}):
        f=turbo.Field(rules,r.Spec((4,8)),**kwargs)
        try: f.actions(2,p,np.array([hand]),True)
        except r.Limit: pass
        else: raise AssertionError('native resource guard ignored')
        f.close()
    witness=fusion_witness()
    result={'status':'pass','seconds':time.perf_counter()-start,'oracle_cases':cases,
        'native_public_transitions':mechanics,'enumerated_support_ranks':rank_cases,
        'cache_order_invariance':True,'sample_prefix_invariance':True,'resource_guards':True,
        'strategy_fusion_witness':witness}
    Path(output).write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({**{k:v for k,v in result.items() if k!='oracle_cases'},'oracle_cases':len(cases)}))

if __name__=='__main__':
    import argparse
    parser=argparse.ArgumentParser();parser.add_argument('--output',default='results/turbo-verification.json')
    run(parser.parse_args().output)
