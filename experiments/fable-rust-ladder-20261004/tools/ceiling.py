#!/usr/bin/env python3
"""Label-ambiguity ceiling of a root-vector label file (reads the raw REC only; no net).
Usage: ceiling.py <val.bin> [dump.jsonl from `ladder agree --dump` to stratify a net's agreement/regret by gap]"""
import json, sys
import numpy as np
sys.argv_saved = sys.argv; sys.argv = [sys.argv[0]]
sys.path.insert(0, '/Users/jason/Documents/Codex/2026-10-04/task-11/research-worktree/.claude/worktrees/agent-a67d09aabae674c6d/experiments/fable-rust-ladder-20261004')
REC = np.dtype([('decl', 'u1'), ('seat', 'u1'), ('maximize', 'u1'), ('leader', 'u1'), ('npl', 'u1'), ('plays', 'u1', 4), ('bt1', 'u1'), ('bt0', 'u1'), ('hand', '<u4'), ('played', '<u4'), ('legal', '<u4'), ('N', '<u2'), ('counts', '<u2', 28), ('choice', 'u1'), ('prod', 'u1'), ('hasvoids', 'u1'), ('voids', '<u4', 4)])
path = sys.argv_saved[1]; r = np.fromfile(path, dtype=REC); n = len(r); N = int(r['N'][0]); assert (r['N'] == N).all()
legal = ((r['legal'][:, None] >> np.arange(28)) & 1).astype(bool); k = r['counts'].astype(np.int64)
mx = r['maximize'] == 1
v = np.where(mx[:, None], k, N - k)  # mover's make-count (minimizers want declarer to fail: value = N - k)
v = np.where(legal, v, -10**6)
best = v.max(1); lowest = np.where(v == best[:, None], np.arange(28), 99).min(1)
print('rows', n, 'N', N, 'choice==lowest argmax', float((lowest == r['choice']).mean()))
nleg = legal.sum(1); print('rows with 1 legal tile', float((nleg == 1).mean()), 'mean legal', float(nleg.mean()))
nbest = (v == best[:, None]).sum(1); srt = np.sort(v, 1)[:, ::-1]; gap = srt[:, 0] - srt[:, 1]; multi = nleg > 1
print('rows tied (>=2 argmax tiles):', float((nbest > 1).mean()), ' among multi-legal:', float((nbest[multi] > 1).mean()))
for g in (0, 1, 2, 3, 5, 10):
    print(f'gap<={g} worlds (multi-legal rows): {float((gap[multi] <= g).mean()):.4f}   (all rows) {float(((gap <= g) & multi).mean()):.4f}')
print('best value 0 or N (decided, all tiles equal?):', float(((srt[:, 0] == srt[:, 1]) & multi).mean()))
# oracle that knows the exact label vector but must pick the lowest tile among ties -> 1.0; a predictor that picks ANY argmax -> 1.0 zero-regret.
# Noise ceiling: a predictor that knows the true probabilities p (take p_hat as truth), scored against a fresh independent N-world label.
rng = np.random.default_rng(1); ph = np.where(legal, v, 0) / N
for corr, name in ((0.0, 'independent binomial per tile (pessimistic: ignores common-world correlation)'),):
    ag = []; rg = []
    for rep in range(5):
        kn = rng.binomial(N, np.clip(ph, 0, 1)); kn = np.where(legal, kn, -10**6)
        bn = kn.max(1); ln = np.where(kn == bn[:, None], np.arange(28), 99).min(1)
        # truth-knowing predictor: lowest argmax of exact p_hat (=label itself), compared with fresh-noise label's lowest argmax
        ag.append(float((ln == lowest).mean()))
        pick = lowest; rg.append(float(((bn - kn[np.arange(n), pick]) / N).mean()))
    print(f'oracle-on-p_hat vs fresh label, {name}: agree {np.mean(ag):.4f}  regret-vs-fresh {np.mean(rg):.4f}')
if len(sys.argv_saved) > 2:
    sc = {}; ch = {}
    for l in open(sys.argv_saved[2]):
        d = json.loads(l); sc[d['i']] = d['scores']; ch[d['i']] = d['choice']
    idx = np.array(sorted(ch)); pick = np.array([ch[i] for i in idx]); m = len(idx)
    vp = v[idx, pick]; reg = (best[idx] - vp) / N; agr = pick == r['choice'][idx]
    print(f'dump rows {m}: agree {agr.mean():.4f} zero-regret {float((reg <= 0).mean()):.4f} regret {reg.mean():.5f}')
    gg = gap[idx]; mm = multi[idx]
    for lo, hi in ((0, 0), (1, 1), (2, 3), (4, 8), (9, 10**6)):
        s = mm & (gg >= lo) & (gg <= hi); print(f'  gap {lo}-{hi}: share {float(s.mean()):.3f} agree {float(agr[s].mean()):.3f} regret {float(reg[s].mean()):.5f}')
