import numpy as np, time
from engine42 import *
from exact_tape import public, clone_rows, worlds, BID
import itertools
def choose(side_bid, vals, legal):
    return max(legal, key=lambda t: (vals[t], -t)) if side_bid else min(legal, key=lambda t: (vals[t], t))
def all_deals(rules, seat, hand_now, history, cap=4000):
    played_by = [[t for s, t in history if s == q] for q in range(4)]
    seen = set(t for _, t in history) | set(t for t in range(28) if (hand_now >> t) & 1)
    unseen = [t for t in range(28) if t not in seen]; need = {s: 7 - len(played_by[s]) for s in range(4) if s != seat}
    a, b, c = [s for s in range(4) if s != seat]; out = []
    for ha in itertools.combinations(unseen, need[a]):
        r1 = [t for t in unseen if t not in ha]
        for hb in itertools.combinations(r1, need[b]):
            hc = tuple(t for t in r1 if t not in hb); d = np.zeros(4, np.int64); d[seat] = hand_now
            d[a] = BIT[list(ha)].sum() if ha else 0; d[b] = BIT[list(hb)].sum() if hb else 0; d[c] = BIT[list(hc)].sum() if hc else 0; out.append(d)
            if len(out) > cap: return None
    rem = np.array(out)
    if history:
        full = rem.copy()
        for s in range(4):
            if played_by[s]: full[:, s] |= int(BIT[played_by[s]].sum())
        g = Game(rules, full, BID, history[0][0]); ok = np.ones(len(rem), bool)
        for s, t in history:
            L = g.legal(np.full(len(rem), s)); ok &= ((L >> t) & 1) == 1; g.play(np.full(len(rem), t))
        rem = rem[ok]
    return rem

def expectimax(rules, bidder, seat, hand_now, history, deal_full, rem, tape=None, fused=False):
    """rem: (D,4) remaining hands (enumerated deals or sampled worlds). Others: branch on every legal tile (tape None) or follow the tape.
    Me: one decision per information set (public history), unless fused=True (one per deal)."""
    D = len(rem); g1 = public(rules, bidder, deal_full, history); L0 = int(g1.legal(np.array([seat]))[0]); legal = [t for t in range(28) if (L0 >> t) & 1]
    side_bid = (seat % 2) == (bidder % 2)
    g = Game(rules, rem, BID, bidder, scores=(np.repeat(g1.bid_pts, D), np.repeat(g1.def_pts, D)), leader=np.repeat(g1.leader, D), table=np.repeat(g1.table, D, axis=0), trick=np.repeat(g1.trick, D))
    key = np.zeros(D, np.int64); widx = np.arange(D); levels = []; ply = 0
    while not g.done.all():
        s = g.seat_to_play(); L = g.legal(s); live = ~g.done; mine = (s == seat) & live
        if tape is None or mine.any():
            branch = live if tape is None else mine
            n = np.where(branch, popcount(L), 1); idx = np.repeat(np.arange(g.R), n); j = np.arange(len(idx)) - np.repeat(np.cumsum(n) - n, n)
            pk = key[idx]; g = clone_rows(g, idx); L = L[idx]; s = s[idx]; live = live[idx]; mine = mine[idx]; widx = widx[idx]
            t = np.where(branch[idx], kth_set_bit(L, j), -1); w = np.where(branch[idx] & ~mine, 1.0 / np.repeat(n, n), 1.0)
        else:
            idx = np.arange(g.R); pk = key.copy(); t = np.full(g.R, -1, np.int64); w = np.ones(g.R)
        if tape is not None:
            oth = live & ~mine; n_o = popcount(L); k = np.minimum((tape[widx, ply] * n_o).astype(np.int64), np.maximum(n_o - 1, 0)); t = np.where(oth, kth_set_bit(L, k), t)
        t = np.where(live, t, -1)
        levels.append((idx, s.copy(), w, mine.copy(), pk, t.copy()))
        key = pk * 29 + (t + 1); g.play(t); ply += 1
    val = g.made().astype(float)
    for li, (idx, s, w, mine, pk, t) in enumerate(reversed(levels)):
        R = idx.max() + 1; agg = np.zeros(R)
        if mine.any():
            if fused or li == len(levels) - 1:     # the root level is returned per candidate, not folded
                if li == len(levels) - 1: break
                tmp = np.full(R, -1.0 if side_bid else 2.0); (np.maximum.at if side_bid else np.minimum.at)(tmp, idx[mine], val[mine]); u = np.unique(idx[mine]); agg[u] = tmp[u]
            else:
                # one decision per information set: group parent rows by public history, sum over the group per tile
                gkey, gid = np.unique(pk[mine], return_inverse=True); G = len(gkey)
                cid = gid * 28 + t[mine]; sums = np.full(G * 28, -1e18 if side_bid else 1e18); seen = np.zeros(G * 28, bool)
                np.add.at(seen, cid, True); tot = np.zeros(G * 28); np.add.at(tot, cid, val[mine]); sums[seen] = tot[seen]
                best = (sums.reshape(G, 28).argmax(1) if side_bid else sums.reshape(G, 28).argmin(1))
                chosen = t[mine] == best[gid]
                agg[idx[mine][chosen]] = val[mine][chosen]
        oth = ~mine; np.add.at(agg, idx[oth], val[oth] * w[oth]); val = agg
    idx, s, w, mine, pk, t = levels[0]
    vals = {tt: float(val[t == tt].sum() / D) for tt in legal}
    return vals, choose(side_bid, vals, legal), legal

def tail_nofusion(rules, bidder, seat, hand_now, history, deal_full, rng=None):
    rem = all_deals(rules, seat, hand_now, history)
    if rem is None: return None
    v, c, _ = expectimax(rules, bidder, seat, hand_now, history, deal_full, rem); return v, c

def tape_nofusion(rules, bidder, seat, hand_now, history, deal_full, rng, W):
    rem, full = worlds(rng, rules, seat, hand_now, history, W); tape = rng.random((len(rem), 28))
    v, c, _ = expectimax(rules, bidder, seat, hand_now, history, full[0] if deal_full is None else deal_full, rem, tape=tape); return v, c
