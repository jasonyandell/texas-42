#!/usr/bin/env python3
"""LAD7 bidding table: a monotone map from the net's standalone root score (max over opening leads of sigmoid(score), the
`bideval` "standalone" number) to the realized make rate under the population that played the bideval games (net self-play
unless the files came from --prod). Isotonic regression (pool-adjacent-violators) on all rows, then 20 quantile knots.
usage: bidcal.py OUT.json BID=FILE.jsonl[,FILE...] [BID=... ...]   e.g. bidcal.py models/x-bidcal.json 30=a.jsonl,b.jsonl 35=...
Held-out check printed per bid: fit on even deals, score odd deals (Brier, mean p vs base rate)."""
import json, sys, numpy as np
def load(files):
    S, Y, D = [], [], []
    for f in files:
        for l in open(f):
            v = json.loads(l)
            for r in v['rows']: S.append(r['standalone']); Y.append(float(r['real_made'])); D.append(v['deal'])
    return np.array(S), np.array(Y), np.array(D)
def pav(s, y):
    o = np.argsort(s); s, y = s[o], y[o]; vals = list(y.astype(float)); w = [1.0] * len(vals); lo = list(s)
    i = 0; blocks = []  # (sum, n, lo)
    for k in range(len(vals)):
        blocks.append([vals[k], 1.0, s[k]])
        while len(blocks) > 1 and blocks[-2][0] / blocks[-2][1] > blocks[-1][0] / blocks[-1][1]:
            b = blocks.pop(); blocks[-1][0] += b[0]; blocks[-1][1] += b[1]
    return [(b[2], b[0] / b[1]) for b in blocks]
def apply(table, s):
    lo = np.array([t[0] for t in table]); p = np.array([t[1] for t in table]); return p[np.clip(np.searchsorted(lo, s, side='right') - 1, 0, len(p) - 1)]
out = sys.argv[1]; res = {}
for spec in sys.argv[2:]:
    bid, files = spec.split('='); s, y, d = load(files.split(','))
    ev = d % 2 == 0; t = pav(s[ev], y[ev]); ph = apply(t, s[~ev])
    print(json.dumps(dict(bid=int(bid), rows=len(s), base=round(float(y.mean()), 4), heldout=dict(brier=round(float(((ph - y[~ev]) ** 2).mean()), 4), mean_p=round(float(ph.mean()), 4), base=round(float(y[~ev].mean()), 4), raw_brier=round(float(((s[~ev] - y[~ev]) ** 2).mean()), 4)))))
    full = pav(s, y); knots = np.quantile(s, np.linspace(0, 1, 21)[:-1]); res[bid] = [[round(float(k), 4), round(float(apply(full, np.array([k]))[0]), 4)] for k in knots]
json.dump(dict(note='standalone root score -> realized make rate (net self-play, bideval seeds 9000-9499); lookup: largest knot <= score', table=res), open(out, 'w'), indent=1)
print('wrote', out); print({b: [(k, p) for k, p in v[::4]] for b, v in res.items()})
