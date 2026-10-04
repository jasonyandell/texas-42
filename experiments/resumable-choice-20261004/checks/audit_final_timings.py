# Attributed adaptation of prior six-variant independent raw arithmetic checker.
#!/usr/bin/env python3
"""Fresh six-variant raw measurement audit, every worker and complete vector."""
import gzip,hashlib,json,math,statistics,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[3];HERE=ROOT/'experiments/resumable-choice-20261004'
def read(p):
 if p.exists():return json.loads(p.read_text())
 with gzip.open(str(p)+'.gz','rt')as f:return json.load(f)
def eq(a,b):assert math.isclose(a,b,rel_tol=1e-12,abs_tol=1e-12),(a,b)
def stats(x):
 out={}
 for w in x.get('worker_stats',[]):
  for k,v in w.items():
   if isinstance(v,list):
    prior=out.setdefault(k,[]);prior.extend([0]*(len(v)-len(prior)))
    for i,n in enumerate(v):prior[i]+=n
   else:out[k]=out.get(k,0)+v
 if 'cold_stats'in x:
  if out:assert out==x['cold_stats']
  return x['cold_stats']
 return out
vectors=refusals=0;panels=[]
for folder in sys.argv[1:]:
 base=HERE/'results'/folder;s=read(base/'summary.json');rr=read(base/'records.json');args=s['arguments'];names=list(s['binary_sha256']);assert len(names)==6 and args['repeats']==6
 for name,sha in s['binary_sha256'].items():
  binary=HERE/('native/target/release/native-frontier'if name=='resumable'else'checks/old-arena-target/release/native-frontier'if name=='arena'else'checks/old-demand-target/release/native-frontier');assert hashlib.sha256(binary.read_bytes()).hexdigest()==sha
 for n in args['counts']:
  rows=read(base/f'inputs-{n}.json');assert [r['seed']for r in rows]==s['selected'];assert all(len(r['worlds'])==n for r in rows)
  assert s['generation_source_sha256']==hashlib.sha256((ROOT/'experiments/adversarial-20261004/parallel_roots.py').read_bytes()).hexdigest()
 for r in rr:
  assert r['selected']==s['selected'] and r['roots']==len(s['selected']);assert r['order']==names[r['repeat']:]+names[:r['repeat']]
  expected=r['runs']['recursive']['result']['answers'];assert len(expected)==r['roots']
  for name,v in r['runs'].items():
   x=v['result'];assert v['host_s']>0 and v['process_peak_rss_bytes']>0;assert all(v[k]>=0 for k in ['serialization_s','output_parse_s','harness_overhead_s','child_cpu_s'])
   if name=='recursive':continue
   if 'answers'in x:assert x['answers']==expected;vectors+=len(expected)
   else:assert 'error'in x;refusals+=1
   st=stats(x);assert st['generated_actor_queries']==sum(st['unique_actor_misses_by_level']);assert st['completed_actor_queries']<=st['generated_actor_queries']
   if name in ['resumable','arena','bounded','full']:assert st['inner_worlds_by_level']==[a*b for a,b in zip(st['unique_actor_misses_by_level'],args['budgets'])]
 for p in s['panels']:
  runs=[r for r in rr if r['worlds']==p['worlds']and r['field']==p['field']];assert len(runs)==6
  assert p['generation_details']['all_proposed_roots']==args['count'];assert p['generation_details']['ready_before_pip_filter']>=p['roots'];ratios={}
  for name,c in p['variants'].items():
   vv=[r['runs'][name]for r in runs];eq(c['cold_or_refusal_s'],statistics.median(v['result'].get('frontier_cold_us',v['result'].get('reference_total_us',v['result'].get('refusal_us')))for v in vv)/1e6)
   standalone=statistics.median(sum(v[k]for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s'])for v in vv);eq(c['standalone_s'],standalone);eq(c['total_with_generation_s'],standalone+p['generation_actual_n_s'])
   for k in ['serialization_s','host_s','output_parse_s','harness_overhead_s','child_cpu_s','process_peak_rss_bytes']:eq(c[k],statistics.median(v[k]for v in vv))
   assert c['completed']==all('answers'in v['result']for v in vv);assert c['errors']==[v['result'].get('error')for v in vv]
   if name!='recursive':
    assert c['stats']==stats(vv[-1]['result'])
    for k,n in c['phase_median_worker_us'].items():eq(n,statistics.median(stats(v['result']).get(k,0)for v in vv))
  arena=p['variants']['resumable']
  if arena['completed']:
   for other in ['arena','bounded','full','hybrid','recursive']:
    v=p['variants'][other]
    if v['completed']:ratios[other+'_over_resumable']={k:v[k]/arena[k]for k in ['cold_or_refusal_s','standalone_s','total_with_generation_s']}
  panels.append({'folder':folder,'worlds':p['worlds'],'field':p['field'],'roots':p['roots'],'completed':{n:v['completed']for n,v in p['variants'].items()},'ratios':ratios,'actor_misses':{n:v.get('stats',{}).get('generated_actor_queries')for n,v in p['variants'].items()}})
 assert s['vector_comparisons']==sum(r['roots']*sum('answers'in r['runs'][n]['result']for n in names if n!='recursive')for r in rr)
assert all(p['ratios']['arena_over_resumable']['cold_or_refusal_s']>1 for p in panels if 'arena_over_resumable' in p['ratios'])
assert sum('arena_over_resumable'in p['ratios'] for p in panels)==21
assert sum(p['ratios']['bounded_over_resumable']['cold_or_refusal_s']>1 for p in panels if 'bounded_over_resumable'in p['ratios'])==14
print(json.dumps({'completed_vector_comparisons':vectors,'refused_variant_observations':refusals,'current_binary_identity_verified':True,'all_worker_refusal_stats_summed':True,'balanced_six_way_order':True,'panels':panels,'scope':'Local finite repeated component measurements. No refused speed ratio; phase scopes overlap, retained arena estimates exclude spare capacity/temporary storage/allocator/cache/query ownership. Whole PID RSS is total measurement.'},indent=2))
