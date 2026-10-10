"""Whole-game repaired-ladder benchmark; incomplete games are explicit."""
import argparse
from concurrent.futures import ProcessPoolExecutor,ThreadPoolExecutor,as_completed
import json
import multiprocessing
from pathlib import Path
import time
import repaired as r

def run_one(job):
    seed,samples,delta,fieldseed,level,batch,seconds,cache,fiber_cap,engine,reuse,addressed,prune,tables,node_cache=job
    if engine=='cpp':
        import turbo
        return turbo.game(seed,r.Spec(tuple(samples),delta,fieldseed),level,batch,seconds,cache,fiber_cap=fiber_cap,reuse_deals=reuse,addressed=addressed,prune=prune,tables=tables,node_cache=node_cache)
    return r.game(seed,r.Spec(tuple(samples),delta,fieldseed),level,batch,seconds,cache,fiber_cap=fiber_cap,engine=engine)

def run(args):
    if args.reuse_deals and args.engine!='cpp': raise ValueError('--reuse-deals requires --engine cpp')
    if (args.addressed or args.no_prune or args.no_tables or args.no_node_cache) and args.engine!='cpp': raise ValueError('native options require --engine cpp')
    if args.games<1 or args.workers<1: raise ValueError('positive games and workers required')
    samples=tuple(map(int,args.samples.split(',')))
    jobs=[(seed,samples,args.delta,args.field_seed,args.level,args.batch,args.seconds,not args.no_cache,args.fiber_cap,args.engine,args.reuse_deals,args.addressed,not args.no_prune,not args.no_tables,not args.no_node_cache)
          for seed in range(args.seed,args.seed+args.games)]
    start=time.perf_counter();results=[];last_snapshot=start
    def record(result):
        nonlocal last_snapshot
        results.append(result)
        print(json.dumps({k:v for k,v in result.items() if k not in ('moves','hands')}),flush=True)
        # Avoid quadratic JSON rewriting when games finish in milliseconds.
        if time.perf_counter()-last_snapshot>=1:
            save(False);last_snapshot=time.perf_counter()
    def save(finished):
        elapsed=time.perf_counter()-start
        complete=sum(j['status']=='complete' for j in results)
        payload={'finished':finished,'wall_seconds':elapsed,'workers':args.workers,'executor':args.executor,
            'snapshot_interval_seconds':1,
            'completed_games':complete,'requested_games':len(jobs),
            'games_per_second':complete/elapsed,'results':sorted(results,key=lambda x:x['seed'])}
        Path(args.output).write_text(json.dumps(payload,indent=2)+'\n')
    if args.workers==1:
        for job in jobs: record(run_one(job))
    elif args.executor=='process':
        with ProcessPoolExecutor(args.workers,mp_context=multiprocessing.get_context('spawn')) as pool:
            for future in as_completed([pool.submit(run_one,job) for job in jobs]): record(future.result())
    else:
        with ThreadPoolExecutor(args.workers) as pool:
            for future in as_completed([pool.submit(run_one,job) for job in jobs]):record(future.result())
    save(True)

if __name__=='__main__':
    p=argparse.ArgumentParser()
    p.add_argument('--samples',default='8,30',help='L1,L2,... deal counts, bottom rung first')
    p.add_argument('--level',type=int,default=2)
    p.add_argument('--delta',type=float,default=1)
    p.add_argument('--field-seed',type=int,default=42)
    p.add_argument('--batch',type=int,default=256)
    p.add_argument('--seed',type=int,default=1)
    p.add_argument('--games',type=int,default=1)
    p.add_argument('--workers',type=int,default=1)
    p.add_argument('--executor',choices=('process','thread'),default='process',help='C++ releases the GIL; thread workers amortize imports and library loading')
    p.add_argument('--seconds',type=float,default=60)
    p.add_argument('--fiber-cap',type=int,default=1_000_000)
    p.add_argument('--engine',choices=('numpy','native','cpp'),default='numpy')
    p.add_argument('--reuse-deals',action='store_true',help='C++ only: lawful common support sample prefixes across levels')
    p.add_argument('--addressed',action='store_true',help='C++ v2: branch-addressed randomness enables exact bound pruning')
    p.add_argument('--no-prune',action='store_true',help='verification: disable addressed-field bound pruning')
    p.add_argument('--no-tables',action='store_true',help='verification: disable canonical allocation-pattern tables')
    p.add_argument('--no-node-cache',action='store_true',help='verification: disable same-node acting-hand query deduplication')
    p.add_argument('--no-cache',action='store_true')
    p.add_argument('--output',required=True)
    run(p.parse_args())
