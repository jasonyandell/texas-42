#!/usr/bin/env python3
"""Audit final retained evidence, scope, source identity and bounded receipts."""
import gzip,hashlib,json,re,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
BASE='c2ab1eb138ed5d19afbd9e9ec9262e0594e6e6a4'
PANELS=['final-one-v2','final-four-v2','holdout-four','final-policy0-four','final-hybrid0-four','rung0-one','rung1-one','rung2-one']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):
 if p.exists():return json.loads(p.read_text())
 with gzip.open(str(p)+'.gz','rt') as f:return json.load(f)
def main():
 paths=subprocess.check_output(['git','diff','--name-only',BASE],cwd=ROOT,text=True).splitlines()
 paths+=subprocess.check_output(['git','ls-files','--others','--exclude-standard'],cwd=ROOT,text=True).splitlines()
 assert paths and all(p.startswith('experiments/prefix-state-20261004/') for p in paths),paths
 subprocess.run(['git','merge-base','--is-ancestor',BASE,'HEAD'],cwd=ROOT,check=True,timeout=5)
 for name,h in read(HERE/'checks/SOURCE_HASHES.json').items():assert sha(ROOT/name)==h,name
 identity=read(HERE/'results/identity/stdout.log');assert identity['source_inputs_match'] and identity['production_git_blobs_match']
 assert identity['production_commit']=='a0d9fa806166b0e63fe016bb49d93f91f47b1af8'
 assert sha(ROOT/'experiments/native-frontier-20261004/native/src/lib.rs')=='995064fa56678660b93188d91dc00a0e297530b129120b150f9c4726cc61e6eb'
 receipts=[]
 for p in sorted(HERE.rglob('run.json')):
  if any(x=='target' or x.endswith('-target') for x in p.parts):continue
  r=read(p);assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<=r['allowance_seconds']+3.5<300
  assert not r['cleanup_errors'] and r['status'] in ['completed','failed']
  receipts.append(dict(path=str(p.relative_to(HERE)),status=r['status'],elapsed_s=r['elapsed_seconds'],allowance_s=r['allowance_seconds']))
 binary=HERE/'native/target/release/native-frontier';control=HERE/'control-target/release/native-frontier';bs=sha(binary);cs=sha(control)
 vectors=0;panels=[]
 for name in PANELS:
  d=HERE/'results'/name;s=read(d/'summary.json');records=read(d/'records.json')
  assert s['binary_sha256']=={'factored':bs,'factored-hash':bs,'predecessor':cs,'recursive':bs}
  for r in records:
   inputs=read(d/f"inputs-{r['worlds']}.json");reference=r['runs']['recursive']['result']['answers']
   for variant in ['factored','factored-hash','predecessor']:
    a=r['runs'][variant]['result']['answers'];assert a==reference and len(a)==len(inputs);vectors+=len(a)
    for inp,ans in zip(inputs,a):
     assert ans['tiles']==sorted(set(ans['tiles'])) and len(ans['tiles'])==len(ans['counts'])
     assert all(0<=n<=r['worlds'] for n in ans['counts'])
     req=inp['request'];maximize=req['seat']%2==req['bidder']%2
     best=(max if maximize else min)(ans['counts']);assert ans['choice']==next(t for t,n in zip(ans['tiles'],ans['counts']) if n==best)
    out=r['runs'][variant]['result']
    for key,value in out['cold_stats'].items():
     parts=[w[key] for w in out['worker_stats']]
     if isinstance(value,list):assert value==[sum((p[i] if i<len(p) else 0) for p in parts) for i in range(len(value))]
     else:assert value==sum(parts)
   f=r['runs']['factored']['result']['cold_stats'];assert f['public_step_calls']+f['reused_public_steps']<=f['row_edges'];assert f['public_record_calls']+f['avoided_record_calls']<=f['row_edges']
  assert s['vector_checks']==len(records)*len(s['selected'])*3
  panels.append(dict(name=name,roots=len(s['selected']),vector_checks=s['vector_checks']))
 assert vectors==sum(p['vector_checks'] for p in panels)==9336
 tape=read(HERE/'results/final-tape/records.json');assert all(r['python_full_vectors_equal'] for r in tape);tape_vectors=sum(len(r['result']['answers']) for r in tape);assert tape_vectors==282
 demand=read(HERE/'results/demand-matrix/records.json');dv=refusals=0
 for r in demand:
  for n,v in r['runs'].items():
   x=v['result']
   if 'answers' in x:assert x['answers']==r['reference']['result']['answers'];dv+=len(x['answers'])
   else:assert x['error']=='actor query cap' and 'answers' not in x;refusals+=1
 assert (dv,refusals)==(102,1)
 lean=(HERE/'results/lean-prefix-fixed/stdout.log').read_text();assert lean.count("WaltResearch.PrefixState.")==6 and 'sorryAx' not in lean
 source=(HERE/'math/PrefixState.lean').read_text();assert not re.search(r'\b(sorry|admit|axiom|native_decide)\b',source)
 for entry in read(HERE/'COMPRESSED.json')['files']:
  p=HERE/entry['path'];assert sha(p)==entry['gzip_sha256'];assert hashlib.sha256(gzip.decompress(p.read_bytes())).hexdigest()==entry['original_sha256']
 result=dict(base_checkpoint=BASE,unchanged_predecessor_paths=True,production_identity=identity,final_binary_sha256=bs,control_binary_sha256=cs,measured_repeated_vector_comparisons=vectors,historical_tape_repeated_vectors=tape_vectors,demand_matrix_completed_vectors=dv,demand_matrix_refusals=refusals,panels=panels,receipts=receipts,failed_receipts=[r['path'] for r in receipts if r['status']=='failed'],source_sha256={str(p.relative_to(ROOT)):sha(p) for p in [HERE/'native/src/lib.rs',HERE/'native/src/main.rs',HERE/'compare.py',HERE/'demand.py',HERE/'math/PrefixState.lean',HERE/'verify.py']},scope='Finite component parity and retained local performance; repeated vectors are not independent deals. Six Lean lemmas are conditional representation results, not Rust refinement. No phone/strength/general-k claim.')
 (HERE/'AUDIT.json').write_text(json.dumps(result,indent=2)+'\n')
 print(json.dumps({k:result[k] for k in ['base_checkpoint','unchanged_predecessor_paths','measured_repeated_vector_comparisons','historical_tape_repeated_vectors','demand_matrix_completed_vectors','demand_matrix_refusals','failed_receipts']},indent=2))
if __name__=='__main__':main()
