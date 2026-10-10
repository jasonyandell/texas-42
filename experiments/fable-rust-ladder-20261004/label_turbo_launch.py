#!/usr/bin/env python3
"""Label the void-aware call logs with the compiled engine at N worlds (delta 1) in capped,
resumable shards: each worker owns a contiguous slice of unique calls; every round appends a new
part file starting after the records the previous rounds wrote (100-byte fixed records)."""
import json, subprocess, sys, time
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]; CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; B = HERE / 'ladder/target/release/ladder'
split, worlds, workers, rounds = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
total = int(sys.argv[5])  # unique multi/forced calls in the log (upper bound is fine)
OUT = HERE / 'data'; tag = f'labelsT{worlds}-{split}'
per = -(-total // workers)
for r in range(rounds):
    procs = []
    for w in range(workers):
        done = sum(p.stat().st_size // 100 for p in OUT.glob(f'{tag}-{w:02d}-r*.bin'))
        if done >= per: continue
        part = OUT / f'{tag}-{w:02d}-r{r}.bin'
        if part.exists(): continue
        cmd = [sys.executable, str(CAP), '--seconds', '295', '--output-dir', str(HERE / 'results' / f'label-{tag}-{w:02d}-r{r}-log'), '--', str(B), 'label', '--engine', 'turbo', '--in', str(OUT / f'callsv-{split}.jsonl'), '--out', str(part), '--worlds', str(worlds), '--skip', str(w * per + done), '--limit', str(per - done)]
        procs.append(subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
    if not procs: break
    t0 = time.time()
    for p in procs: p.wait(timeout=299)
    n = sum(p.stat().st_size // 100 for p in OUT.glob(f'{tag}-*-r*.bin'))
    print(json.dumps({'round': r, 'wall': round(time.time() - t0, 1), 'records': n, 'target': total}), flush=True)
parts = sorted(OUT.glob(f'{tag}-*-r*.bin'))
with open(OUT / f'{tag}.bin', 'wb') as f:
    for p in parts: f.write(p.read_bytes())
print('merged', (OUT / f'{tag}.bin').stat().st_size // 100)
