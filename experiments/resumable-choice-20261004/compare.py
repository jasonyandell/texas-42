#!/usr/bin/env python3
"""Matched complete-vector and resource comparison. Invoke via run_capped.py."""
import argparse,gzip,hashlib,importlib.util,itertools,json,statistics,sys,time,resource
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('prefix_compare',HERE.parent/'prefix-state-20261004/compare.py')
prefix_compare=importlib.util.module_from_spec(spec);spec.loader.exec_module(prefix_compare)
call,fixture=prefix_compare.call,prefix_compare.fixture

def stats_of(result):
 if 'cold_stats' in result:return result['cold_stats']
 out={}
 for worker in result.get('worker_stats',[]):
  for key,v in worker.items():
   if isinstance(v,list):
    prior=out.setdefault(key,[])
    prior.extend([0]*(len(v)-len(prior)))
    for i,n in enumerate(v):prior[i]+=n
   else:out[key]=out.get(key,0)+v
 return out

def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
 p.add_argument('--start',type=int,default=962000);p.add_argument('--count',type=int,default=32);p.add_argument('--min-ply',type=int,default=16)
 p.add_argument('--counts',type=int,nargs='+',default=[2,8,40]);p.add_argument('--fields',type=int,nargs='+',default=[0,1,2])
 p.add_argument('--workers',type=int,choices=[1,4],default=1);p.add_argument('--repeats',type=int,default=6);p.add_argument('--counted',action='store_true')
 p.add_argument('--budgets',type=int,nargs='+',default=[4,2,2,2]);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
 candidate=HERE/'native/target/release/native-frontier';baseline=HERE/'checks/old-demand-target/release/native-frontier';arena=HERE/'checks/old-arena-target/release/native-frontier'
 bins={'resumable':candidate,'arena':arena,'bounded':baseline,'full':baseline,'hybrid':baseline,'recursive':baseline}
 records=[];generations={};generation_details={};selected=None;started=time.monotonic();names=list(bins)
 for n in args.counts:
  t=time.perf_counter();cpu=time.process_time();proposed=[fixture(args.start+i,args.min_ply+i%4,n) for i in range(args.count)];generations[n]=time.perf_counter()-t;generation_details[n]={'cpu_s':time.process_time()-cpu,'parent_lifetime_peak_rss_bytes':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin' else 1024),'sampling_attempts':sum(r.get('sampling_attempts',r.get('attempts',0)) for r in proposed),'statuses':{s:sum(r['status']==s for r in proposed) for s in sorted({r['status'] for r in proposed})},'all_proposed_roots':len(proposed),'ready_before_pip_filter':sum(r['status']=='ready' for r in proposed)}
  rows=[r for r in proposed if r['status']=='ready' and r['request']['decl']<=6]
  assert rows
  if selected is None:selected=[r['seed'] for r in rows]
  assert selected==[r['seed'] for r in rows]
  with gzip.open(args.out/f'inputs-{n}.json.gz','wt') as f:json.dump(rows,f)
  for level in args.fields:
   inp=dict(rows=rows,mode='policy',field_level=level,budgets=args.budgets,counted=args.counted,warm=False,workers=args.workers,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
   for repeat in range(args.repeats):
    if time.monotonic()-started>220:raise TimeoutError('panel220s cap')
    order=names[repeat%len(names):]+names[:repeat%len(names)];runs={}
    for name in order:
     payload=inp|({'reference':True,'reference_only':True,'batch_reference':args.workers==4} if name=='recursive' else {'reference':False,'native_choices':name=='hybrid','bounded_choices':name in ('bounded','eagerzero'),'eager_zero':name=='eagerzero','arena_choices':name=='arena','resumable_choices':name=='resumable'})
     runs[name]=call(bins[name],payload)
    expected=runs['recursive']['result']['answers']
    for name in names[:-1]:
     result=runs[name]['result']
     if 'answers' in result:assert result['answers']==expected,(name,n,level,repeat)
     else:assert 'error' in result
    r=dict(worlds=n,field=level,repeat=repeat,roots=len(rows),selected=selected,order=order,runs=runs);records.append(r)
    with gzip.open(args.out/'records.json.gz','wt') as f:json.dump(records,f)
    print(json.dumps(dict(worlds=n,field=level,repeat=repeat,roots=len(rows),runs={name:dict(us=x['result'].get('frontier_cold_us',x['result'].get('reference_total_us',x['result'].get('refusal_us'))),error=x['result'].get('error'),misses=stats_of(x['result']).get('unique_actor_misses_by_level')) for name,x in runs.items()})),flush=True)
 panels=[]
 for (n,level),group in itertools.groupby(records,key=lambda r:(r['worlds'],r['field'])):
  rr=list(group);panel=dict(worlds=n,field=level,roots=len(selected),generation_actual_n_s=generations[n],generation_details=generation_details[n],variants={})
  for name in names:
   vv=[r['runs'][name] for r in rr];complete=all('answers' in v['result'] for v in vv)
   v={key:statistics.median(x[key] for x in vv) for key in ['serialization_s','host_s','output_parse_s','harness_overhead_s','child_cpu_s','process_peak_rss_bytes']}
   v['completed']=complete;v['errors']=[x['result'].get('error') for x in vv]
   v['cold_or_refusal_s']=statistics.median(x['result'].get('frontier_cold_us',x['result'].get('reference_total_us',x['result'].get('refusal_us'))) for x in vv)/1e6
   v['standalone_s']=statistics.median(x['serialization_s']+x['host_s']+x['output_parse_s']+x['harness_overhead_s'] for x in vv)
   v['total_with_generation_s']=generations[n]+v['standalone_s'];v['outer_validation_s']=statistics.median(x['result'].get('outer_validation_us',0) for x in vv)/1e6;v['request_s']=statistics.median(x['result'].get('request_total_us',0) for x in vv)/1e6
   if name!='recursive':v['stats']=stats_of(vv[-1]['result']);v['phase_median_worker_us']={k:statistics.median(stats_of(x['result']).get(k,0) for x in vv) for k in ['sample_us','preparation_us','demand_grouping_us','forward_us','fold_us','cache_lookup_us','arena_schedule_us','arena_expand_us','arena_lower_us','arena_setup_us','res_schedule_us','res_expand_us','res_lower_us','res_setup_us','res_propagate_us']}
   panel['variants'][name]=v
  panels.append(panel)
 summary=dict(arguments=vars(args),selected=selected,panels=panels,elapsed_s=time.monotonic()-started,binary_sha256={n:hashlib.sha256(b.read_bytes()).hexdigest() for n,b in bins.items()},vector_comparisons=sum(r['roots']*sum('answers' in r['runs'][n]['result'] for n in names[:-1]) for r in records),generation_source_sha256=hashlib.sha256((HERE.parent/'adversarial-20261004/parallel_roots.py').read_bytes()).hexdigest(),scope='Finite pip-trump late-root policy component. Equal native workers. Full root vectors; only lower consumers use exact bounded choices. New resumable versus separately built pinned arena/demand predecessors. Six-way rotated processes balance every variant position when repeats=6. Sampling actual n over every proposed deal, serialization, process/replay/service/partition setup, parse and temp-file harness charged. Separate wait4 CPU/RSS. Refusals excluded from speed ratios. No phone or generic-k inference.')
 (args.out/'summary.json').write_text(json.dumps(summary,indent=2,default=str)+'\n')
if __name__=='__main__':main()
