"""flatplan.v2.py — flat, fusion-free L1: a plan is a priority ordering over my tiles (depends only on my hand and the led suit, so it is
information-set-consistent across worlds). Rows = worlds x plans, one pass, aggregate by plan (SUM over worlds), pick the best plan,
play its first legal tile. Compared on the same worlds and tape to exact-tape (full own tree per world) — the restricted-class gap.

v2 (audit revision): plans_for draws ONE shared plan list, independent of any hidden world, keyed by canonical ordering tuple and
DEDUPLICATED (v1 could repeat a plan via random draws or the root-coverage overwrite; duplicates cannot change the argmin but waste rows).
Each world's tape row is reused across all plans (np.repeat(tape, P)); plans are tiled across worlds (np.tile(plans, (W, 1))), so every
column is one policy across all worlds and the SUM-over-worlds fold is lawful.

RESULT (Oct 4 2026, 128 worlds, ref exact-tape@384, 50 positions per hand; identical before/after dedupe):
  defending hand C: same decision as tree 86%, flat regret 0.0087 vs tree 0.0000, value lost to plan class 0.064, 26k rows, ~180 ms
  bidding hand A:   same decision 78% (trick 1: 25%), flat regret 0.0131 (trick 1: 0.034) vs tree 0.0023, value lost 0.111 (trick 1: 0.34), ~270 ms
  -> lawful and fusion-free by construction; beats Walt's 8-world inner rung for defenders; WORSE for a bidder at trick 1.
     Not sufficient as a universal inner rung. The shared-ordering class is strictly smaller than history-dependent information-set
     policies; the per-world realization lemma (exact-tape) does not extend to it.
usage: python3 flatplan.v2.py <seed> <bidder-seat> "<label>"   (my seat is 2)"""
import numpy as np, itertools, time, sys
from engine42 import *
from exact_tape import worlds, public, clone_rows, hand_setup, sample_positions, exact_tape, BID

def plans_for(rng, mine, legal, m):
    """ONE shared plan list, drawn independently of any hidden world, keyed by canonical ordering tuple (deduplicated)."""
    allp = list(itertools.permutations(mine))
    if len(allp) <= m: return np.array(allp, np.int64)
    P = {}
    for t in legal: P[tuple([t] + [int(x) for x in rng.permutation(mine) if x != t])] = True       # every legal root tile covered
    while len(P) < m: P[tuple(int(x) for x in rng.permutation(mine))] = True
    return np.array(list(P.keys()), np.int64)

