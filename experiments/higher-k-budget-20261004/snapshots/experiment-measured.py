#!/usr/bin/env python3
"""Local frozen k ladder. Execute each command ONLY through run_capped <=295s."""
import argparse,collections,concurrent.futures,hashlib,importlib.util,json,math,os,random,select,statistics,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
spec=importlib.util.spec_from_file_location('prior',HERE.parent/'native-policy-check-20261004/play.py')
prior=importlib.util.module_from_spec(spec);spec.loader.exec_module(prior)
phone=prior.phone
NATIVE=HERE/'adapter/target/release/higher-k-player'
WASM=HERE/'adapter/target/wasm32-unknown-unknown/release/higher_k_player.wasm'
PLAN=HERE/'plan.json';FIXTURES=HERE/'fixtures.json'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(p,v):
 p=Path(p);assert not p.exists(),f'preserve {p}';p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,separators=(',',':'))+'\n')
def load(p):return json.loads(Path(p).read_text())
def identity():
 return dict(native_sha256=sha(NATIVE),wasm_sha256=sha(WASM),plan_sha256=sha(PLAN),fixtures_sha256=sha(FIXTURES),source_sha256={str(p.relative_to(ROOT)):sha(p)for p in [Path(__file__),HERE/'adapter/src/lib.rs',HERE/'adapter/src/main.rs',HERE.parent/'native-policy-check-20261004/wasm_worker.mjs',ROOT/'experiments/partnership/rules.py']})
def initialize():
 # Fresh PRNG draws; declaration fixed by index, never selected by outcome.
 seeds=random.Random(202610041202).sample(range(3000000,4000000),54)
 fixtures=[]
 for i,seed in enumerate(seeds):
  decl=list(range(8))+[9];decl=decl[i%9]
  deck=list(range(28));random.Random(seed).shuffle(deck);hands=[sorted(deck[s*7:(s+1)*7])for s in range(4)]
  rng=random.Random(seed^0x507265666978);remain=list(map(set,hands));plays=[];leader=0;prefixes={}
  for t in range(4):
   trick=[]
   for j in range(4):
    seat=(leader+j)%4;tile=rng.choice(phone.legal_tiles(remain[seat],trick,decl));remain[seat].remove(tile);plays.extend([seat,tile]);trick.append((seat,tile))
   leader=phone.winner(trick,decl)
   if t in (2,3):prefixes[str(4*(t+1))]=list(plays)
  fixtures.append(dict(seed=seed,decl=decl,hands=hands,prefixes=prefixes))
 plan=dict(schema='higher-k-frozen-plan-v1',source_checkpoint='56794dc321f3546223ff1359ed30b87911130fd9',tier='exploratory finite local experiment',seed_generator='Random(202610041202).sample(range(3000000,4000000),54)',declarations=list(range(8))+[9],deals=54,rotations=[0,1,2,3],roles=['declaring','defending'],prefix_plies=16,prefix_policy='Uniform choice from ascending legal list using Random(source_seed xor0x507265666978), independent of all tested k. Rotate complete hands and played seats together.',independence='Source PRNG deal draws assumed independent. Four rotations/two roles share a deal and are one cluster. This is the random-prefix suffix distribution, not a production-game or entire-player strength panel.',objective='pmake declaring bid30; defenders minimize same binary value',outer_worlds=40,inner_budgets=[4,2,2,2,2,2],inner_belief='Voidless',selection='Fixed ascending-tile ties',rung='k=1..5 is outer best response against Field::Level(k-1); modeled L0 responds to Dice',root_fixed_budgets_ms=[20,200],root_k=[1,2,3,4,5],unconstrained='First9 source deals: all4 rotations at16 plies, one rotation at12 plies, k1..5, nominal30s/request. Refusal is censored work, never unconstrained-complete cost.',matches=[dict(reference='k1',candidate=k,budget_ms=b)for b in [20,200]for k in [3,5]]+[dict(reference='phone',candidate=k,budget_ms=200)for k in [1,3,5]],reference_phone='Unmodified pinned production L1/partner/worlds40 at experimental200ms nominal allowance; not ordinary production14s profile or physical phone.',fallback='ascending first legal tile on solver/sampler refusal, forced or settled; no partial value used; no additional solve budget',budgets='Nominal allowance includes status/replay/outer sampling/solver. Call wall includes transport/process startup; all overruns recorded. Validation disabled during performance. Every invocation capped <=295s. All cells planned, refusals retained.',parallel='Frozen complete k5 position queries; one/four serial solver worker processes, no recursive internal rayon. Every query same30s allowance. Fresh caches each request; completed values/order/RNG unchanged. Whole batch charges process/setup/sampling/JSON/teardown; CPU and sum PID maxima reported separately.',strength_claim=False,seeds=seeds)
 save(PLAN,plan);save(FIXTURES,fixtures)
 print(json.dumps(dict(plan_sha256=sha(PLAN),fixtures_sha256=sha(FIXTURES),deals=len(fixtures))))
