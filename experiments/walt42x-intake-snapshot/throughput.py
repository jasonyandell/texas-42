"""Complete-game throughput, fixed inputs and per-decision parity records.

Process startup and pool construction excluded from steady-state batch timing;
pool warmup runs separately. Each seed owns its RNG. No concurrent GPU runs.
"""
import argparse
import concurrent.futures
import contextlib
import io
import json
import multiprocessing
import os
from pathlib import Path
import statistics
import time
import numpy as np
import walt42x as ref
import fast_numpy

REFERENCE_SEARCH=ref.search

def game(job):
    seed,delta,deals,level,backend=job
    solve={'reference':REFERENCE_SEARCH,'numpy':fast_numpy.search,
           'compiled':fast_numpy.compiled_search}[backend]
    choices=[]
    searches=0
    def measured(*args,**kwargs):
        nonlocal searches
        searches+=1
        values=solve(*args,**kwargs)
        choices.append({'seat':int(args[1]),'values':values})
        return values
    ref.search=measured
    ref.DEALS=deals
    transcript=io.StringIO()
    start=time.perf_counter()
    with contextlib.redirect_stdout(transcript): ref.main(seed,delta,level)
    seconds=time.perf_counter()-start
    lines=transcript.getvalue().splitlines()
    return {'seed':seed,'seconds':seconds,'searches':searches,
            'plays':[s for s in lines if s.startswith('seat ')],
            'outcome':lines[-1],'values':choices}

def run(args):
    jobs=[(seed,args.delta,args.deals,args.level,args.backend)
          for seed in range(args.seed,args.seed+args.games)]
    durations=[]
    def measure(mapper):
        last=None
        for _ in range(args.repeats):
            start=time.perf_counter()
            last=list(mapper(jobs))
            durations.append(time.perf_counter()-start)
        return last
    if args.workers==1:
        warm=time.perf_counter();game(jobs[0]);warm=time.perf_counter()-warm
        results=measure(lambda work:map(game,work))
    else:
        with concurrent.futures.ProcessPoolExecutor(max_workers=args.workers,
                mp_context=multiprocessing.get_context('spawn')) as pool:
            warm=time.perf_counter()
            list(pool.map(game,[jobs[0]]*args.workers))
            warm=time.perf_counter()-warm
            results=measure(lambda work:pool.map(game,work,chunksize=1))
    elapsed=statistics.median(durations)
    report={'backend':args.backend,'delta':args.delta,'deals':args.deals,'level':args.level,
            'games':args.games,'workers':args.workers,'warmup_seconds':warm,
            'batch_seconds':elapsed,'games_per_second':args.games/elapsed,
            'repeat_seconds':durations,'repeats':args.repeats,
            'numpy':np.__version__,'cpu_count':os.cpu_count(),'results':results}
    Path(args.output).write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:v for k,v in report.items() if k!='results'}),flush=True)

if __name__=='__main__':
    p=argparse.ArgumentParser()
    p.add_argument('--backend',choices=['reference','numpy','compiled'],default='numpy')
    p.add_argument('--delta',type=float,default=1)
    p.add_argument('--deals',type=int,default=30)
    p.add_argument('--level',type=int,choices=[0,1],default=0)
    p.add_argument('--games',type=int,default=24)
    p.add_argument('--seed',type=int,default=1)
    p.add_argument('--workers',type=int,default=1)
    p.add_argument('--repeats',type=int,default=1)
    p.add_argument('--output',required=True)
    run(p.parse_args())
