#!/usr/bin/env python3
"""Native/WASM production routing conformance to exact pinned phone blob.

Phone mode is intentionally L1/partner, separate from the experimental late L3
policy. Compare all non-clock result fields. Finite completed-call evidence.
"""
import argparse,gzip,json,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
from play import HERE,NATIVE,WASM,Worker,phone
TIMING={'elapsed_us','solver_us','over_budget'}
def semantic(v):
 if isinstance(v,dict):return {k:semantic(x)for k,x in v.items()if k not in TIMING}
 if isinstance(v,list):return list(map(semantic,v))
 return v
def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,default=HERE/'results/phone-conformance');args=p.parse_args()
 out=args.out;out.mkdir(parents=True,exist_ok=False)
 roots=json.loads((HERE/'checks/fresh-roots.json').read_text())
 roots+=list(json.loads((HERE/'checks/ineligible-roots.json').read_text()).values())
 for r in [roots[0],roots[20],roots[32]]:
  roots.append(dict(r,plays=[],bidder=r['seat']))
 workers=[Worker(phone.PHONE),Worker(WASM)];records=[]
 start=time.monotonic()
 try:
  for i,req in enumerate(roots):
   call=phone.profile(req,True)
   raw=json.dumps(dict(call,backend='phone'))+'\n'
   p=subprocess.run([str(NATIVE)],input=raw,capture_output=True,text=True,timeout=25)
   assert p.returncode==0,p.stderr
   native=json.loads(p.stdout)
   pinned,pinned_timing=workers[0].call(call,time.monotonic()+25)
   rebuilt,rebuilt_timing=workers[1].call(dict(call,backend='phone'),time.monotonic()+25)
   row=dict(index=i,call=call,native=native,pinned=pinned,rebuilt=rebuilt,pinned_timing=pinned_timing,rebuilt_timing=rebuilt_timing)
   records.append(row)
   with gzip.open(out/'records.json.gz','wt')as f:json.dump(records,f)
   assert semantic(native)==semantic(pinned)==semantic(rebuilt),(i,native,pinned,rebuilt)
   assert not pinned_timing['interrupted']and not rebuilt_timing['interrupted']
 finally:
  for w in workers:w.close()
 summary=dict(calls=len(records),complete_semantic_comparisons=len(records)*2,late_public_inputs=36,other_public_inputs=6,native_sha256=phone.sha(NATIVE),rebuilt_wasm_sha256=phone.sha(WASM),pinned_wasm_sha256=phone.sha(phone.PHONE),phone_cpu_and_rss=workers[0].usage,rebuilt_cpu_and_rss=workers[1].usage,phone_linear_memory_peak=workers[0].memory_peak,rebuilt_linear_memory_peak=workers[1].memory_peak,elapsed_s=time.monotonic()-start,
  excluded_fields=sorted(TIMING),scope='All remaining recursive result fields identical across production L1/partner route native, rebuilt linked WASM and exact pinned production phone WASM. Public only requests, all9declarations, early/forced/settled/opening. Different late L3 policy checked separately; no L3-versus-L1 equivalence assertion or physical phone timing.')
 (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
