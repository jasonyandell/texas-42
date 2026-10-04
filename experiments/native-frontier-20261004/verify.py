#!/usr/bin/env python3
"""Receipt/source/retained-vector audit. Run with the inherited watchdog."""
import hashlib,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
BASE='490997b13b161099dcfbcf19f2dec0819fd5a019'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    paths=subprocess.check_output(['git','diff','--name-only',BASE],cwd=ROOT,text=True).splitlines()
    assert all(p.startswith('experiments/native-frontier-20261004/') for p in paths),paths
    subprocess.run(['git','merge-base','--is-ancestor',BASE,'HEAD'],cwd=ROOT,check=True,timeout=5)
    receipts=[]
    for p in sorted(HERE.rglob('run.json')):
        if 'target' in p.parts or any(x.endswith('-target') for x in p.parts):continue
        r=json.loads(p.read_text());assert 0<r['allowance_seconds']<=295
        assert r['elapsed_seconds']<=r['allowance_seconds']+3.5
        assert not r['cleanup_errors'] and r['status'] in ['completed','failed']
        receipts.append(dict(path=str(p.relative_to(HERE)),status=r['status'],elapsed_s=r['elapsed_seconds'],allowance_s=r['allowance_seconds']))
    binary=HERE/'native/target/release/native-frontier';bsha=sha(binary)
    vectors=0;refusals=[];panels=[]
    for d in sorted((HERE/'results').glob('final-*')):
        if not (d/'plan.json').exists():continue
        plan=json.loads((d/'plan.json').read_text());assert plan['binary_sha256']==bsha,(d,plan['binary_sha256'],bsha)
        summary=json.loads((d/'summary.json').read_text());records=json.loads((d/'records.json').read_text())
        for r in records:
            if 'error' in r['result']:
                assert 'answers' not in r['result'];refusals.append(dict(panel=d.name,worlds=r['worlds'],reason=r['result']['error'],refusal_us=r['result']['refusal_us']));continue
            answers=r['result']['answers'];vectors+=len(answers)
            if 'baseline' in r:assert answers==r['baseline']['result']['answers']
            else:assert r.get('python_full_vectors_equal') or r['result']['all_vectors_and_choices_equal']
            assert len(answers)==summary['selected']
            byseed={row['seed']:row for row in plan['rows']}
            for seed,a in zip(plan['selected'],answers):
                assert all(0<=c<=r['worlds'] for c in a['counts']) and a['tiles']==sorted(set(a['tiles']))
                req=byseed[seed]['request'];maximize=req['seat']%2==req['bidder']%2
                best=max(a['counts']) if maximize else min(a['counts'])
                expected=next(t for t,c in zip(a['tiles'],a['counts']) if c==best);assert a['choice']==expected
            for key in ['row_edges','generated_actor_queries','cache_entries','peak_counted_live_array_bytes','native_core_nodes']:
                assert r['result']['cold_stats'][key]==sum(w[key] for w in r['result']['worker_stats'])
        panels.append(dict(name=d.name,roots=summary['selected'],panels=len(summary['panels'])))
    identity=json.loads((HERE/'results/identity/stdout.log').read_text());assert identity['source_inputs_match'] and identity['production_git_blobs_match']
    assert identity['production_commit']=='a0d9fa806166b0e63fe016bb49d93f91f47b1af8'
    result=dict(base_checkpoint=BASE,unchanged_preexisting_paths=True,source_sha256={str(p.relative_to(ROOT)):sha(p) for p in [HERE/'native/src/lib.rs',HERE/'native/src/main.rs',HERE/'panel.py',HERE/'verify.py']},binary_sha256=bsha,final_completed_vector_checks=vectors,final_refusals=refusals,final_panels=panels,receipts=receipts,retained_failed_receipts=[r['path'] for r in receipts if r['status']=='failed'],production_identity=identity)
    (HERE/'AUDIT.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:result[k] for k in ['base_checkpoint','unchanged_preexisting_paths','source_sha256','binary_sha256','final_completed_vector_checks','final_refusals','retained_failed_receipts']},indent=2))
if __name__=='__main__':main()
