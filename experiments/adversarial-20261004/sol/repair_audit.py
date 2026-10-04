#!/usr/bin/env python3
"""One fixed finite-support census and repaired sampled-support property checks."""
import sys
sys.dont_write_bytecode=True
import itertools,json
from pathlib import Path
from repaired_sampler import engine,consistent_worlds,accepts_fixture_world
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import replay_record
import nofusion_sc
np=engine.np
fixture=json.loads((HERE/'receipts/bidder-counterexample-numpy/stdout.log').read_text())
history=fixture['history'];viewer=fixture['viewer'];trump=fixture['trump'];bidder=fixture['bidder']
own=sum(1<<t for t in fixture['actual']['remaining'][viewer]);rules=engine.Rules(trump)
reference=nofusion_sc.all_deals(rules,viewer,own,history)
expected={tuple(map(int,row)) for row in reference};assert len(expected)==700
played=[{t for s,t in history if s==seat} for seat in range(4)]
unseen=sorted(set(range(28))-set(t for _,t in history)-set(fixture['actual']['remaining'][viewer]))
others=[s for s in range(4) if s!=viewer];candidate_rem=[]
for partition in itertools.product(range(3),repeat=len(unseen)):
    if any(partition.count(s)!=3 for s in range(3)):continue
    rem=[0]*4;rem[viewer]=own
    for t,s in zip(unseen,partition):rem[others[s]] |= 1<<t
    candidate_rem.append(rem)
assert len(candidate_rem)==1680
full=np.array([[int(mask)|sum(1<<t for t in played[s]) for s,mask in enumerate(row)] for row in candidate_rem])
accepted=accepts_fixture_world(rules,full,history)
actual={tuple(row) for row,ok in zip(candidate_rem,accepted) if ok};assert actual==expected
bad=np.array([[sum(1<<t for t in h) for h in fixture['falsely_accepted_deal']]])
assert not accepts_fixture_world(rules,bad,history)[0]
sampled_count=0
for seed in (70002,1,999999):
    rem,full=consistent_worlds(np.random.default_rng(seed),rules,viewer,own,history,128)
    assert len(rem)==128 and all(tuple(map(int,row)) in expected for row in rem)
    assert all(int(row[viewer])==own for row in rem)
    for row in full:
        original=[{t for t in range(28) if int(mask)>>t&1} for mask in row]
        points,leader,remaining,trick=replay_record(original,[x for pair in history for x in pair],trump,bidder)
        assert points==[7,17] and leader==viewer and not trick
    sampled_count+=len(rem)
print(json.dumps(dict(candidate_assignments=len(candidate_rem),repaired_support=len(actual),
    exact_reference_support=len(expected),all_support_sets_equal=True,
    known_illegal_world_rejected=True,sampled_worlds_checked=sampled_count,
    sampled_seeds=[70002,1,999999],all_samples_independently_legal=True,own_remaining_preserved=True,
    scope='Known valid public history; one exhaustive finite support and three sample-property checks, no statistical posterior/uniformity theorem.'),indent=2))
