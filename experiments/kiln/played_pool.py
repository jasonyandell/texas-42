#!/usr/bin/env python3
"""Compare pool sizes on the same complete-game workload, within one minute."""
import argparse
import asyncio
import hashlib
import json
import os
from pathlib import Path
import time

import played as p
from played_benchmark import semantic


async def measure(binary,requests,workers,seconds):
    queue = asyncio.Queue()
    for i,request in enumerate(requests):queue.put_nowait((i,request))
    rows = []
    async def worker():
        proc = await asyncio.create_subprocess_exec(str(Path(binary).resolve()),
            stdin=asyncio.subprocess.PIPE,stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.DEVNULL,
            limit=4*1024*1024,env={**os.environ,'RAYON_NUM_THREADS':'1'})
        try:
            while not queue.empty():
                i,request = queue.get_nowait()
                proc.stdin.write((p.canonical(request)+'\n').encode());await proc.stdin.drain()
                value = json.loads(await proc.stdout.readline())
                p.validate_game(request,value)
                rows.append((i,hashlib.sha256(p.canonical(semantic(value)).encode()).hexdigest()))
        finally:
            if proc.returncode is None:proc.kill()
            await proc.wait()
    tasks = [asyncio.create_task(worker()) for _ in range(workers)]
    start = time.monotonic()
    try:
        await asyncio.wait_for(asyncio.gather(*tasks),seconds)
    finally:
        for task in tasks:task.cancel()
        await asyncio.gather(*tasks,return_exceptions=True)
    elapsed = time.monotonic()-start
    return {'workers':workers,'seconds':elapsed,'games':len(rows),'games_per_second':len(rows)/elapsed,
            'outcome_digest':hashlib.sha256(p.canonical(sorted(rows)).encode()).hexdigest()}


async def main(args):
    db = p.connect(args.directory)
    jobs = [dict(r) for r in db.execute(p.JOB_SQL+" WHERE j.state='done' ORDER BY j.id")]
    db.close()
    if len(jobs)<args.count:raise ValueError('Insufficient saved games')
    requests = [p.game_request(jobs[i*len(jobs)//args.count]) for i in range(args.count)]
    start = time.monotonic();rows=[]
    for workers in args.workers:
        result = await measure(args.binary,requests,workers,max(.01,args.seconds-(time.monotonic()-start)))
        if rows and rows[0]['outcome_digest'] != result['outcome_digest']:
            raise ValueError('Pool layout changed game behavior')
        rows.append(result)
        print(p.canonical(result),flush=True)
    result = {'schema':'kiln-played-pool-v1','binary_sha256':p.digest(args.binary),
              'requests':requests,'rows':rows,'elapsed_seconds':time.monotonic()-start}
    p.atomic_json(args.output,result)


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory');parser.add_argument('output');parser.add_argument('--binary',default=str(p.BINARY))
    parser.add_argument('--count',type=int,default=72);parser.add_argument('--workers',type=int,nargs='+',default=[18,12,24,18])
    parser.add_argument('--seconds',type=float,default=60)
    asyncio.run(main(parser.parse_args()))
