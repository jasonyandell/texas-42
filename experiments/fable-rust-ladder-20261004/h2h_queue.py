#!/usr/bin/env python3
"""Run paired h2h arms sequentially, each in resumable capped rounds until all deals are paired."""
import json, subprocess, sys, time
from pathlib import Path
HERE = Path(__file__).resolve().parent
arms = sys.argv[1].split(',') ; deals = int(sys.argv[2]); workers = int(sys.argv[3]); seed = int(sys.argv[4]); max_rounds = int(sys.argv[5]) if len(sys.argv) > 5 else 4
for arm in arms:
    tag = f"{arm.replace(':', '-').replace('/', 'o')}-s{seed}-{deals}"
    for r in range(max_rounds):
        existing = sorted((HERE / 'results' / f'h2h-{tag}').glob('worker-00*log')) if (HERE / 'results' / f'h2h-{tag}').exists() else []
        if r < len(existing): continue
        cmd = [sys.executable, str(HERE / 'h2h.py'), '--net', arm, '--seed', str(seed), '--deals', str(deals), '--workers', str(workers), '--tag', tag, '--round', str(r)]
        t0 = time.time(); out = subprocess.run(cmd, capture_output=True, text=True)
        summ = json.loads((HERE / 'results' / f'h2h-{tag}' / 'summary.json').read_text())
        print(json.dumps({'arm': arm, 'round': r, 'wall': round(time.time() - t0, 1), 'deals_paired': summ['deals_paired'], 'adv': summ['mean_paired_advantage'], 'ci': summ['ci95'], 'hyb_us': summ['hybrid_decision_us']['median'], 'nat_us': summ['native_decision_us']['median']}), flush=True)
        if summ['deals_paired'] >= deals: break
