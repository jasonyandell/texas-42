#!/usr/bin/env python3
"""Fresh arithmetic/provenance audit and independent native rerun of pipeline."""
import json,gzip,hashlib,statistics,math,importlib.util,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/choice-arena-20261004'
spec=importlib.util.spec_from_file_location('sample_pipeline_check',HERE/'sampling.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
def read(p):
 if p.exists():return json.loads(p.read_text())
 with gzip.open(str(p)+'.gz','rt')as f:return json.load(f)
def eq(a,b):assert math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12),(a,b)
binary=ROOT/'experiments/resumable-choice-20261004/checks/old-demand-target/release/native-frontier';results=[];vectors=0;rerun_vectors=0
for folder in ['pipeline-primary-final','pipeline-holdout-final']:
 base=HERE/'results'/folder;s=read(base/'summary.json');rr=read(base/'records.json');rows=read(base/'all-inputs.json');args=s['arguments']
 assert len(s['binary_sha256'])==64;assert args['repeats']==8 and len(rr)==16
 assert rows==json.loads(json.dumps([m.original(args['start']+i,16+i%4,args['worlds'])for i in range(args['count'])]))
 selected=[r for r in rows if r['status']=='ready'and r['request']['decl']<=6];assert [r['seed']for r in selected]==s['selected']
 expected=rr[0]['run']['result']['answers'];assert len(expected)==len(selected)
 for r in rr:
  assert r['run']['result']['answers']==expected;vectors+=len(expected)
  assert r['order']==(['original','compiled']if r['repeat']%2==0 else['compiled','original'])
  assert r['complete_pipeline_s']>=r['generation_s']+sum(r['run'][k]for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s'])
  assert all(r[k]>=0 for k in ['generation_s','complete_pipeline_s','parent_cpu_s']);assert r['run']['process_peak_rss_bytes']>0
 for name,v in s['variants'].items():
  records=[r for r in rr if r['variant']==name];assert len(records)==8
  for k in ['generation_s','complete_pipeline_s','parent_cpu_s']:eq(v[k],statistics.median(r[k]for r in records))
  for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s','child_cpu_s','process_peak_rss_bytes']:eq(v[k],statistics.median(r['run'][k]for r in records))
  for k,out in [('reference_total_us','native_wall_s'),('outer_validation_us','replay_validation_s')]:eq(v[out],statistics.median(r['run']['result'][k]for r in records)/1e6)
 eq(s['original_over_compiled'],s['variants']['original']['complete_pipeline_s']/s['variants']['compiled']['complete_pipeline_s'])
 assert s['vector_comparisons']==len(rr)*len(expected)
 payload=dict(rows=selected,mode='policy',field_level=2,budgets=[4,2,2,2],counted=False,workers=args['workers'],reference=True,reference_only=True,batch_reference=args['workers']==4,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
 fresh=m.compare.call(binary,payload);assert fresh['result']['answers']==expected;rerun_vectors+=len(expected)
 results.append({'folder':folder,'proposed':len(rows),'selected':len(selected),'ratio':s['original_over_compiled'],'original_pipeline_s':s['variants']['original']['complete_pipeline_s'],'compiled_pipeline_s':s['variants']['compiled']['complete_pipeline_s'],'shared_import_setup_s':s['shared_python_import_setup_s']})
print(json.dumps({'retained_complete_vector_comparisons':vectors,'fresh_native_vector_comparisons':rerun_vectors,'fresh_rebuilt_comparator_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'historical_binary_identity_recorded_only':True,'all_proposed_fixtures_regenerated':True,'balanced_eight_pairs':True,'panels':results,'qualification':'Direct generation+filter/payload+cold native+JSON/replay/tempfile/parse component. Shared imports and receipt writing outside component, within capped process. Parent RSS lifetime both variants; no per-variant RSS claim. Unchanged known-lawful support proposals; not a behavioral posterior, full phone player, playing strength or generic-k claim.'},indent=2))
