#!/usr/bin/env python3
"""Two-worker resumable batches, each within inherited process-group cap."""
import argparse,concurrent.futures,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];CAP=ROOT/'experiments/astra-sol-20261004/tools/run_capped.py';PY='/Users/jason/code/flux/python/.venv/bin/python'
from coverage import selected_rows
p=argparse.ArgumentParser();p.add_argument('--kind',required=True);p.add_argument('--batches',default='0:14');p.add_argument('--model');a=p.parse_args();lo,hi=map(int,a.batches.split(':'))
def run(i):
 if not selected_rows(a.kind,i):return
 out=HERE/'data'/a.kind/f'batch-{i:03d}.npz'
 if out.exists() and out.with_suffix('.json').exists():return
 log=HERE/'results'/f'{a.kind}-{i:03d}-log';assert not log.exists()
 cmd=[sys.executable,str(CAP),'--seconds','295','--output-dir',str(log),'--',PY,str(HERE/'coverage.py'),'generate','--kind',a.kind,'--batch',str(i)]
 if a.model:cmd+=['--model',a.model]
 r=subprocess.run(cmd,timeout=299);assert r.returncode==0,i
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(run,range(lo,hi)))
