#!/usr/bin/env python3
"""Level-2 teacher self-play labeling in capped rounds; each round uses a fresh seed range.
Parts: data/labels2-{tag}-r{round}-w{worker}.bin; merged train/val (worker 15 = val)."""
import json, os, subprocess, sys, time
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]; CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; B = HERE / 'ladder/target-c/release/ladder'
inner, outer, workers, rounds = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]); eps = sys.argv[5] if len(sys.argv) > 5 else '0.1'
tag = sys.argv[6] if len(sys.argv) > 6 else f'{outer}-{inner}'; extra = sys.argv[7:]  # e.g. --play np:models/x.w  --model models/y.w
seed_base = int(os.environ.get('LOG2_SEED', '1000000')); B = Path(os.environ.get('LADDER_BIN', B)); D = HERE / 'data'
for r in range(rounds):
    procs = []
    for w in range(workers):
        part = D / f'labels2-{tag}-r{r}-w{w:02d}.bin'
        if part.exists(): continue
        cmd = [sys.executable, str(CAP), '--seconds', '295', '--output-dir', str(HERE / 'results' / f'log2-{tag}-r{r}-w{w:02d}-log'), '--', str(B), 'log2', '--seed', str(seed_base + 100_000 * r), '--inner', str(inner), '--outer', str(outer), '--eps', eps, '--worker', str(w), '--workers', str(workers), '--out', str(part)] + extra
        procs.append(subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
    t0 = time.time()
    for p in procs: p.wait(timeout=299)
    n = sum(p.stat().st_size // 100 for p in D.glob(f'labels2-{tag}-r*-w*.bin'))
    print(json.dumps({'round': r, 'wall': round(time.time() - t0, 1), 'states': n}), flush=True)
tr = sorted(p for p in D.glob(f'labels2-{tag}-r*-w*.bin') if not p.name.endswith(f'w{workers-1:02d}.bin')); va = sorted(D.glob(f'labels2-{tag}-r*-w{workers-1:02d}.bin'))
for name, parts in (('train', tr), ('val', va)):
    with open(D / f'labels2-{tag}-{name}.bin', 'wb') as f:
        for p in parts: f.write(p.read_bytes())
    print(name, (D / f'labels2-{tag}-{name}.bin').stat().st_size // 100)
