#!/usr/bin/env python3
"""Capped resumable batch runner (<= 2 workers, each child under run_capped.py)."""
import argparse, concurrent.futures, subprocess, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]; CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; PY = '/Users/jason/code/flux/python/.venv/bin/python'
STAGES = {'mine': ('positions', 'block-{i:03d}.json.gz', ['mine', '--block', '{i}']), 'labels': ('data/train', 'batch-{i:03d}.npz', ['generate', '--kind', 'train', '--block', '{i}']), 'features': ('features/train', 'batch-{i:03d}.npz', ['features', '--kind', 'train', '--block', '{i}']),
          'test-labels': ('data/test', 'batch-{i:03d}.npz', ['generate', '--kind', 'test', '--block', '{i}']), 'test-features': ('features/test', 'batch-{i:03d}.npz', ['features', '--kind', 'test', '--block', '{i}'])}
ap = argparse.ArgumentParser(); ap.add_argument('--stage', choices=list(STAGES), required=True); ap.add_argument('--batches', required=True); ap.add_argument('--workers', type=int, default=2); a = ap.parse_args(); lo, hi = map(int, a.batches.split(':')); assert 1 <= a.workers <= 2
folder, pattern, args = STAGES[a.stage]
def run(i):
    if (HERE / folder / pattern.format(i=i)).exists(): return 'exists'
    log = HERE / 'results' / f'{a.stage}-{i:03d}-log'; assert not log.exists(), log
    r = subprocess.run([sys.executable, str(CAP), '--seconds', '295', '--output-dir', str(log), '--', PY, str(HERE / 'inner.py')] + [x.format(i=i) for x in args], timeout=299); assert r.returncode == 0, (a.stage, i); return 'done'
with concurrent.futures.ThreadPoolExecutor(max_workers=a.workers) as pool: print(dict(zip(range(lo, hi), pool.map(run, range(lo, hi)))))
