import numpy as np, time, itertools, json, sys
from engine42 import *

t0 = time.time()
rng = np.random.default_rng(42)
MY = 2; BIDDER = 1; BID = 30
# ---- the fresh hand
deal0 = random_deals(rng, 1)[0]
my_hand = int(deal0[MY]); bidder_hand = int(deal0[BIDDER])
# trump = pip the bidder holds most (ties -> higher pip)
cnt = [sum(1 for t in range(28) if (bidder_hand >> t) & 1 and p in TILES[t]) for p in range(7)]
TRUMP = max(range(7), key=lambda p: (cnt[p], p))
rules = Rules(TRUMP)
my_tiles = [t for t in range(28) if (my_hand >> t) & 1]
print('my hand (seat 2, defending):', [NAME[t] for t in my_tiles], ' trump:', TRUMP, ' bid 30 by seat 1')

NSTATE = 30   # 0 = in my hand, 1 = unseen, 2 + seat*7 + trick = played by seat at trick
def enc_state_init(G):
    st = np.ones((G, 28), np.int64); st[:, my_tiles] = 0; return st

def dense_feats(g):
    d = np.zeros((g.R, 10), np.float32)
    d[np.arange(g.R), g.leader] = 1
    d[:, 4] = g.bid_pts / 42; d[:, 5] = g.def_pts / 42
    d[np.arange(g.R), 6 + g.npl] = 1
    return d

# ---------------------------------------------------------------- A. training data from uniform games
def gen_uniform(rng, G):
    hands = random_deals(rng, G, MY, my_hand)
    g = Game(rules, hands, BID, BIDDER)
    st = enc_state_init(G); tape = rng.random((G, 28))
    X_idx, X_den, gid = [], [], []
    ply = 0
    while not g.done.all():
        live = ~g.done
        X_idx.append((np.arange(28) * NSTATE + st)[live]); X_den.append(dense_feats(g)[live]); gid.append(np.nonzero(live)[0])
        seat = g.seat_to_play(); L = g.legal(seat); n = popcount(L)
        k = np.minimum((tape[:, ply] * n).astype(np.int64), np.maximum(n - 1, 0))
        t = np.where(live, kth_set_bit(L, k), -1)
        ok = t >= 0
        st[ok, t[ok]] = 2 + seat[ok] * 7 + g.trick[ok]
        g.play(t); ply += 1
    y = g.made()
    Xi = np.concatenate(X_idx); Xd = np.concatenate(X_den); gi = np.concatenate(gid)
    return Xi, Xd, y[gi].astype(np.float32), y

G = 14000
Xi, Xd, y, ygame = gen_uniform(rng, G)
print(f'training samples {len(y):,} from {G:,} uniform games; base rate P(made)={ygame.mean():.3f}  [{time.time()-t0:.0f}s]')

# ---------------------------------------------------------------- B. tiny net: embedding-sum (NNUE-style accumulator) -> 64 -> 32 -> 1
H1, H2 = 64, 32
def init(shape, s): return (rng.standard_normal(shape) * s).astype(np.float32)
P = {'E': init((28 * NSTATE, H1), 0.05), 'Wd': init((10, H1), 0.1), 'b1': np.zeros(H1, np.float32),
     'W2': init((H1, H2), 0.15), 'b2': np.zeros(H2, np.float32), 'w3': init((H2,), 0.2), 'b3': np.zeros(1, np.float32)}
M = {k: np.zeros_like(v) for k, v in P.items()}; V = {k: np.zeros_like(v) for k, v in P.items()}

def forward(idx, den):
    a1 = P['E'][idx].sum(1) + den @ P['Wd'] + P['b1']; h1 = np.maximum(a1, 0)
    a2 = h1 @ P['W2'] + P['b2']; h2 = np.maximum(a2, 0)
    z = h2 @ P['w3'] + P['b3']
    return z, (a1, h1, a2, h2)

