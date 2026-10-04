#!/usr/bin/env python3
"""Final local source, exact-vector, numbered-claim and execution-bound audit.

Run through the inherited capped runner. Independent checking remains in checks/.
"""
import gzip,hashlib,json,re,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];BASE='07ee42b9'
def read(p):
 if p.exists():return json.loads(p.read_text())
 with gzip.open(str(p)+'.gz','rt') as f:return json.load(f)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 prefix=str(HERE.relative_to(ROOT))+'/'
 for args in [['git','diff','--name-only',BASE],['git','ls-files','--others','--exclude-standard']]:
  paths=subprocess.check_output(args,cwd=ROOT,text=True).splitlines()
  assert all(p.startswith(prefix) for p in paths),paths
 source=read(HERE/'checks/SOURCE_HASHES.json')
 for p,h in source['sha256'].items():assert sha(HERE/p)==h,p
 assert all(source['unchanged_sampler_eager_fold_validation'].values())
 vectors=refusals=arena_completed=arena_slower=0
 for folder in ['final-one','final-four','final-holdout']:
  s=read(HERE/'results'/folder/'summary.json');rows=read(HERE/'results'/folder/'records.json')
  for name,h in s['binary_sha256'].items():
   p=HERE/('native/target/release/native-frontier' if name=='arena' else 'checks/baseline-target/release/native-frontier');assert sha(p)==h
  count=0
  for r in rows:
   expected=r['runs']['recursive']['result']['answers'];assert len(expected)==r['roots']
   assert len(r['order'])==6 and len(set(r['order']))==6
   for name,run in r['runs'].items():
    if name=='recursive':continue
    result=run['result']
    if 'answers' in result:assert result['answers']==expected;count+=r['roots']
    else:assert 'error' in result and 'answers' not in result;refusals+=1
  assert count==s['vector_comparisons'];vectors+=count
  for p in s['panels']:
   a,b=p['variants']['arena'],p['variants']['bounded']
   if a['completed']:
    arena_completed+=1;assert b['completed']
    arena_slower+=int(a['cold_or_refusal_s']>b['cold_or_refusal_s'])
    assert a['cold_or_refusal_s']>p['variants']['recursive']['cold_or_refusal_s']
    if p['field']==2:assert a['cold_or_refusal_s']>b['cold_or_refusal_s']
    assert a['stats']['unique_actor_misses_by_level']==b['stats']['unique_actor_misses_by_level']
 assert (vectors,refusals,arena_completed,arena_slower)==(6180,30,21,20)
 identity=read(HERE/'checks/identity/stdout.log')
 assert identity['production_commit']=='a0d9fa806166b0e63fe016bb49d93f91f47b1af8'
 assert identity['source_commit']=='cb1ef3b23072e4c268f31f625f2b61d5facc1929'
 assert identity['source_inputs_match'] and identity['production_git_blobs_match']
 for p in (HERE/'math').glob('*.lean'):
  assert not re.search(r'\b(sorry|admit|axiom|native_decide)\b',p.read_text())
 for folder,n in [('lean-bounds',7),('lean-arena',6)]:
  out=(HERE/'checks'/folder/'stdout.log').read_text();assert 'error:' not in out and 'sorryAx' not in out and out.count('axioms')==n
 required=['baseline-parity','arena-parity-extended-fixed','sampling-support-final','pipeline-audit-final','cold-audit','arena-timing-audit','preserved-history-final']
 for folder in required:assert read(HERE/'checks'/folder/'run.json')['status']=='completed',folder
 cold=[]
 for folder in ['cold-primary','cold-holdout']:
  s=read(HERE/'results'/folder/'summary.json');rs=read(HERE/'results'/folder/'records.json')
  assert sha(HERE/'checks/baseline-target/release/native-frontier')==s['binary_sha256']
  for p,h in s['source_sha256'].items():assert sha(HERE/p)==h,p
  first=rs[0]['run']['result']
  for r in rs:
   result=r['run']['result'];assert result['all_rows_sha256']==s['exact_all_rows_sha256'];assert result['run']['result']['answers']==first['run']['result']['answers']
  assert len(rs)*len(first['selected'])==s['vector_comparisons']
  cold.append(dict(folder=folder,ratio=s['original_over_compiled'],variants=s['variants']))
 receipts=[]
 for p in sorted(HERE.rglob('run.json')):
  r=read(p)
  if r.get('schema')!='texas42-partnership-run-v1':continue
  assert 0<r['allowance_seconds']<=295 and r['elapsed_seconds']<300
  assert r['status'] in ['completed','failed'];assert not r['cleanup_errors']
  receipts.append(dict(path=str(p.relative_to(HERE)),status=r['status'],allowance=r['allowance_seconds'],elapsed=r['elapsed_seconds']))
 out=dict(base_checkpoint=BASE,only_new_experiment_changed=True,production_identity=identity,
  completed_vector_comparisons=vectors,refused_observations=refusals,arena_completed_cells=arena_completed,
  arena_cells_slower_than_lazy=arena_slower,all_completed_field2_arena_cells_slower_than_lazy=True,cold_panels=cold,receipts=receipts,
  failed_receipts=[r['path'] for r in receipts if r['status']=='failed'],
  source_sha256={p.name:sha(p) for p in HERE.glob('*.py')},
  scope='Exploratory finite component only. Fresh independent Sol checker, conditional Lean, exact trace/group/key/stream checks. No whole phone player/strength/higher-k linearity. Shared cold harness import/AST costs and partial allocation/overlapping phase counters qualified.')
 (HERE/'AUDIT.json').write_text(json.dumps(out,indent=2)+'\n')
 print(json.dumps({k:out[k] for k in ['base_checkpoint','only_new_experiment_changed','completed_vector_comparisons','refused_observations','arena_completed_cells','failed_receipts']},indent=2))
if __name__=='__main__':main()
