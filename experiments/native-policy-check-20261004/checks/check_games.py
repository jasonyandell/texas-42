#!/usr/bin/env python3
"""Fresh full-game audit with reviewer-local mechanics and arithmetic."""
import argparse,collections,hashlib,json,math,random,statistics,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/native-policy-check-20261004';CHECK=HERE/'checks'
T=[(hi,lo)for hi in range(7)for lo in range(hi+1)]
def trump(t,d):return d in T[t]if d<=6 else d==7 and T[t][0]==T[t][1]
def ctx(t,d):return -1 if trump(t,d)else T[t][0]
def follows(t,led,d):return trump(t,d)if led==-1 else not trump(t,d)and led in T[t]
def legal(hand,trick,d):
 f=[t for t in hand if trick and follows(t,ctx(trick[0][1],d),d)];return sorted(f or hand)
def win(trick,d):
 def rank(p):
  t=p[1];h,l=T[t];return(2 if trump(t,d)else 1 if follows(t,ctx(trick[0][1],d),d)else 0,h if h==l and d==7 else 12 if h==l else h+l)
 return max(trick,key=rank)[0]
def points(trick):return 1+sum(sum(T[t])if sum(T[t])in(5,10)else 0 for _,t in trick)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def audit(a):
 plan=json.loads((HERE/'plan.json').read_text());expected=[r for r in plan['games']if r['backend']==a.backend];rows=[json.loads(p.read_text())for p in sorted(a.directory.glob('game-*.json'))];summary=json.loads((a.directory/'summary.json').read_text());assert len(rows)==len(expected)==144
 keys={(r['seed'],r['rotation'],r['role'],r['decl'],r['backend'])for r in rows};assert len(keys)==144;assert keys=={(r['seed'],r['rotation'],r['role'],r['decl'],r['backend'])for r in expected}
 binary=HERE/'adapter/target'/('release/native-late-player'if a.backend=='native'else'wasm32-unknown-unknown/release/native_late_player.wasm');refbinary=HERE/'adapter/target/release/native-late-player';candidate=[];phone=[];unique_late={};late=0;refusals=0
 for r in rows:
  assert r['candidate_binary_sha256']==sha(binary);assert r['phone_wasm_sha256']==sha(ROOT/'experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm')
  for name,h in r['source_sha256'].items():assert sha((ROOT if name.startswith('experiments/')else HERE)/name)==h,name
  deck=list(range(28));random.Random(r['seed']).shuffle(deck);base=[sorted(deck[7*s:7*s+7])for s in range(4)];hands=[base[(s-r['rotation'])%4]for s in range(4)];assert hands==r['hands']and r['bidder']==r['rotation'];assert r['bid']==30 and r['complete']and len(r['moves'])==28
  remain=list(map(set,hands));trick=[];plays=[];leader=r['bidder'];score=[0,0];team=(r['bidder']+(r['role']=='defending'))%2
  for index,m in enumerate(r['moves']):
   actor=(leader+len(trick))%4;c=m['call'];req=c['request'];answer=m['response'];allowed=legal(remain[actor],trick,r['decl'])
   assert req==dict(decl=r['decl'],bid=30,bidder=r['bidder'],seat=actor,hand=hands[actor],plays=plays,seed=7042104);assert len(req)==7
   opening=actor==r['bidder']and not plays;assert c['worlds']==(160 if opening else 40)and c['budget_ms']==(20000 if opening else 14000)and c['partner']==(not opening)
   assert m['candidate']==(actor%2==team);assert answer['choice']in allowed and answer['legal']==allowed and answer['points']==score and answer['leader']==leader and answer['trick']==index//4+1
   assert m['timing']['host_ms']>=0 and type(m['timing']['interrupted'])is bool;(candidate if m['candidate']else phone).append(m)
   if answer['route']=='late-l3-native-40-4-2-2':
    assert m['candidate']and 16<=index<=24 and len(allowed)>1 and score[r['bidder']%2]<30 and score[1-r['bidder']%2]<=12;sig=json.dumps(req,sort_keys=True);e=answer['evaluation'];assert e['tiles']==allowed;vals=e['counts'];best=max(vals)if actor%2==r['bidder']%2 else min(vals);assert e['choice']==next(t for t,n in zip(allowed,vals)if n==best)
    if sig in unique_late:assert unique_late[sig][1]==e
    else:unique_late[sig]=(req,e)
    late+=1
   refusals+=bool(m['timing'].get('late_refusal')or answer.get('late_refusal'));tile=answer['choice'];remain[actor].remove(tile);plays.extend([actor,tile]);trick.append((actor,tile))
   if len(trick)==4:leader=win(trick,r['decl']);score[leader%2]+=points(trick);trick=[]
  assert score==r['points']and sum(score)==42 and all(not h for h in remain)and not trick;assert r['made']==(score[r['bidder']%2]>=30)
  assert all(u.get('cpu_s',-1)>=0 and u.get('peak_rss_bytes',0)>0 for k in ['phone_usage','candidate_usage','candidate_phone_usage']for u in r[k])
 fresh=0
 for req,e in unique_late.values():
  p=subprocess.run([str(refbinary)],input=json.dumps(dict(action='late',request=req,validate=True))+'\n',capture_output=True,text=True,timeout=8);assert p.returncode==0;ref=json.loads(p.stdout);assert ref['validated']and ref['evaluation']==e;fresh+=1
 groups={(r['seed'],r['rotation'],r['role']):r for r in rows};clusters=[];pairs=[]
 for seed in sorted({r['seed']for r in rows}):
  ds=[int(groups[(seed,rot,'declaring')]['made'])-int(groups[(seed,rot,'defending')]['made'])for rot in range(4)];clusters.append(sum(ds)/4);pairs+=ds
 assert summary['complete_games']==144 and summary['moves']==4032 and summary['complete_source_deals']==18 and not summary['missing']
 assert summary['pair_wins']==pairs.count(1)and summary['pair_losses']==pairs.count(-1)and summary['pair_ties']==pairs.count(0);mean=sum(clusters)/len(clusters);radius=math.sqrt(2*math.log(40)/len(clusters));assert summary['mean_paired_make_advantage']==mean;assert summary['hoeffding95']==[max(-1,mean-radius),min(1,mean+radius)]
 for moves,label in [(candidate,'candidate'),(phone,'phone')]:
  times=sorted(m['timing']['host_ms']for m in moves);s=summary[label];assert s['turns']==len(moves)==2016;assert math.isclose(s['host_total_ms'],sum(times),rel_tol=1e-12);assert s['host_median_ms']==statistics.median(times)and s['host_p95_ms']==times[math.ceil(.95*len(times))-1];assert s['routes']==dict(collections.Counter(m['response']['route']for m in moves))
 assert summary['candidate']['late_completed']==late and summary['candidate']['late_refusals']==refusals
 print(json.dumps(dict(backend=a.backend,complete_games=len(rows),independent_source_deals=18,independently_replayed_moves=len(rows)*28,late_complete_vectors=late,unique_late_vectors_freshly_native_revalidated=fresh,late_refusals=refusals,pair_wins=pairs.count(1),pair_losses=pairs.count(-1),pair_ties=pairs.count(0),mean_paired_make_advantage=mean,hoeffding95=summary['hoeffding95'],strength_claim=False,scope='Finite prespecified balanced fixedbid30 panel. Intervals conditional on independent source-deal sampling, rotations clustered. No generic play-strength or phone-device timing theorem.'),indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('directory',type=Path);p.add_argument('--backend',choices=['native','wasm'],required=True);audit(p.parse_args())
