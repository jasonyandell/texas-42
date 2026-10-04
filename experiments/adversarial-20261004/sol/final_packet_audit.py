#!/usr/bin/env python3
"""Local exact witness, exhaustive support and properties for saved engine42.v2.

Does not transmit fixtures or substitute dependencies into the incoming packet.
"""
import sys
sys.dont_write_bytecode = True
import importlib.util, itertools, json, hashlib
from pathlib import Path
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
source = HERE.parent / 'incoming/drive-final/engine42.v2.py'
spec = importlib.util.spec_from_file_location('final_engine42', source)
engine = importlib.util.module_from_spec(spec)
spec.loader.exec_module(engine)
np = engine.np
sys.path.insert(0, str(ROOT/'experiments/partnership'))
from rules import replay_record
sys.path.insert(0, str(ROOT/'experiments/astra-sol-20261004/phase2/incoming/drive'))
import nofusion_sc
fixture = json.loads((HERE/'receipts/bidder-counterexample-numpy/stdout.log').read_text())
history = fixture['history']; viewer = fixture['viewer']; bidder = fixture['bidder']
rules = engine.Rules(fixture['trump'])
own = sum(1 << t for t in fixture['actual']['remaining'][viewer])
record = [x for play in history for x in play]
played = [{t for s,t in history if s == seat} for seat in range(4)]

def replay_accepts(full, no_stop):
    g = engine.Game(rules, full, 30, 0, no_stop=no_stop)
    g.leader[:] = history[0][0]
    ok = np.ones(len(full), bool); checks = []
    for s,t in history:
        legal = ((g.legal(np.full(len(full), s)) >> t) & 1) == 1
        checks.append(legal.copy()); ok &= legal
        g.play(np.full(len(full), t))
    return ok, checks, g

bad = np.array([[sum(1 << t for t in h) for h in fixture['falsely_accepted_deal']]])
old,_,_ = replay_accepts(bad, False)
fixed,checks,g = replay_accepts(bad, True)
assert old[0] and not fixed[0] and not checks[7][0]
assert not g.done.any() and int(g.trick[0]) == 4
reference = nofusion_sc.all_deals(rules, viewer, own, history)
expected = {tuple(map(int,row)) for row in reference}
assert len(expected) == 700
unseen = sorted(set(range(28))-set(t for _,t in history)-set(fixture['actual']['remaining'][viewer]))
others = [s for s in range(4) if s != viewer]
candidate_rem = []
for partition in itertools.product(range(3), repeat=len(unseen)):
    if any(partition.count(s) != 3 for s in range(3)): continue
    rem = [0]*4; rem[viewer] = own
    for t,s in zip(unseen, partition): rem[others[s]] |= 1 << t
    candidate_rem.append(rem)
assert len(candidate_rem) == 1680
full = np.array([[int(mask)|sum(1 << t for t in played[s]) for s,mask in enumerate(row)] for row in candidate_rem])
accepted,_,_ = replay_accepts(full, True)
actual = {tuple(row) for row,ok in zip(candidate_rem,accepted) if ok}
assert actual == expected
for seed in (70002,1,999999):
    rem,full = engine.consistent_worlds(np.random.default_rng(seed), rules, viewer, own, history, 128)
    assert len(rem) == 128 and all(tuple(map(int,row)) in expected for row in rem)
    for row in full:
        original = [{t for t in range(28) if int(mask) >> t & 1} for mask in row]
        assert all(len(h) == 7 for h in original) and len(set.union(*original)) == 28
        points,leader,remaining,trick = replay_record(original,record,fixture['trump'],bidder)
        assert points == [7,17] and leader == viewer and not trick
        assert sum(1 << t for t in remaining[viewer]) == own

# Reproduce test_support.py's OLD-path proposal construction, independently of
# its acceptance filter. A current remainder is not an original seven-tile hand.
cand = engine.random_deals(np.random.default_rng(99),256,viewer,own)
old_proposals = cand.copy()
for s in range(4):
    old_proposals[:,s] |= sum(1 << t for t in played[s])
def valid_deal(row):
    hs = [{t for t in range(28) if int(mask) >> t & 1} for mask in row]
    return all(len(h) == 7 for h in hs) and len(set.union(*hs)) == 28
invalid = sum(not valid_deal(row) for row in old_proposals)
old_ok,_,_ = replay_accepts(old_proposals,False)
invalid_accepted = sum(not valid_deal(row) for row in old_proposals[old_ok])
assert invalid > 0
print(json.dumps(dict(engine_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
    original_witness_reproduced=True, v2_rejects_exact_illegal_witness_at_ply=7,
    candidate_assignments=1680,v2_support=700,all_support_sets_equal=True,
    sampled_worlds_independently_legal=384,own_remaining_preserved=True,
    test_support_old_arm=dict(candidates=256,invalid_original_deals=invalid,
        accepted=int(old_ok.sum()),invalid_original_deals_among_accepted=invalid_accepted),
    scope='One exact finite legal-history support; property checks, not a statistical uniformity or policy-posterior guarantee. Full incoming test_support result not rerun: its modified nofusion_sc and batched dependencies are absent.'),indent=2))
