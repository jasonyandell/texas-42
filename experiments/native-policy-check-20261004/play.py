#!/usr/bin/env python3
"""Independent-deal balanced full games. Every batch uses run_capped <=295s."""
import argparse,collections,hashlib,importlib.util,json,math,os,random,select,statistics,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('phone',HERE.parent/'astra-sol-20261004/tools/arena.py')
phone=importlib.util.module_from_spec(spec);spec.loader.exec_module(phone)
NATIVE=HERE/'adapter/target/release/native-late-player'
WASM=HERE/'adapter/target/wasm32-unknown-unknown/release/native_late_player.wasm'

def close_process(proc):
 if proc is None:return None
 if proc.poll() is None:proc.kill()
 try:
  _,status,usage=os.wait4(proc.pid,0)
  proc.returncode=os.waitstatus_to_exitcode(status)
  result=dict(cpu_s=usage.ru_utime+usage.ru_stime,peak_rss_bytes=usage.ru_maxrss*(1 if sys.platform=='darwin' else 1024))
 except ChildProcessError:result=dict(usage_unavailable=True)
 for stream in (proc.stdin,proc.stdout,proc.stderr):stream.close()
 return result

class Worker(phone.Worker):
 def __init__(self,wasm):super().__init__(wasm);self.usage=[];self.memory_peak=0
 def start(self):
  self.proc=subprocess.Popen(['node',str(HERE/'wasm_worker.mjs'),str(self.wasm)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,bufsize=0)
 def close(self):
  if self.proc:self.usage.append(close_process(self.proc));self.proc=None;self.buffer=b''
 def call(self,call,deadline):
  # Preserve independent original protocol including checkpoints and host cap.
  start=time.monotonic()
  if self.proc is None:self.start()
  end=min(deadline,start+call.get('budget_ms',14000)/1000+4)
  self.proc.stdin.write((json.dumps(call)+'\n').encode());self.proc.stdin.flush()
  saved=None;checkpoints=0
  def interrupted(reason):
   self.close()
   if saved is None:raise RuntimeError(reason+'; no retained checkpoint')
   return saved,dict(host_ms=(time.monotonic()-start)*1000,checkpoints=checkpoints,interrupted=True,interruption_reason=reason)
  while True:
   left=end-time.monotonic()
   if left<=0 or not select.select([self.proc.stdout],[],[],left)[0]:
    return interrupted('host deadline')
   chunk=os.read(self.proc.stdout.fileno(),65536)
   if not chunk:
    return interrupted('worker exit')
   self.buffer+=chunk
   assert len(self.buffer)<2000000
   while b'\n' in self.buffer:
    line,self.buffer=self.buffer.split(b'\n',1);msg=json.loads(line)
    if 'error' in msg:return interrupted(msg['error'])
    if 'checkpoint' in msg:saved=msg['checkpoint'];checkpoints+=1
    if 'result' in msg:
     assert 'error' not in msg['result'],msg
     self.memory_peak=max(self.memory_peak,msg['wasm_memory_bytes'])
     return msg['result'],dict(host_ms=(time.monotonic()-start)*1000,checkpoints=checkpoints,interrupted=False,initialization_ms=msg['initialization_ms'],wasm_memory_bytes=msg['wasm_memory_bytes'])

class NativeHybrid:
 def __init__(self):self.phone=Worker(phone.PHONE);self.proc=None;self.usage=[]
 def close(self):
  self.phone.close()
  if self.proc:self.usage.append(close_process(self.proc));self.proc=None
 def call(self,call,deadline):
  start=time.monotonic();req=call['request'];st=phone.information_state(req)
  eligible=16<=len(req['plays'])//2<=24 and len(st['legal'])>1 and st['points'][req['bidder']%2]<req['bid'] and st['points'][1-req['bidder']%2]<=42-req['bid']
  refusal=None
  if eligible:
   try:
    if self.proc is None:self.proc=subprocess.Popen([str(NATIVE)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,bufsize=1)
    remaining=max(0,call['budget_ms']-int((time.monotonic()-start)*1000))
    self.proc.stdin.write(json.dumps(dict(action='late',request=req,budget_ms=remaining))+'\n');self.proc.stdin.flush()
    if select.select([self.proc.stdout],[],[],min(4,max(0,deadline-time.monotonic())))[0]:
     result=json.loads(self.proc.stdout.readline())
     if 'error' not in result and not result.get('ineligible'):
      assert result['legal']==st['legal'] and result['points']==st['points'] and result['leader']==st['leader'] and result['choice'] in st['legal']
      return result,dict(host_ms=(time.monotonic()-start)*1000,checkpoints=1,interrupted=False,late_attempt=True,late_refusal=None)
     refusal=result.get('error','ineligible')
    else:refusal='late host deadline'
   except (OSError,ValueError,KeyError,AssertionError) as e:refusal=f'{type(e).__name__}: {e}'
   if refusal and self.proc:self.usage.append(close_process(self.proc));self.proc=None
  if eligible:
   remaining=max(0,call['budget_ms']-int((time.monotonic()-start)*1000))
   if remaining<100:
    return dict(choice=st['legal'][0],legal=st['legal'],points=st['points'],leader=st['leader'],route='legal-fallback',late_refusal=refusal),dict(host_ms=(time.monotonic()-start)*1000,checkpoints=1,interrupted=False,late_attempt=True,late_refusal=refusal)
   call=dict(call,budget_ms=remaining)
  answer,timing=self.phone.call(call,deadline)
  timing.update(host_ms=(time.monotonic()-start)*1000,late_attempt=eligible,late_refusal=refusal)
  return answer,timing

def game(seed,rotation,role,decl,backend,out):
 assert not out.exists(),'preserve games'
 out.parent.mkdir(parents=True,exist_ok=True)
 assert phone.sha(phone.PHONE)==json.loads((phone.ROOT/'reference/production-phone/manifest.json').read_text())['wasm_sha256']
 rng=random.Random(seed);deck=list(range(28));rng.shuffle(deck);base=[sorted(deck[7*s:7*s+7])for s in range(4)]
 hands=[base[(s-rotation)%4]for s in range(4)];bidder=rotation;team=(bidder+(role=='defending'))%2
 baseline=Worker(phone.PHONE);candidate=NativeHybrid() if backend=='native' else Worker(WASM)
 workers=[baseline,candidate];remain=list(map(set,hands));plays=[];moves=[];points=[0,0];leader=bidder
 started=time.monotonic();deadline=started+240
 try:
  for _ in range(7):
   trick=[]
   for _ in range(4):
    if time.monotonic()>=deadline:raise TimeoutError('game cap; incomplete blocks excluded')
    seat=(leader+len(trick))%4;is_candidate=seat%2==team
    req=phone.make_request(hands,seat,bidder,plays,decl);call=phone.profile(req,True)
    answer,timing=workers[int(is_candidate)].call(call,deadline)
    legal=phone.legal_tiles(remain[seat],trick,decl)
    assert answer['choice'] in legal and answer['legal']==legal and answer['points']==points and answer['leader']==leader
    moves.append(dict(call=call,response=answer,timing=timing,candidate=is_candidate));tile=answer['choice']
    remain[seat].remove(tile);plays.extend([seat,tile]);trick.append((seat,tile))
   leader=phone.winner(trick,decl);points[leader%2]+=phone.trick_points(trick)
 finally:
  for worker in workers:worker.close()
 audited,_,remaining,tail=phone.replay_record(hands,plays,decl,bidder)
 assert audited==points and sum(points)==42 and not tail and all(not h for h in remaining)
 sources=[HERE/'play.py',HERE/'wasm_worker.mjs',HERE/'adapter/src/lib.rs',HERE/'adapter/src/main.rs',phone.ROOT.parent/'partnership/rules.py']
 report=dict(schema='native-policy-balanced-game-v1',seed=seed,rotation=rotation,role=role,decl=decl,bid=30,bidder=bidder,hands=hands,points=points,made=points[bidder%2]>=30,moves=moves,complete=True,game_seconds=time.monotonic()-started,
  backend=backend,policy_seed=phone.PUBLIC_POLICY_SEED,late_policy='uniform40/voidless-inner/[4,2,2,2]/Field::Level(2)/ascending-ties',
  candidate_binary_sha256=phone.sha(NATIVE if backend=='native' else WASM),phone_wasm_sha256=phone.sha(phone.PHONE),
  source_sha256={str(p.relative_to(HERE)) if p.is_relative_to(HERE) else str(p.relative_to(HERE.parent.parent)):phone.sha(p)for p in sources},
  phone_usage=baseline.usage,candidate_usage=candidate.usage,candidate_phone_usage=candidate.phone.usage if backend=='native' else [],
  phone_linear_memory_peak=baseline.memory_peak,candidate_linear_memory_peak=candidate.phone.memory_peak if backend=='native' else candidate.memory_peak)
 out.write_text(json.dumps(report,separators=(',',':'))+'\n')
 print(json.dumps({k:report[k]for k in ['seed','rotation','role','decl','backend','points','made','game_seconds']}),flush=True)

def batch(a):
 plan=json.loads(a.plan.read_text());started=time.monotonic()
 for row in plan['games']:
  if row['seed']!=a.seed or row['backend']!=a.backend:continue
  if time.monotonic()-started>250:raise TimeoutError('bounded batch; rerun remaining games separately')
  out=a.out/f"game-{row['seed']}-{row['rotation']}-{row['role']}.json"
  if out.exists():raise ValueError('use single game command for partial retry; preserve receipts')
  game(**row,out=out)

def summarize(a):
 plan=json.loads(a.plan.read_text());expected=[r for r in plan['games']if r['backend']==a.backend]
 rows=[json.loads(p.read_text())for p in sorted(a.out.glob('game-*.json'))]
 groups={(r['seed'],r['rotation'],r['role']):r for r in rows};assert len(rows)==len(groups)
 assert rows and all(r['complete'] and len(r['moves'])==28 and r['backend']==a.backend for r in rows)
 signatures={(r['candidate_binary_sha256'],r['phone_wasm_sha256'],r['policy_seed'],r['late_policy'],json.dumps(r['source_sha256'],sort_keys=True))for r in rows};assert len(signatures)==1
 missing=[r for r in expected if (r['seed'],r['rotation'],r['role'])not in groups]
 pairs=[];clusters=[]
 for seed in sorted({r['seed']for r in expected}):
  dd=[]
  for rotation in range(4):
   x=groups.get((seed,rotation,'declaring'));y=groups.get((seed,rotation,'defending'))
   if x and y:assert x['decl']==y['decl'];dd.append(int(x['made'])-int(y['made']))
  if len(dd)==4:clusters.append(dict(seed=seed,paired_mean=sum(dd)/4,rotation_differences=dd));pairs+=dd
 result=dict(schema='native-policy-independent-deal-summary-v1',backend=a.backend,planned_games=len(expected),complete_games=len(rows),moves=28*len(rows),complete_source_deals=len(clusters),missing=missing,independent_deal_clusters=clusters,pair_wins=pairs.count(1),pair_losses=pairs.count(-1),pair_ties=pairs.count(0),strength_claim=False)
 if not missing:
  mean=statistics.mean(c['paired_mean']for c in clusters);radius=math.sqrt(2*math.log(40)/len(clusters))
  result.update(mean_paired_make_advantage=mean,hoeffding95=[max(-1,mean-radius),min(1,mean+radius)],uncertainty='Independent source-deal PRNG draws, fixed declared panel; source deal is the unit, four rotations correlated; bounded differences[-1,1]. Finite panel, no strength/equivalence conclusion.')
 else:result['interval_refusal']='Incomplete planned panel; no population interval'
 for candidate,label in [(False,'phone'),(True,'candidate')]:
  moves=[m for r in rows for m in r['moves']if m['candidate']==candidate];times=sorted(m['timing']['host_ms']for m in moves)
  late=[m for m in moves if m['response']['route']=='late-l3-native-40-4-2-2']
  result[label]=dict(turns=len(moves),host_total_ms=sum(times),host_median_ms=statistics.median(times),host_p95_ms=times[math.ceil(.95*len(times))-1],budget_total_ms=sum(m['call']['budget_ms']for m in moves),routes=dict(collections.Counter(m['response']['route']for m in moves)),late_completed=len(late),late_host_total_ms=sum(m['timing']['host_ms']for m in late),late_host_median_ms=statistics.median(m['timing']['host_ms']for m in late)if late else None,late_refusals=sum(bool(m['timing'].get('late_refusal')or m['response'].get('late_refusal'))for m in moves),interruptions=sum(m['timing']['interrupted']for m in moves))
 usage=[u for r in rows for key in ['phone_usage','candidate_usage','candidate_phone_usage']for u in r[key]]
 assert all('cpu_s'in u for u in usage),usage
 result.update(game_total_seconds=sum(r['game_seconds']for r in rows),worker_processes=len(usage),worker_total_cpu_s=sum(u['cpu_s']for u in usage),worker_max_rss_bytes=max(u['peak_rss_bytes']for u in usage),phone_linear_memory_peak=max(r['phone_linear_memory_peak']for r in rows),candidate_linear_memory_peak=max(r['candidate_linear_memory_peak']for r in rows),identity=dict(candidate_binary_sha256=rows[0]['candidate_binary_sha256'],phone_wasm_sha256=rows[0]['phone_wasm_sha256'],source_sha256=rows[0]['source_sha256']))
 (a.out/'summary.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))

if __name__=='__main__':
 p=argparse.ArgumentParser();sp=p.add_subparsers(dest='cmd',required=True)
 b=sp.add_parser('batch');b.add_argument('--seed',type=int,required=True)
 s=sp.add_parser('summarize')
 for x in [b,s]:x.add_argument('--plan',type=Path,required=True);x.add_argument('--backend',choices=['native','wasm'],required=True);x.add_argument('--out',type=Path,required=True)
 a=p.parse_args();batch(a)if a.cmd=='batch'else summarize(a)
