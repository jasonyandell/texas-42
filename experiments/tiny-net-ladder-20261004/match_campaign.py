#!/usr/bin/env python3
import concurrent.futures,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];CAP=ROOT/'experiments/astra-sol-20261004/tools/run_capped.py';PY='/Users/jason/code/flux/python/.venv/bin/python'
tasks=[(v,s,r,role) for v in ['teacher0','n0-32','n1-32'] for s in [61010531,61010532] for r in range(4) for role in ['declaring','defending']]
def run(task):
 v,s,r,role=task;out=HERE/'results/h2h'/v/f'game-{s}-{r}-{role}.json'
 if out.exists():return
 log=HERE/'results'/f'h2h-{v}-{s}-{r}-{role}-log';assert not log.exists()
 cmd=[sys.executable,str(CAP),'--seconds','120','--output-dir',str(log),'--',PY,str(HERE/'match.py'),'run','--variant',v,'--seed',str(s),'--rotation',str(r),'--role',role]
 result=subprocess.run(cmd,timeout=125);assert result.returncode==0,task
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(run,tasks))
