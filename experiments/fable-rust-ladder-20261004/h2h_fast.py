#!/usr/bin/env python3
"""Paired mirrored head-to-head vs pinned production, protocol identical to h2h.py (same deals, contract,
seatings, 40/8 production through partnership_wire with the 14 s budget), but:
  * production's decisions come from a shared on-disk memo (`ladder h2h --prod-cache`), keyed by the full wire
    input, so every arm after the first on a seed skips the expensive openings (identical decisions, see PERF.md);
  * capped rounds are run automatically until every deal is paired (resume = the same command again);
  * more workers by default (the memo makes production cheap, so 8 workers finish 4,096 deals in one round).
Output layout and summary.json are the same as h2h.py's, so downstream readers need no change.
"""
import argparse, json, os, subprocess, sys, time
from pathlib import Path
HERE = Path(__file__).resolve().parent; ROOT = HERE.parents[1]; CAP = ROOT / 'experiments/astra-sol-20261004/tools/run_capped.py'
B = Path(os.environ.get('LADDER_BIN', HERE / 'ladder/target-s/release/ladder'))
ap = argparse.ArgumentParser(); ap.add_argument('--net', required=True); ap.add_argument('--seed', type=int, default=5000); ap.add_argument('--deals', type=int, default=4096); ap.add_argument('--workers', type=int, default=8); ap.add_argument('--tag', required=True)
ap.add_argument('--voids', action='store_true'); ap.add_argument('--max-rounds', type=int, default=6); ap.add_argument('--prod-cache', default='', help="memo directory; default results/prod-cache-s<seed>; 'none' disables")
ap.add_argument('--summarize-only', action='store_true'); ap.add_argument('--mixed', action='store_true', help='diagnostic: newcomer in one seat of its partnership, production partner (ladder h2h --mixed)')
a = ap.parse_args(); out = HERE / 'results' / f'h2h-{a.tag}'; out.mkdir(parents=True, exist_ok=True)
pc = None if a.prod_cache == 'none' else Path(a.prod_cache or (HERE / 'results' / f'prod-cache-s{a.seed}'))

def paired_count():
    by = {}
    for f in sorted(out.glob('games-*.jsonl')):
        for l in open(f):
            try: v = json.loads(l)
            except Exception: continue
            by.setdefault(v['deal'], set()).add(v['side'])
    return sum(1 for s in by.values() if s == {0, 1})

t_all = time.time(); rounds_run = 0
if not a.summarize_only:
    for r in range(a.max_rounds):
        if paired_count() >= a.deals: break
        procs = []; t0 = time.time()
        for w in range(a.workers):
            log = out / (f'worker-{w:02d}-log' if r == 0 else f'worker-{w:02d}-r{r}-log')
            k = 0
            while log.exists(): k += 1; log = out / f'worker-{w:02d}-r{r}-{k}-log'  # a rerun after a crash gets a fresh log dir
            cmd = [sys.executable, str(CAP), '--seconds', '295', '--output-dir', str(log), '--', str(B), 'h2h', '--seed', str(a.seed), '--deals', str(a.deals), '--worker', str(w), '--workers', str(a.workers), '--net', a.net, '--out', str(out / f'games-{w:02d}.jsonl')]
            if a.voids: cmd.append('--voids')
            if a.mixed: cmd.append('--mixed')
            if pc: cmd += ['--prod-cache', str(pc)]
            procs.append(subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
        for p in procs: p.wait(timeout=299)
        rounds_run += 1
        print(json.dumps({'round': r, 'wall': round(time.time() - t0, 1), 'deals_paired': paired_count()}), flush=True)
summ = subprocess.run([sys.executable, str(HERE / 'h2h.py'), '--net', a.net, '--seed', str(a.seed), '--deals', str(a.deals), '--workers', str(a.workers), '--tag', a.tag, '--summarize-only'], capture_output=True, text=True)
res = json.loads((out / 'summary.json').read_text())
games = [json.loads(l) for f in sorted(out.glob('games-*.jsonl')) for l in open(f) if l.strip() and l.rstrip().endswith('}')]
res['prod_cache'] = None if pc is None else dict(dir=str(pc), native_decisions=sum(len(g['native_us']) for g in games), native_cached=sum(g.get('native_cached', 0) for g in games))
res['wall_seconds'] = round(time.time() - t_all, 1); res['rounds_run'] = rounds_run; res['workers'] = a.workers
(out / 'summary.json').write_text(json.dumps(res, indent=1) + '\n'); print(json.dumps(res))
