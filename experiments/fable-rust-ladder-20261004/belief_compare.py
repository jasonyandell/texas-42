#!/usr/bin/env python3
"""Paired comparison of two h2h arms on the same deals (exploratory): per deal, each arm's paired score vs production
(+1 / 0 / -1 over both seatings); difference B - A with a bootstrap 95% CI; the difference split by the trick of the
first decision where the two arms' games diverge (both seatings; production is deterministic given the state, so
games coincide up to the first differing hybrid play); belief ESS/N by trick summed over the worker logs of arm B.
Usage: belief_compare.py results/h2h-<A> results/h2h-<B>"""
import json, sys
from pathlib import Path
import numpy as np


def games(d):
    g = {}
    for f in sorted(Path(d).glob('games-*.jsonl')):
        for l in open(f):
            if l.strip().endswith('}'): v = json.loads(l); g[(v['deal'], v['side'])] = v
    return g


def score(g):
    out = {}
    for (d, s), v in g.items():
        if (d, 1 - s) in g and s == 0: out[d] = int(v['hybrid_won']) + int(g[(d, 1)]['hybrid_won']) - 1
    return out


def main():
    A, B = sys.argv[1], sys.argv[2]; ga, gb = games(A), games(B); sa, sb = score(ga), score(gb)
    deals = sorted(set(sa) & set(sb)); da = np.array([sa[d] for d in deals]); db = np.array([sb[d] for d in deals]); diff = db - da
    rng = np.random.default_rng(7); boot = diff[rng.integers(len(diff), size=(5000, len(diff)))].mean(1)
    res = dict(A=A, B=B, deals=len(deals), mean_A=float(da.mean()), mean_B=float(db.mean()), diff=float(diff.mean()), ci95=[float(np.quantile(boot, .025)), float(np.quantile(boot, .975))])
    # first divergence trick per deal (minimum over the two seatings; None = identical games in both seatings)
    by = {}
    for d in deals:
        t = None
        for s in (0, 1):
            pa, pb = ga[(d, s)]['plays'], gb[(d, s)]['plays']
            k = next((i for i in range(0, min(len(pa), len(pb)), 2) if pa[i:i + 2] != pb[i:i + 2]), None)
            if k is not None: tt = (k // 2) // 4; t = tt if t is None else min(t, tt)
        by.setdefault('same' if t is None else t, []).append(sb[d] - sa[d])
    res['by_divergence_trick'] = [dict(trick=k, deals=len(v), sum_diff=int(np.sum(v)), mean_diff=float(np.mean(v))) for k, v in sorted(by.items(), key=lambda x: (isinstance(x[0], str), x[0]))]
    ess = {}
    for f in Path(B).glob('worker-*/stdout.log'):
        for l in open(f):
            try: v = json.loads(l)
            except Exception: continue
            for e in v.get('belief_ess', []):
                x = ess.setdefault(e['trick'], [0.0, 0.0, 1.0]); x[0] += e['decisions']; x[1] += e['mean_ess_frac'] * e['decisions']; x[2] = min(x[2], e['min_ess_frac'])
    if ess: res['ess'] = [dict(trick=t, decisions=int(x[0]), mean_ess_frac=x[1] / x[0], min_ess_frac=x[2]) for t, x in sorted(ess.items())]
    print(json.dumps(res, indent=1))


if __name__ == '__main__': main()