def flat_and_tree(rules, bidder, seat, hand_now, history, deal_full, rng, W, m=720):
    """returns (flat_vals per root tile, flat_choice, tree_vals, tree_choice, flat_best_plan_value, n_plans, rows) on shared worlds+tape."""
    rem, full = worlds(rng, rules, seat, hand_now, history, W); W = len(rem)
    g1 = public(rules, bidder, full[0], history); L0 = int(g1.legal(np.array([seat]))[0]); legal = [t for t in range(28) if (L0 >> t) & 1]
    side_bid = (seat % 2) == (bidder % 2); mine = [t for t in range(28) if (hand_now >> t) & 1]
    tape = rng.random((W, 28))
    # ---- flat: worlds x plans (one shared plan list; each world's tape row reused across plans)
    plans = plans_for(rng, mine, legal, m); P = len(plans); R = W * P
    g = Game(rules, np.repeat(rem, P, axis=0), BID, bidder, scores=(np.repeat(g1.bid_pts, R), np.repeat(g1.def_pts, R)), leader=np.repeat(g1.leader, R), table=np.repeat(g1.table, R, axis=0), trick=np.repeat(g1.trick, R))
    orders = np.tile(plans, (W, 1)); tp = np.repeat(tape, P, axis=0)
    s0 = g.seat_to_play(); root = first_in_order(g.legal(s0), orders)[:P]          # same in every world (legality is public)
    run_out(g, tp, my_seat=seat, orders=orders)
    made = g.made().reshape(W, P)
    per_plan = made.sum(0) / W                                                      # SUM over worlds first: one plan for all worlds
    best = int(per_plan.argmax() if side_bid else per_plan.argmin())
    flat_vals = {}
    for t in legal:
        sel = root == t
        flat_vals[t] = float(per_plan[sel].max() if side_bid else per_plan[sel].min()) if sel.any() else float('nan')
    flat_choice = int(root[best])
    # ---- tree on the SAME worlds and tape: per-world best response
    tree_vals = {}
    gT = Game(rules, rem, BID, bidder, scores=(np.repeat(g1.bid_pts, W), np.repeat(g1.def_pts, W)), leader=np.repeat(g1.leader, W), table=np.repeat(g1.table, W, axis=0), trick=np.repeat(g1.trick, W))
    widx = np.arange(W); rootT = np.full(W, -1, np.int64); ply = 0
    while not gT.done.all():
        s = gT.seat_to_play(); L = gT.legal(s); mn = (s == seat) & ~gT.done
        if mn.any():
            n = np.where(mn, popcount(L), 1); idx = np.repeat(np.arange(gT.R), n); j = np.arange(len(idx)) - np.repeat(np.cumsum(n) - n, n)
            gT = clone_rows(gT, idx); widx = widx[idx]; rootT = rootT[idx]; L = L[idx]; mn = mn[idx]; tm = kth_set_bit(L, j)
        no = popcount(L); k = np.minimum((tape[widx, ply] * no).astype(np.int64), np.maximum(no - 1, 0)); t = kth_set_bit(L, k)
        if mn.any(): t = np.where(mn, tm, t); f = mn & (rootT < 0); rootT[f] = t[f]
        gT.play(np.where(gT.done, -1, t)); ply += 1
    mT = gT.made()
    for t in legal:
        sel = rootT == t; pw = np.full(W, -9 if side_bid else 9, np.int64)
        (np.maximum.at if side_bid else np.minimum.at)(pw, widx[sel], mT[sel]); tree_vals[t] = pw.mean()
    tree_choice = max(legal, key=lambda t: (tree_vals[t], -t)) if side_bid else min(legal, key=lambda t: (tree_vals[t], t))
    return flat_vals, flat_choice, tree_vals, tree_choice, float(per_plan[best]), P, R

if __name__ == '__main__':
    seed, my, bidder, label = int(sys.argv[1]), 2, int(sys.argv[2]), sys.argv[3]
    rules, my_hand, d0 = hand_setup(seed, my, bidder); bidside = (my % 2) == (bidder % 2)
    test = sample_positions(rules, bidder, my, my_hand, np.random.default_rng(seed + 5), 300, {0: 4, 1: 8, 2: 10, 3: 10, 4: 10, 5: 8})
    t0 = time.time()
    REF = [exact_tape(rules, bidder, my, p['hand_now'], p['history'], p['deal'], np.random.default_rng(90_000 + i), 384)[0] for i, p in enumerate(test)]
    tref = time.time() - t0
    agree = []; reg_flat = []; reg_tree = []; gap = []; rows = []; tf = []
    for i, p in enumerate(test):
        a = time.time(); fv, fc, tv, tc, fbest, P, R = flat_and_tree(rules, bidder, my, p['hand_now'], p['history'], p['deal'], np.random.default_rng(i), 128); tf.append(time.time() - a)
        ref = REF[i]; best = max(ref.values()) if bidside else min(ref.values())
        agree.append(fc == tc); reg_flat.append(abs(ref[fc] - best)); reg_tree.append(abs(ref[tc] - best))
        gap.append(abs(tv[tc] - fbest)); rows.append(R)
    print(f'{label}: {len(test)} positions (ref exact-tape@384, {tref:.0f}s). flat-plan vs exact tree on the SAME 128 worlds+tape:')
    print(f'  same decision {100*np.mean(agree):.0f}% | regret vs ref: flat {np.mean(reg_flat):.4f}, tree {np.mean(reg_tree):.4f} | mean |tree value - best flat plan value| {np.mean(gap):.4f} | rows/decision mean {np.mean(rows):,.0f} | ms/decision {1000*np.mean(tf):.0f}')
    for k in range(6):
        idx = [i for i, p in enumerate(test) if p['trick'] == k]
        if idx: print(f'  trick {k+1}: agree {100*np.mean([agree[i] for i in idx]):3.0f}%  flat regret {np.mean([reg_flat[i] for i in idx]):.4f}  tree regret {np.mean([reg_tree[i] for i in idx]):.4f}  value gap {np.mean([gap[i] for i in idx]):.4f}')
