#!/usr/bin/env python3
"""Audit saved four-tape union coverage and single-bid cost accounting."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import sys
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT))
from compiled_tape import Kernel,make_tape,popcount

panel=ROOT/'results/panel-coverage'
plan=json.loads((panel/'plan.json').read_text())
summary=json.loads((panel/'summary.json').read_text())
old=ROOT/'results/panel-a'
depth_physical=Counter();depth_used=Counter();totals=Counter();receipts=[]
for row in plan['fixtures']:
    filename='case-%04d.json'%row['id']
    r=json.loads((panel/filename).read_text())
    previous=json.loads((old/filename).read_text())
    assert r['status']=='completed' and r['request']==previous['request']==row['request']
    assert r['worlds']==previous['worlds'] and r['universal_statistics']==previous['universal_statistics']
    k=Kernel(row['request'],r['worlds'],row['points'],row['request']['seat'])
    universal=k.compile(node_cap=100000,seconds=5)
    used={};visits=0;support_visits=0
    for seed,t,prior in zip(plan['tape_seeds'],r['tapes'],previous['tapes']):
        tape=make_tape(seed,k.n,k.plies)
        assert t['seed']==seed and t['tape']==prior['tape']==[list(x) for x in tape]
        assert t['values']==prior['values']
        active=universal.route(tape)
        visits+=len(active);support_visits+=sum(popcount(mask) for _,mask in active)
        assert len(active)==t['adaptive_statistics']['nodes']
        for i,mask in active: used[i]=used.get(i,0)|mask
        values,calls=k.reference(tape,30)
        assert values==universal.reduce(active,30)=={int(a):v for a,v in t['values']['30'].items()}
        assert calls==t['reference_calls']['30']
        for key in ('reference_bid30_us','universal_reduce_bid30_us','adaptive_reduce_bid30_us',
                    'universal_route_us','adaptive_compile_us','adaptive_route_us',
                    'reference_13_bids_us','universal_reduce_13_bids_us','adaptive_reduce_13_bids_us'):
            assert t[key]>=0;totals[key]+=t[key]
    actual=dict(union_coordinates=len(used),union_support_incidences=sum(popcount(m) for m in used.values()),
        tape_coordinate_visits=visits,tape_support_visits=support_visits,
        unused_coordinates=len(universal.nodes)-len(used),
        depth_counts={str(d):dict(physical=sum(n.depth==d for n in universal.nodes),
                     used=sum(universal.nodes[i].depth==d for i in used)) for d in range(k.plies+1)})
    assert actual==r['coverage']
    assert sum(x['physical'] for x in actual['depth_counts'].values())==len(universal.nodes)
    assert sum(x['used'] for x in actual['depth_counts'].values())==len(used)
    for d,v in actual['depth_counts'].items():depth_physical[int(d)]+=v['physical'];depth_used[int(d)]+=v['used']
    totals['universal_compile_us']+=r['universal_compile_us']
    totals['nodes']+=len(universal.nodes);totals['union']+=len(used)
    totals['union_support']+=actual['union_support_incidences'];totals['visits']+=visits
    receipts.append(r)

assert summary['planned']==summary['completed']==len(receipts)==15
assert summary['tapes']==60 and summary['refused_attempt_seconds']==0
assert totals['nodes']==summary['universal_nodes']==592717
assert totals['union']==summary['union_used_coordinates']==58835
assert totals['nodes']-totals['union']==summary['unused_coordinates']==533882
assert totals['union_support']==summary['union_used_support_incidences']==71816
assert totals['visits']==summary['tape_coordinate_visits']==73104
for key in ('universal_compile_us','universal_route_us','adaptive_compile_us','adaptive_route_us',
            'reference_bid30_us','universal_reduce_bid30_us','adaptive_reduce_bid30_us',
            'reference_13_bids_us','universal_reduce_13_bids_us','adaptive_reduce_13_bids_us'):
    assert totals[key]==summary[key]
ref=totals['reference_bid30_us']
universal_warm=totals['universal_route_us']+totals['universal_reduce_bid30_us']
universal_cold=totals['universal_compile_us']+universal_warm
adaptive_cold=totals['adaptive_compile_us']+totals['adaptive_route_us']+totals['adaptive_reduce_bid30_us']
print(json.dumps(dict(cases=15,tapes=60,all_coverage_and_single_bid_checks=True,
    compiler_sha256=hashlib.sha256((ROOT/'compiled_tape.py').read_bytes()).hexdigest(),
    universal_coordinates=totals['nodes'],union_used_coordinates=totals['union'],
    truly_unused_coordinates=totals['nodes']-totals['union'],
    truly_unused_fraction=(totals['nodes']-totals['union'])/totals['nodes'],
    union_support_incidences=totals['union_support'],repeated_tape_coordinate_visits=totals['visits'],
    depth_counts={d:dict(physical=depth_physical[d],union_used=depth_used[d]) for d in sorted(depth_physical)},
    recursive_bid30_us=ref,universal_cold_bid30_us=universal_cold,
    universal_warm_bid30_us=universal_warm,adaptive_cold_bid30_us=adaptive_cold,
    universal_cold_ratio=universal_cold/ref,universal_warm_ratio=universal_warm/ref,
    adaptive_cold_ratio=adaptive_cold/ref,
    scope='Same frozen worlds, four tapes per root; universal compilation charged once across four bid30 queries. Retained Python timing only; no production speed/strength claim.'),indent=2))
