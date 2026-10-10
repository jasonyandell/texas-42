#!/usr/bin/env python3
"""Capped, resumable batch runner. Every child runs under run_capped.py (<=295 s).

Skips batches whose output exists; refuses to reuse a log directory; at most two
concurrent workers. Parent timeout 299 s per child is a second guard only.
"""
import argparse, concurrent.futures, subprocess, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]
CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; PY = '/Users/jason/code/flux/python/.venv/bin/python'
STAGES = {
    'mine': ('positions', 'block-{i:03d}.json.gz', ['mine', '--block', '{i}']),
    'labels': ('data/train', 'batch-{i:03d}.npz', ['generate', '--kind', 'train', '--batch', '{i}']),
    'features': ('features/train', 'batch-{i:03d}.npz', ['features', '--kind', 'train', '--batch', '{i}']),
    'test-teacher': ('data/test-teacher128', 'batch-{i:03d}.npz', ['generate', '--kind', 'test-teacher128', '--batch', '{i}']),
    'test-reference': ('data/test-reference', 'batch-{i:03d}.npz', ['generate', '--kind', 'test-reference', '--batch', '{i}']),
    'test-features': ('features/test', 'batch-{i:03d}.npz', ['features', '--kind', 'test', '--batch', '{i}']),
}
ap = argparse.ArgumentParser(); ap.add_argument('--stage', choices=list(STAGES), required=True); ap.add_argument('--batches', required=True); ap.add_argument('--workers', type=int, default=2); ap.add_argument('--seconds', type=int, default=295)
a = ap.parse_args(); lo, hi = map(int, a.batches.split(':')); assert 1 <= a.workers <= 2 and a.seconds <= 295
folder, pattern, args = STAGES[a.stage]


def run(i):
    out = HERE / folder / pattern.format(i=i)
    if out.exists(): return 'exists'
    log = HERE / 'results' / f'{a.stage}-{i:03d}-log'; assert not log.exists(), log
    cmd = [sys.executable, str(CAP), '--seconds', str(a.seconds), '--output-dir', str(log), '--', PY, str(HERE / 'ladder.py')] + [x.format(i=i) for x in args]
    r = subprocess.run(cmd, timeout=299); assert r.returncode == 0, (a.stage, i, r.returncode); return 'done'


with concurrent.futures.ThreadPoolExecutor(max_workers=a.workers) as pool:
    print(dict(zip(range(lo, hi), pool.map(run, range(lo, hi)))))
