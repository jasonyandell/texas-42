"""flatplan.v3.py — flat, fusion-free L1 (shared priority-ordering plans; rows = worlds x plans; SUM over worlds per plan; best plan's
first legal tile). Requires engine42.v2 (no_stop replay), exact_tape, nofusion_sc (with the no_stop patch).

v3 (audit): reports TWO gap objectives on the same worlds and tape —
  (a) vs the fused per-world-extrema tree  = plan-class restriction + fusion slack (upper bound on the class gap)
  (b) vs the LAWFUL shared-history tree    = the class gap proper (one decision per information set; nofusion_sc.expectimax with tape)
and the shared plan list is drawn once, independent of any hidden world, keyed by canonical ordering tuple, deduplicated.

RESULT (Oct 4 2026, fixed sampler, 128 worlds, 50 positions/hand, reference exact-tape@384 = per-world extrema):
  defending C: flat vs lawful tree same decision 92%; flat regret 0.0051; class gap proper 0.054 (fused 0.065)
  bidding A:   same decision 78% (trick 1: 25%); flat regret 0.0131; class gap proper 0.103, trick 1: 0.336 (fused 0.338)
usage: python3 flatplan.v3.py <seed> <bidder-seat> "<label>"   (my seat is 2)"""
import numpy as np, itertools, time, sys
from engine42 import *
from exact_tape import worlds, public, clone_rows, hand_setup, sample_positions, exact_tape, BID
import nofusion_sc

def plans_for(rng, mine, legal, m):
    """ONE shared plan list, drawn independently of any hidden world, keyed by canonical ordering tuple (deduplicated)."""
    allp = list(itertools.permutations(mine))
    if len(allp) <= m: return np.array(allp, np.int64)
    P = {}
    for t in legal: P[tuple([t] + [int(x) for x in rng.permutation(mine) if x != t])] = True       # every legal root tile covered
    while len(P) < m: P[tuple(int(x) for x in rng.permutation(mine))] = True
    return np.array(list(P.keys()), np.int64)

def flat_and_tree(rules, bidder, seat, hand_now, history, deal_full, rng, W, m=720):
    rem, full = worlds(rng, rules, seat, hand_now, history, W); W = len(rem)
    g1 = public(rules, bidder, full[0], history); L0 = int(g1.legal(np.array([seat]))[0]); legal = [t for t in range(28) if (L0 >> t) & 1]
    side_bid = (seat % 2) == (bidder % 2); mine = [t for t in range(28) if (hand_now >> t) & 1]
    tape = rng.random((W, 28))
    # ---- flat: worlds x plans (one shared plan list; each world's tape row reused across plans)
    plans = plans_for(rng, mine, legal, m); P = len(plans); R = W * P
    g = Game(rules, np.repeat(rem, P, axis=0), BID, bidder, scores=(np.repeat(g1.bid_pts, R), np.repeat(g1.def_pts, R)), leader=np.repeat(g1.leader, R), table=np.repeat(g1.table, R, axis=0), trick=np.repeat(g1.trick, R))
    orders = np.tile(plans, (W, 1)); tp = np.repeat(tape, P, axis=0)
    s0 = g.seat_to_play(); root = first_in_order(g.legal(s0), orders)[:P]
    run_out(g, tp, my_seat=seat, orders=orders)
    made = g.made().reshape(W, P)
    per_plan = made.sum(0) / W                                                      # SUM over worlds first: one plan for all worlds
    best = int(per_plan.argmax() if side_bid else per_plan.argmin())
    flat_vals = {}
    for t in legal:
        sel = root == t
        flat_vals[t] = float(per_plan[sel].max() if side_bid else per_plan[sel].min()) if sel.any() else float('nan')
    flat_choice = int(root[best])
    # ---- (a) fused tree on the SAME worlds and tape: per-world best response (per-world extrema)
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
    # ---- (b) LAWFUL shared-history tree on the SAME worlds and tape (one decision per information set): the class gap proper
    law_vals, law_choice, _ = nofusion_sc.expectimax(rules, bidder, seat, hand_now, history, full[0], rem, tape=tape)
    return flat_vals, flat_choice, tree_vals, tree_choice, float(per_plan[best]), P, R, law_vals, law_choice

if __name__ == '__main__':
    seed, my, bidder, label = int(sys.argv[1]), 2, int(sys.argv[2]), sys.argv[3]
    rules, my_hand, d0 = hand_setup(seed, my, bidder); bidside = (my % 2) == (bidder % 2)
    test = sample_positions(rules, bidder, my, my_hand, np.random.default_rng(seed + 5), 300, {0: 4, 1: 8, 2: 10, 3: 10, 4: 10, 5: 8})
    t0 = time.time()
    REF = [exact_tape(rules, bidder, my, p['hand_now'], p['history'], p['deal'], np.random.default_rng(90_000 + i), 384)[0] for i, p in enumerate(test)]
    tref = time.time() - t0
    agree = []; agree_law = []; reg_flat = []; reg_tree = []; reg_law = []; gap_fused = []; gap_law = []; rows = []; tf = []
    for i, p in enumerate(test):
        a = time.time(); fv, fc, tv, tc, fbest, P, R, lv, lc = flat_and_tree(rules, bidder, my, p['hand_now'], p['history'], p['deal'], np.random.default_rng(i), 128); tf.append(time.time() - a)
        ref = REF[i]; best = max(ref.values()) if bidside else min(ref.values())
        agree.append(fc == tc); agree_law.append(fc == lc); reg_flat.append(abs(ref[fc] - best)); reg_tree.append(abs(ref[tc] - best)); reg_law.append(abs(ref[lc] - best))
        gap_fused.append(abs(tv[tc] - fbest)); gap_law.append(abs(lv[lc] - fbest)); rows.append(R)
    print(f'{label}: {len(test)} positions (ref exact-tape@384 = per-world extrema, {tref:.0f}s). Same 128 worlds+tape for all three estimators.')
    print(f'  decision agreement: flat vs fused tree {100*np.mean(agree):.0f}%, flat vs LAWFUL grouped tree {100*np.mean(agree_law):.0f}%')
    print(f'  regret vs ref: flat {np.mean(reg_flat):.4f}, fused tree {np.mean(reg_tree):.4f}, lawful tree {np.mean(reg_law):.4f}')
    print(f'  value gap of best shared plan: vs fused tree (class + fusion slack) {np.mean(gap_fused):.4f}; vs LAWFUL tree (class gap proper) {np.mean(gap_law):.4f}')
    print(f'  rows/decision {np.mean(rows):,.0f}; ms/decision {1000*np.mean(tf):.0f}')
    for k in range(6):
        idx = [i for i, p in enumerate(test) if p['trick'] == k]
        if idx: print(f'  trick {k+1}: agree(lawful) {100*np.mean([agree_law[i] for i in idx]):3.0f}%  flat regret {np.mean([reg_flat[i] for i in idx]):.4f}  lawful-tree regret {np.mean([reg_law[i] for i in idx]):.4f}  class gap {np.mean([gap_law[i] for i in idx]):.4f}  (+fusion slack: {np.mean([gap_fused[i] for i in idx]):.4f})')
