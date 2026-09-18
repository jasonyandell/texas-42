#!/usr/bin/env python3
"""Bounded paired full-game replay, including every intermediate action vector."""
import argparse
import asyncio
import json
import os
from pathlib import Path
import statistics
import time

import played as p


def semantic(value):
    if isinstance(value,list):return [semantic(v) for v in value]
    if isinstance(value,dict):
        return {k:semantic(v) for k,v in value.items() if k not in
            ('elapsed_us','solver_us','nodes','pi_calls_by_level','inner_worlds_by_level')}
    return value


async def benchmark(args):
    db = p.connect(args.directory)
    all_jobs = [dict(r) for r in db.execute(p.JOB_SQL+" WHERE j.state='done' ORDER BY j.id")]
    db.close()
    if len(all_jobs)<args.count:raise ValueError('Not enough saved games')
    jobs = [all_jobs[i*len(all_jobs)//args.count] for i in range(args.count)]
    queue = asyncio.Queue()
    for repeat in range(args.repeats):
        for i,job in enumerate(jobs):queue.put_nowait((repeat,i,job))
    rows = []
    async def worker():
        processes = [await asyncio.create_subprocess_exec(str(Path(binary).resolve()),
            stdin=asyncio.subprocess.PIPE,stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.DEVNULL,
            limit=4*1024*1024,env={**os.environ,'RAYON_NUM_THREADS':'1'})
            for binary in (args.baseline,args.candidate)]
        try:
            while not queue.empty():
                repeat,i,job = queue.get_nowait()
                request = p.game_request(job)
                values = [None,None]
                walls = [0,0]
                for which in ((0,1) if (repeat+i)%2==0 else (1,0)):
                    proc = processes[which]
                    before = time.monotonic()
                    proc.stdin.write((p.canonical(request)+'\n').encode());await proc.stdin.drain()
                    value = json.loads(await proc.stdout.readline())
                    walls[which] = time.monotonic()-before
                    p.validate_game(request,value)
                    values[which] = value
                if semantic(values[0]) != semantic(values[1]):
                    p.atomic_json(Path(args.output).with_suffix('.mismatch.json'),{'request':request,'values':values})
                    raise ValueError(f'Behavior changed for job {job["id"]}; saved mismatch')
                rows.append({'job':job['id'],'repeat':repeat,'request':request,
                    'score':values[0]['score'],'baseline_seconds':walls[0],'candidate_seconds':walls[1],
                    'baseline_policy_us':sum(d['response']['elapsed_us'] for d in values[0]['decisions']),
                    'candidate_policy_us':sum(d['response']['elapsed_us'] for d in values[1]['decisions'])})
        finally:
            for proc in processes:
                if proc.returncode is None:proc.kill()
            await asyncio.gather(*(proc.wait() for proc in processes))
    start = time.monotonic()
    tasks = [asyncio.create_task(worker()) for _ in range(args.workers)]
    timed_out = False
    try:
        await asyncio.wait_for(asyncio.gather(*tasks),args.seconds)
    except asyncio.TimeoutError:
        timed_out = True
    finally:
        for task in tasks:task.cancel()
        await asyncio.gather(*tasks,return_exceptions=True)
    result = {'schema':'kiln-played-paired-benchmark-v1','workers':args.workers,
        'elapsed_seconds':time.monotonic()-start,'timed_out':timed_out,'pairs':len(rows),
        'requested_pairs':args.count*args.repeats,'compared_moves':len(rows)*28,
        'baseline_sha256':p.digest(args.baseline),'candidate_sha256':p.digest(args.candidate),
        'median_speedup':statistics.median(r['baseline_seconds']/r['candidate_seconds'] for r in rows) if rows else None,
        'aggregate_speedup':sum(r['baseline_seconds'] for r in rows)/sum(r['candidate_seconds'] for r in rows) if rows else None,
        'rows':sorted(rows,key=lambda r:(r['repeat'],r['job']))}
    p.atomic_json(args.output,result)
    print(p.canonical({k:v for k,v in result.items() if k!='rows'}))


if __name__=='__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory');parser.add_argument('baseline');parser.add_argument('candidate');parser.add_argument('output')
    parser.add_argument('--count',type=int,default=36);parser.add_argument('--workers',type=int,default=18)
    parser.add_argument('--repeats',type=int,default=2);parser.add_argument('--seconds',type=float,default=60)
    args = parser.parse_args()
    asyncio.run(benchmark(args))
