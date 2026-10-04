#!/usr/bin/env python3
"""Capped legal-fixture search: exact rational weighted observation tree vs archive."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
from fractions import Fraction as F
import random,json,time,hashlib
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import legal_tiles,winner,trick_points,replay_record
ARCHIVE=ROOT/'experiments/astra-sol-20261004/phase2/incoming/drive'
sys.path.insert(0,str(ARCHIVE))
import nofusion_sc as old
np=old.np

def decode(mask):return tuple(t for t in range(28) if int(mask)>>t&1)
def after(state,tile):
    played,leader,trick,points=state;seat=(leader+len(trick))%4
    trick=trick+((seat,tile),);points=list(points)
    if len(trick)==4:
        leader=winner(trick,TRUMP);points[leader%2]+=trick_points(trick);trick=()
    return played|1<<tile,leader,trick,tuple(points)

def exact_values(rem,root,focal):
    worlds=[list(map(decode,r)) for r in rem]; calls=0
    def rec(state,bundle):
        nonlocal calls;calls+=1
        played,leader,trick,points=state
        if points[0]>=30 or points[1]>=13 or played.bit_count()==28:
            return sum(weight for _,weight in bundle)*int(points[0]>=30)
        seat=(leader+len(trick))%4
        def moves(w):return legal_tiles([t for t in worlds[w][seat] if not(played>>t&1)],trick,TRUMP)
        if seat==focal:
            legal=moves(bundle[0][0]);assert all(moves(w)==legal for w,_ in bundle)
            values=[rec(after(state,t),bundle) for t in legal]
            return (max if focal%2==0 else min)(values)
        groups={}
        for w,weight in bundle:
            legal=moves(w)
            for t in legal:groups.setdefault(t,[]).append((w,weight/len(legal)))
        return sum(rec(after(state,t),group) for t,group in groups.items())
    bundle=[(i,F(1,len(worlds))) for i in range(len(worlds))]
    legal=legal_tiles([t for t in worlds[0][focal] if not(root[0]>>t&1)],root[2],TRUMP)
    return {t:rec(after(root,t),bundle) for t in legal},calls

HISTORY_PLIES=int(sys.argv[1]) if len(sys.argv)>1 else 16
rng=random.Random(451007);started=time.monotonic();checked=0
for case in range(1000):
    TRUMP=case%7;tiles=list(range(28));rng.shuffle(tiles);full=[tiles[7*s:7*s+7] for s in range(4)]
    remaining=list(map(set,full));leader=0;trick=[];points=[0,0];history=[]
    for ply in range(HISTORY_PLIES):
        seat=(leader+len(trick))%4;t=rng.choice(legal_tiles(remaining[seat],trick,TRUMP))
        history.append((seat,t));remaining[seat].remove(t);trick.append((seat,t))
        if len(trick)==4:leader=winner(trick,TRUMP);points[leader%2]+=trick_points(trick);trick=[]
        if points[0]>=30 or points[1]>=13:break
    if len(history)!=HISTORY_PLIES or points[0]>=30 or points[1]>=13:continue
    focal=leader;own=sum(1<<t for t in remaining[focal]);rules=old.Rules(TRUMP)
    if len(legal_tiles(remaining[focal],[],TRUMP))<2:continue
    deals=old.all_deals(rules,focal,own,history)
    if deals is None:
        deals,_=old.consistent_worlds(np.random.default_rng(801000+case),rules,focal,own,history,5)
    if len(deals)<2:continue
    chosen=rng.sample(range(len(deals)),min(5,len(deals)));rem=deals[chosen]
    for row in rem:
        original=[set(decode(h)) for h in row]
        for s,t in history:original[s].add(t)
        replay_record(original,[x for play in history for x in play],TRUMP,0)
    root=(sum(1<<t for _,t in history),leader,(),tuple(points))
    expected,calls=exact_values(rem,root,focal)
    fm=np.array([sum(1<<t for t in h) for h in full])
    actual,choice,legal=old.expectimax(rules,0,focal,own,history,fm,rem)
    checked+=1
    if any(abs(actual[t]-float(expected[t]))>1e-9 for t in legal):
        best=(max if focal%2==0 else min)(legal,key=lambda t:(expected[t],-t if focal%2==0 else t))
        print(json.dumps(dict(schema='adversarial-nofusion-likelihood-v1',case=case,checked=checked,
            trump=TRUMP,bid=30,bidder=0,focal=focal,full_deal=full,history=history,root_points=points,
            remaining_worlds=[list(map(decode,r)) for r in rem],
            expected={str(t):str(expected[t]) for t in legal},actual=actual,
            expected_choice=best,archived_choice=choice,choice_reversed=best!=choice,calls=calls,
            source_sha256=hashlib.sha256((ARCHIVE/'nofusion_sc.py').read_bytes()).hexdigest()),indent=2));sys.exit(0)
    if time.monotonic()-started>50:break
print(json.dumps(dict(found=False,checked=checked)));sys.exit(1)
