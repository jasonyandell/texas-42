#!/usr/bin/env python3
"""Independent raw-record arithmetic, parity, demand and process-cost audit."""
import json,gzip,statistics,math,sys,hashlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/demand-bounds-20261004'
def read(p):
 if p.exists():return json.loads(p.read_text())
 with gzip.open(str(p)+'.gz','rt') as f:return json.load(f)
def eq(a,b):assert math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12),(a,b)
def summed_workers(workers):
 out={}
 for worker in workers:
  for key,value in worker.items():
   if isinstance(value,list):
    prior=out.setdefault(key,[])
    if len(prior)<len(value):prior.extend([0]*(len(value)-len(prior)))
    for i,item in enumerate(value):prior[i]+=item
   else:out[key]=out.get(key,0)+value
 return out
def measured_stats(result):
 if 'cold_stats' in result:
  if 'worker_stats' in result:assert summed_workers(result['worker_stats'])==result['cold_stats']
  return result['cold_stats']
 return summed_workers(result['worker_stats'])
report=[];total_vectors=0;refusals=0
for folder in sys.argv[1:]:
 path=HERE/'results'/folder;s=read(path/'summary.json');records=read(path/'records.json');args=s['arguments'];selected=s['selected'];vectors=0
 names=list(s['binary_sha256']);identity={}
 for name in names:
  binary=HERE/('native/target/release/native-frontier' if name in ['bounded','eagerzero'] else 'checks/prefix-target/release/native-frontier')
  identity[name]=hashlib.sha256(binary.read_bytes()).hexdigest()==s['binary_sha256'][name]
  if folder.startswith('final-'):assert identity[name],(folder,name,'binary identity changed')
 for n in args['counts']:
  rows=read(path/f'inputs-{n}.json');assert [r['seed'] for r in rows]==selected;assert all(len(r['worlds'])==n for r in rows)
 for r in records:
  assert r['selected']==selected;assert r['roots']==len(selected);assert set(r['order'])==set(names)
  expected=r['runs']['recursive']['result']['answers'];assert len(expected)==r['roots']
  for name,v in r['runs'].items():
   assert v['child_cpu_s']>=0 and v['host_s']>0 and v['process_peak_rss_bytes']>0 and v['harness_overhead_s']>=0
   x=v['result'];complete='answers' in x
   if name!='recursive':
    if complete:assert x['answers']==expected;vectors+=len(expected)
    else:assert 'answers' not in x;assert 'error' in x;refusals+=1
    stats=measured_stats(x);assert sum(stats['unique_actor_misses_by_level'])==stats['generated_actor_queries'];assert stats['completed_actor_queries']<=stats['generated_actor_queries']
    if name in ['bounded','eagerzero','full']:assert stats['inner_worlds_by_level']==[a*b for a,b in zip(stats['unique_actor_misses_by_level'],args['budgets'])]
 for p in s['panels']:
  rr=[r for r in records if (r['worlds'],r['field'])==(p['worlds'],p['field'])];assert len(rr)==args['repeats']
  if args['repeats']==len(names):
   for i in range(len(names)):assert len({r['order'][i] for r in rr})==len(names)
  details={}
  for name,claims in p['variants'].items():
   vv=[r['runs'][name] for r in rr];complete=all('answers' in v['result'] for v in vv);assert complete==claims['completed'];assert claims['errors']==[v['result'].get('error') for v in vv]
   for key in ['serialization_s','host_s','output_parse_s','harness_overhead_s','child_cpu_s','process_peak_rss_bytes']:eq(claims[key],statistics.median(v[key] for v in vv))
   cold=statistics.median(v['result'].get('frontier_cold_us',v['result'].get('reference_total_us',v['result'].get('refusal_us'))) for v in vv)/1e6
   standalone=statistics.median(sum(v[k] for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s']) for v in vv);eq(cold,claims['cold_or_refusal_s']);eq(standalone,claims['standalone_s']);eq(p['generation_actual_n_s']+standalone,claims['total_with_generation_s'])
   if name!='recursive':
    stats=measured_stats(vv[-1]['result']);assert claims['stats']==stats
    for run in vv:
     current=measured_stats(run['result']);assert current['unique_actor_misses_by_level']==stats['unique_actor_misses_by_level'];assert current['inner_worlds_by_level']==stats['inner_worlds_by_level']
    details[name]={'complete':complete,'cold_or_refusal_ms':cold*1000,'misses':stats['unique_actor_misses_by_level'],'inner_worlds':stats['inner_worlds_by_level'],'native_misses':stats['native_core_pi_misses_by_level'],'rss_bytes':claims['process_peak_rss_bytes']}
   else:details[name]={'complete':complete,'cold_ms':cold*1000}
  comparisons={}
  for candidate in ['bounded','eagerzero']:
   if candidate not in names:continue
   for name in ['full','hybrid','recursive']:
    if p['variants'][name]['completed'] and p['variants'][candidate]['completed']:comparisons[name+'_over_'+candidate]=p['variants'][name]['cold_or_refusal_s']/p['variants'][candidate]['cold_or_refusal_s']
  report.append({'directory':folder,'current_binary_identity_matches':identity,'worlds':p['worlds'],'field':p['field'],'workers':args['workers'],'roots':p['roots'],'details':details,'complete_service_ratios':comparisons})
 assert vectors==s['vector_comparisons'];total_vectors+=vectors
print(json.dumps({'completed_vector_comparisons':total_vectors,'refused_variant_observations':refusals,'panels':report,'interpretation':'Local repeated finite components. Refusal time excluded from speed ratios. Counts are actual sampled actor misses; hybrid native counters separate. RSS total; counted allocations partial.'},indent=2))
