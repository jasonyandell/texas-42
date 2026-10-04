#!/usr/bin/env python3
"""Check archived small-fiber enumeration uniqueness and independent legality."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
import json
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import replay_record
sys.path.insert(0,str(ROOT/'experiments/astra-sol-20261004/phase2/incoming/drive'))
import nofusion_sc as archive
fixture=json.loads((HERE/'receipts/bidder-counterexample-numpy/stdout.log').read_text())
history=fixture['history'];viewer=fixture['viewer'];own=sum(1<<t for t in fixture['actual']['remaining'][viewer])
rem=archive.all_deals(archive.Rules(fixture['trump']),viewer,own,history)
assert rem is not None
keys=[tuple(map(int,row)) for row in rem];assert len(keys)==len(set(keys))
for row in rem:
    full=[{t for t in range(28) if int(h)>>t&1} for h in row]
    for seat,tile in history:full[seat].add(tile)
    points,leader,remaining,trick=replay_record(full,[x for pair in history for x in pair],fixture['trump'],fixture['bidder'])
    assert points==[7,17] and leader==viewer and not trick
bad=tuple(sum(1<<t for t in h if t not in {tile for _,tile in history}) for h in fixture['falsely_accepted_deal'])
assert bad not in set(keys)
# Integer tape mapping is floor(u*n /2^64). At u=0 it selects index0.
# Finite-grid bucket populations differ by at most one; exact equal mass only
# when n divides the denominator. Check a small finite grid, no RNG simulation.
grid=256
bucket_counts={n:[sum((u*n)//grid==k for u in range(grid)) for k in range(n)] for n in range(1,8)}
assert all(max(c)-min(c)<=1 for c in bucket_counts.values())
print(json.dumps(dict(all_deals_worlds=len(rem),unique_worlds=len(set(keys)),
    all_worlds_independently_legal=True,false_sampler_world_excluded=True,
    tape_u_zero_index=0,finite_grid=grid,bucket_counts=bucket_counts,
    scope='One legal small-fiber fixture; finite-RNG mapping arithmetic, not universal support proof.'),indent=2))
