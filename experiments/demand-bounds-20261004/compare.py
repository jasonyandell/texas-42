#!/usr/bin/env python3
"""Matched complete-vector and resource comparison. Invoke via run_capped.py."""
import argparse,gzip,hashlib,importlib.util,itertools,json,statistics,sys,time
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
 p.add_argument('--workers',type=int,choices=[1,4],default=1);p.add_argument('--repeats',type=int,default=4);p.add_argument('--counted',action='store_true')
 p.add_argument('--budgets',type=int,nargs='+',default=[4,2,2,2]);args=p.parse_args();args.out.mkdir(parents=True,exist_ok=False)
 candidate=HERE/'native/target/release/native-frontier';baseline=HERE/'checks/prefix-target/release/native-frontier'
 bins={'bounded':candidate,'eagerzero':candidate,'full':baseline,'hybrid':baseline,'recursive':baseline}
 records=[];generations={};selected=None;started=time.monotonic();names=list(bins)
 for n in args.counts:
  t=time.perf_counter();proposed=[fixture(args.start+i,args.min_ply+i%4,n) for i in range(args.count)];generations[n]=time.perf_counter()-t
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
     payload=inp|({'reference':True,'reference_only':True,'batch_reference':args.workers==4} if name=='recursive' else {'reference':False,'native_choices':name=='hybrid','bounded_choices':name in ('bounded','eagerzero'),'eager_zero':name=='eagerzero'})
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
  rr=list(group);panel=dict(worlds=n,field=level,roots=len(selected),generation_actual_n_s=generations[n],variants={})
  for name in names:
   vv=[r['runs'][name] for r in rr];complete=all('answers' in v['result'] for v in vv)
   v={key:statistics.median(x[key] for x in vv) for key in ['serialization_s','host_s','output_parse_s','harness_overhead_s','child_cpu_s','process_peak_rss_bytes']}
   v['completed']=complete;v['errors']=[x['result'].get('error') for x in vv]
   v['cold_or_refusal_s']=statistics.median(x['result'].get('frontier_cold_us',x['result'].get('reference_total_us',x['result'].get('refusal_us'))) for x in vv)/1e6
   v['standalone_s']=statistics.median(x['serialization_s']+x['host_s']+x['output_parse_s']+x['harness_overhead_s'] for x in vv)
   v['total_with_generation_s']=generations[n]+v['standalone_s']
   if name!='recursive':v['stats']=stats_of(vv[-1]['result'])
   panel['variants'][name]=v
  panels.append(panel)
 summary=dict(arguments=vars(args),selected=selected,panels=panels,elapsed_s=time.monotonic()-started,binary_sha256={n:hashlib.sha256(b.read_bytes()).hexdigest() for n,b in bins.items()},vector_comparisons=sum(r['roots']*sum('answers' in r['runs'][n]['result'] for n in names[:-1]) for r in records),scope='Finite pip-trump late-root policy component. Equal native workers. Full root vectors; only lower consumers use lazy choices. Sampling actual n over every proposed deal, serialization, process/replay/service/partition setup, parse and temp-file harness charged. Separate wait4 CPU/RSS. Refusals excluded from speed ratios. No phone or generic-k inference.')
 (args.out/'summary.json').write_text(json.dumps(summary,indent=2,default=str)+'\n')
if __name__=='__main__':main()
