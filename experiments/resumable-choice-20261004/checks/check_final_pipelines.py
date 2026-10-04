#!/usr/bin/env python3
"""Independent final cold/frontend, adapter and eager-L0 arithmetic audit."""
import gzip,hashlib,importlib.util,json,math,statistics,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/resumable-choice-20261004'
spec=importlib.util.spec_from_file_location('pipeline_sampling',ROOT/'experiments/choice-arena-20261004/sampling.py');sam=importlib.util.module_from_spec(spec);spec.loader.exec_module(sam)
def read(p):
 if p.exists():return json.loads(p.read_text())
 with gzip.open(str(p)+'.gz','rt')as f:return json.load(f)
def eq(a,b):assert math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12),(a,b)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binary(n):return HERE/('native/target/release/native-frontier'if n=='resumable'else'checks/old-arena-target/release/native-frontier'if n=='arena'else'checks/old-demand-target/release/native-frontier')
counts={};panels=[]
for folder in ['cold-primary','cold-holdout']:
 b=HERE/'results'/folder;s=read(b/'summary.json');rr=read(b/'records.json');a=s['arguments'];names=['resumable','arena','bounded','recursive'];assert a['repeats']==8 and len(rr)==8
 rows=[sam.original(a['start']+i,16+i%4,a['worlds'])for i in range(a['count'])];rowsha=hashlib.sha256(json.dumps(rows,sort_keys=True).encode()).hexdigest();assert rowsha==s['exact_rows_sha256'];assert [r['seed']for r in rows if r['status']=='ready'and r['request']['decl']<=6]==s['selected']
 compared=0
 for r in rr:
  assert r['order']==names[r['repeat']%4:]+names[:r['repeat']%4];expected=r['runs']['recursive']['result']['run']['result']['answers']
  for n,v in r['runs'].items():
   x=v['result'];assert x['variant']==n and x['all_rows_sha256']==rowsha and x['selected']==s['selected'];assert sha(binary(n))==s['binary_sha256'][n]==x['binary_sha256']
   eq(v['complete_cold_pipeline_s'],v['python_process_host_s']+v['wrapper_parse_s']+v['wrapper_overhead_s']);assert v['python_process_host_s']>0 and v['frontend_peak_rss_bytes']>0
   y=x['run']['result']
   if 'answers'in y:assert y['answers']==expected;compared+=len(expected)*(n!='recursive')
   else:assert 'error'in y
 for n,c in s['variants'].items():
  vv=[r['runs'][n]for r in rr];assert c['completed']==all('answers'in v['result']['run']['result']for v in vv);assert c['errors']==[v['result']['run']['result'].get('error')for v in vv]
  for k in ['complete_cold_pipeline_s','python_process_host_s','wrapper_parse_s','wrapper_overhead_s','frontend_wait4_cpu_s','frontend_peak_rss_bytes']:eq(c[k],statistics.median(v[k]for v in vv))
  for k in ['python_import_s','generation_s']:eq(c[k],statistics.median(v['result'][k]for v in vv))
  for k in ['child_cpu_s','process_peak_rss_bytes','serialization_s','host_s','output_parse_s','harness_overhead_s']:eq(c[k],statistics.median(v['result']['run'][k]for v in vv))
 assert compared==s['vector_comparisons'];counts[folder]=compared
 res=s['variants']['resumable'];panels.append(dict(folder=folder,roots=len(s['selected']),complete={n:c['completed']for n,c in s['variants'].items()},cold_s={n:c['complete_cold_pipeline_s']for n,c in s['variants'].items()},completed_ratios={n:c['complete_cold_pipeline_s']/res['complete_cold_pipeline_s']for n,c in s['variants'].items()if n!='resumable'and c['completed']and res['completed']}))
