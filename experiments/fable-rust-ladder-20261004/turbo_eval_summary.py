#!/usr/bin/env python3
"""Summarize turbo-eval rows: per arm, per depth bucket: mean us, RMSE of the pmake vector vs the
4096-world reference, selector agreement with the reference's selector, and reference regret of the
arm's choice (maximize for odd seats, minimize for even; lowest tile on ties)."""
import json, sys
from pathlib import Path
import numpy as np
HERE = Path(__file__).resolve().parent; D = HERE / (sys.argv[1] if len(sys.argv) > 1 else 'results/turbo-eval'); TRUTH = sys.argv[2] if len(sys.argv) > 2 else None
rows = [json.loads(l) for f in sorted(D.glob('*.jsonl')) for l in open(f) if l.strip()]
if TRUTH:  # use an exact arm as the truth where it exists (drop rows without it)
    rows = [r for r in rows if TRUTH in r['arms'] and 'v' in r['arms'][TRUTH]]
    for r in rows: r['ref'] = r['arms'][TRUTH]['v']
if rows and 'support' in rows[0]:
    import collections; sup = collections.defaultdict(list)
    for r in rows: sup[r['depth']].append(r['support'])
    print('support size by depth (median, max):', {d: (int(np.median(v)), int(max(v))) for d, v in sorted(sup.items())})
def pick(v, seat):
    v = np.asarray(v); return int(np.argmax(v)) if seat % 2 == 1 else int(np.argmin(v))  # argmax/argmin take the first (lowest tile) on ties
arms = sorted({a for r in rows for a in r['arms']}, key=lambda a: (a.startswith('walt'), a))
out = {}
for arm in arms:
    per = {}
    for r in rows:
        if arm not in r['arms']: continue
        a = r['arms'][arm]; b = r['depth'] // 4; d = per.setdefault(b, dict(n=0, err=0, us=[], se=[], agree=0, regret=[]))
        d['n'] += 1; d['us'].append(a['us'])
        if 'error' in a: d['err'] += 1; continue
        ref = np.asarray(r['ref']); v = np.asarray(a['v']); d['se'].extend(((v - ref) ** 2).tolist())
        rb = pick(ref, r['seat']); ab = pick(v, r['seat']); d['agree'] += int(rb == ab); d['regret'].append(abs(ref[rb] - ref[ab]))
    out[arm] = {b: dict(n=d['n'], errors=d['err'], us_mean=float(np.mean(d['us'])), us_median=float(np.median(d['us'])), rmse=float(np.sqrt(np.mean(d['se']))) if d['se'] else None, agree=d['agree'] / max(1, d['n'] - d['err']), regret_mean=float(np.mean(d['regret'])) if d['regret'] else None) for b, d in sorted(per.items())}
(D / 'summary.json').write_text(json.dumps(out, indent=1))
buckets = sorted({b for a in out.values() for b in a})
print('rows', len(rows), 'buckets', buckets)
for metric in ('us_mean', 'rmse', 'regret_mean', 'agree', 'errors'):
    print(f'\n== {metric} by trick bucket ==')
    print('arm'.ljust(10) + ''.join(f'{"T"+str(b):>9}' for b in buckets) + f'{"all":>9}')
    for arm in arms:
        vals = [out[arm].get(b, {}).get(metric) for b in buckets]
        allv = None
        if metric == 'us_mean': allv = np.mean([x['us'] for x in [dict(us=out[arm][b]['us_mean']) for b in out[arm]]])
        if metric == 'rmse': allv = np.sqrt(np.mean([out[arm][b]['rmse'] ** 2 for b in out[arm] if out[arm][b]['rmse'] is not None]))
        if metric in ('regret_mean', 'agree'): allv = np.mean([out[arm][b][metric] for b in out[arm] if out[arm][b][metric] is not None])
        if metric == 'errors': allv = sum(out[arm][b]['errors'] for b in out[arm])
        fmt = lambda x: '   -' if x is None else (f'{x:9.0f}' if metric in ('us_mean', 'errors') else f'{x:9.3f}')
        print(arm.ljust(10) + ''.join(fmt(v) for v in vals) + fmt(allv))
