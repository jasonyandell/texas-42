#!/usr/bin/env python3
"""Bounded paired 8/40/160 ladders through persistent native workers.

Every warm scalar must match a cold calculation. Work counts intentionally differ
when completed pure policy answers are reused. Never mixes the sample estimates.
"""
import argparse, asyncio, json, math, os, statistics
from pathlib import Path
from kiln import connect, canonical, digest, atomic_json


async def run(a):
    db=connect(a.directory)
    rows=db.execute('SELECT payload FROM results WHERE worlds=160 ORDER BY (job_id*7919)%104729 LIMIT ?', (a.cases,)).fetchall()
    db.close()
    if len(rows)!=a.cases: raise ValueError('Not enough retained deep requests')
    sem=asyncio.Semaphore(a.workers); pairs=[]

    async def call(binary, requests):
        proc=await asyncio.create_subprocess_exec(str(binary),stdin=asyncio.subprocess.PIPE,
            stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.DEVNULL,
            env={**os.environ,'RAYON_NUM_THREADS':'1'})
        try:
            out,_=await proc.communicate(('\n'.join(map(canonical,requests))+'\n').encode())
            if proc.returncode!=0: raise RuntimeError('Worker failed')
            values=[json.loads(line) for line in out.splitlines()]
            if len(values)!=len(requests) or any('error' in v for v in values): raise ValueError(values)
            return values
        finally:
            if proc.returncode is None: proc.kill(); await proc.wait()

    def report():
        ratios=[p['baseline_us']/max(1,p['candidate_us']) for p in pairs]
        return {'schema':'kiln-carry-timing-v1','baseline':digest(a.baseline),'candidate':digest(a.candidate),
            'requested':a.cases,'completed':len(pairs),'worlds':[8,40,160],'workers':a.workers,
            'median_speedup':statistics.median(ratios) if ratios else None,
            'geometric_speedup':math.exp(sum(map(math.log,ratios))/len(ratios)) if ratios else None,'pairs':pairs}

    async def pair(index, record):
        async with sem:
            requests=[{'auction':record['auction'],'decl':record['price'][0],'worlds':n,'budget_ms':60000} for n in (8,40,160)]
            order=[('baseline',a.baseline),('candidate',a.candidate)]
            if index%2: order.reverse()
            got={name:await call(binary,requests) for name,binary in order}
            for before,after in zip(got['baseline'],got['candidate']):
                for key in ('auction','worlds','inner_worlds','price'):
                    if before[key]!=after[key]: raise ValueError((key,requests,before,after))
                if after['work']['nodes']>before['work']['nodes']: raise ValueError('Warm cache added search work')
            if got['candidate'][-1]['price']!=record['price']: raise ValueError('Retained deep price mismatch')
            row={'requests':requests}
            for name, values in got.items():
                row[name+'_us']=sum(v['work']['elapsed_us'] for v in values)
                row[name+'_nodes']=sum(v['work']['nodes'] for v in values)
                row[name+'_work']=[v['work'] for v in values]
            pairs.append(row); atomic_json(a.output,report())

    tasks=[asyncio.create_task(pair(i,json.loads(row[0]))) for i,row in enumerate(rows)]
    try: await asyncio.wait_for(asyncio.gather(*tasks),a.seconds)
    except asyncio.TimeoutError: pass
    finally:
        for task in tasks: task.cancel()
        await asyncio.gather(*tasks,return_exceptions=True)
        result=report();atomic_json(a.output,result)
        print(canonical({k:v for k,v in result.items() if k!='pairs'}))


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('directory');p.add_argument('baseline',type=Path);p.add_argument('candidate',type=Path);p.add_argument('output')
    p.add_argument('--cases',type=int,default=32);p.add_argument('--workers',type=int,default=4);p.add_argument('--seconds',type=float,default=60)
    a=p.parse_args()
    if min(a.cases,a.workers,a.seconds)<=0: p.error('Bounds must be positive')
    asyncio.run(run(a))
