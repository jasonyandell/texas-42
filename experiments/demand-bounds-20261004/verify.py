#!/usr/bin/env python3
"""Final local scope, identity, source, vector and capped-receipt audit."""
import gzip,hashlib,json,re,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
BASE='7f4c68a01a5d3caae5345695750cdd5b4ee8e736'
def read(path):
 if path.exists():return json.loads(path.read_text())
 with gzip.open(str(path)+'.gz','rt') as f:return json.load(f)
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
 prefix=str(HERE.relative_to(ROOT))+'/'
 for args in [['git','diff','--name-only',BASE],['git','ls-files','--others','--exclude-standard']]:
  names=subprocess.check_output(args,cwd=ROOT,text=True).splitlines();assert all(n.startswith(prefix) for n in names),names
 identity=read(HERE/'results/identity/stdout.log');assert identity['source_inputs_match'] and identity['production_git_blobs_match']
 assert identity['production_commit']=='a0d9fa806166b0e63fe016bb49d93f91f47b1af8';assert identity['source_commit']=='cb1ef3b23072e4c268f31f625f2b61d5facc1929'
 vectors=0;refusals=0;panels=[]
 for folder in ['final-one','final-four','final-holdout']:
  s=read(HERE/'results'/folder/'summary.json');records=read(HERE/'results'/folder/'records.json')
  for name,h in s['binary_sha256'].items():
   binary=HERE/('native/target/release/native-frontier' if name in ['bounded','eagerzero'] else 'checks/prefix-target/release/native-frontier');assert sha(binary)==h
  count=0
  for row in records:
   expected=row['runs']['recursive']['result']['answers'];assert len(expected)==row['roots'];assert len(row['order'])==5 and len(set(row['order']))==5
   for name,run in row['runs'].items():
    if name=='recursive':continue
    result=run['result']
    if 'answers' in result:assert result['answers']==expected;count+=row['roots']
    else:assert 'error' in result and 'answers' not in result;refusals+=1
  assert count==s['vector_comparisons'];vectors+=count;panels.append(dict(folder=folder,roots=len(s['selected']),vector_comparisons=count))
 assert vectors==4145 and refusals==20
 sourcehash=read(HERE/'checks/SOURCE_HASHES.json')
 for name,h in sourcehash['sha256'].items():assert sha(ROOT/name)==h,name
 lean=(HERE/'math/DemandBounds.lean').read_text();assert not re.search(r'\b(sorry|admit|axiom|native_decide)\b',lean)
 for path in [HERE/'results/lean-2/stdout.log',HERE/'checks/lean-independent-fixed/stdout.log']:
  out=path.read_text();assert 'sorryAx' not in out and 'error:' not in out;assert out.count('axioms')==7
 reuse=read(HERE/'results/reuse-final/result.json')['result'];assert reuse['vector_checks']==90
 for r in reuse['records']:
  assert sum(x['cold_misses'] for x in r['roots_detail'])==r['independent_cold_misses']
  assert sum(x['reused_misses'] for x in r['roots_detail'])==r['successive_service_misses']
  assert r['actor_queries_avoided_by_cross_root_cache']==r['independent_cold_misses']-r['successive_service_misses']
  assert r['inner_worlds_avoided']==r['independent_inner_worlds']-r['successive_inner_worlds']
 receipts=[]
 for path in sorted(HERE.rglob('run.json')):
  receipt=read(path)
  if receipt.get('schema')!='texas42-partnership-run-v1':continue
  assert 0<receipt['allowance_seconds']<=295;assert receipt['elapsed_seconds']<300
  assert receipt['status'] in ['completed','failed'];receipts.append(dict(path=str(path.relative_to(HERE)),status=receipt['status'],allowance=receipt['allowance_seconds'],elapsed=receipt['elapsed_seconds']))
 result=dict(base_checkpoint=BASE,unchanged_preexisting_paths=True,production_identity=identity,completed_timed_vector_comparisons=vectors,refused_timed_observations=refusals,reuse_vector_checks=90,panels=panels,receipts=receipts,failed_receipts=[r['path'] for r in receipts if r['status']=='failed'],source_sha256={str(p.relative_to(ROOT)):sha(p) for p in [HERE/'native/src/lib.rs',HERE/'native/src/bounds.rs',HERE/'native/src/main.rs',HERE/'math/DemandBounds.lean',HERE/'compare.py',HERE/'reuse.py',HERE/'reuse/src/main.rs',HERE/'verify.py']},scope='Finite exploratory component. Every demanded choice on six-root8field2trace separately checked by independent runner; seven conditional Lean arithmetic/view lemmas. No Rust refinement/privacy/phone/strength/generic-k claim. Timings include frontend components; reuse wait4 spans whole validation study.')
 (HERE/'AUDIT.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:result[k] for k in ['base_checkpoint','unchanged_preexisting_paths','completed_timed_vector_comparisons','refused_timed_observations','reuse_vector_checks','failed_receipts']},indent=2))
if __name__=='__main__':main()
