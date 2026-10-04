#!/usr/bin/env python3
"""Profile the unchanged historical fixture frontend, never timed solver cells.

Call only via the inherited process-group watchdog. cProfile times include its
own instrumentation; the normal comparison charges uninstrumented generation.
"""
import argparse,cProfile,hashlib,importlib.util,json,pstats,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('compare',HERE/'compare.py')
compare=importlib.util.module_from_spec(spec);spec.loader.exec_module(compare)

def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
 p.add_argument('--start',type=int,default=962000);p.add_argument('--count',type=int,default=32)
 p.add_argument('--worlds',type=int,default=40);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
 profile=cProfile.Profile();t=time.perf_counter();cpu=time.process_time()
 rows=profile.runcall(lambda:[compare.fixture(a.start+i,16+i%4,a.worlds) for i in range(a.count)])
 wall=time.perf_counter()-t;cpu=time.process_time()-cpu
 profile.dump_stats(str(a.out/'generation.prof'));stats=pstats.Stats(profile)
 entries=[]
 for (file,line,name),(cc,nc,tt,ct,callers) in stats.stats.items():
  entries.append(dict(file=file,line=line,name=name,primitive_calls=cc,total_calls=nc,self_s=tt,cumulative_s=ct))
 entries.sort(key=lambda x:x['cumulative_s'],reverse=True)
 summary=dict(arguments=vars(a),instrumented_wall_s=wall,instrumented_cpu_s=cpu,
   proposed=len(rows),selected=[r['seed'] for r in rows if r['status']=='ready' and r['request']['decl']<=6],
   attempts=sum(r.get('sampling_attempts',r.get('attempts',0)) for r in rows),
   ready_worlds=sum(len(r.get('worlds',[])) for r in rows),
   statuses={s:sum(r['status']==s for r in rows) for s in sorted({r['status'] for r in rows})},
   exact_rows_sha256=hashlib.sha256(json.dumps(rows,sort_keys=True).encode()).hexdigest(),
   fixture_source_sha256=hashlib.sha256((HERE.parent/'adversarial-20261004/parallel_roots.py').read_bytes()).hexdigest(),
   top_cumulative=entries[:24],top_self=sorted(entries,key=lambda x:x['self_s'],reverse=True)[:24],
   scope='Unchanged Python support-rejection fixture, full-history replay. Instrumentation overhead included. Diagnostic only; solver speed cells use uninstrumented complete actual-n generation over every proposal.')
 (a.out/'summary.json').write_text(json.dumps(summary,indent=2,default=str)+'\n')
 print(json.dumps({k:summary[k] for k in ['proposed','selected','attempts','ready_worlds','instrumented_wall_s']}))
if __name__=='__main__':main()