def step(idx, den, yb, lr, it):
    z, (a1, h1, a2, h2) = forward(idx, den)
    p = 1 / (1 + np.exp(-z)); dz = (p - yb) / len(yb)
    g = {}
    g['w3'] = h2.T @ dz; g['b3'] = dz.sum(keepdims=True)
    dh2 = np.outer(dz, P['w3']) * (a2 > 0)
    g['W2'] = h1.T @ dh2; g['b2'] = dh2.sum(0)
    dh1 = (dh2 @ P['W2'].T) * (a1 > 0)
    g['Wd'] = den.T @ dh1; g['b1'] = dh1.sum(0)
    gE = np.zeros_like(P['E']); np.add.at(gE, idx.ravel(), np.repeat(dh1, 28, axis=0)); g['E'] = gE
    for k in P:
        M[k] = 0.9 * M[k] + 0.1 * g[k]; V[k] = 0.999 * V[k] + 0.001 * g[k] ** 2
        mh = M[k] / (1 - 0.9 ** it); vh = V[k] / (1 - 0.999 ** it)
        P[k] -= (lr * mh / (np.sqrt(vh) + 1e-8)).astype(np.float32)
    return -np.mean(yb * np.log(p + 1e-7) + (1 - yb) * np.log(1 - p + 1e-7))

N = len(y); perm = rng.permutation(N); nval = 20000
val, tr = perm[:nval], perm[nval:]
it = 0; bs = 512
for ep in range(4):
    rng.shuffle(tr); losses = []
    lr = 2e-3 if ep < 3 else 7e-4
    for b in range(0, len(tr), bs):
        j = tr[b:b+bs]; it += 1
        losses.append(step(Xi[j], Xd[j], y[j], lr, it))
    zv, _ = forward(Xi[val], Xd[val]); pv = 1 / (1 + np.exp(-zv))
    vl = -np.mean(y[val] * np.log(pv + 1e-7) + (1 - y[val]) * np.log(1 - pv + 1e-7))
    print(f'epoch {ep+1}: train loss {np.mean(losses):.4f}  val loss {vl:.4f}  val acc {((pv > .5) == (y[val] > .5)).mean():.3f}  [{time.time()-t0:.0f}s]')

def net_value(g, st):
    """P(made) for each row of a Game g with tile-state st (R,28)."""
    z, _ = forward(np.arange(28) * NSTATE + st, dense_feats(g)); return 1 / (1 + np.exp(-z))

# ---------------------------------------------------------------- C. test positions: seat 2 to play, >=2 legal, from uniform games
def sample_positions(rng, n_games):
    hands = random_deals(rng, n_games, MY, my_hand)
    g = Game(rules, hands, BID, BIDDER); tape = rng.random((n_games, 28))
    hist = [[] for _ in range(n_games)]; positions = []
    ply = 0
    while not g.done.all():
        seat = g.seat_to_play(); L = g.legal(seat); n = popcount(L)
        for i in np.nonzero((seat == MY) & ~g.done & (n >= 2))[0]:
            positions.append(dict(history=list(hist[i]), my_hand_now=int(g.hands[i, MY]), trick=int(g.trick[i]), deal=hands[i].copy()))
        k = np.minimum((tape[:, ply] * n).astype(np.int64), np.maximum(n - 1, 0))
        t = np.where(g.done, -1, kth_set_bit(L, k))
        for i in np.nonzero(t >= 0)[0]: hist[i].append((int(seat[i]), int(t[i])))
        g.play(t); ply += 1
    return positions

pos = sample_positions(np.random.default_rng(7), 400)
by_trick = {}
for p in pos: by_trick.setdefault(p['trick'], []).append(p)
quota = {0: 10, 1: 25, 2: 40, 3: 45, 4: 45, 5: 45, 6: 30}
test = []
for tr_, q in quota.items():
    lst = by_trick.get(tr_, []); rng.shuffle(lst); test += lst[:q]
print('test positions:', len(test), {k: len([p for p in test if p['trick'] == k]) for k in sorted(quota)})

