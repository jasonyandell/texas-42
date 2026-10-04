#!/usr/bin/env python3
"""Search archived sampler against independently replayed legal straight-42 deals.

No training code, new dependencies, or changed incoming source. Run under
run_capped.py; bytecode writing disabled. Output is a self-contained fixture.
"""
import sys
sys.dont_write_bytecode = True
from pathlib import Path
import importlib.util
import json
import random
import hashlib

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT / 'experiments/astra-sol-20261004/phase2/incoming/drive/engine42.py'
sys.path.insert(0, str(ROOT / 'experiments/partnership'))
from rules import legal_tiles, winner, trick_points
spec = importlib.util.spec_from_file_location('archive_engine', SOURCE)
archive = importlib.util.module_from_spec(spec)
spec.loader.exec_module(archive)
np = archive.np

def scalar_replay(hands, history, trump, bidder):
    hands = list(map(set, hands)); leader = bidder; trick = []; points = [0, 0]
    for index, (seat, tile) in enumerate(history):
        legal = legal_tiles(hands[seat], trick, trump)
        if seat != (leader + len(trick)) % 4 or tile not in legal:
            return dict(valid=False, ply=index, actor=seat, tile=tile, legal=legal,
                        trick=trick, remaining=list(map(sorted, hands)))
        hands[seat].remove(tile); trick.append((seat, tile))
        if len(trick) == 4:
            leader = winner(trick, trump); points[leader % 2] += trick_points(trick); trick=[]
        if index != len(history)-1 and (points[bidder % 2] >= 30 or points[1-bidder % 2] >= 13):
            return dict(valid=False, ply=index, reason='continued after actual settlement')
    return dict(valid=True, points=points, leader=leader, trick=trick,
                remaining=list(map(sorted, hands)))

def decode(mask):
    return [t for t in range(28) if (int(mask) >> t) & 1]

rng = random.Random(991704)
for case in range(1500):
    tiles = list(range(28)); rng.shuffle(tiles)
    full = [tiles[7*s:7*s+7] for s in range(4)]
    remaining = list(map(set, full)); trump=case % 7; bidder=1
    leader=bidder; trick=[]; points=[0,0]; history=[]
    for ply in range(24):
        seat=(leader+len(trick))%4; tile=rng.choice(legal_tiles(remaining[seat],trick,trump))
        remaining[seat].remove(tile); history.append((seat,tile)); trick.append((seat,tile))
        if len(trick)==4:
            leader=winner(trick,trump); points[leader%2]+=trick_points(trick);trick=[]
        if points[1]>=30 or points[0]>=13:break
        if len(history)<16 or trick or points[1]<13:continue
        viewer=leader
        own=sum(1<<t for t in remaining[viewer])
        rem, sampled = archive.consistent_worlds(np.random.default_rng(70000+case), archive.Rules(trump),
                                                 viewer, own, history, 128)
        valid = scalar_replay(full,history,trump,bidder)
        assert valid['valid']
        for candidate in sampled:
            decoded=list(map(decode,candidate)); result=scalar_replay(decoded,history,trump,bidder)
            if not result['valid']:
                fake=archive.Game(archive.Rules(trump),np.array([candidate]),30,0)
                fake.leader[:]=bidder
                checks=[]
                for s,t in history:
                    checks.append(bool((int(fake.legal(np.array([s]))[0])>>t)&1))
                    fake.play(np.array([t]))
                assert all(checks)
                output=dict(schema='adversarial-bidder-counterexample-v1', search_case=case,
                    source_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(), trump=trump,bid=30,
                    bidder=bidder,viewer=viewer, full_deal=full, history=history, actual=valid,
                    falsely_accepted_deal=decoded, independent_failure=result,
                    archived_all_legality_checks=True,
                    archived_replay=dict(done=bool(fake.done[0]),tricks=int(fake.trick[0]),
                        bid_pts=int(fake.bid_pts[0]),def_pts=int(fake.def_pts[0]),
                        remaining=list(map(decode,fake.hands[0]))), sampled_world_count=len(sampled))
                print(json.dumps(output,indent=2));sys.exit(0)
print(json.dumps(dict(found=False, searched=1500)));sys.exit(1)
