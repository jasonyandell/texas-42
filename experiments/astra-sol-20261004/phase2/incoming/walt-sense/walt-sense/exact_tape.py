"""L1 with an exact 'me': best response per sampled world on a fixed tape, enumerating only realized own-play sequences."""
import numpy as np, itertools, time, sys, os, pickle
from engine42 import *
BID = 30

def pick_trump(hand):
    c = [sum(1 for t in range(28) if (hand >> t) & 1 and p in TILES[t]) for p in range(7)]; return max(range(7), key=lambda p: (c[p], p))

def worlds(rng, rules, seat, hand_now, history, W):
    if not history:
        rem = random_deals(rng, W, seat, hand_now); return rem, rem
    return consistent_worlds(rng, rules, seat, hand_now, history, W)

def public(rules, bidder, deal_full, history):
    g1 = Game(rules, deal_full[None, :], BID, bidder)
    for s, t in history: g1.play(np.array([t]))
    return g1

def clone_rows(g, idx):
    """new Game whose rows are g's rows at idx (with repetition)."""
    h = Game(g.rules, g.hands[idx], g.bid, g.bidder, scores=(g.bid_pts[idx], g.def_pts[idx]), leader=g.leader[idx], table=g.table[idx], trick=g.trick[idx])
    h.done = g.done[idx].copy(); return h

def jth_set_bit_rows(mask, j):
    return kth_set_bit(mask, j)

def exact_tape(rules, bidder, seat, hand_now, history, deal_full, rng, W, return_cost=False):
    """vals: tile -> mean over worlds of (best response value in that world); choice."""
    rem, full = worlds(rng, rules, seat, hand_now, history, W); W = len(rem)
    g1 = public(rules, bidder, full[0], history)
    L0 = g1.legal(np.array([seat]))[0]; legal = [t for t in range(28) if (L0 >> t) & 1]
    side_bid = (seat % 2) == (bidder % 2)
    tape = rng.random((W, 28))
    g = Game(rules, rem, BID, bidder, scores=(np.repeat(g1.bid_pts, W), np.repeat(g1.def_pts, W)), leader=np.repeat(g1.leader, W), table=np.repeat(g1.table, W, axis=0), trick=np.repeat(g1.trick, W))
    widx = np.arange(W); root = np.full(W, -1, np.int64); ply = 0; rows_touched = 0
    while not g.done.all():
        rows_touched += g.R
        s = g.seat_to_play(); L = g.legal(s)
        mine = (s == seat) & ~g.done
        if mine.any():
            n = np.where(mine, popcount(L), 1)                      # branch factor per row (1 for non-mine rows)
            idx = np.repeat(np.arange(g.R), n); j = np.arange(len(idx)) - np.repeat(np.cumsum(n) - n, n)
            g = clone_rows(g, idx); widx = widx[idx]; root = root[idx]; L = L[idx]; mine = mine[idx]; s = s[idx]
            t_mine = jth_set_bit_rows(L, j)
        n_oth = popcount(L); k = np.minimum((tape[widx, ply] * n_oth).astype(np.int64), np.maximum(n_oth - 1, 0))
        t = kth_set_bit(L, k)
        if mine.any():
            t = np.where(mine, t_mine, t)
            first = mine & (root < 0); root[first] = t[first]
        t = np.where(g.done, -1, t); g.play(t); ply += 1
    made = g.made()
    vals = {}
    for tt in legal:
        sel = root == tt
        per_world = np.full(W, -9 if side_bid else 9, np.int64)
        if side_bid: np.maximum.at(per_world, widx[sel], made[sel])
        else: np.minimum.at(per_world, widx[sel], made[sel])
        vals[tt] = per_world.mean()
    choice = max(legal, key=lambda t: (vals[t], -t)) if side_bid else min(legal, key=lambda t: (vals[t], t))
    return (vals, choice, rows_touched, g.R) if return_cost else (vals, choice)

