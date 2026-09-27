#!/usr/bin/env python3
"""Small reproducible research panel; each case has its own process watchdog."""
from concurrent.futures import ThreadPoolExecutor, as_completed
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]
BIN=ROOT/'walt/target/release/nello-counterexample'
WATCHDOG=ROOT/'experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py'
POSITIONS=json.loads((HERE.parent/'nello-ruby-doubles-2026-09-21/positions.json').read_text())['positions']
OUT=HERE/(sys.argv[1] if len(sys.argv)>1 else 'panel-v2')

def run(job):
    position,n,seed=job
    name=f"ply{position['ply']}-n{n}-seed{seed}"
    config={'request':{**position['request'],'seed':seed},'ordinary':n,'rounds':3,'witnesses_per_round':4,'attack_pool':256,'examination':4096,'n0':8,'budget_ms':240000}
    source=OUT/'inputs'/f'{name}.json'; output=OUT/'cases'/f'{name}.json'
    if output.exists(): raise RuntimeError(f'refusing to overwrite {output}')
    source.write_text(json.dumps(config,indent=2)+'\n')
    result=subprocess.run([sys.executable,str(WATCHDOG),'--seconds','295','--output-dir',str(OUT/'runs'/name),'--',str(BIN),str(source),str(output)],cwd=ROOT,capture_output=True,text=True)
    data=json.loads(output.read_text()) if output.exists() else {}
    return {'name':name,'returncode':result.returncode,'status':data.get('status'),'paired_augmented':data.get('paired_augmented'),'total_ms':data.get('total_ms'),'stdout':result.stdout.strip()}

if __name__=='__main__':
    (OUT/'inputs').mkdir(parents=True,exist_ok=True);(OUT/'cases').mkdir(exist_ok=True)
    jobs=[(p,n,s) for p in POSITIONS for n in [40,160] for s in range(1,9)]
    started=time.monotonic(); rows=[]
    binary_hash=hashlib.sha256(BIN.read_bytes()).hexdigest()
    sources=['walt/walt/src/solver/mod.rs','walt/walt/src/solver/nello_counterexample.rs','walt/walt-player/src/bin/nello-counterexample.rs']
    with ThreadPoolExecutor(max_workers=2) as pool:
        for f in as_completed([pool.submit(run,j) for j in jobs]):
            row=f.result();rows.append(row);print(json.dumps({'completed':len(rows),'total':len(jobs),**row}),flush=True)
    report={'binary_sha256':binary_hash,'source_sha256':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources},'pool_elapsed_seconds':time.monotonic()-started,'cases':rows}
    (OUT/'panel-run.json').write_text(json.dumps(report,indent=2)+'\n')
    if any(r['returncode'] for r in rows):sys.exit(2)
