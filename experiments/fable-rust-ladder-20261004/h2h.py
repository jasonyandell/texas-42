#!/usr/bin/env python3
"""Paired mirrored head-to-head: hybrid (net replaces the level-0 mind) vs native pinned baseline.
Launches capped worker processes over disjoint deal slices, then aggregates.
"""
import argparse, json, subprocess, sys, time
from pathlib import Path
import numpy as np
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]; CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'; import os; B = Path(os.environ.get('LADDER_BIN', HERE / 'ladder/target/release/ladder'))
ap = argparse.ArgumentParser(); ap.add_argument('--net', required=True); ap.add_argument('--seed', type=int, default=3000); ap.add_argument('--deals', type=int, default=64); ap.add_argument('--workers', type=int, default=12); ap.add_argument('--tag', required=True); ap.add_argument('--summarize-only', action='store_true'); ap.add_argument('--round', type=int, default=0); ap.add_argument('--voids', action='store_true', help='outer carries public voids in the Key (needed by void-aware nets)')
a = ap.parse_args(); out = HERE / 'results' / f'h2h-{a.tag}'; out.mkdir(parents=True, exist_ok=True)
if not a.summarize_only:
    procs = []; t0 = time.time()
    for w in range(a.workers):
        log = out / (f'worker-{w:02d}-log' if a.round == 0 else f'worker-{w:02d}-r{a.round}-log'); assert not log.exists(), log
        cmd = [sys.executable, str(CAP), '--seconds', '295', '--output-dir', str(log), '--', str(B), 'h2h', '--seed', str(a.seed), '--deals', str(a.deals), '--worker', str(w), '--workers', str(a.workers), '--net', a.net, '--out', str(out / f'games-{w:02d}.jsonl')] + (['--voids'] if a.voids else [])
        procs.append(subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
    for p in procs: p.wait(timeout=299)
    statuses = [json.loads((out / (f'worker-{w:02d}-log' if a.round == 0 else f'worker-{w:02d}-r{a.round}-log') / 'run.json').read_text())['status'] for w in range(a.workers)]
    print('workers', dict(zip(*np.unique(statuses, return_counts=True))), 'wall', round(time.time() - t0, 1))
games = [json.loads(l) for f in sorted(out.glob('games-*.jsonl')) for l in open(f) if l.strip()]
by = {}
for g in games: by.setdefault(g['deal'], {})[g['side']] = g
pairs = [(d, v[0], v[1]) for d, v in by.items() if 0 in v and 1 in v]
diff = np.array([int(x[1]['hybrid_won']) - int(1 - x[2]['hybrid_won']) for x in pairs])  # +1: hybrid won where native lost on the mirror
# Mirror reading: side0 game has hybrid on partnership 0; side1 game has hybrid on partnership 1. For each deal, compare
# partnership-0 outcome when played by hybrid (side0) vs by native (side1): flip = hybrid_won(side0) - (not hybrid_won(side1)).
hyb_win = np.array([int(x[1]['hybrid_won']) for x in pairs]); nat_win_same_seat = np.array([int(not x[2]['hybrid_won']) for x in pairs])
d = hyb_win - nat_win_same_seat; n = len(d)
rng = np.random.default_rng(7); boot = d[rng.integers(n, size=(5000, n))].mean(1) if n else np.array([0])
hus = np.concatenate([g['hybrid_us'] for g in games]) if games else np.array([0]); nus = np.concatenate([g['native_us'] for g in games]) if games else np.array([0])
res = dict(tag=a.tag, net=a.net, deals_paired=n, games=len(games), hybrid_wins_where_native_lost=int((d == 1).sum()), native_wins_where_hybrid_lost=int((d == -1).sum()), same_outcome=int((d == 0).sum()), mean_paired_advantage=float(d.mean()) if n else None, ci95=[float(np.quantile(boot, .025)), float(np.quantile(boot, .975))],
           hybrid_overall_win_rate=float(np.mean([g['hybrid_won'] for g in games])) if games else None, hybrid_decision_us=dict(median=float(np.median(hus)), p95=float(np.quantile(hus, .95)), max=float(hus.max())), native_decision_us=dict(median=float(np.median(nus)), p95=float(np.quantile(nus, .95)), max=float(nus.max())), game_seconds_mean=float(np.mean([g['game_seconds'] for g in games])) if games else None,
           qualification='same deal, same contract, both seatings; outcome = contract won by the hybrid partnership; paired flips; exploratory, no multiplicity correction')
(out / 'summary.json').write_text(json.dumps(res, indent=1) + '\n'); print(json.dumps(res))
