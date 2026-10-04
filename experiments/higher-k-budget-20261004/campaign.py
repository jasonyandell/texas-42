#!/usr/bin/env python3
"""Sequential orchestration; each individual experiment gets its own watchdog."""
import argparse,json,os,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
RESULTS=Path(os.environ.get('WALT_K_RESULTS',str(HERE/'results'))).resolve()
p=argparse.ArgumentParser();p.add_argument('panel',choices=['roots','unconstrained','games','parallel']);a=p.parse_args();plan=json.loads((HERE/'plan.json').read_text())
commands=[]
if a.panel=='parallel':
 for rep in range(6):
  for workers in ([1,4]if rep%2==0 else [4,1]):
   if not(RESULTS/'parallel'/f'workers-{workers}-rep-{rep}.json').exists():commands.append((f'parallel-log-{workers}-{rep}',['parallel','--workers',str(workers),'--rep',str(rep)]))
else:
 directory={'roots':'fixed-roots','unconstrained':'unconstrained','games':'games'}[a.panel]
 for seed in (plan['seeds'][:9]if a.panel=='unconstrained'else plan['seeds']):
  if not(RESULTS/directory/f'deal-{seed}.json').exists():commands.append((f'{a.panel}-log-{seed}',[a.panel,'--seed',str(seed)]))
for label,args in commands:
 command=[sys.executable,str(ROOT/'experiments/astra-sol-20261004/tools/run_capped.py'),'--seconds','295','--output-dir',str(RESULTS/label),'--',sys.executable,str(HERE/'experiment.py'),*args]
 subprocess.run(command,cwd=ROOT,check=True)
print(f'Completed {a.panel}: {len(commands)} separately capped batches',flush=True)
