#!/usr/bin/env python3
"""Fresh independent exhaustive support check; no sampler implementation copied."""
import importlib.util,itertools,json,random,gzip,ast,hashlib,statistics,math,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/choice-arena-20261004'
spec=importlib.util.spec_from_file_location('sampling_check_target',HERE/'sampling.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
g=m.original.__globals__;legal,winner,replay=g['legal_tiles'],g['winner'],g['replay_record']
# All ordered seat partitions of the unknown set with the exact capacities.
def parts(tiles,sizes):
 if len(sizes)==1:
  assert len(tiles)==sizes[0];yield [list(tiles)];return
 for chosen in itertools.combinations(tiles,sizes[0]):
  rest=[t for t in tiles if t not in chosen]
  for p in parts(rest,sizes[1:]):yield [list(chosen)]+p
checked=accepted=rejected=histories=observations=known_future=after_settled=0
for seed in range(72):
 rng=random.Random(979000+seed);deck=list(range(28));rng.shuffle(deck);hands=[sorted(deck[7*s:7*s+7]) for s in range(4)]
 bidder=seed%4;decl=(*range(8),9)[seed%9];leader=bidder;remaining=list(map(set,hands));trick=[];plays=[]
 for ply in range(24):
  seat=(leader+len(trick))%4;tile=rng.choice(legal(remaining[seat],trick,decl));remaining[seat].remove(tile);plays.extend([seat,tile]);trick.append((seat,tile))
  if len(trick)==4:leader=winner(trick,decl);trick=[]
  if ply+1 not in [20,21,22,23]:continue
  points,lead,remain,tail=replay(hands,plays,decl,bidder);actor=(lead+len(tail))%4
  req=dict(decl=decl,bid=30,bidder=bidder,seat=actor,hand=hands[actor],plays=plays[:]);masks=m.void_masks(req);past=[[t for s,t in zip(plays[::2],plays[1::2]) if s==a] for a in range(4)]
  # Verify the precondition omitted by optimized acceptance: at every off-suit
  # observation, later exposed own tiles also do not follow its led suit.
  tr=[]
  for i,(s,t) in enumerate(zip(plays[::2],plays[1::2])):
   if tr:
    led=m.context(tr[0][1],decl)
    if not m.follows(t,led,decl):
     observations+=1
     for ss,tt in zip(plays[2*(i+1)::2],plays[2*(i+1)+1::2]):
      if ss==s:assert not m.follows(tt,led,decl);known_future+=1
   tr.append((s,t))
   if len(tr)==4:tr=[]
  hidden=sorted(t for s in range(4) if s!=actor for t in remain[s]);seats=[s for s in range(4) if s!=actor];sizes=[len(remain[s]) for s in seats]
  for p in parts(hidden,sizes):
   final=[set(remain[actor]) if s==actor else set(p[seats.index(s)]) for s in range(4)]
   full=[hands[actor] if s==actor else past[s]+sorted(final[s]) for s in range(4)]
   optimized=all(not (sum(1<<t for t in final[s]) & masks[s]) for s in range(4))
   try:pp,ll,rr,tt=replay(full,plays,decl,bidder);lawful=True
   except AssertionError:lawful=False
   assert lawful==optimized,(seed,ply,full,masks)
   if lawful:assert (pp,ll,tt)==(points,lead,tail);assert rr==final;accepted+=1
   else:rejected+=1
   checked+=1
  if points[bidder%2]>=30 or points[1-bidder%2]>12:after_settled+=1
  histories+=1
# Fresh exact stream checks at original and prospective seed ranges, including
# preexcluded rows and every ordered duplicate world/tape/proposal count.
stream_rows=0
for start,count in [(962000,32),(967000,128),(981000,32)]:
 for n in [8,40]:
  for i in range(count):
   assert m.original(start+i,16+i%4,n)==m.compiled_fixture(start+i,16+i%4,n);stream_rows+=1
# Audit owner's repeated timing arithmetic and saved exact ordered input bytes.
panels=[]
for folder in ['sampling-primary-final','sampling-holdout-final']:
 base=HERE/'results'/folder
 if not (base/'summary.json').exists():continue
 s=json.loads((base/'summary.json').read_text());a=s['arguments']
 for p in s['panels']:
  n=p['worlds'];rr=[r for r in s['records'] if r['worlds']==n];assert len(rr)==2*a['repeats']
  with gzip.open(base/f'inputs-{n}.json.gz','rt') as f:rows=json.load(f)
  assert rows==json.loads(json.dumps([m.original(a['start']+i,16+i%4,n) for i in range(a['count'])]))
  assert p['sampling_attempts']==sum(r.get('sampling_attempts',r.get('attempts',0)) for r in rows)
  sha=hashlib.sha256(json.dumps(rows,sort_keys=True).encode()).hexdigest();assert all(v==sha for v in p['exact_rows_sha256'].values())
  for name in ['original','compiled']:
   vv=[r for r in rr if r['variant']==name]
   for key,out in [('wall_s','generation_s'),('cpu_s','generation_cpu_s')]:assert math.isclose(statistics.median(r[key] for r in vv),p['variants'][name][out],rel_tol=1e-12)
  assert math.isclose(p['original_over_compiled'],p['variants']['original']['generation_s']/p['variants']['compiled']['generation_s'],rel_tol=1e-12)
  assert all(r['order']==(['original','compiled'] if r['repeat']%2==0 else ['compiled','original']) for r in rr)
  panels.append({'folder':folder,'worlds':n,'ratio':p['original_over_compiled'],'proposed':len(rows)})
print(json.dumps({'exhaustive_histories':histories,'complete_unknown_partitions':checked,'accepted':accepted,'rejected':rejected,'all_nine_declarations':True,'off_suit_observations':observations,'known_future_tiles_checked':known_future,'histories_after_contract_settled':after_settled,'fresh_exact_stream_rows':stream_rows,'sampling_timing_arithmetic':panels,'scope':'Known-lawful generator only. Full replay reference runs entire history. Complete capacity partitions finite20..23ply. No arbitrary malformed request validation or behavioral posterior claim.'},indent=2))
