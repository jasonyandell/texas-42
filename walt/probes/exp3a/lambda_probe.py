#!/usr/bin/env python3
"""
lambda_probe.py -- First computational experiment from the GPT-Pro integration
document (Sec. 14): one-parameter exact solve.

Tier: exploratory probe. Sits below every evidentiary tier; its numbers are
quotable only via amendment to a verifier receipt. Stdlib only, exact
Fractions everywhere, no floats.

Domain: last two tricks of frozen receipt hand 0 (rob/receipts/verify_player.txt,
declaration P3 = threes trump). At trick-6 start: S1 leads, every seat holds 2
tiles, S1's hidden fiber = 90 worlds (voids observed in tricks 1-5 are vacuous
for the six unseen tiles -- none contains a 3).

Valuation: w_lambda = lambda * e_{4-1}, lambda >= 0; trick value b = 1.
Objective: continuation differential for team T1 = {S1, S3} over the last two
tricks:  Psi(lambda) = (tricks_T1 - tricks_T0) + lambda * (cap_T1(4-1) - cap_T0(4-1)).

Part A (doc Sec. 14.1-14.2): fixed-field information-set solve for viewer S1.
  Belief: uniform over the 90-world fiber. Field: the three hidden seats play
  uniformly at random among legal tiles. S1's only decision is the trick-6
  lead (trick 7 is forced), so each root action yields ONE exact line
  a + lambda*b; the upper envelope of the two lines gives the breakpoint.

Part B (doc Sec. 14.3 q4-q6): per-world parametric minimax (perfect
  information, both teams optimal) -> piecewise-linear Q(lead; lambda) per
  world; signature census over the 90 worlds; class counts at lambda = 0
  versus over the whole lambda >= 0 family; role-fact correlation.
"""

from fractions import Fraction
from itertools import combinations
import re, sys

F = Fraction
TRUMP = 3

# ----------------------------------------------------------------- tiles/rules

def T(s):
    a, b = s.split("-")
    hi, lo = int(a), int(b)
    assert lo <= hi
    return (hi, lo)

def has_pip(t, p):
    return t[0] == p or t[1] == p

def is_trump(t):
    return has_pip(t, TRUMP)

def led_suit(t):
    return TRUMP if is_trump(t) else t[0]

def in_suit(t, s):
    """Membership under absorption: a trump tile belongs to trump only."""
    if s == TRUMP:
        return is_trump(t)
    return (not is_trump(t)) and has_pip(t, s)

def rank_key(t, s):
    """Order within suit s: double first, then descending off-pip."""
    assert in_suit(t, s)
    if t[0] == t[1]:
        return 100
    return t[1] if t[0] == s else t[0]

def count_pts(t):
    tot = t[0] + t[1]
    if tot == 5:
        return 5
    if tot == 10:
        return 10
    return 0

def trick_winner(plays):
    """plays: list of (seat, tile) in play order; returns winning seat."""
    led = led_suit(plays[0][1])
    trumps = [(rank_key(t, TRUMP), s) for s, t in plays if in_suit(t, TRUMP)]
    if trumps:
        return max(trumps)[1]
    ins = [(rank_key(t, led), s) for s, t in plays if in_suit(t, led)]
    return max(ins)[1]

def legal(hand, led):
    """led = led suit or None (leading). Must follow if able."""
    if led is None:
        return list(hand)
    follows = [t for t in hand if in_suit(t, led)]
    return follows if follows else list(hand)

# ------------------------------------------------------- receipt replay check

RECEIPT = "/Users/jason/code/texas-42/rob/receipts/verify_player.txt"

def parse_hand0():
    tricks = []
    with open(RECEIPT) as f:
        lines = f.readlines()
    i = next(k for k, l in enumerate(lines) if l.startswith("hand 0:"))
    header = lines[i]
    assert "declaration P3" in header
    for l in lines[i + 1 : i + 8]:
        m = re.match(r"\s+trick \d+: (\S+) (\S+) (\S+) (\S+) -> (S\d) \+(\d+)", l)
        assert m, l
        plays = []
        for tok in m.group(1, 2, 3, 4):
            seat, tile = tok.split(":")
            plays.append((int(seat[1]), T(tile)))
        tricks.append((plays, int(m.group(5)[1]), int(m.group(6))))
    return tricks

