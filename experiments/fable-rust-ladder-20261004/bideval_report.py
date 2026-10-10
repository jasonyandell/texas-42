#!/usr/bin/env python3
"""Report for `ladder bideval` output (exploratory tier). Compares three P(make 30) estimators per (deal, seat, trump):
standalone = net root estimate (no search); rollout = K-world net-self-play make rate; against a realized outcome
(net self-play by default; --prod FILE adds production-played outcomes for the deals it covers)."""
import json, sys, glob, argparse
import numpy as np
ap = argparse.ArgumentParser(); ap.add_argument('files', nargs='+'); ap.add_argument('--prod', default=''); ap.add_argument('--bid', type=int, default=30, help='make threshold for the net-side rows (the probe was run with the same --bid)')
a = ap.parse_args()
deals = {}
for f in a.files:
    for fn in glob.glob(f):
        for l in open(fn): v = json.loads(l); deals[v['deal']] = v
prod = {}
if a.prod:
    for fn in glob.glob(a.prod):
        for l in open(fn): v = json.loads(l); prod[v['deal']] = {(r['seat'], r['decl']): (r['real_pts'], r['real_made']) for r in v['rows']}
print(f"deals {len(deals)}  prod-played deals {len(prod)}")
def auc(p, y):
    o = np.argsort(p); r = np.empty(len(p)); r[o] = np.arange(1, len(p) + 1)
    n1 = y.sum(); n0 = len(y) - n1; return (r[y == 1].sum() - n1 * (n1 + 1) / 2) / (n1 * n0)
def metrics(p, y, name):
    pc = np.clip(p, 1e-4, 1 - 1e-4); ll = -(y * np.log(pc) + (1 - y) * np.log(1 - pc)).mean()
    print(f"  {name:12s} brier {((p - y) ** 2).mean():.4f}  logloss {ll:.4f}  auc {auc(p, y):.4f}  mean p {p.mean():.3f} (base rate {y.mean():.3f})")
def calib(p, y, name):
    print(f"  {name} calibration (bin: mean p / observed / n):", ' '.join(f"{p[m].mean():.2f}/{y[m].mean():.2f}/{m.sum()}" for lo in np.arange(0, 1, .1) for m in [(p >= lo) & (p < lo + .1)] if m.sum() > 0))
for label, realized in [('net self-play', None), ('production play', prod)]:
    if realized is not None and not realized: continue
    ds = [d for d in sorted(deals) if realized is None or d in realized]
    S, R, Y, PTS, keys, SM, H = [], [], [], [], [], [], []
    for d in ds:
        for r in deals[d]['rows']:
            k = (r['seat'], r['decl']); K = sum(r['hist'])
            y, pts = (r['real_made'], r['real_pts']) if realized is None else (realized[d][k][1], realized[d][k][0])
            S.append(r['standalone']); R.append(sum(r['hist'][a.bid:]) / K); Y.append(float(y)); PTS.append(pts); keys.append((d,) + k); SM.append(float(np.mean(r['leads']))); H.append(r['hist'])
    S, R, Y, SM, H, PTS = np.array(S), np.array(R), np.array(Y), np.array(SM), np.array(H), np.array(PTS)
    print(f"\n== realized outcome: {label} ({len(ds)} deals, {len(Y)} contracts)")
    metrics(S, Y, 'standalone'); metrics(SM, Y, 'stand-mean'); metrics(R, Y, 'rollout'); calib(S, Y, 'standalone'); calib(R, Y, 'rollout')
    K = H.sum(1)
    for b in (35, 42):
        Pb = H[:, b:].sum(1) / K; Yb = (PTS >= b).astype(float); print(f"  make-{b}: base rate {Yb.mean():.3f}"); metrics(Pb, Yb, f'rollout>={b}'); metrics(S, Yb, f'standalone')
    # choice quality: per (deal, seat) pick the trump; per deal pick the contract
    n = len(ds); S4 = S.reshape(n, 4, 7); R4 = R.reshape(n, 4, 7); Y4 = Y.reshape(n, 4, 7)
    rb = np.array([deals[d]['rule_bidder'] for d in ds]); rd = np.array([deals[d]['rule_decl'] for d in ds])
    ts, tr = S4.argmax(2), R4.argmax(2); ii = np.arange(n)[:, None]; jj = np.arange(4)[None, :]
    print(f"  trump choice per (deal, seat): standalone==rollout {np.mean(ts == tr):.3f};  realized make rate of chosen trump: standalone {Y4[ii, jj, ts].mean():.3f}  rollout {Y4[ii, jj, tr].mean():.3f}  oracle {Y4.max(2).mean():.3f}  random {Y4.mean():.3f}")
    print(f"  rule bidder's seat only: rule trump {Y4[np.arange(n), rb, rd].mean():.3f}  standalone {Y4[np.arange(n), rb, ts[np.arange(n), rb]].mean():.3f}  rollout {Y4[np.arange(n), rb, tr[np.arange(n), rb]].mean():.3f}  oracle {Y4[np.arange(n), rb].max(1).mean():.3f}")
    cs = S4.reshape(n, 28).argmax(1); cr = R4.reshape(n, 28).argmax(1); Yf = Y4.reshape(n, 28)
    print(f"  contract per deal (28 options): standalone {Yf[np.arange(n), cs].mean():.3f}  rollout {Yf[np.arange(n), cr].mean():.3f}  rule {Y4[np.arange(n), rb, rd].mean():.3f}  oracle {Yf.max(1).mean():.3f}; standalone picks rule's seat {np.mean(cs // 7 == rb):.3f}, rollout {np.mean(cr // 7 == rb):.3f}")
    for thr in (.5, .6, .7):
        for nm, P in (('standalone', S), ('rollout', R)):
            m = P >= thr; print(f"  bid-30 if {nm} >= {thr}: bids {m.mean():.3f} of contracts, made {Y[m].mean() if m.any() else float('nan'):.3f}")
    # standalone rank-recalibrated (isotonic by deciles of S, fit on even deals, eval on odd) to separate ranking from calibration
    ev = (np.arange(len(S)) // 28) % 2 == 0; qs = np.quantile(S[ev], np.linspace(0, 1, 21)); b = np.clip(np.searchsorted(qs, S, side='right') - 1, 0, 19)
    tab = np.array([Y[ev & (b == i)].mean() if (ev & (b == i)).any() else 0 for i in range(20)]); Sr = tab[b]
    metrics(Sr[~ev], Y[~ev], 'standalone recalibrated by ventile (held-out odd deals)'); metrics(R[~ev], Y[~ev], 'rollout (same held-out deals)')
