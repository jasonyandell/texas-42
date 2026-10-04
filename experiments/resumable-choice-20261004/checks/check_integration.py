#!/usr/bin/env python3
"""Independent late bridge audit, all declarations and own/public boundaries."""
import copy,hashlib,importlib.util,json,random,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/resumable-choice-20261004';CHECK=HERE/'checks'
sys.path.insert(0,str(ROOT/'experiments/partnership'))
from rules import legal_tiles,winner,replay_record,information_state,trick_points
binary=CHECK/'integration-target/release/late-choice-player'
def call(payload):
 p=subprocess.run([str(binary)],input=json.dumps(payload)+'\n',capture_output=True,text=True,timeout=8)
 assert p.returncode==0,(p.returncode,p.stderr)
 return json.loads(p.stdout)
def eligible(req):
 st=information_state(req)
 return 16<=len(req['plays'])//2<=24 and len(st['legal'])>1 and st['points'][req['bidder']%2]<req['bid'] and st['points'][1-req['bidder']%2]<=42-req['bid']
requests=[];counts={d:0 for d in (*range(8),9)};ineligible={};proposals=0
for seed in range(986000,986300):
 d=(*range(8),9)[seed%9];bidder=seed%4;rng=random.Random(seed);deck=list(range(28));rng.shuffle(deck);hands=[sorted(deck[s*7:s*7+7])for s in range(4)];remaining=list(map(set,hands));leader=bidder;trick=[];plays=[];added=False
 for ply in range(28):
  seat=(leader+len(trick))%4
  req=dict(decl=d,bid=30,bidder=bidder,seat=seat,hand=hands[seat][:],plays=plays[:],seed=7042104)
  st=information_state(req)
  if not eligible(req):
   if ply<16:kind='early'
   elif len(st['legal'])==1:kind='forced'
   else:kind='settled'
   ineligible.setdefault(kind,req)
  if not added and eligible(req) and counts[d]<4:
   requests.append(req);counts[d]+=1;added=True
  tile=rng.choice(legal_tiles(remaining[seat],trick,d));remaining[seat].remove(tile);trick.append((seat,tile));plays.extend([seat,tile])
  if len(trick)==4:leader=winner(trick,d);trick=[]
 proposals+=1
 if all(n==4 for n in counts.values()) and len(ineligible)==3:break
assert len(requests)==36,counts
vectors=worlds=0;declarations=set();seats=set();histories=[]
for req in requests:
 results=[];st=information_state(req)
 for variant in ['resumable','arena','lazy','native']:
  result=call(dict(request=req,variant=variant,validate=True))
  assert 'error' not in result and not result.get('ineligible'),(req,result)
  assert result['legal']==st['legal'] and result['points']==st['points'] and result['leader']==st['leader']
  assert result['choice'] in st['legal'] and result['validated'];assert len(result['worlds'])==40
  results.append(result)
 for r in results[1:]:assert r['evaluation']==results[0]['evaluation'] and r['worlds']==results[0]['worlds']
 # Each normalized sampled hand reconstructs one full lawfully replayed deal.
 rotation=1 if req['bidder']%2==0 else 0
 for w in results[0]['worlds']:
  hh=[]
  for seat in range(4):
   live=[t for t in range(28)if w[(seat+rotation)%4]&(1<<t)]
   past=[t for s,t in zip(req['plays'][::2],req['plays'][1::2])if s==seat]
   hh.append(sorted(past+live))
  assert hh[req['seat']]==sorted(req['hand']) and all(len(h)==7 for h in hh)
  pp,ll,rr,tt=replay_record(hh,req['plays'],req['decl'],req['bidder']);assert pp==st['points'] and ll==st['leader'];worlds+=1
 e=results[0]['evaluation'];assert e['tiles']==st['legal'];a=(req['seat']+rotation)%4
 vals=e['counts'];best=(max(vals)if a%2 else min(vals));assert e['choice']==next(t for t,v in zip(e['tiles'],vals)if v==best)
 vectors+=4;declarations.add(req['decl']);seats.add(req['seat']);histories.append(req)
for kind,req in ineligible.items():
 r=call(dict(request=req,validate=True));assert r.get('ineligible'),(kind,r)
base=requests[0];bad=[]
for key,value in [('worlds',[[1,2,3,4]]),('opponent_hands',[[1,2]]),('source_deal_seed',986000),('contract','nello')]:
 r=copy.deepcopy(base);r[key]=value;bad.append(r)
for key,value in [('decl',8),('decl',10),('bid',29),('bid',286),('bidder',4),('seat',4),('seed',-1),('seed',2**64),('seed',True),('hand',[0]*7),('plays',[0]),('plays',[4,0]),('hand',[28]+base['hand'][1:]),('bid','30'),('seat',False)]:
 r=copy.deepcopy(base);r[key]=value;bad.append(r)