def replay_validate(tricks):
    """Re-run rules over the receipt line; assert winners, points, legality."""
    hands = {s: set() for s in range(4)}
    for plays, _, _ in tricks:
        for s, t in plays:
            hands[s].add(t)
    assert all(len(h) == 7 for h in hands.values())
    leader = tricks[0][0][0][0]
    for plays, rec_winner, rec_pts in tricks:
        assert plays[0][0] == leader
        order = [(leader + k) % 4 for k in range(4)]
        assert [s for s, _ in plays] == order
        led = led_suit(plays[0][1])
        for s, t in plays:
            assert t in hands[s]
            if s != plays[0][0]:
                assert t in legal(hands[s], led), (s, t, led)
            hands[s].discard(t)
        w = trick_winner(plays)
        pts = 1 + sum(count_pts(t) for _, t in plays)
        assert w == rec_winner, (plays, w, rec_winner)
        assert pts == rec_pts, (plays, pts, rec_pts)
        leader = w
    return True

# --------------------------------------------------------------- PWL machinery
# A PWL function on [0, inf) as a list of (x_start, slope, intercept),
# x_start increasing, first x_start = 0. Segment value at x: slope*x + intercept.

def line(a, b):
    return [(F(0), F(b), F(a))]  # value a + b*lambda

def _eval(f, x):
    seg = None
    for xs, sl, ic in f:
        if xs <= x:
            seg = (sl, ic)
        else:
            break
    sl, ic = seg
    return sl * x + ic

def _breaks(f):
    return [xs for xs, _, _ in f]

def _combine(f, g, pick):
    xs = sorted(set(_breaks(f)) | set(_breaks(g)))
    out = []
    for i, x0 in enumerate(xs):
        x1 = xs[i + 1] if i + 1 < len(xs) else None
        fs = [(sl, ic) for xx, sl, ic in f if xx <= x0][-1]
        gs = [(sl, ic) for xx, sl, ic in g if xx <= x0][-1]
        (fsl, fic), (gsl, gic) = fs, gs
        # crossing inside (x0, x1)?
        cross = None
        if fsl != gsl:
            xc = F(gic - fic, fsl - gsl)
            if xc > x0 and (x1 is None or xc < x1):
                cross = xc
        # BUGFIX 2026-08-09 (v3 session): the no-cross case previously built
        # [(x0, None)], dropping the interval's true end x1, so the winner was
        # probed at x0+1 -- possibly outside [x0, x1). Harmless for single
        # lines (v0.1/v0.2 results unaffected; re-verified), wrong when a
        # multi-segment PWL is combined.
        pieces = [(x0, x1)] if cross is None else [(x0, cross), (cross, x1)]
        for (a0, a1) in pieces:
            probe = a0 + 1 if a1 is None else F(a0 + a1, 2)
            fv = fsl * probe + fic
            gv = gsl * probe + gic
            sl, ic = (fsl, fic) if pick(fv, gv) else (gsl, gic)
            if out and out[-1][1] == sl and out[-1][2] == ic:
                continue
            out.append((a0, sl, ic))
    return out

def pwl_max(fs):
    r = fs[0]
    for f in fs[1:]:
        r = _combine(r, f, lambda a, b: a >= b)
    return r

def pwl_min(fs):
    r = fs[0]
    for f in fs[1:]:
        r = _combine(r, f, lambda a, b: a <= b)
    return r

def pwl_str(f):
    parts = []
    for i, (xs, sl, ic) in enumerate(f):
        end = f[i + 1][0] if i + 1 < len(f) else "inf"
        parts.append(f"[{xs},{end}): {ic}+{sl}L")
    return "  ".join(parts)

# ------------------------------------------------------------- endgame domain

