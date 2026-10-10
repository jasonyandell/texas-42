"""Independent Fraction-weighted recursive oracle for small legal endgames."""
from fractions import Fraction
import json
import numpy as np
import torch
import walt42x as ref
from bench import fixture
from metal import Backend, Rules, Pub, search, group_edges

def legal(hand, trick, trump):
    tiles = [t for t in range(28) if hand >> t & 1]
    if not trick:
        return tiles
    lead = ref.TILES[trick[0]]
    suit = trump if trump in lead else lead[0]
    follows = [t for t in tiles if suit in ref.TILES[t]
               and (suit == trump or trump not in ref.TILES[t])]
    return follows or tiles

def play(state, tile, trump):
    played, leader, trick, t1, t0 = state
    trick = trick + (tile,)
    played |= 1 << tile
    if len(trick) < 4:
        return played,leader,trick,t1,t0
    lead = ref.TILES[trick[0]]
    led = trump if trump in lead else lead[0]
    def strength(t):
        h,l=ref.TILES[t]
        tier=2 if trump in (h,l) else 1 if led in (h,l) else 0
        return tier, h==l, h+l
    winner=(leader+max(range(4),key=lambda i:strength(trick[i])))%4
    points=1+sum(sum(ref.TILES[t]) for t in trick if sum(ref.TILES[t]) in (5,10))
    return played,winner,(),t1+(points if winner%2 else 0),t0+(0 if winner%2 else points)

def oracle(trump, me, pub, deals):
    state=(int(pub.played[0]),int(pub.leader[0]),tuple(map(int,pub.trick[0,:pub.tlen[0]])),int(pub.t1[0]),int(pub.t0[0]))
    def edges(st, fibers):
        seat=(st[1]+len(st[2]))%4
        branches={}
        for deal,w in fibers:
            actions=legal(deal[seat]&~st[0],st[2],trump)
            for t in actions:
                branches.setdefault(t,[]).append((deal,w if seat==me else w/len(actions)))
        return {t:visit(play(st,t,trump),fs) for t,fs in branches.items()}
    def visit(st,fibers):
        if st[3]>=30: return sum((w for _,w in fibers),Fraction())
        if st[4]>12: return Fraction()
        vals=edges(st,fibers).values()
        return (max(vals) if me%2 else min(vals)) if (st[1]+len(st[2]))%4==me else sum(vals,Fraction())
    return edges(state,[(tuple(map(int,d)),Fraction(1)) for d in deals])

def main():
    torch.set_num_threads(1)
    b=Backend('mps')
    keys_cpu=torch.tensor([20_000_001,20_000_000,20_000_001,0,20_000_099])
    keys=keys_cpu.to('mps')
    u,inv=group_edges(keys,714_290,'mps')
    assert torch.equal(u[inv].cpu(),keys_cpu)
    expected,ei=torch.unique(keys_cpu,return_inverse=True)
    assert torch.equal(u.cpu(),expected) and torch.equal(inv.cpu(),ei)
    rows=[]
    # One nonterminal fixture for each trump, plus every partial-trick offset
    # and both maximizing and minimizing focal seats.
    coverage=set()
    for seed in range(1,300):
        for played in (20,21,22,23):
            try: trump,seat,pub,deals,_=fixture(seed,played,5)
            except ValueError: continue
            keys={('trump',trump),('offset',played%4),('team',seat%2)}
            if keys <= coverage: continue
            exact=oracle(trump,seat,pub,deals)
            nv=ref.search(ref.Rules(trump),seat,pub,deals,ref.random_policy,np.random.default_rng(0),0)
            gv,_=search(Rules(b,trump),seat,Pub.from_ref(b,pub),b.array(deals),np.random.default_rng(0),0)
            assert exact.keys()==nv.keys()==gv.keys()
            assert max(abs(nv[t]-float(exact[t])) for t in exact)<1e-12
            assert max(abs(gv[t]-float(exact[t])) for t in exact)<1e-5
            coverage |= keys
            rows.append({'seed':seed,'played':played,'trump':trump,'seat':seat,
                         'exact':{t:str(v) for t,v in exact.items()},'gpu':gv})
        if len(coverage)==13: break
    assert len(coverage)==13
    trump,seat,pub,deals,history=fixture(2,16,8)
    n=12
    q=ref.Pub(*(np.repeat(getattr(pub,k),n,axis=0) for k in
                ('played','leader','trick','tlen','t1','t0')))
    actions=ref.walt(ref.random_policy,8,0)(ref.Rules(trump),np.full(n,seat),
               np.full(n,int(deals[0,seat])),q,np.random.default_rng(42)).argmax(1).tolist()
    assert len(set(actions))>1, actions
    # Terminal root is a confirmed reference edge-case failure.
    terminal=ref.Pub(np.array([0]),np.array([1]),np.zeros((1,4),dtype=np.int64),np.array([0]),np.array([30]),np.array([0]))
    try:
        ref.search(ref.Rules(0),1,terminal,np.zeros((1,4),dtype=np.int64),ref.random_policy,np.random.default_rng(0))
    except IndexError:
        terminal_bug=True
    else: raise AssertionError('expected terminal-root failure')
    report={'status':'pass','fraction_oracle_cases':rows,'coverage':sorted(coverage),
            'reference_terminal_root_IndexError_reproduced':terminal_bug,
            'duplicate_information_state_actions':actions}
    with open('results/verification.json','w') as f: json.dump(report,f,indent=2)
    print(json.dumps(report,indent=2))

if __name__=='__main__': main()
