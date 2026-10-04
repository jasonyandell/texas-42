#!/usr/bin/env python3
"""Independent complete scalar payoff matrix and shared-observation policy check."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
import itertools,json,time,hashlib
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
sys.path.insert(0,str(HERE.parent))
import refined_orders as vector
from rules import legal_tiles,winner,trick_points

rows=[r for r in json.loads((HERE.parent/'results/parallel-panel/plan.json').read_text())['rows']
      if r['status']=='ready' and r['request']['decl']<=6]
saved={str(r['seed']):r for r in json.loads((HERE.parent/'results/refined-orders/comparisons.json').read_text())}
result=vector.run(rows,retain=True);cells=0;observations=0;repeated=0;comp=[]
for row in rows:
    req=row['request'];focal=req['seat'];bidder=req['bidder'];decl=req['decl'];bid=req['bid'];assert bid==30
    own=[t for t in range(28) if row['worlds'][0][focal]>>t&1]
    plans=[]
    for order in itertools.permutations(own):
        plans.append((order,order))
        for i in range(len(order)-1):
            alt=list(order);alt[i],alt[i+1]=alt[i+1],alt[i];plans.append((order,tuple(alt)))
    got=result['answers'][str(row['seed'])];assert [tuple(map(tuple,p)) for p in got['plan_pairs']]==plans
    table=[];actions={};P=len(plans);N=len(row['worlds'])
    for world,masks in enumerate(row['worlds']):
        for p,(base,alt) in enumerate(plans):
            hands=[{t for t in range(28) if mask>>t&1} for mask in masks]
            leader=row['leader'];trick=list(map(tuple,row['trick']));points=list(row['points']);trace=[];depth=0
            while points[bidder%2]<bid and points[1-bidder%2]<=42-bid and any(hands):
                actor=(leader+len(trick))%4;legal=legal_tiles(hands[actor],trick,decl)
                if actor==focal:
                    bit=bool(trick) and winner(trick,decl)==(focal+2)%4
                    chosen=next(t for t in (alt if bit else base) if t in legal)
                    key=(p,tuple(trace));value=(tuple(sorted(hands[focal])),tuple(legal),bit,chosen)
                    if key in actions:assert actions[key]==value;repeated+=1
                    else:actions[key]=value;observations+=1
                else:chosen=legal[(row['tape'][world][depth]*len(legal))>>64]
                hands[actor].remove(chosen);trace.append((actor,chosen));trick.append((actor,chosen));depth+=1
                if len(trick)==4:
                    leader=winner(trick,decl);points[leader%2]+=trick_points(trick);trick=[]
            made=points[bidder%2]>=bid;table.append(int(made if focal%2==bidder%2 else not made))
    assert table==got['table'];cells+=len(table)
    rootlegal=legal_tiles(own,row['trick'],decl)
    rootbit=bool(row['trick']) and winner(row['trick'],decl)==(focal+2)%4
    root=[next(t for t in pair[int(rootbit)] if t in rootlegal) for pair in plans]
    sums=[sum(table[w*P+p] for w in range(N)) for p in range(P)]
    refined={str(a):max(sums[p] for p in range(P) if root[p]==a) for a in rootlegal}
    static=[p for p,(a,b) in enumerate(plans) if a==b]
    base={str(a):max(sums[p] for p in static if root[p]==a) for a in rootlegal}
    lawful={str(a):v for a,v in vector.kernel_of(row).reference(row['tape'],bid)[0].items()}
    assert base==saved[str(row['seed'])]['base'] and refined==saved[str(row['seed'])]['refined']
    assert lawful==saved[str(row['seed'])]['lawful'] and all(base[a]<=refined[a]<=lawful[a] for a in base)
    bc=max(base,key=lambda a:(base[a],-int(a)));rc=max(refined,key=lambda a:(refined[a],-int(a)))
    comp.append(dict(seed=row['seed'],base_gap=max(lawful.values())-max(base.values()),
        refined_gap=max(lawful.values())-max(refined.values()),
        base_regret=max(lawful.values())-lawful[bc],refined_regret=max(lawful.values())-lawful[rc]))
print(json.dumps(dict(roots=len(rows),complete_scalar_cells=cells,all_cells_match=True,
    distinct_plan_public_observations=observations,repeated_observation_choices_checked=repeated,
    policies_shared_on_equal_public_observation=True,all_static_plans_included=True,
    all_action_values_nested_and_bounded=True,
    base_value_gap=sum(c['base_gap'] for c in comp),refined_value_gap=sum(c['refined_gap'] for c in comp),
    base_action_regret=sum(c['base_regret'] for c in comp),refined_action_regret=sum(c['refined_regret'] for c in comp),
    gap_improvement_roots=sum(c['refined_gap']<c['base_gap'] for c in comp),
    regret_improvement_roots=sum(c['refined_regret']<c['base_regret'] for c in comp),
    regret_worsening_roots=sum(c['refined_regret']>c['base_regret'] for c in comp),
    source_sha256=hashlib.sha256((HERE.parent/'refined_orders.py').read_bytes()).hexdigest(),
    scope='Exact declared finite-bundle policy evaluation, not held-out strength or native/GPU throughput.'),indent=2))
