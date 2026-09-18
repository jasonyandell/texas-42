#!/usr/bin/env python3
"""Compare a candidate binary with retained prices and deeper baseline jobs."""
import argparse,asyncio,json,os,time
from pathlib import Path
from kiln import connect,canonical,digest,atomic_json

async def main(a):
    db=connect(a.directory)
    rows=db.execute('SELECT payload FROM results WHERE producer=? ORDER BY (job_id*7919)%104729 LIMIT ?',
                    (digest(a.baseline),a.cases)).fetchall()
    assert len(rows)==a.cases
    requests=[]
    for row in rows:
        old=json.loads(row[0]);req={'auction':old['auction'],'decl':old['price'][0],'worlds':old['worlds'],'budget_ms':60000}
        requests.append((req,old))
    # Larger bundles must remain information-consistent too.
    for i,n in enumerate([4,12,40,160]):
        req={**requests[i*7][0],'worlds':n};requests.append((req,None))
    sem=asyncio.Semaphore(a.workers);evidence=[]
    async def call(binary,req):
        proc=await asyncio.create_subprocess_exec(str(binary),stdin=asyncio.subprocess.PIPE,stdout=asyncio.subprocess.PIPE,
             stderr=asyncio.subprocess.PIPE,env={**os.environ,'RAYON_NUM_THREADS':'1'})
        try:
            out,err=await asyncio.wait_for(proc.communicate((canonical(req)+'\n').encode()),65)
            assert proc.returncode==0,err.decode()
            value=json.loads(out);assert 'error' not in value,value
            return value
        finally:
            if proc.returncode is None:proc.kill();await proc.wait()
    async def check(req,old):
        async with sem:
            if old is None:old=await call(a.baseline,req)
            candidate=await call(a.candidate,req)
            assert candidate['price']==old['price'],(req,candidate,old)
            # Same visit counts provide stronger evidence than the scalar alone.
            for key in ['nodes','pi_calls','inner_worlds']:
                assert candidate['work'][key]==old['work'][key],(key,req,candidate['work'],old['work'])
            evidence.append({'request':req,'price':candidate['price'],'baseline_us':old['work']['elapsed_us'],
                'candidate_us':candidate['work']['elapsed_us'],'nodes':candidate['work']['nodes']})
    await asyncio.gather(*(check(req,old) for req,old in requests))
    report={'schema':'kiln-parity-v1','baseline':digest(a.baseline),'candidate':digest(a.candidate),'cases':len(evidence),'passed':True,'evidence':evidence}
    atomic_json(a.output,report);print(canonical({k:v for k,v in report.items() if k!='evidence'}))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('directory');p.add_argument('baseline',type=Path);p.add_argument('candidate',type=Path);p.add_argument('output');p.add_argument('--cases',type=int,default=117);p.add_argument('--workers',type=int,default=4)
    asyncio.run(main(p.parse_args()))
