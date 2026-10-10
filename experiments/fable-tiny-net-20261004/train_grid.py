#!/usr/bin/env python3
"""Sequential capped GPU trainings for the frozen grid; skips finished arms."""
import argparse, subprocess, sys
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]
CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; PY = '/Users/jason/code/flux/python/.venv/bin/python'
GRID = [(f'{arch}-{d}', arch, d, []) for arch in ['raw-h32', 'raw-h256', 'raw-d256'] for d in ['d1', 'd4', 'd16']]
GRID += [(f'feat-s64-{d}', 'feat-s64', d, []) for d in ['d1', 'd4', 'd16']]
GRID += [('raw-h256-reg-d1', 'raw-h256', 'd1', ['--lr', '0.0005', '--l2', '0.001'])]
ap = argparse.ArgumentParser(); ap.add_argument('--only', nargs='*'); a = ap.parse_args()
for name, arch, deals, extra in GRID:
    if a.only and name not in a.only: continue
    if (HERE / 'models' / f'{name}.npz').exists() and (HERE / 'models' / f'{name}.json').exists(): print('exists', name); continue
    log = HERE / 'results' / f'train-{name}-log'; assert not log.exists(), log
    cmd = [sys.executable, str(CAP), '--seconds', '295', '--output-dir', str(log), '--', PY, str(HERE / 'ladder.py'), 'train', '--arch', arch, '--deals', deals, '--name', name] + extra
    r = subprocess.run(cmd, timeout=299); assert r.returncode == 0, name
    print(open(log / 'stdout.log').read().strip().splitlines()[-1][:400])
