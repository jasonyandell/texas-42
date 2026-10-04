#!/usr/bin/env python3
"""Complete local hybrid player. Actor request has exactly seven public fields.

Phone owns early/forced/settled/refused turns. Eligible late calls use the new
exact kernel's experimental L3 policy, not the phone L1/partner policy.
"""
import importlib.util,json,select,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('phone_arena',HERE.parent/'astra-sol-20261004/tools/arena.py')
phone=importlib.util.module_from_spec(spec);spec.loader.exec_module(phone)

class Player:
 def __init__(self,variant='resumable'):
  self.phone=phone.Worker(phone.PHONE);self.native=None;self.variant=variant
 def close(self):
  self.phone.close()
  if self.native:
   self.native.kill();self.native.wait(timeout=2)
   for f in [self.native.stdin,self.native.stdout,self.native.stderr]:f.close()
   self.native=None
 def call(self,call,game_deadline):
  start=time.monotonic();req=call['request'];state=phone.information_state(req)
  late=16<=len(req['plays'])//2<=24 and len(state['legal'])>1 and state['points'][req['bidder']%2]<req['bid'] and state['points'][1-req['bidder']%2]<=42-req['bid']
  refusal=None
  if late:
   try:
    if self.native is None:
     self.native=subprocess.Popen([str(HERE/'integration/target/release/late-choice-player')],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,bufsize=1)
    self.native.stdin.write(json.dumps({'request':req,'variant':self.variant})+'\n');self.native.stdin.flush()
    left=min(4,max(0,game_deadline-time.monotonic()))
    if select.select([self.native.stdout],[],[],left)[0]:
     line=self.native.stdout.readline()
     result=json.loads(line) if line else {'error':'late worker exited'}
     assert isinstance(result,dict), 'late response object required'
     if 'error' not in result and not result.get('ineligible'):
      assert result['choice'] in state['legal'] and result['legal']==state['legal']
      assert result['points']==state['points'] and result['leader']==state['leader']
      return result,dict(host_ms=(time.monotonic()-start)*1000,checkpoints=1,interrupted=False,initialization_ms=None,late_attempt=True,late_refusal=None)
     refusal=result.get('error','late eligibility rejected')
    else:refusal='late host deadline'
   except (OSError,ValueError,AssertionError,KeyError) as exc:
    refusal=f'late transport/validation: {type(exc).__name__}: {exc}'
   finally:
    if refusal and self.native:
     self.native.kill();self.native.wait(timeout=2)
     for f in [self.native.stdin,self.native.stdout,self.native.stderr]:f.close()
     self.native=None
  # Failed kernel time consumes the same per-turn compute allowance. The phone
  # wrapper still emits its ordinary legal checkpoint at any remaining budget.
  if late:
   remaining=max(100,call['budget_ms']-int((time.monotonic()-start)*1000))
   call=dict(call,budget_ms=remaining)
  response,timing=self.phone.call(call,game_deadline)
  timing.update(host_ms=(time.monotonic()-start)*1000,late_attempt=late,late_refusal=refusal)
  return response,timing

if __name__=='__main__':
 p=Player()
 try:
  for line in sys.stdin:
   call=json.loads(line);result,timing=p.call(call,time.monotonic()+25)
   print(json.dumps({'result':result,'timing':timing}),flush=True)
 finally:p.close()