for key in base:
 r=copy.deepcopy(base);del r[key];bad.append(r)
r=copy.deepcopy(base);r['plays'][0]=(r['plays'][0]+1)%4;bad.append(r)
r=copy.deepcopy(base);r['plays'][3]=r['plays'][1];bad.append(r)
for req in bad:assert 'error' in call(dict(request=req)),req
assert 'error' in call(dict(request=base,variant='unknown'))
# Test fallback routing without repeating expensive phone solves. Fake phone
# checks the forwarded public request exactly and captures remaining allowance.
spec=importlib.util.spec_from_file_location('hybrid_check',HERE/'player.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
class FakePhone:
 def __init__(self,*args):self.calls=[];self.closed=False
 def call(self,c,deadline):
  self.calls.append(c);st=information_state(c['request'])
  return dict(choice=st['legal'][0],legal=st['legal'],points=st['points'],leader=st['leader'],route='fake-phone'),dict(host_ms=0,checkpoints=1,interrupted=False)
 def close(self):self.closed=True
module.phone.Worker=FakePhone
fallbacks=[]
for kind,req in ineligible.items():
 p=module.Player();c=dict(request=req,budget_ms=14000,worlds=40,partner=True)
 answer,timing=p.call(c,time.monotonic()+20);assert answer['route']=='fake-phone' and not timing['late_attempt'] and p.native is None;assert p.phone.calls[0]==c;p.close();fallbacks.append(kind)
p=module.Player('unknown');c=dict(request=base,budget_ms=14000,worlds=40,partner=True)
a,t=p.call(c,time.monotonic()+20);assert a['route']=='fake-phone' and t['late_attempt'] and t['late_refusal']=='unknown kernel variant' and p.native is None;assert p.phone.calls[0]['request']==base and p.phone.calls[0]['budget_ms']<=14000;p.close();fallbacks.append('kernel-refusal')
original_popen=module.subprocess.Popen
try:
 def missing(*args,**kwargs):raise FileNotFoundError('checker controlled missing worker')
 module.subprocess.Popen=missing;p=module.Player();a,t=p.call(c,time.monotonic()+20);assert a['route']=='fake-phone' and 'FileNotFoundError' in t['late_refusal'] and p.native is None;p.close();fallbacks.append('spawn-failure')
finally:module.subprocess.Popen=original_popen
# Control transport/error surfaces deterministically; no timer sleeps or real
# phone work. Every refusal kills/reaps the worker and calls the phone once.
import io
original_select=module.select.select
class Process:
 def __init__(self,text,broken=False):
  class Broken(io.StringIO):
   def write(self,*args):raise BrokenPipeError('checker controlled broken pipe')
  self.stdin=Broken() if broken else io.StringIO();self.stdout=io.StringIO(text);self.stderr=io.StringIO();self.killed=False;self.reaped=False
 def kill(self):self.killed=True
 def wait(self,timeout):self.reaped=True;return 0
try:
 for kind,text,ready,broken in [('malformed','{\n',True,False),('nonobject','[]\n',True,False),('empty','',True,False),('missing','{}\n',True,False),('illegal',json.dumps(dict(choice=99,legal=[]))+'\n',True,False),('timeout','',False,False),('brokenpipe','',True,True)]:
  proc=Process(text,broken);module.subprocess.Popen=lambda *a,**k:proc;module.select.select=lambda *a:([proc.stdout],[],[])if ready else([],[],[])
  p=module.Player();a,t=p.call(c,time.monotonic()+20);assert a['route']=='fake-phone' and t['late_attempt'] and t['late_refusal'] and p.native is None;assert proc.killed and proc.reaped and len(p.phone.calls)==1;p.close();fallbacks.append(kind)
finally:module.subprocess.Popen=original_popen;module.select.select=original_select
out=dict(proposed_independent_deals=proposals,fresh_public_roots=len(requests),declarations=sorted(declarations),actor_seats=sorted(seats),complete_variant_vectors=vectors,independently_full_replayed_sampled_worlds=worlds,production_sample_and_rng_validation=True,ascending_ties=True,malformed_or_private_requests_refused=len(bad)+1,ineligible_routes=sorted(ineligible),mocked_phone_fallbacks=fallbacks,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),source_sha256={str(p.relative_to(HERE)):hashlib.sha256(p.read_bytes()).hexdigest()for p in [HERE/'integration/src/main.rs',HERE/'player.py',Path(__file__)]},scope='Finite local late policy integration. Phone routing uses a mock here; actual complete games are separately audited. Native status/sampler/reference use existing production routines; Python full historical replay is independent. No phone deployment or strength conclusion.')
(CHECK/'integration-roots.json').write_text(json.dumps(histories,indent=2)+'\n');print(json.dumps(out,indent=2))
