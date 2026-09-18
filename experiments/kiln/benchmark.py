#!/usr/bin/env python3
"""Bounded paired timing on identical retained requests; never changes policy.
This diagnostic repeats completed work. Production firings remain the primary
throughput instrument. Completed pairs survive a deadline, unfinished pairs do not.
"""
import argparse,asyncio,json,math,os,statistics,time
from pathlib import Path
from kiln import connect,canonical,digest,atomic_json

async def run(a):
    db=connect(a.directory)
    rows=db.execute('SELECT payload FROM results WHERE worlds=? ORDER BY (job_id*7919)%104729 LIMIT ?',
                    (a.worlds,a.cases)).fetchall()
    db.close()
    if len(rows)!=a.cases:raise ValueError('Not enough completed jobs at this depth')
    records=[json.loads(r[0]) for r in rows];pairs=[];sem=asyncio.Semaphore(a.workers)
    async def call(binary,request):
        proc=await asyncio.create_subprocess_exec(str(binary),stdin=asyncio.subprocess.PIPE,stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.DEVNULL,env={**os.environ,'RAYON_NUM_THREADS':'1'})
        try:
            out,_=await proc.communicate((canonical(request)+'\n').encode())
            if proc.returncode!=0:raise RuntimeError('Worker failed')
            value=json.loads(out)
            if 'error' in value:raise ValueError(value['error'])
            return value
        finally:
            if proc.returncode is None:proc.kill();await proc.wait()
    def report():
        ratios=[p['baseline_us']/max(p['candidate_us'],1) for p in pairs]
        return {'schema':'kiln-paired-timing-v1','baseline':digest(a.baseline),'candidate':digest(a.candidate),
            'requested':a.cases,'completed':len(pairs),'worlds':a.worlds,'workers':a.workers,
            'median_speedup':statistics.median(ratios) if ratios else None,
            'geometric_speedup':math.exp(sum(map(math.log,ratios))/len(ratios)) if ratios else None,
            'pairs':pairs}
    async def pair(i,old):
        async with sem:
            req={'auction':old['auction'],'decl':old['price'][0],'worlds':old['worlds'],'budget_ms':60000}
            bins=[('baseline',a.baseline),('candidate',a.candidate)]
            if i%2:bins.reverse()
            got={}
            for label,binary in bins:
                v=await call(binary,req)
                if v['price']!=old['price']:raise ValueError('Price mismatch')
                for k in ('nodes','pi_calls','inner_worlds'):
                    if v['work'][k]!=old['work'][k]:raise ValueError(f'{k} mismatch')
                got[label+'_us']=v['work']['elapsed_us']
            pairs.append({'request':req,**got})
            atomic_json(a.output,report())
    tasks=[asyncio.create_task(pair(i,r)) for i,r in enumerate(records)]
    try:await asyncio.wait_for(asyncio.gather(*tasks),a.seconds)
    except asyncio.TimeoutError:pass
    finally:
        for t in tasks:t.cancel()
        await asyncio.gather(*tasks,return_exceptions=True)
        r=report();atomic_json(a.output,r);print(canonical({k:v for k,v in r.items() if k!='pairs'}))

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('directory');p.add_argument('baseline',type=Path);p.add_argument('candidate',type=Path);p.add_argument('output');p.add_argument('--cases',type=int,default=16);p.add_argument('--worlds',type=int,default=160);p.add_argument('--workers',type=int,default=4);p.add_argument('--seconds',type=float,default=60)
    a=p.parse_args()
    if min(a.cases,a.worlds,a.workers,a.seconds)<=0:p.error('Bounds must be positive')
    asyncio.run(run(a))
