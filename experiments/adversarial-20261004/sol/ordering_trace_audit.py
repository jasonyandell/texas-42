#!/usr/bin/env python3
"""Compare every priority ordering to adaptive own-choice leaves on fixed scenarios."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
import itertools,json,hashlib
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
sys.path.insert(0,str(ROOT/'experiments/astra-sol-20261004/phase3'))
from compiled_tape import Kernel,pick,bits,popcount
PLAN=HERE.parent/'results/parallel-panel/plan.json'
plan=json.loads(PLAN.read_text());rows=[r for r in plan['rows'] if r['status']=='ready']
cases=[]
for row in rows:
    for scenario in range(min(3,len(row['worlds']))):
        kernel=Kernel(row['request'],[row['worlds'][scenario]],row['points'],row['leader'],
                      tuple(tuple(x) for x in row['trick']))
        tape=[row['tape'][scenario]];bid=30;leaf_traces=set()
        def settled(state):
            return state.points[kernel.bidder%2]>=bid or state.points[1-kernel.bidder%2]>42-bid or popcount(state.played)==28
        def rec(state,trace,depth):
            if settled(state):leaf_traces.add(trace);return
            actor=(state.leader+len(state.trick))%4;legal=kernel.moves(state,0)
            choices=legal if actor==kernel.focal else [pick(tape,0,depth,legal)]
            for tile in choices:rec(kernel.after(state,tile),trace+((actor,tile),),depth+1)
        rec(kernel.root,(),0)
        own=list(bits(kernel.worlds[0][kernel.focal] & ~kernel.root.played))
        order_traces=set();order_count=0
        for order in itertools.permutations(own):
            order_count+=1;state=kernel.root;depth=0;trace=[]
            while not settled(state):
                actor=(state.leader+len(state.trick))%4;legal=kernel.moves(state,0)
                tile=next(t for t in order if t in legal) if actor==kernel.focal else pick(tape,0,depth,legal)
                trace.append((actor,tile));state=kernel.after(state,tile);depth+=1
            order_traces.add(tuple(trace))
        assert leaf_traces==order_traces
        cases.append(dict(seed=row['seed'],scenario=scenario,own_cards=len(own),
                          orderings=order_count,adaptive_leaves=len(leaf_traces)))
print(json.dumps(dict(plan_sha256=hashlib.sha256(PLAN.read_bytes()).hexdigest(),
    cases=len(cases),roots=len(rows),all_trace_sets_equal=True,
    orderings_checked=sum(c['orderings'] for c in cases),
    adaptive_leaves=sum(c['adaptive_leaves'] for c in cases),
    scope='Fixed physical world + fixed other-seat tape; not a shared-order completeness result across worlds.',
    rows=cases),indent=2))
