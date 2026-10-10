#!/usr/bin/env python3
"""Launch one capped turbo-eval worker per depth bucket (trick index 0..6)."""
import subprocess, sys, time
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]; CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; B = HERE / 'ladder/target/release/ladder'
OUT = HERE / 'results/turbo-eval'; OUT.mkdir(exist_ok=True)
EARLY = '8:1,16:1,24:1,32:1,64:1,160:1,8:1/8,24:1/8,8:1/24,24:1/24'
LATE = EARLY + ',8:0,24:0'
procs = []
for b in range(7):
    arms = EARLY if b < 3 else LATE; per = '240' if b < 3 else '300'
    cmd = [sys.executable, str(CAP), '--seconds', '295', '--output-dir', str(OUT / f'bucket-{b}-log'), '--', str(B), 'turbo-eval', '--in', str(HERE / 'data/callsv-val.jsonl'), '--bucket', str(b), '--per', per, '--ref', '4096', '--arms', arms, '--walt', '8,160', '--out', str(OUT / f'bucket-{b}.jsonl'), '--seed', '77']
    procs.append(subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
t0 = time.time()
for p in procs: p.wait(timeout=299)
print('done', round(time.time() - t0, 1))
