#!/usr/bin/env python3
"""Fresh balanced complete-game replay using reviewer-local mechanics."""
import hashlib,json,random,subprocess,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/resumable-choice-20261004';base=HERE/'results/h2h';binary=HERE/'integration/target/release/late-choice-player'
T=[(h,l)for h in range(7)for l in range(h+1)]
def trump(t,d):return d in T[t] if d<=6 else d==7 and T[t][0]==T[t][1]
def ctx(t,d):return -1 if trump(t,d)else T[t][0]
def follows(t,led,d):return trump(t,d)if led==-1 else not trump(t,d)and led in T[t]
def legal(hand,trick,d):
 allowed=[t for t in hand if trick and follows(t,ctx(trick[0][1],d),d)];return sorted(allowed or hand)
def win(trick,d):
 def rank(p):
  t=p[1];h,l=T[t];return (2 if trump(t,d)else 1 if follows(t,ctx(trick[0][1],d),d)else 0,h if h==l and d==7 else 12 if h==l else h+l)
 return max(trick,key=rank)[0]
def points(trick):return 1+sum(sum(T[t])if sum(T[t])in(5,10)else 0 for _,t in trick)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
rows=[json.loads(p.read_text())for p in sorted(base.glob('game-*.json'))];s=json.loads((base/'summary.json').read_text());assert len(rows)==8 and s['games']==8 and s['moves']==224
keys={(r['seed'],r['rotation'],r['role'])for r in rows};assert len(keys)==8;assert {r['seed']for r in rows}=={975102};assert keys=={(975102,i,role)for i in range(4)for role in ['declaring','defending']}
late=0;candidate=[];baseline=[];fresh=0;signatures=set()
for r in rows:
 assert r['complete'] and len(r['moves'])==28 and r['decl']==6 and r['bid']==30 and r['variant']=='resumable';assert r['binary_sha256']==sha(binary)==s['binary_sha256'];assert r['phone_wasm_sha256']==sha(ROOT/'experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm')==s['phone_wasm_sha256']
 for name,path in [('arena.py',HERE/'arena.py'),('player.py',HERE/'player.py'),('main.rs',HERE/'integration/src/main.rs'),('rules.py',ROOT/'experiments/partnership/rules.py')]:assert r['source_sha256'][name]==sha(path)
 deck=list(range(28));random.Random(r['seed']).shuffle(deck);hh=[sorted(deck[7*i:7*i+7])for i in range(4)];hh=[hh[(i-r['rotation'])%4]for i in range(4)];assert hh==r['hands'] and r['bidder']==r['rotation'];assert sorted(t for h in hh for t in h)==list(range(28))
 remaining=list(map(set,hh));leader=r['bidder'];trick=[];plays=[];score=[0,0];team=(r['bidder']+(r['role']=='defending'))%2
 for idx,m in enumerate(r['moves']):
  actor=(leader+len(trick))%4;c=m['call'];req=c['request'];res=m['response'];allowed=legal(remaining[actor],trick,r['decl']);assert set(req)=={'decl','bid','bidder','seat','hand','plays','seed'}
  assert req==dict(decl=r['decl'],bid=30,bidder=r['bidder'],seat=actor,hand=hh[actor],plays=plays,seed=7042104)
  opening=actor==r['bidder']and not plays;assert c['worlds']==(160 if opening else 40) and c['partner']==(not opening) and c['budget_ms']==(20000 if opening else 14000)
  assert m['candidate']==(actor%2==team);assert res['legal']==allowed and res['choice']in allowed and res['points']==score and res['leader']==leader and res['trick']==idx//4+1;assert m['timing']['host_ms']>=0
  (candidate if m['candidate']else baseline).append(m)
  if res['route']=='late-l3-resumable-40-4-2-2':
   assert m['candidate'] and res['kernel_variant']=='resumable' and 16<=idx<=24 and len(allowed)>1 and score[r['bidder']%2]<30 and score[1-r['bidder']%2]<=12
   p=subprocess.run([str(binary)],input=json.dumps(dict(request=req,variant='native',validate=True))+'\n',capture_output=True,text=True,timeout=8);assert p.returncode==0;ref=json.loads(p.stdout);assert ref['validated']and ref['evaluation']==res['evaluation'];late+=1;fresh+=1;signatures.add(json.dumps(req,sort_keys=True))
  tile=res['choice'];remaining[actor].remove(tile);plays.extend([actor,tile]);trick.append((actor,tile))
  if len(trick)==4:leader=win(trick,r['decl']);score[leader%2]+=points(trick);trick=[]
 assert score==r['points'] and sum(score)==42 and all(not h for h in remaining)and not trick;assert r['made']==(score[r['bidder']%2]>=30)
paired=[]
for i in range(4):
 x=next(r for r in rows if r['rotation']==i and r['role']=='declaring');y=next(r for r in rows if r['rotation']==i and r['role']=='defending');paired.append(int(x['made'])-int(y['made']))
assert s['source_deals']==1 and s['missing_pairs']==[] and s['strength_claim']is False;assert s['pair_wins']==paired.count(1)and s['pair_losses']==paired.count(-1)and s['pair_ties']==paired.count(0);assert s['mean_paired_make_difference']==sum(paired)/4;assert s['late_completed']==late and s['late_refusals']==sum(bool(m['timing'].get('late_refusal'))for m in candidate)
for field,moves in [('candidate_host_ms',candidate),('phone_host_ms',baseline),('late_host_ms',[m for m in candidate if m['response']['route']=='late-l3-resumable-40-4-2-2'])]:assert abs(s[field]-sum(m['timing']['host_ms']for m in moves))<1e-8
print(json.dumps(dict(complete_games=len(rows),moves=224,independent_local_mechanics_replay=True,regenerated_full_deal_and_rotations=True,seven_field_actor_boundary_every_turn=True,current_phone_source_and_compiled_candidate_identities=True,all_late_complete_vectors_fresh_native=fresh,distinct_late_public_signatures=len(signatures),late_completed=late,paired_differences=paired,summary_arithmetic_verified=True,scope='One independent generating deal, four correlated rotations and two team assignments. Complete game/adapter smoke evidence; no independent-deal strength estimate. Conservative independent-deal95% interval remains[-1,1]. Early/forced/settled turns use pinned phone procedure; experimental late L3 policy differs from phone L1/partner.'),indent=2))
