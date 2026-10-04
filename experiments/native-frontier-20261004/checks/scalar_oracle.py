#!/usr/bin/env python3
"""Independent scalar grouped recursion for native state-seeded Dice bundles.
Uses the separately audited Python rules, not the native frontier's transitions.
No hidden-world choice and no process execution outside watchdog.
"""
import json
import sys
from pathlib import Path
sys.dont_write_bytecode = True
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'experiments/adversarial-20261004'))
from parallel_roots import kernel_of
MASK=(1<<64)-1
STEP=0x9e3779b97f4a7c15

def mix(h):
    z=(h+STEP)&MASK
    z=((z^(z>>30))*0xbf58476d1ce4e5b9)&MASK
    z=((z^(z>>27))*0x94d049bb133111eb)&MASK
    return z^(z>>31)

def record(state,bidder):
    rotation=1 if bidder%2==0 else 0
    h=mix(state.played)
    h=mix(h^(((state.leader+rotation)%4)<<32))
    for _,tile in state.trick:
        h=mix(h^(0x100|tile))
    return h

def below(seed,n):
    assert n>0
    zone=MASK-MASK%n
    while True:
        value=mix(seed);seed=(seed+STEP)&MASK
        if value<zone:return value%n

def draw(seed,state,bidder,legal):
    if len(legal)==1:return legal[0]
    return legal[below(seed^record(state,bidder),len(legal))]

def native_values(row,seeds,bid=None):
    k=kernel_of(row)
    assert len(seeds)==k.n
    bid=row['request']['bid'] if bid is None else bid
    nodes=0
    def rec(state,worlds):
        nonlocal nodes
        nodes+=1
        if state.points[k.bidder%2]>=bid or state.points[1-k.bidder%2]>42-bid:
            return int(k.success(state.points,bid))*len(worlds)
        actor=(state.leader+len(state.trick))%4
        if actor==k.focal:
            legal=k.moves(state,worlds[0])
            assert all(k.moves(state,w)==legal for w in worlds)
            return max(rec(k.after(state,t),worlds) for t in legal)
        buckets={}
        for w in worlds:
            tile=draw(seeds[w],state,k.bidder,k.moves(state,w))
            buckets.setdefault(tile,[]).append(w)
        return sum(rec(k.after(state,t),ws) for t,ws in sorted(buckets.items()))
    answers={t:rec(k.after(k.root,t),list(range(k.n))) for t in k.moves(k.root,0)}
    return answers,nodes

if __name__=='__main__':
    rows=json.loads((ROOT/'experiments/adversarial-20261004/results/parallel-panel/plan.json').read_text())['rows']
    rows=[r for r in rows if r['status']=='ready' and r['request']['decl']<=6]
    checked=0;duplicate_cases=0;node_sum=0
    for row in rows:
        seeds=[mix(row['seed']^i) for i in range(len(row['worlds']))]
        values,nodes=native_values(row,seeds);node_sum+=nodes
        assert all(0<=v<=len(seeds) for v in values.values())
        if checked<12:
            doubled=dict(row,worlds=[w for w in row['worlds'] for _ in range(3)])
            ds=[s for s in seeds for _ in range(3)]
            d,_=native_values(doubled,ds)
            assert d=={a:3*v for a,v in values.items()};duplicate_cases+=1
        checked+=1
    print(json.dumps({'independent_native_scalar_roots':checked,'tripled_scenario_weight_cases':duplicate_cases,'node_visits':node_sum,'note':'Scalar baseline created; this self-check is NOT a native implementation parity result.'},indent=2))
