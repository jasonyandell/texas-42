#!/usr/bin/env python3
"""Fresh cold-process provenance/arithmetic checker."""
import json,gzip,hashlib,statistics,math,importlib.util,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/choice-arena-20261004'
spec=importlib.util.spec_from_file_location('cold_sampling_check',HERE/'sampling.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
def read(p):
 if p.exists():return json.loads(p.read_text())
 with gzip.open(str(p)+'.gz','rt')as f:return json.load(f)
def eq(a,b):assert math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12),(a,b)
binary=HERE/'checks/baseline-target/release/native-frontier';panels=[];vectors=0
for folder,resident in [('cold-primary','pipeline-primary-final'),('cold-holdout','pipeline-holdout-final')]:
 base=HERE/'results'/folder;s=read(base/'summary.json');rr=read(base/'records.json');args=s['arguments'];assert args['repeats']==8 and len(rr)==16
 assert s['binary_sha256']==hashlib.sha256(binary.read_bytes()).hexdigest()
 for name,sha in s['source_sha256'].items():assert hashlib.sha256((HERE/name).read_bytes()).hexdigest()==sha
 rows=[m.original(args['start']+i,16+i%4,args['worlds'])for i in range(args['count'])]
 sha=hashlib.sha256(json.dumps(rows,sort_keys=True).encode()).hexdigest();assert s['exact_all_rows_sha256']==sha
 assert [r['seed']for r in rows if r['status']=='ready'and r['request']['decl']<=6]==s['selected']
 expected=read(HERE/'results'/resident/'records.json')[0]['run']['result']['answers'];assert len(expected)==len(s['selected'])
 for r in rr:
  run=r['run'];x=run['result'];assert x['variant']==r['variant'];assert x['selected']==s['selected'];assert x['all_rows_sha256']==sha;assert x['run']['result']['answers']==expected;vectors+=len(expected)
  assert r['order']==(['original','compiled']if r['repeat']%2==0 else['compiled','original'])
  eq(run['complete_cold_pipeline_s'],run['python_process_host_s']+run['wrapper_parse_s']+run['wrapper_overhead_s'])
  assert run['python_process_host_s']>=x['python_body_before_output_s'];assert run['frontend_peak_rss_bytes']>0 and x['run']['process_peak_rss_bytes']>0
  for k in ['complete_cold_pipeline_s','python_process_host_s','wrapper_parse_s','wrapper_overhead_s','frontend_wait4_cpu_s']:assert run[k]>=0
 for name,c in s['variants'].items():
  vv=[r['run']for r in rr if r['variant']==name];assert len(vv)==8
  for k in ['complete_cold_pipeline_s','python_process_host_s','wrapper_parse_s','wrapper_overhead_s','frontend_wait4_cpu_s','frontend_peak_rss_bytes']:eq(c[k],statistics.median(v[k]for v in vv))
  for k in ['python_import_s','generation_s','python_cpu_after_import_s']:eq(c[k],statistics.median(v['result'][k]for v in vv))
  for k in ['child_cpu_s','process_peak_rss_bytes','serialization_s','host_s','output_parse_s','harness_overhead_s']:eq(c[k],statistics.median(v['result']['run'][k]for v in vv))
 eq(s['original_over_compiled'],s['variants']['original']['complete_cold_pipeline_s']/s['variants']['compiled']['complete_cold_pipeline_s']);assert s['vector_comparisons']==len(rr)*len(expected)
 panels.append({'folder':folder,'proposed':len(rows),'roots':len(expected),'original_cold_s':s['variants']['original']['complete_cold_pipeline_s'],'compiled_cold_s':s['variants']['compiled']['complete_cold_pipeline_s'],'ratio':s['original_over_compiled'],'frontend_RSS_medians':{n:v['frontend_peak_rss_bytes']for n,v in s['variants'].items()},'nested_native_RSS_medians':{n:v['process_peak_rss_bytes']for n,v in s['variants'].items()}})
print(json.dumps({'complete_vector_comparisons':vectors,'all_proposed_input_hashes_freshly_regenerated':True,'current_source_and_binary_identities_verified':True,'balanced_eight_pairs':True,'panels':panels,'scope':'Fresh Python+native process per request. Matched harness imports sampling/compiled AST setup in both variants. Driver shared startup/aggregate receipt outside per-request wall. Frontend wait4 may include waited descendants: do not add nested CPU; no per-variant memory improvement established. Finite40world late Straight support component, no full player/strength/general-k claim.'},indent=2))