def endgame_state(tricks):
    """Hands at trick-6 start, plus leader."""
    hands = {s: set() for s in range(4)}
    for plays, _, _ in tricks:
        for s, t in plays:
            hands[s].add(t)
    leader = tricks[0][0][0][0]
    for plays, w, _ in tricks[:5]:
        for s, t in plays:
            hands[s].discard(t)
        leader = w
    return hands, leader

D_TILE = T("4-1")           # the valued tile
VIEWER = 1                  # S1 leads trick 6
TEAM = {0: 0, 1: 1, 2: 0, 3: 1}   # T1 = {S1,S3}; differential is T1 - T0

def team_sign(seat):
    return 1 if TEAM[seat] == 1 else -1

def leaf_line(trickdiff, capsign):
    """Psi(lambda) = trickdiff + lambda*capsign."""
    return line(trickdiff, capsign)

def play_out(hands, leader, chooser):
    """
    Generic 2-trick playout skeleton. `chooser(seat, options, ctx)` returns list
    of (weight_or_None, tile) branches. Returns list of (prob_or_None, leaf).
    Used in expectation mode (weights) by Part A.
    """
    raise NotImplementedError

# ---- Part A: fixed-field expectation (uniform field), viewer decision at root

def field_expect(hands0, leader, lead_tile):
    """E over uniform-random legal field plays of the leaf line, one world."""
    results = []  # (Fraction prob, trickdiff, capsign)

    def rec_trick(hands, order, plays, prob, tricknum, tdiff):
        if len(plays) == 4:
            w = trick_winner(plays)
            td = tdiff + team_sign(w)
            capt = TEAM[w] if any(t == D_TILE for _, t in plays) else None
            if tricknum == 6:
                # trick 7: all forced (1 tile each), order from winner
                order7 = [(w + k) % 4 for k in range(4)]
                plays7 = [(s, next(iter(hands[s]))) for s in order7]
                w7 = trick_winner(plays7)
                td7 = td + team_sign(w7)
                if capt is None:
                    capt = TEAM[w7] if any(t == D_TILE for _, t in plays7) else None
                assert capt is not None
                results.append((prob, td7, 1 if capt == 1 else -1))
            return
        seat = order[len(plays)]
        led = led_suit(plays[0][1])
        opts = legal(hands[seat], led)
        for t in opts:
            h2 = {s: set(v) for s, v in hands.items()}
            h2[seat].discard(t)
            rec_trick(h2, order, plays + [(seat, t)], prob / len(opts), tricknum, tdiff)

    h = {s: set(v) for s, v in hands0.items()}
    h[leader].discard(lead_tile)
    order = [(leader + k) % 4 for k in range(4)]
    rec_trick(h, order, [(leader, lead_tile)], F(1), 6, 0)
    a = sum(p * td for p, td, _ in results)
    b = sum(p * cs for p, _, cs in results)
    assert sum(p for p, _, _ in results) == 1
    return a, b  # E[Psi] = a + lambda*b

# ---- Part B: per-world parametric minimax (perfect information)

def minimax_pwl(hands0, leader, lead_tile):
    """Exact parametric minimax value of leading lead_tile, one world."""

    def rec(hands, order, plays, tricknum, tdiff):
        if len(plays) == 4:
            w = trick_winner(plays)
            td = tdiff + team_sign(w)
            capt = TEAM[w] if any(t == D_TILE for _, t in plays) else None
            if tricknum == 7:
                assert capt is not None or True
                return leaf_line(td, (1 if capt == 1 else -1)) if capt is not None else leaf_line(td, 0)
            # trick 7 from winner; winner chooses lead (only 1 tile: forced)
            order7 = [(w + k) % 4 for k in range(4)]
            plays7 = [(s, next(iter(hands[s]))) for s in order7]
            w7 = trick_winner(plays7)
            td7 = td + team_sign(w7)
            if capt is None:
                capt = TEAM[w7] if any(t == D_TILE for _, t in plays7) else None
            assert capt is not None
            return leaf_line(td7, 1 if capt == 1 else -1)
        seat = order[len(plays)]
        led = led_suit(plays[0][1])
        opts = legal(hands[seat], led)
        vals = []
        for t in opts:
            h2 = {s: set(v) for s, v in hands.items()}
            h2[seat].discard(t)
            vals.append(rec(h2, order, plays + [(seat, t)], tricknum, tdiff))
        return pwl_max(vals) if TEAM[seat] == 1 else pwl_min(vals)

    h = {s: set(v) for s, v in hands0.items()}
    h[leader].discard(lead_tile)
    order = [(leader + k) % 4 for k in range(4)]
    return rec(h, order, [(leader, lead_tile)], 6, 0)

