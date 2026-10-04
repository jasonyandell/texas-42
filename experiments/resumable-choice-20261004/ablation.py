#!/usr/bin/env python3
"""Controlled existing eager-L0 ablation on the resumable kernel."""
import argparse,gzip,hashlib,importlib.util,json,statistics,sys
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('sampler',HERE.parent/'choice-arena-20261004/sampling.py')
s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s)
p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
records=[];names=['resumable','resumable_floor','bounded','recursive'];candidate=HERE/'native/target/release/native-frontier';old=HERE/'checks/old-demand-target/release/native-frontier'
for n in [8,40]:
 rows=[s.compiled_fixture(962000+i,16+i%4,n) for i in range(32)];rows=[r for r in rows if r['status']=='ready' and r['request']['decl']<=6]
 for repeat in range(8):
  order=names[repeat%4:]+names[:repeat%4];runs={}
  for name in order:
   inp=dict(rows=rows,mode='policy',field_level=2,budgets=[4,2,2,2],counted=False,workers=4,warm=False,caps=dict(rows=500000,work=20000000,queries=50000,cache_entries=50000,seconds=30))
   if name.startswith('resumable'):inp.update(resumable_choices=True,eager_zero=name=='resumable_floor')
   if name=='bounded':inp['bounded_choices']=True
   if name=='recursive':inp.update(reference=True,reference_only=True,batch_reference=True)
   runs[name]=s.compare.call(candidate if name.startswith('resumable') else old,inp)
  expected=runs['recursive']['result']['answers']
  for name in names[:-1]:assert runs[name]['result']['answers']==expected
  records.append(dict(worlds=n,repeat=repeat,roots=len(rows),order=order,runs=runs))
 with gzip.open(a.out/'records.json.gz','wt') as f:json.dump(records,f)
panels=[]
for n in [8,40]:
 rr=[r for r in records if r['worlds']==n];variants={}
 for name in names:
  runs=[r['runs'][name] for r in rr]
  variants[name]=dict(service_s=statistics.median(r['result'].get('frontier_cold_us',r['result'].get('reference_total_us')) for r in runs)/1e6,standalone_s=statistics.median(r['serialization_s']+r['host_s']+r['output_parse_s']+r['harness_overhead_s'] for r in runs),cpu_s=statistics.median(r['child_cpu_s'] for r in runs),rss_bytes=statistics.median(r['process_peak_rss_bytes'] for r in runs),stats=runs[-1]['result'].get('cold_stats'))
 panels.append(dict(worlds=n,variants=variants))
out=dict(panels=panels,vector_comparisons=sum(r['roots']*3 for r in records),binary_sha256={'candidate':hashlib.sha256(candidate.read_bytes()).hexdigest(),'old':hashlib.sha256(old.read_bytes()).hexdigest()},scope='Existing eager-L0 floor trades demand for eager work; not stack-only. Same complete input/order/counts/ties/budgets/workers, four-position balanced8reps, cold native processes. Generation shared outside perprocess wall.')
(a.out/'summary.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'vector_comparisons':out['vector_comparisons']}))