def net_choice(p):
    """One-step greedy: play each legal tile in a 1-row replay, evaluate the resulting info set."""
    g1 = replay(rules, p['deal'][None, :], BID, BIDDER, p['history'])
    L = g1.legal(np.array([MY]))[0]
    legal = [t for t in range(28) if (L >> t) & 1]
    st = enc_state_init(1)
    for s, t in p['history']:
        pass
    # rebuild tile-state from history
    trick_of = []; g2 = Game(rules, p['deal'][None, :], BID, BIDDER)
    for s, t in p['history']:
        st[0, t] = 2 + s * 7 + g2.trick[0]; g2.play(np.array([t]))
    vals = {}
    for t in legal:
        g3 = replay(rules, p['deal'][None, :], BID, BIDDER, p['history'])
        st3 = st.copy(); st3[0, t] = 2 + MY * 7 + g3.trick[0]
        g3.play(np.array([t]))
        vals[t] = float(net_value(g3, st3)[0])
    choice = min(legal, key=lambda t: (vals[t], t))   # defending: minimise P(made)
    return vals, choice, legal

import pickle; np.savez('net.npz', **P); pickle.dump(test, open('test.pkl','wb'))
W_REF = 96
rows = []
for i, p in enumerate(test):
    counts, wchoice, legal, W = walt_decide(rng, rules, BID, BIDDER, MY, p['history'], p['my_hand_now'], n_worlds=W_REF)
    vals, nchoice, _ = net_choice(p)
    cs = sorted(counts.values()); margin = (cs[1] - cs[0]) / W if len(cs) > 1 else 0
    regret = (counts[nchoice] - counts[wchoice]) / W
    rows.append(dict(trick=p['trick'], n_legal=len(legal), W=W, agree=int(nchoice == wchoice), regret=regret, margin=margin,
                     walt={NAME[t]: round(c / W, 2) for t, c in counts.items()}, net={NAME[t]: round(v, 3) for t, v in vals.items()},
                     wchoice=NAME[wchoice], nchoice=NAME[nchoice]))
    if i % 40 == 0: print(f'  {i}/{len(test)} positions  [{time.time()-t0:.0f}s]'); sys.stdout.flush()
json.dump(rows, open('compare.json', 'w'))
R = rows
print('\n=== Net (one-step greedy on rollout-fit V0) vs Walt (best response to uniform, %d worlds, CRN) ===' % W_REF)
print('overall agreement: %.0f%%  (%d positions)' % (100 * np.mean([r['agree'] for r in R]), len(R)))
for k in sorted(set(r['trick'] for r in R)):
    rr = [r for r in R if r['trick'] == k]
    print('  trick %d: agree %3.0f%%  n=%d  mean regret on disagreements %.3f' % (k + 1, 100 * np.mean([r['agree'] for r in rr]), len(rr),
          np.mean([r['regret'] for r in rr if not r['agree']]) if any(not r['agree'] for r in rr) else 0))
dis = [r for r in R if not r['agree']]
print('disagreements: %d; regret distribution (worlds lost, as a fraction):' % len(dis))
for lo, hi in [(0, 0.0001), (0.0001, 0.05), (0.05, 0.1), (0.1, 0.2), (0.2, 1.01)]:
    print('   %.2f–%.2f: %d' % (lo, hi, sum(1 for r in dis if lo <= r['regret'] < hi)))
close = [r for r in R if r['margin'] <= 0.03]
print('positions where Walt itself is within 3%% between its top two: %d of %d; agreement there %.0f%%, elsewhere %.0f%%' % (
    len(close), len(R), 100 * np.mean([r['agree'] for r in close]) if close else 0,
    100 * np.mean([r['agree'] for r in R if r['margin'] > 0.03])))
print('mean regret over ALL positions: %.4f   worst: %.3f' % (np.mean([r['regret'] for r in R]), max(r['regret'] for r in R)))
print('total time %.0fs' % (time.time() - t0))