# Native-own/public call includes sampler and status, replay checked separately.
b=HERE/'results/adapter-cold';s=read(b/'summary.json');rr=read(b/'records.json');names=['resumable','arena','lazy','native'];assert s['repeats']==8;assert s['binary_sha256']==sha(HERE/'integration/target/release/late-choice-player');assert len(rr)==s['turns']*s['repeats'];compared=0;unique={}
for r in rr:
 assert r['order']==names[(r['repeat']+r['turn'])%4:]+names[:(r['repeat']+r['turn'])%4];assert set(r['request'])=={'decl','bid','bidder','seat','hand','plays','seed'};unique[json.dumps(r['request'],sort_keys=True)]=r['request'];expected=r['runs']['native']['result']['evaluation']
 for n,v in r['runs'].items():
  if 'evaluation'in v['result']:assert v['result']['evaluation']==expected;compared+=n!='native'
  else:assert 'error'in v['result']
for n,c in s['variants'].items():
 vv=[r['runs'][n]for r in rr];assert c['completed']==all('error'not in v['result']for v in vv);assert c['errors']==[v['result'].get('error')for v in vv]
 eq(c['cold_complete_calls_s'],sum(sum(v[k]for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s'])for v in vv)/s['repeats']);eq(c['cpu_s'],sum(v['child_cpu_s']for v in vv)/s['repeats']);eq(c['peak_rss_median_bytes'],statistics.median(v['process_peak_rss_bytes']for v in vv))
assert compared==s['complete_vector_comparisons'];counts['adapter-cold']=compared
# Actual current native reference vectors for every saved public adapter turn.
for encoded,req in unique.items():
 p=subprocess.run([str(HERE/'integration/target/release/late-choice-player')],input=json.dumps(dict(request=req,variant='native',validate=True))+'\n',capture_output=True,text=True,timeout=8);assert p.returncode==0;r=json.loads(p.stdout);assert r['validated']
 for raw in rr:
  if raw['request']==req:assert raw['runs']['native']['result']['evaluation']==r['evaluation']
counts['adapter_fresh_native_vectors']=len(unique)
# Controlled eager-L0 floor is a different evaluation strategy, same decisions.
b=HERE/'results/ablation';s=read(b/'summary.json');rr=read(b/'records.json');names=['resumable','resumable_floor','bounded','recursive'];assert s['binary_sha256']['candidate']==sha(binary('resumable')) and s['binary_sha256']['old']==sha(binary('bounded'));compared=0
for r in rr:
 assert r['order']==names[r['repeat']%4:]+names[:r['repeat']%4];expected=r['runs']['recursive']['result']['answers']
 for n in names[:-1]:assert r['runs'][n]['result']['answers']==expected;compared+=len(expected)
for p in s['panels']:
 for n,c in p['variants'].items():
  vv=[r['runs'][n]for r in rr if r['worlds']==p['worlds']];assert len(vv)==8
  eq(c['service_s'],statistics.median(v['result'].get('frontier_cold_us',v['result'].get('reference_total_us'))for v in vv)/1e6);eq(c['standalone_s'],statistics.median(sum(v[k]for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s'])for v in vv));eq(c['cpu_s'],statistics.median(v['child_cpu_s']for v in vv));eq(c['rss_bytes'],statistics.median(v['process_peak_rss_bytes']for v in vv));assert c['stats']==vv[-1]['result'].get('cold_stats')
assert compared==s['vector_comparisons'];counts['eager-L0-ablation']=compared
print(json.dumps(dict(complete_vector_comparisons=counts,panels=panels,adapter_summary=read(HERE/'results/adapter-cold/summary.json'),ablation_panels=s['panels'],scope='Finite final raw measurements, actual binary hashes, all proposals freshly regenerated, balanced process orders, medians or explicitly stated per-panel mean sum. Frontend CPU may include nested children; do not add nested native CPU. Refusals yield no complete vectors/speed ratios. Native deadline/node model is not frontier logical cap equivalence. No independent-deal performance distribution or phone deployment inference.'),indent=2))