# ---------------------------------------------------------------------- main

def main():
    tricks = parse_hand0()
    assert replay_validate(tricks)
    print("receipt replay: winners, points, legality all validated against hand 0")

    hands, leader = endgame_state(tricks)
    assert leader == VIEWER == 1
    s1_hand = sorted(hands[1])
    unseen = sorted(hands[0] | hands[2] | hands[3])
    print(f"trick-6 leader: S{leader}; S1 hand: {s1_hand}; unseen: {unseen}")
    assert D_TILE in unseen and len(unseen) == 6

    # fiber: 90 assignments of unseen into S2/S3/S0 (2 each); trump voids vacuous
    worlds = []
    for s2 in combinations(unseen, 2):
        rest = [t for t in unseen if t not in s2]
        for s3 in combinations(rest, 2):
            s0 = tuple(t for t in rest if t not in s3)
            worlds.append({2: set(s2), 3: set(s3), 0: set(s0), 1: set(hands[1])})
    assert len(worlds) == 90
    true_world = {s: set(hands[s]) for s in range(4)}
    assert any(all(w[s] == true_world[s] for s in range(4)) for w in worlds)
    print(f"fiber enumerated: {len(worlds)} worlds (true world present)")

    leads = sorted(hands[1])

    # ---------- Part A
    print("\n== Part A: fixed-field lines (uniform belief, uniform-random field) ==")
    lines = {}
    for ld in leads:
        a_tot, b_tot = F(0), F(0)
        for w in worlds:
            a, b = field_expect(w, 1, ld)
            a_tot += a / 90
            b_tot += b / 90
        lines[ld] = (a_tot, b_tot)
        print(f"  lead {ld}:  E[Psi](L) = {a_tot} + {b_tot}*L")
    (a1, b1), (a2, b2) = lines[leads[0]], lines[leads[1]]
    if b1 != b2:
        lam = F(a1 - a2, b2 - b1)
        print(f"  line crossing at L = {lam}" + (
            "  (in domain L>=0)" if lam >= 0 else "  (outside L>=0: no switch)"))
    else:
        print("  parallel lines: no crossing")
    env = pwl_max([line(a, b) for a, b in lines.values()])
    print(f"  upper envelope: {pwl_str(env)}")
    for ld in leads:
        av, bv = lines[ld]
        print(f"  action {ld} optimal on: ", end="")
        segs = [seg for seg in env if seg[1] == bv and seg[2] == av]
        print("never" if not segs else ", ".join(
            f"[{s[0]}, {'inf' if i+1 >= len(env) else env[env.index(s)+1][0]})"
            for i, s in enumerate(segs)))

    # ---------- Part B
    print("\n== Part B: per-world parametric minimax over 90 worlds ==")
    sigs = {}
    zero_sigs = {}
    world_info = []
    for w in worlds:
        qs = {ld: minimax_pwl(w, 1, ld) for ld in leads}
        sig = tuple((ld, tuple(qs[ld])) for ld in leads)
        z = tuple((ld, _eval(qs[ld], F(0))) for ld in leads)
        sigs.setdefault(sig, []).append(w)
        zero_sigs.setdefault(z, []).append(w)
        holder_d = next(s for s in (0, 2, 3) if D_TILE in w[s])
        world_info.append((w, qs, holder_d))
    print(f"  distinct response classes at L=0 (pair of Q-values): {len(zero_sigs)}")
    print(f"  distinct parametric response classes (pair of Q-curves, L>=0): {len(sigs)}")
    print(f"  -> classes created by valuing one tile: {len(sigs) - len(zero_sigs)} new splits")

    print("\n  class census (parametric):")
    for i, (sig, ws) in enumerate(sorted(sigs.items(), key=lambda kv: -len(kv[1]))):
        holders = {}
        for w in ws:
            h = next(s for s in (0, 2, 3) if D_TILE in w[s])
            holders[h] = holders.get(h, 0) + 1
        desc = "; ".join(f"Q({ld})={pwl_str(list(q))}" for ld, q in sig)
        print(f"   class {i}: {len(ws)} worlds; 4-1 holder counts {holders}")
        print(f"     {desc}")

    # argmax structure per class: where does the optimal action change with L?
    print("\n  optimal-action structure (ties and switches):")
    switch_worlds = 0
    for sig, ws in sorted(sigs.items(), key=lambda kv: -len(kv[1])):
        q = {ld: list(qv) for ld, qv in sig}
        v0 = {ld: _eval(q[ld], F(0)) for ld in leads}
        v1 = {ld: _eval(q[ld], F(1)) for ld in leads}
        best0 = {ld for ld in leads if v0[ld] == max(v0.values())}
        best1 = {ld for ld in leads if v1[ld] == max(v1.values())}
        note = ""
        if best0 != best1:
            note = f"  ARGMAX CHANGES: {sorted(best0)} at L=0 -> {sorted(best1)} for L>0"
            switch_worlds += len(ws)
        print(f"   {len(ws):3d} worlds: best@0={sorted(best0)} best@L>0={sorted(best1)}{note}")
    print(f"   worlds whose optimal-action set changes somewhere on L>=0: {switch_worlds}/90")

    # question 4/5: which role-fact sets predict the parametric signature?
    print("\n  role-fact audit (doc Sec. 14.3 q4-q5):")
    def audit(fact_tiles, label):
        cells = {}
        for w, qs, hd in world_info:
            key = tuple(next(s for s in (0, 2, 3) if t in w[s]) for t in fact_tiles)
            sig = tuple((ld, tuple(qs[ld])) for ld in leads)
            cells.setdefault(key, set()).add(sig)
        worst = max(len(ss) for ss in cells.values())
        sound = worst == 1
        print(f"   facts {label}: {len(cells)} cells, worst cell holds "
              f"{worst} signatures -> {'purpose-sound' if sound else 'overshoots'}")
        return sound
    audit([D_TILE], "holder(4-1)")
    audit([D_TILE, T('2-2')], "holder(4-1), holder(2-2)")
    audit([D_TILE, T('2-2'), T('2-0')], "holder(4-1), holder(2-2), holder(2-0)")
    audit([D_TILE, T('2-2'), T('4-2')], "holder(4-1), holder(2-2), holder(4-2)")
    # exhaustive: smallest tile-holder fact set that is purpose-sound
    from itertools import chain
    found = None
    for r in range(1, 7):
        for facts in combinations(unseen, r):
            cells = {}
            ok = True
            for w, qs, hd in world_info:
                key = tuple(next(s for s in (0, 2, 3) if t in w[s]) for t in facts)
                sig = tuple((ld, tuple(qs[ld])) for ld in leads)
                cells.setdefault(key, set()).add(sig)
                if len(cells[key]) > 1:
                    ok = False
                    break
            if ok:
                found = facts
                break
        if found:
            break
    print(f"   smallest purpose-sound holder-fact set: {list(found) if found else 'none below full'}"
          f" (size {len(found) if found else 6})")

    # true-world check
    tw_qs = {ld: minimax_pwl(true_world, 1, ld) for ld in leads}
    print("\n  true world (receipt holders) minimax:")
    for ld in leads:
        print(f"   Q({ld}) = {pwl_str(tw_qs[ld])}")
    print(f"   actual receipt line led {tricks[5][0][0][1]}; "
          f"actual outcome: T1 won both tricks, captured 4-1 (Psi = 2 + L)")

if __name__ == "__main__":
    main()
