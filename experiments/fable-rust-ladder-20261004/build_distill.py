#!/usr/bin/env python3
"""STU: average several teachers' make-probabilities into one distillation label file (same 100-byte record format).
usage: build_distill.py OUT.bin MEMBER [MEMBER ...]
  MEMBER = a relabeled .bin (train_stu.py --relabel output, counts/N = teacher probability), or
           a comma-separated list of `ladder agree --dump` .jsonl files (logits of legal tiles, in record order;
           e.g. a token net run in chunks), prefixed `dump:`.
All members must cover the same records in the same order (the first .bin member supplies the records).
Output: N = 10000, counts = round(10000 * mean probability) on legal tiles, choice = argmax (maximizer) / argmin (minimizer),
lowest tile on ties."""
import json, sys, numpy as np
argv = sys.argv; sys.argv = argv[:1]
import train as T
out, members = argv[1], argv[2:]
base = None; acc = None; k = 0
bins = [m for m in members if not m.startswith('dump:')]; base = np.fromfile(bins[0], dtype=T.REC); n = len(base)
legal = ((base['legal'][:, None] >> np.arange(28)) & 1).astype(bool); acc = np.zeros((n, 28), np.float64)
for m in members:
    if m.startswith('dump:'):
        off = 0
        for f in m[5:].split(','):
            for l in open(f):
                sc = np.array(json.loads(l)['scores'], np.float64); acc[off, legal[off]] += 1 / (1 + np.exp(-sc)); off += 1
        assert off == n, (m, off, n)
    else:
        r = np.fromfile(m, dtype=T.REC); assert len(r) == n and (r['legal'] == base['legal']).all(), m
        acc += r['counts'] / r['N'][:, None].astype(np.float64)
    k += 1
acc /= k; base['N'] = 10000; base['counts'] = np.where(legal, np.rint(acc * 10000), 0).astype(np.uint16)
v = np.where(legal, np.where((base['maximize'][:, None] & 1) == 1, acc, -acc), -1e9); base['choice'] = np.argmax(v, 1).astype(np.uint8)
base.tofile(out); print(json.dumps(dict(out=out, rows=n, members=k)))