# ---- orderings-based L1 (full or m lines) and V0, for comparison; same signatures
def l1_orders(rules, bidder, seat, hand_now, history, deal_full, rng, W, m=None, return_cost=False):
    rem, full = worlds(rng, rules, seat, hand_now, history, W); W = len(rem)
    g1 = public(rules, bidder, full[0], history); L0 = g1.legal(np.array([seat]))[0]; legal = [t for t in range(28) if (L0 >> t) & 1]
    side_bid = (seat % 2) == (bidder % 2); mine = [t for t in range(28) if (hand_now >> t) & 1]
    allp = list(itertools.permutations(mine))
    if m is None or len(allp) <= m: orders = np.tile(np.array(allp, np.int64), (W, 1)); P = len(allp)
    else:
        lines = []
        for w in range(W):
            Ls = [tuple(rng.permutation(mine)) for _ in range(m)]
            for j, t in enumerate(legal): Ls[j % m] = tuple([t] + [x for x in rng.permutation(mine) if x != t])
            lines += Ls
        orders = np.array(lines, np.int64); P = m
    R = W * P
    g = Game(rules, np.repeat(rem, P, axis=0), BID, bidder, scores=(np.repeat(g1.bid_pts, R), np.repeat(g1.def_pts, R)), leader=np.repeat(g1.leader, R), table=np.repeat(g1.table, R, axis=0), trick=np.repeat(g1.trick, R))
    tape = np.repeat(rng.random((W, 28)), P, axis=0); sd = g.seat_to_play(); L = g.legal(sd); root = first_in_order(L, orders)
    rows = 0; ply = 0
    while not g.done.all():
        rows += (~g.done).sum(); s = g.seat_to_play(); L = g.legal(s); n = popcount(L); k = np.minimum((tape[:, ply] * n).astype(np.int64), np.maximum(n - 1, 0))
        t = kth_set_bit(L, k); mn = (s == seat) & ~g.done
        if mn.any(): t = np.where(mn, first_in_order(L, orders), t)
        g.play(np.where(g.done, -1, t)); ply += 1
    made = g.made().reshape(W, P); root = root.reshape(W, P); vals = {}
    for t in legal:
        pw = np.where(root == t, made, -9 if side_bid else 9); vals[t] = (pw.max(1) if side_bid else pw.min(1)).mean()
    choice = max(legal, key=lambda t: (vals[t], -t)) if side_bid else min(legal, key=lambda t: (vals[t], t))
    return (vals, choice, rows, R) if return_cost else (vals, choice)

def v0(rules, bidder, seat, hand_now, history, deal_full, rng, W, T, return_cost=False):
    rem, full = worlds(rng, rules, seat, hand_now, history, W); g1 = public(rules, bidder, full[0], history)
    L0 = g1.legal(np.array([seat]))[0]; legal = [t for t in range(28) if (L0 >> t) & 1]; side_bid = (seat % 2) == (bidder % 2)
    tape = rng.random((len(rem) * T, 28)); vals = {}; rows = 0
    for t in legal:
        R = len(rem) * T
        g = Game(rules, np.repeat(rem, T, axis=0), BID, bidder, scores=(np.repeat(g1.bid_pts, R), np.repeat(g1.def_pts, R)), leader=np.repeat(g1.leader, R), table=np.repeat(g1.table, R, axis=0), trick=np.repeat(g1.trick, R))
        g.play(np.full(R, t)); run_out(g, tape); vals[t] = g.made().mean(); rows += R * 20
    choice = max(legal, key=lambda t: (vals[t], -t)) if side_bid else min(legal, key=lambda t: (vals[t], t))
    return (vals, choice, rows, len(rem) * T) if return_cost else (vals, choice)

RUNGS = {
    'L0':            None,
    'V0 32x8':       lambda *a: v0(*a, 32, 8),
    'L1 8 full':     lambda *a: l1_orders(*a, 8),
    'L1 32x24 lines': lambda *a: l1_orders(*a, 32, 24),
    'exact 8':       lambda *a: exact_tape(*a, 8),
    'exact 32':      lambda *a: exact_tape(*a, 32),
    'exact 128':     lambda *a: exact_tape(*a, 128),
}

def hand_setup(seed, my, bidder):
    r = np.random.default_rng(seed); d = random_deals(r, 1)[0]; return Rules(pick_trump(int(d[bidder]))), int(d[my]), d

def sample_positions(rules, bidder, my, my_hand, r, n_games, quota):
    hands = random_deals(r, n_games, my, my_hand); g = Game(rules, hands, BID, bidder); tape = r.random((n_games, 28)); hist = [[] for _ in range(n_games)]; out = []; ply = 0
    while not g.done.all():
        seat = g.seat_to_play(); L = g.legal(seat); n = popcount(L)
        for i in np.nonzero((seat == my) & ~g.done & (n >= 2))[0]: out.append(dict(history=list(hist[i]), hand_now=int(g.hands[i, my]), trick=int(g.trick[i]), deal=hands[i].copy()))
        k = np.minimum((tape[:, ply] * n).astype(np.int64), np.maximum(n - 1, 0)); t = np.where(g.done, -1, kth_set_bit(L, k))
        for i in np.nonzero(t >= 0)[0]: hist[i].append((int(seat[i]), int(t[i])))
        g.play(t); ply += 1
    by = {}; [by.setdefault(p['trick'], []).append(p) for p in out]; test = []
    for k, q in quota.items(): lst = by.get(k, []); r.shuffle(lst); test += lst[:q]
    return test