def rotated(f,rotation,plies):
 hands=[f['hands'][(s-rotation)%4]for s in range(4)]
 p=f['prefixes'][str(plies)];plays=[v if j%2 else (v+rotation)%4 for j,v in enumerate(p)]
 return hands,plays
class Native:
 def __init__(self):self.proc=None;self.usage=[];self.buffer=b''
 def start(self):self.proc=subprocess.Popen([str(NATIVE)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,bufsize=0)
 def call(self,call):
  start=time.monotonic()
  if self.proc is None:self.start()
  self.proc.stdin.write((json.dumps(call)+'\n').encode());self.proc.stdin.flush()
  end=start+call.get('budget_ms',30000)/1000+2
  while b'\n'not in self.buffer:
   left=end-time.monotonic()
   if left<=0 or not select.select([self.proc.stdout],[],[],left)[0]:raise TimeoutError('native host deadline; no partial vector admitted')
   chunk=os.read(self.proc.stdout.fileno(),65536)
   if not chunk:raise RuntimeError('native exit')
   self.buffer+=chunk;assert len(self.buffer)<2_000_000
  line,self.buffer=self.buffer.split(b'\n',1);v=json.loads(line);assert 'error'not in v,v
  ms=(time.monotonic()-start)*1000
  return v,dict(host_ms=ms,host_overrun_ms=max(0,ms-call.get('budget_ms',0)),wrapper_overrun_ms=max(0,v.get('elapsed_us',0)/1000-call.get('budget_ms',0)),interrupted=False)
 def close(self):
  if self.proc:self.usage.append(prior.close_process(self.proc));self.proc=None

def req_for(hands,plays,decl,bidder):
 _,leader,_,trick=phone.replay_record(hands,plays,decl,bidder)
 seat=(leader+len(trick))%4
 return phone.make_request(hands,seat,bidder,plays,decl)
def ladder(req,k,budget,validate=False):return dict(action='ladder',request=req,k=k,budget_ms=budget,validate=validate)
def response(worker,req,policy,budget):
 if policy=='phone':return worker.call(dict(action='phone',request=req,worlds=40,partner=True,budget_ms=budget))
 return worker.call(ladder(req,int(policy),budget))
def roots(seed,unconstrained=False):
 f=next(f for f in load(FIXTURES)if f['seed']==seed);records=[];w=Native();started=time.monotonic()
 partial=HERE/'results'/('unconstrained'if unconstrained else 'fixed-roots')/f'partial-{seed}.jsonl';partial.parent.mkdir(parents=True,exist_ok=True);assert not partial.exists()
 try:
  stages=[(16,r)for r in range(4)]+([(12,0)]if unconstrained else [])
  for plies,r in stages:
   hands,plays=rotated(f,r,plies);req=req_for(hands,plays,f['decl'],r)
   for b in ([30000]if unconstrained else [20,200]):
    # Rotate rung position by deal/rotation to distribute process order.
    order=list(range(1,6));offset=(seed+r+(b==200))%5;order=order[offset:]+order[:offset]
    for k in order:
     if time.monotonic()-started>250:raise TimeoutError('split bounded source batch')
     call=ladder(req,k,b);v,t=response(w,req,k,b)
     record=dict(seed=seed,decl=f['decl'],rotation=r,plies=plies,call=call,response=v,timing=t);records.append(record)
     with partial.open('a')as stream:stream.write(json.dumps(record,separators=(',',':'))+'\n')
 finally:w.close()
 save(HERE/'results'/('unconstrained'if unconstrained else 'fixed-roots')/f'deal-{seed}.json',dict(records=records,usage=w.usage,elapsed_s=time.monotonic()-started,identity=identity()))
 print(json.dumps(dict(seed=seed,calls=len(records),refusals=sum('refusal'in q['response']for q in records),seconds=time.monotonic()-started)))
def suffix_game(f,r,role,reference,candidate,budget,w):
 hands,plays=rotated(f,r,16);initial=list(plays);bidder=r;team=(bidder+(role=='defending'))%2;moves=[];started=time.monotonic()
 for _ in range(12):
  req=req_for(hands,plays,f['decl'],bidder);seat=req['seat'];pol=candidate if seat%2==team else reference
  v,t=response(w,req,pol,budget);st=phone.information_state(req)
  assert v['choice']in st['legal'] and v['legal']==st['legal'] and v['points']==st['points'] and v['leader']==st['leader']
  moves.append(dict(request=req,response=v,timing=t,candidate=seat%2==team,policy=pol));plays.extend([seat,v['choice']])
 points,_,remain,tail=phone.replay_record(hands,plays,f['decl'],bidder);assert sum(points)==42 and not tail and all(not h for h in remain)
 return dict(seed=f['seed'],decl=f['decl'],rotation=r,role=role,reference=reference,candidate=candidate,budget_ms=budget,initial_plays=initial,points=points,made=points[bidder%2]>=30,moves=moves,complete=True,elapsed_s=time.monotonic()-started)
def games(seed):
 f=next(f for f in load(FIXTURES)if f['seed']==seed);rows=[];w=Native();started=time.monotonic()
 try:
  cells=load(PLAN)['matches'];offset=seed%len(cells);cells=cells[offset:]+cells[:offset]
  for cell in cells:
   for r in range(4):
    for role in ['declaring','defending']:
     if time.monotonic()-started>250:raise TimeoutError('bounded source match block incomplete')
     ref=1 if cell['reference']=='k1'else 'phone'
     rows.append(suffix_game(f,r,role,ref,cell['candidate'],cell['budget_ms'],w))
 finally:w.close()
 save(HERE/'results'/'games'/f'deal-{seed}.json',dict(games=rows,usage=w.usage,elapsed_s=time.monotonic()-started,identity=identity()))
 print(json.dumps(dict(seed=seed,games=len(rows),seconds=time.monotonic()-started)))
def stats(xs):
 xs=sorted(xs)
 return dict(n=len(xs),sum=sum(xs),median=statistics.median(xs)if xs else None,p95=xs[math.ceil(.95*len(xs))-1]if xs else None,max=max(xs)if xs else None)
def summarize():
 plan=load(PLAN);files=list((HERE/'results/games').glob('deal-*.json'));groups=[load(p)for p in files];games=[g for f in groups for g in f['games']]
 assert sorted(f['games'][0]['seed']for f in groups)==sorted(plan['seeds'])
 result=dict(schema='higher-k-summary-v1',source_deals=54,suffix_games=len(games),suffix_moves=sum(len(g['moves'])for g in games),strength_claim=False,matches=[])
 for cell in plan['matches']:
  ref=1 if cell['reference']=='k1'else 'phone';rows=[g for g in games if(g['reference'],g['candidate'],g['budget_ms'])==(ref,cell['candidate'],cell['budget_ms'])]
  index={(g['seed'],g['rotation'],g['role']):g for g in rows};assert len(index)==54*8
  pairs=[];clusters=[]
  for seed in plan['seeds']:
   dd=[int(index[seed,r,'declaring']['made'])-int(index[seed,r,'defending']['made'])for r in range(4)];pairs+=dd;clusters.append(dict(seed=seed,differences=dd,mean=sum(dd)/4))
  mean=statistics.mean(q['mean']for q in clusters);radius=math.sqrt(2*math.log(40)/54)
  out=dict(**cell,games=len(rows),clusters=clusters,pair_wins=pairs.count(1),pair_losses=pairs.count(-1),pair_ties=pairs.count(0),mean=mean,hoeffding95=[max(-1,mean-radius),min(1,mean+radius)],simultaneous_family_claim=False)
  for cand,label in [(True,'candidate'),(False,'reference')]:
   mm=[m for g in rows for m in g['moves']if m['candidate']==cand];attempts=[m for m in mm if not m['response'].get('ineligible') and 'completed'in m['response']]
   out[label]=dict(turns=len(mm),host_ms=stats([m['timing']['host_ms']for m in mm]),scheduled_ms=sum(g['budget_ms']*sum(m['candidate']==cand for m in g['moves'])for g in rows),ladder_attempts=len(attempts),completed=sum(m['response']['completed']for m in attempts),refused=sum('refusal'in m['response']for m in attempts),refusal_reasons=dict(collections.Counter(m['response'].get('refusal')for m in attempts if 'refusal'in m['response'])),nodes=sum(m['response'].get('nodes',0)for m in attempts),pi_computations_by_level=[sum(m['response'].get('pi_calls_by_level',[0]*6)[k]for m in attempts)for k in range(6)],host_overruns=stats([m['timing']['host_overrun_ms']for m in mm]),wrapper_overruns=stats([m['timing']['wrapper_overrun_ms']for m in mm]))
  result['matches'].append(out)
 root_files=[load(p)for p in (HERE/'results/fixed-roots').glob('deal-*.json')];assert len(root_files)==54
 rootrows=[q for f in root_files for q in f['records']];result['fixed_roots']=[]
 for b in [20,200]:
  for k in range(1,6):
   qs=[q for q in rootrows if q['call']['k']==k and q['call']['budget_ms']==b];assert len(qs)==216
   eligible=[q for q in qs if not q['response'].get('ineligible')]
   result['fixed_roots'].append(dict(k=k,budget_ms=b,planned=len(qs),eligible=len(eligible),completed=sum(q['response']['completed']for q in eligible),refused=sum('refusal'in q['response']for q in eligible),host_ms=stats([q['timing']['host_ms']for q in qs]),sample_us=stats([q['response'].get('outer_sample_us',0)for q in eligible]),nodes=stats([q['response'].get('nodes',0)for q in eligible]),pi_computations_by_level=[sum(q['response'].get('pi_calls_by_level',[0]*6)[lev]for q in eligible)for lev in range(6)],inner_worlds_by_level=[sum(q['response'].get('inner_worlds_by_level',[0]*6)[lev]for q in eligible)for lev in range(6)],policy_cache_entries=stats([q['response'].get('policy_cache_entries',0)for q in eligible])))
 usage=[u for f in groups+root_files for u in f['usage']];result['worker_resources']=dict(cpu_s=sum(q['cpu_s']for q in usage),max_pid_rss_bytes=max(q['peak_rss_bytes']for q in usage),pid_count=len(usage),note='Per-PID lifetime maximum, not concurrent aggregate; fixed root and suffix runs combined. Full capped walls include referee/driver work separately.')
 result['uncertainty']=plan['independence'];save(HERE/'results/summary.json',result);print(json.dumps({k:result[k]for k in ['source_deals','suffix_games','suffix_moves']}))
def parallel(workers,rep):
 fxs=load(FIXTURES)[:9];calls=[]
 for f in fxs:
  for r in range(4):
   hands,plays=rotated(f,r,12);req=req_for(hands,plays,f['decl'],r)
   calls.append(ladder(req,5,30000))
 start=time.monotonic()
 def chunk(i):
  w=Native();records=[]
  try:
   for j in range(i,len(calls),workers):
    v,t=w.call(calls[j]);records.append(dict(index=j,call=calls[j],response=v,timing=t))
  finally:w.close()
  return dict(records=records,usage=w.usage)
 with concurrent.futures.ThreadPoolExecutor(max_workers=workers)as pool:rr=list(pool.map(chunk,range(workers)))
 wall=(time.monotonic()-start)*1000;records=sorted([q for x in rr for q in x['records']],key=lambda q:q['index']);usage=[u for x in rr for u in x['usage']]
 save(HERE/'results/parallel'/f'workers-{workers}-rep-{rep}.json',dict(workers=workers,rep=rep,batch_ms=wall,records=records,usage=usage,cpu_ms=1000*sum(u['cpu_s']for u in usage),sum_pid_peak_rss_bytes=sum(u['peak_rss_bytes']for u in usage),identity=identity()))
 print(json.dumps(dict(workers=workers,rep=rep,batch_ms=wall,completed=sum(q['response']['completed']for q in records),refused=sum('refusal'in q['response']for q in records))))
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('cmd',choices=['initialize','roots','unconstrained','games','summarize','parallel']);p.add_argument('--seed',type=int);p.add_argument('--workers',type=int,choices=[1,4]);p.add_argument('--rep',type=int);a=p.parse_args()
 if a.cmd=='initialize':initialize()
 elif a.cmd in ['roots','unconstrained']:roots(a.seed,a.cmd=='unconstrained')
 elif a.cmd=='games':games(a.seed)
 elif a.cmd=='summarize':summarize()
 else:parallel(a.workers,a.rep)
