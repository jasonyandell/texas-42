#!/usr/bin/env python3
"""Independent native/WASM JSON and checkpoint contract checks (bounded externally)."""
import copy,hashlib,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
CHECK=Path(__file__).resolve().parent;ROOT=CHECK.parents[2]
sys.path.insert(0,str(CHECK))
from check_adapter import call
native=ROOT/'experiments/native-policy-check-20261004/adapter/target/release/native-late-player'
wasm=ROOT/'experiments/native-policy-check-20261004/adapter/target/wasm32-unknown-unknown/release/native_late_player.wasm'
rows=json.loads((CHECK/'fresh-roots.json').read_text());expected=json.loads((CHECK/'independent-fresh-fixed/stdout.log').read_text())['cases'];full=0;checks=0
for req,e in zip(rows,expected):
 c=dict(request=req,worlds=40,partner=True,budget_ms=14000)
 nv,_=call(native,c);wv,env=call(wasm,c)
 assert nv['evaluation']==wv['evaluation']==e['evaluation']
 for k in ['choice','legal','points','leader','trick','route','policy','kernel_variant']:assert nv[k]==wv[k],k
 ck=env['checkpoints'];assert len(ck)==2 and ck[0]['route']=='legal-fallback'and ck[-1]==wv
 assert all(v['choice']in v['legal']and v['legal']==wv['legal']and v['points']==wv['points']and v['leader']==wv['leader']for v in ck)
 full+=1;checks+=len(ck)
refusals=0
for req in rows[:4]:
 for b in [native,wasm]:
  v,env=call(b,dict(action='late',request=req,budget_ms=0,validate=True));assert 'error'in v and 'evaluation'not in v and 'worlds'not in v,(b,v);refusals+=1
bad=[];base=dict(request=rows[0],worlds=40,partner=True,budget_ms=14000)
for k,v in [('worlds',0),('worlds',641),('worlds',True),('budget_ms',0),('budget_ms',99),('budget_ms',20001),('budget_ms',False),('partner',1),('partner',None),('worlds',160),('private',{})]:
 c=copy.deepcopy(base);c[k]=v;bad.append(c)
for c in bad:
 for b in [native,wasm]:
  v,env=call(b,c);assert 'error'in v and 'evaluation'not in v,(c,v)
  assert not env.get('checkpoints'),env
expired=0
for r in rows[:4]:
 p=subprocess.run(['node',str(CHECK/'wasm_worker.mjs'),str(wasm),'expire'],input=json.dumps(dict(request=r,worlds=40,partner=True,budget_ms=100))+'\n',capture_output=True,text=True,timeout=5);assert p.returncode==0;env=json.loads(p.stdout);v=env['result'];assert v['route']=='legal-fallback'and v.get('late_refusal')and 'evaluation'not in v;assert len(env['checkpoints'])==1 and v['choice']==env['checkpoints'][0]['choice'];expired+=1
print(json.dumps(dict(exhausted_host_clock_retains_legal_checkpoint=expired,full_hybrid_native_wasm_vectors=full,wasm_public_legal_checkpoints=checks,zero_budget_no_vector_refusals=refusals,invalid_complete_profiles_each=len(bad),native_sha256=hashlib.sha256(native.read_bytes()).hexdigest(),wasm_sha256=hashlib.sha256(wasm.read_bytes()).hexdigest(),scope='Finite same-late-policy wire and checkpoint conformance; real elapsed clocks, canonical14s fullcalls. Zero budget tests are late audit interface. No physical-phone/browser timing.'),indent=2))
