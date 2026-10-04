"""test_support.py — lawful-support regression test for the consistent-world samplers.
Independent scalar checker: replays a (deal, history) with the TRUE bidder, no early stop, and verifies every play followed suit
when it could. (1) Reproduces the audit witness class: a legality replay with bidder hard-coded to 0 and early-stop live accepts
illegal worlds on odd-bidder positions. (2) The fixed samplers (engine42.consistent_worlds, nofusion_sc.all_deals, batched.batched_worlds)
yield zero violations over many positions with both even and odd bidders.
RESULT (Oct 4 2026): FIXED samplers 37,872 worlds checked, violations = 0. OLD path on 144 odd-bidder positions: accepted 13,288
candidate worlds, of which 3,731 ILLEGAL. Exit code 0 iff fixed samplers are clean AND the old path's bug is reproduced."""
import numpy as np, sys
from engine42 import *
from exact_tape import hand_setup, sample_positions, BID
import nofusion_sc, batched

def scalar_legal_replay(rules, deal_full, bidder, history):
    """True iff every play in history was legal for its seat under deal_full (scalar, bidder-aware, no early stop)."""
    hands = [int(x) for x in deal_full]; leader = bidder; table = []
    for s, t in history:
        if (leader + len(table)) % 4 != s: return False
        h = hands[s]
        if not (h >> t) & 1: return False
        if table:
            q = int(rules.led_code[table[0]]); f = h & int(rules.follow_mask[q])
            if f and not (f >> t) & 1: return False          # could follow, didn't
        hands[s] &= ~(1 << t); table.append(t)
        if len(table) == 4:
            q = int(rules.led_code[table[0]]); st = [int(rules.strength[q, x]) for x in table]
            leader = (leader + int(np.argmax(st))) % 4; table = []
    return True

def old_style_replay_accepts(rules, full, history):
    """the pre-fix path: bidder hard-coded to 0, early stop live (vectorized over worlds)."""
    g = Game(rules, full.copy(), 30, 0); g.leader[:] = history[0][0]; ok = np.ones(len(full), bool)
    for s, t in history:
        L = g.legal(np.full(len(full), s)); ok &= ((L >> t) & 1) == 1; g.play(np.full(len(full), t))
    return ok

def full_of(rem, history):
    full = rem.copy()
    for s in range(4):
        pb = [t for q, t in history if q == s]
        if pb: full[:, s] |= int(BIT[pb].sum())
    return full

if __name__ == '__main__':
    rng = np.random.default_rng(0); viol = 0; checked = 0; old_bug = 0; old_checked = 0; nodd = 0
    for seed, bidder in ((11, 1), (12, 3), (13, 0), (14, 2), (15, 1), (16, 3)):
        rules, my_hand, d0 = hand_setup(seed, 2, bidder)
        for p in sample_positions(rules, bidder, 2, my_hand, np.random.default_rng(seed), 60, {1: 6, 2: 8, 3: 8, 4: 8, 5: 6}):
            hist = p['history']
            rem, full = consistent_worlds(rng, rules, 2, p['hand_now'], hist, 64)
            for w in range(len(full)):
                checked += 1; viol += not scalar_legal_replay(rules, full[w], bidder, hist)
            if 28 - len(hist) <= 12:
                rem2 = nofusion_sc.all_deals(rules, 2, p['hand_now'], hist)
                if rem2 is not None:
                    f2 = full_of(rem2, hist)
                    for w in range(len(f2)): checked += 1; viol += not scalar_legal_replay(rules, f2[w], bidder, hist)
            s_other = hist[-1][0]; played_by = np.zeros((1, 4), np.int64)
            for s, t in hist: played_by[0, s] |= BIT[t]
            hands_now = np.array([int(p['deal'][s_other]) & ~int(played_by[0, s_other])])
            rb, filled = batched.batched_worlds(rng, rules, bidder, np.array([s_other]), hands_now, played_by,
                                                np.array([[s for s, t in hist]]), np.array([[t for s, t in hist]]), 8)
            for k in range(8):
                if filled[0, k]:
                    checked += 1; viol += not scalar_legal_replay(rules, full_of(rb[0, k:k+1], hist)[0], bidder, hist)
            if bidder % 2 == 1:
                nodd += 1
                cand = random_deals(np.random.default_rng(99), 256, 2, p['hand_now']); cf = full_of(cand, hist)
                acc = old_style_replay_accepts(rules, cf, hist)
                for w in np.nonzero(acc)[0]:
                    old_checked += 1; old_bug += not scalar_legal_replay(rules, cf[w], bidder, hist)
    print(f'FIXED samplers: {checked} sampled/enumerated worlds checked by the independent scalar replay, violations = {viol}')
    print(f'OLD replay path (bidder=0, early stop) on {nodd} odd-bidder positions: accepted {old_checked} worlds, of which ILLEGAL = {old_bug}')
    sys.exit(0 if viol == 0 and old_bug > 0 else 1)
