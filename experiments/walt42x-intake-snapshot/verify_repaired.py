"""Independent mechanics, exact support and fixed-field ladder regressions."""
import itertools
import json
import time
import numpy as np
import walt42x as ref
import repaired as r
from nested_kernels import support_counts,unrank_deals
from verify import legal,play,oracle

def position(seed,depth,pathseed=11):
    rules,hands=r.deal(seed);pub=r.Pub.initial();rng=np.random.default_rng(pathseed)
    scalar=(0,1,(),0,0);voids=[0]*4;events=[]
    for _ in range(depth):
        seat=int(pub.turn()[0]);hand=hands[seat]&~int(pub.played[0])
        allowed=legal(hand,scalar[2],rules.trump)
        tile=int(rng.choice(allowed))
        if pub.tlen[0]:
            suit=int(rules.suit[rules.lead[pub.trick[0,0]]])
            if not (suit>>tile&1): voids[seat]|=suit
        pub=pub.play(rules,np.array([tile]));scalar=play(scalar,tile,rules.trump)
        events.append(seat*32+tile)
        assert [int(pub.history[0,i//9])>>(7*(i%9))&127 for i in range(len(events))]==events
        assert (int(pub.played[0]),int(pub.leader[0]),tuple(map(int,pub.trick[0,:pub.tlen[0]])),
                int(pub.t1[0]),int(pub.t0[0]))==scalar
        assert pub.voids[0].tolist()==voids
    return rules,hands,pub

def scalar_actions(field,level,pub,hands):
    output=[];allvalues=[]
    for i,hand in enumerate(hands):
        p=pub.take(slice(i,i+1));hand=int(hand)
        rng=field.rng(level,p.key(hand))
        deals=r.sample(p,hand,field.spec.samples[level-1],rng)
        def assume(rules,seats,hs,pp,ignored_rng):
            chosen,_=scalar_actions(field,level-1,pp,hs)
            P=np.zeros((len(hs),28));P[np.arange(len(hs)),chosen]=1
            return P
        values=ref.search(field.rules,int(p.turn()[0]),p,deals,
                         ref.random_policy if level==1 else assume,rng,field.spec.delta)
        seat=int(p.turn()[0]);choice=(max if seat%2 else min)(values,key=values.get)
        output.append(choice);allvalues.append(values)
    return np.array(output),allvalues

def check_sampler(rules,hands,pub):
    seat=int(pub.turn()[0]);hand=hands[seat]&~int(pub.played[0])
    deals=r.sample(pub,hand,64,np.random.default_rng(19))
    for d in deals:
        assert int(d[seat])==hand
        assert sum(map(int,d))==((1<<28)-1)^int(pub.played[0])
        assert all(not (int(d[s])&int(pub.voids[0,s])) for s in range(4))
        assert all(not (int(d[a])&int(d[b])) for a in range(4) for b in range(a))
    return deals

def exhaustive_support(pub,hand):
    seat=int(pub.turn()[0]);played=int(pub.played[0])
    tiles=np.array([t for t in range(28) if not ((played|hand)>>t&1)],np.int64)
    assert len(tiles)<=6
    base=7-(pub.depth-int(pub.tlen[0]))//4;sizes=[base]*4
    for i in range(int(pub.tlen[0])): sizes[(int(pub.leader[0])+i)%4]-=1
    seats=np.array([s for s in range(4) if s!=seat]);caps=np.array([sizes[s] for s in seats])
    allowed=np.array([sum(1<<j for j,s in enumerate(seats) if not (int(pub.voids[0,s])>>int(t)&1)) for t in tiles])
    dp=support_counts(allowed,caps);total=int(dp[0,*caps])
    ranked=unrank_deals(tiles,allowed,caps,seats,seat,hand,dp,np.arange(total,dtype=np.int64))
    actual={tuple(map(int,row)) for row in ranked};expected=set()
    for a in itertools.combinations(map(int,tiles),int(caps[0])):
        rest=set(map(int,tiles))-set(a)
        for b in itertools.combinations(sorted(rest),int(caps[1])):
            parts=[a,b,rest-set(b)];d=[0]*4;d[seat]=hand
            for s,part in zip(seats,parts): d[s]=sum(1<<t for t in part)
            if all(not (d[s]&int(pub.voids[0,s])) for s in seats): expected.add(tuple(d))
    assert actual==expected and len(actual)==total
    return total

def main():
    started=time.perf_counter();positions=[];support=[]
    for seed in range(1,21):
        for depth in (12,16,20,21,22,23):
            rules,hands,pub=position(seed,depth)
            check_sampler(rules,hands,pub)
            if depth>=20:
                seat=int(pub.turn()[0]);support.append(exhaustive_support(pub,hands[seat]&~int(pub.played[0])))
            if not any(bool(v[0]) for v in pub.outcome()): positions.append((rules,hands,pub))
    assert max(support)>1
    _,_,full=position(1,28)
    try: full.play(r.Rules(3),np.array([0]))
    except ValueError: pass
    else: raise AssertionError('play beyond tile 28 accepted')
    # The opening support has the known three-hand multinomial count.
    dp=support_counts(np.full(21,7),np.array([7,7,7]))
    assert int(dp[0,7,7,7])==399072960
    # Exhaustive small synthetic void masks exercise all allowed-seat subsets.
    for masks in itertools.product(range(1,8),repeat=3):
        dp=support_counts(np.array(masks),np.array([1,1,1]))
        expected=sum(all(masks[i]&(1<<s) for i,s in enumerate(order))
                     for order in itertools.permutations(range(3)))
        assert int(dp[0,1,1,1])==expected
    cases=[]
    for rules,hands,pub in positions:
        if pub.depth<20: continue
        seat=int(pub.turn()[0]);hand=hands[seat]&~int(pub.played[0])
        for level in (1,2):
            for delta in (0.0,1.0):
                spec=r.Spec((3,4),delta,57)
                field=r.Field(rules,spec,seconds=30)
                expected,ev=scalar_actions(field,level,pub,np.array([hand]))
                actual,av=field.actions(level,pub,np.array([hand]),return_values=True)
                assert np.array_equal(actual,expected),(level,delta,pub.depth,av,ev)
                assert ev[0].keys()==av[0].keys()
                error=max(abs(av[0][k]-ev[0][k]) for k in ev[0])
                assert error<1e-10,(level,delta,error)
                if level==1 and delta==0:
                    rng=field.rng(1,pub.key(hand));deals=r.sample(pub,hand,3,rng)
                    exact=oracle(rules.trump,seat,pub,deals)
                    assert max(abs(av[0][k]-float(exact[k])) for k in exact)<1e-10
                cases.append({'depth':pub.depth,'trump':rules.trump,'seat':seat,'level':level,'delta':delta,'error':error})
        if len(cases)>=48: break
    assert len(cases)>=48
    # Same public history, different possible acting hands, duplicate rows and
    # a changed evaluation order must not change policy values or choices.
    rules,hands,pub=next(x for x in positions if x[2].depth==16)
    seat=int(pub.turn()[0]);hand=hands[seat]&~int(pub.played[0])
    pubs=r.Pub.concat([pub]*5);hs=np.array([hand]*5)
    spec=r.Spec((8,5),1,42)
    f=r.Field(rules,spec,batch=1,cache=False)
    a,av=f.actions(1,pubs,hs,return_values=True)
    assert len(set(a))==1 and all(v==av[0] for v in av)
    f2=r.Field(rules,spec,batch=32,cache=True)
    b,bv=f2.actions(1,pubs,hs,return_values=True)
    assert np.array_equal(a,b) and av==bv
    assert f2.stats['L1_unique_misses']==1
    before=f2.stats.copy();again=f2.actions(1,pubs,hs)
    assert np.array_equal(a,again) and f2.stats['L1_unique_misses']==before['L1_unique_misses']
    # Several genuine, same-contract roots with different histories and actors.
    mixed=[];mh=[]
    for path in range(1,30):
        rr,hh,pp=position(2,16,path)
        if any(bool(v[0]) for v in pp.outcome()): continue
        mixed.append(pp);mh.append(hh[int(pp.turn()[0])]&~int(pp.played[0]))
        if len(mixed)==6: break
    mixed=r.Pub.concat(mixed);mh=np.array(mh)
    expected,ev=r.Field(rr,spec,batch=1,cache=False).actions(1,mixed,mh,return_values=True)
    reverse=np.arange(len(mh))[::-1]
    actual,av=r.Field(rr,spec,batch=32,cache=False).actions(1,mixed.take(reverse),mh[reverse],return_values=True)
    assert np.array_equal(expected,actual[::-1]) and ev==av[::-1]
    oldkey=pub.key(hand);altered=pub.take(slice(None));altered.history=altered.history.copy();altered.history[0,0]^=np.uint64(1)
    assert altered.key(hand)!=oldkey
    assert r.Field(rules,spec).rng(1,oldkey).bit_generator.state!=r.Field(rules,r.Spec((8,5),1,43)).rng(1,oldkey).bit_generator.state
    assert r.Field(rules,spec).rng(1,oldkey).bit_generator.state==r.Field(rules,r.Spec((8,99),1.0,42)).rng(1,oldkey).bit_generator.state
    assert r.Spec([8,5],1,42)==r.Spec((8,5),1.0,42)
    # A whole opening query gives a larger batching/order regression.
    rr,_=r.deal(1);opening=r.Pub.initial();rng=np.random.default_rng(881)
    hh=np.array([sum(1<<int(t) for t in rng.choice(28,7,replace=False)) for _ in range(4)])
    pp=r.Pub.concat([opening]*4)
    serial=r.Field(rr,spec,batch=1,cache=False)
    one,ov=serial.actions(1,pp,hh,return_values=True)
    many,mv=r.Field(rr,spec,batch=32,cache=False).actions(1,pp.take(np.arange(4)[::-1]),hh[::-1],return_values=True)
    assert np.array_equal(one,many[::-1]) and ov==mv[::-1]
    capped=r.Field(rr,spec,batch=32,cache=False,fiber_cap=serial.stats['L1_peak_fibers']+8)
    ca,cv=capped.actions(1,pp,hh,return_values=True)
    assert np.array_equal(ca,one) and cv==ov and capped.stats['batch_splits']>0
    try: f2.cache['new-entry']=(0,())
    except TypeError: pass
    else: raise AssertionError('cache is externally mutable')
    try: f2.spec=r.Spec((1,),1,9)
    except AttributeError: pass
    else: raise AssertionError('field spec is externally replaceable')
    bad=pub.take(slice(None));bad.voids=bad.voids.copy();bad.voids[0,(seat+1)%4]=(1<<28)-1
    try: r.sample(bad,hand,8,np.random.default_rng(0))
    except ValueError: pass
    else: raise AssertionError('inconsistent support accepted')
    terminal=r.Pub.initial();terminal.t1[:]=30
    try: r.Field(rules).actions(1,terminal,np.array([hands[1]]))
    except ValueError: pass
    else: raise AssertionError('terminal decision accepted')
    limited=r.Field(rules,spec,fiber_cap=1)
    try: limited.actions(2,pub,np.array([hand]),return_values=True)
    except r.Limit: pass
    else: raise AssertionError('resource cap silently ignored')
    assert not any(key[0]==2 for key in limited.cache)
    expired=r.Field(rules,spec,seconds=0)
    try: expired.actions(2,pub,np.array([hand]))
    except r.Limit: pass
    else: raise AssertionError('deadline silently ignored')
    report={'status':'pass','seconds':time.perf_counter()-started,'oracle_cases':cases,
            'exhaustive_support_states':len(support),'synthetic_support_cases':343,
            'opening_support_count':399072960,'duplicate_order_cache_batch_invariance':True,
            'deadline_cap_and_terminal_handling':True}
    open('results/repaired-verification.json','w').write(json.dumps(report,indent=2))
    print(json.dumps({k:v for k,v in report.items() if k!='oracle_cases'}),flush=True)

if __name__=='__main__': main()
