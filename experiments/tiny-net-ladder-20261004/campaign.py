#!/usr/bin/env python3
"""Resumable bounded batch launcher; never detached, at most two label jobs."""
import argparse,concurrent.futures,json,subprocess,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];CAP=ROOT/'experiments/astra-sol-20261004/tools/run_capped.py';PY='/Users/jason/code/flux/python/.venv/bin/python'
p=argparse.ArgumentParser();p.add_argument('--kind',default='labels0');p.add_argument('--batches',default='0:16');p.add_argument('--model');p.add_argument('--worlds',type=int);a=p.parse_args();lo,hi=map(int,a.batches.split(':'))
def run(i):
 out=HERE/'data'/a.kind/f'batch-{i:03d}.npz'
 if out.exists() and out.with_suffix('.json').exists():return {'batch':i,'status':'existing'}
 log=HERE/'results'/f'{a.kind}-{i:03d}';assert not log.exists(),f'preserve failed receipt; use a new kind: {log}'
 cmd=[sys.executable,str(CAP),'--seconds','295','--output-dir',str(log),'--',PY,str(HERE/'pilot.py'),'generate','--kind',a.kind,'--batch',str(i)]
 if a.model:cmd+=['--model',a.model]
 if a.worlds:cmd+=['--worlds',str(a.worlds)]
 r=subprocess.run(cmd,timeout=299);assert r.returncode==0,i
 return {'batch':i,'status':'complete'}
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
 for r in pool.map(run,range(lo,hi)):print(json.dumps(r),flush=True)
