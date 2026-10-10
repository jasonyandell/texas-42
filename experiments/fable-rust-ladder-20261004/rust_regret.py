#!/usr/bin/env python3
"""STU: Rust-side agreement AND regret of a deployed net on a label file, from `ladder agree --dump` output.
usage: rust_regret.py LABELS DUMP.jsonl  -> {"rows", "agree", "zero_regret", "regret", "bce", "rmse"} (regret in make-probability units;
bce/rmse = masked soft-target BCE and RMSE of sigmoid(score) against counts/N over the legal tiles, the trainer's definitions, from the dump's scores)."""
import json, sys, numpy as np
sys.argv, args = sys.argv[:1], sys.argv[1:]
import train as T
r = np.fromfile(args[0], dtype=T.REC); rows_ = [json.loads(l) for l in open(args[1])]; ch = np.array([d['choice'] for d in rows_]); r = r[:len(ch)]
k = r['counts'].astype(float); N = r['N'].astype(float)[:, None]; legal = ((r['legal'][:, None] >> np.arange(28)) & 1).astype(bool)
v = np.where(legal, np.where((r['maximize'][:, None] & 1) == 1, k, -k) / N, -10.0); best = v.max(1); reg = best - v[np.arange(len(ch)), ch]
z = np.zeros((len(ch), 28)); y = k / N
for i, d in enumerate(rows_): z[i, np.flatnonzero(legal[i])] = d['scores']
per = np.logaddexp(0, z) - y * z; bce = float(((per * legal).sum(1) / legal.sum(1)).mean()); pr = 1 / (1 + np.exp(-z)); rmse = float(np.sqrt((((pr - y) ** 2) * legal).sum() / legal.sum()))
print(json.dumps(dict(rows=len(ch), agree=float(np.mean(ch == r['choice'])), zero_regret=float(np.mean(reg <= 1e-9)), regret=float(reg.mean()), bce=bce, rmse=rmse)))
