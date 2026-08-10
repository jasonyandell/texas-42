#!/usr/bin/env python3
"""Diagnostics for lambda_probe_v3: (1) independent scalar validation of the
fixed-field info-set solve and the PI minimax; (2) inspect Q(root) for all 12
directions; (3) debug the crossing detector."""
import sys, os
from fractions import Fraction
from itertools import combinations

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lambda_probe as v1
import lambda_probe_v3 as v3

F = Fraction

tricks = v1.parse_hand0()
assert v1.replay_validate(tricks)
hands, leader = v3.trick5_state(tricks)
unseen9 = sorted(hands[0] | hands[2] | hands[3])
live12 = frozenset(hands[0] | hands[1] | hands[2] | hands[3])
worlds = []
for c2 in combinations(unseen9, 3):
    rest = [t for t in unseen9 if t not in c2]
    for c3 in combinations(rest, 3):
        c0 = tuple(t for t in rest if t not in c3)
        worlds.append({2: set(c2), 3: set(c3), 0: set(c0), 1: set(hands[1])})
roots = sorted(hands[1])
dirs = sorted(live12)

# ---------------------------------------------------------------- rebuild ff
groups = v3.fixed_field_h3(worlds, 1, hands[1], live12)
denom = len(worlds) * v3.WSCALE


def q_of(root, d):
    total = None
    for infoset, opts in groups[root].items():
        seg = []
        for opt, (wsum, a_sum, capmass) in opts.items():
            a = F(a_sum, denom)
            b = F(2 * capmass.get(d, 0) - wsum, denom)
            seg.append(v1.line(a, b))
        best = v1.pwl_max(seg)
        total = best if total is None else v3._pwl_sum(total, best)
    return total


print("== fixed-field Q(root; L) for all 12 directions ==")
for d in dirs:
    print(f"  d = {d}:")
    for root in roots:
        print(f"    Q({root}) = {v1.pwl_str(q_of(root, d))}")

# --------------------------------------------------- crossing detector debug
d = (1, 1)
qlines = {r: q_of(r, d) for r in roots}
env = v1.pwl_max(list(qlines.values()))
print(f"\n== d = {d} envelope ==")
print(f"  env = {v1.pwl_str(env)}")
all_breaks = sorted({x for q in qlines.values() for x in v3.interior_breaks(q)}
                    | set(v3.interior_breaks(env)))
print(f"  all_breaks = {all_breaks}")
for lam in v3.interior_breaks(env):
    lo_pt, hi_pt = v3.side_points(all_breaks, lam)
    lo_vals = {r: v3.pwl_eval(qlines[r], lo_pt) for r in roots}
    hi_vals = {r: v3.pwl_eval(qlines[r], hi_pt) for r in roots}
    print(f"  break {lam}: lo_pt={lo_pt} vals={lo_vals} env={v3.pwl_eval(env, lo_pt)}")
    print(f"             hi_pt={hi_pt} vals={hi_vals} env={v3.pwl_eval(env, hi_pt)}")

# ------------------------------- independent scalar fixed-field best response

def scalar_ff(root, d, lam):
    """Independent scalar computation: enumerate S1 observation prefixes;
    S1 plays argmax of conditional expectation at its trick-6 turn."""
    # reuse the group data ONLY through raw path re-enumeration: full rebuild
    # here, structured differently: collect per infoset per option scalar
    # values, then sum max.
    per = {}
    p_world = F(1, len(worlds))

    def finish(hs, ldr, td, cap1, pr, infoset, opt):
        order = [(ldr + k) % 4 for k in range(4)]
        plays = [(s, next(iter(hs[s]))) for s in order]
        w = v1.trick_winner(plays)
        td2 = td + v1.team_sign(w)
        c2 = cap1 | {t for _, t in plays} if v1.TEAM[w] == 1 else cap1
        val = td2 + lam * (1 if d in c2 else -1)
        cell = per.setdefault(infoset, {}).setdefault(opt, [F(0), F(0)])
        cell[0] += pr
        cell[1] += pr * val

    def t6(hs, ldr, td, cap1, pr, t5pub):
        order = [(ldr + k) % 4 for k in range(4)]

        def rec(hs6, plays, idx, p, infoset, opt):
            if len(plays) == 4:
                w = v1.trick_winner(plays)
                td6 = td + v1.team_sign(w)
                c6 = cap1 | {t for _, t in plays} if v1.TEAM[w] == 1 else cap1
                finish(hs6, w, td6, c6, p, infoset, opt)
                return
            seat = order[idx]
            led = v1.led_suit(plays[0][1]) if plays else None
            opts = v1.legal(hs6[seat], led)
            if seat == 1:
                for t in opts:
                    h2 = {s: set(x) for s, x in hs6.items()}
                    h2[1].discard(t)
                    rec(h2, plays + [(seat, t)], idx + 1, p,
                        (t5pub, tuple(plays)), t)
            else:
                for t in opts:
                    h2 = {s: set(x) for s, x in hs6.items()}
                    h2[seat].discard(t)
                    rec(h2, plays + [(seat, t)], idx + 1, p / len(opts),
                        infoset, opt)

        rec(hs, [], 0, pr, None, None)

    for world in worlds:
        h = {s: set(world[s]) for s in range(4)}
        h[1] = set(hands[1])
        h[1].discard(root)
        order5 = [(1 + k) % 4 for k in range(4)]

        def rec5(hs5, plays, idx, p):
            if len(plays) == 4:
                w = v1.trick_winner(plays)
                td5 = v1.team_sign(w)
                c5 = {t for _, t in plays} if v1.TEAM[w] == 1 else set()
                t6(hs5, w, td5, c5, p, tuple(plays))
                return
            seat = order5[idx]
            led = v1.led_suit(plays[0][1])
            opts = v1.legal(hs5[seat], led)
            for t in opts:
                h2 = {s: set(x) for s, x in hs5.items()}
                h2[seat].discard(t)
                rec5(h2, plays + [(seat, t)], idx + 1, p / len(opts))

        rec5(h, [(1, root)], 1, p_world)

    return sum(max(v for _, v in opts.values())
               for opts in per.values())


print("\n== independent scalar validation of fixed-field Q ==")
for (root, d, lam) in [((0, 0), (1, 1), F(0)), ((0, 0), (1, 1), F(3, 10)),
                       ((0, 0), (1, 1), F(2)), ((0, 0), (1, 1), F(6)),
                       ((0, 0), (4, 4), F(10)), ((2, 1), (1, 1), F(1, 2)),
                       ((2, 1), (4, 1), F(5)), ((3, 2), (4, 1), F(1)),
                       ((3, 2), (5, 1), F(9, 2)), ((0, 0), (5, 2), F(1)),
                       ((0, 0), (2, 1), F(3)), ((2, 1), (6, 2), F(2))]:
    got = v3.pwl_eval(q_of(root, d), lam)
    ind = scalar_ff(root, d, lam)
    tag = "OK" if got == ind else "MISMATCH"
    print(f"  root {root}, d {d}, L={lam}: pwl={got}  scalar={ind}  {tag}")

# --------------------------------------- independent scalar PI minimax check
def scalar_minimax(world, leadr, lead_tile, d, lam):
    h = {s: set(world[s]) for s in range(4)}
    h[leadr].discard(lead_tile)

    def val(hs, ldr):
        if not hs[ldr]:
            return F(0)
        order = [(ldr + k) % 4 for k in range(4)]

        def rec(hh, plays, idx):
            if len(plays) == 4:
                w = v1.trick_winner(plays)
                base = F(v1.team_sign(w))
                tt = {t for _, t in plays}
                if d in tt:
                    base += lam * (1 if v1.TEAM[w] == 1 else -1)
                return base + val(hh, w)
            seat = order[idx]
            led = v1.led_suit(plays[0][1])
            opts = v1.legal(hh[seat], led)
            vs = []
            for t in opts:
                h2 = {s: set(x) for s, x in hh.items()}
                h2[seat].discard(t)
                vs.append(rec(h2, plays + [(seat, t)], idx + 1))
            return max(vs) if v1.TEAM[seat] == 1 else min(vs)

        def top(hh, ldq):
            order2 = [(ldq + k) % 4 for k in range(4)]
            vs = []
            for t in hh[ldq]:
                h2 = {s: set(x) for s, x in hh.items()}
                h2[ldq].discard(t)
                vs.append(rec_with(h2, order2, [(ldq, t)]))
            return max(vs) if v1.TEAM[ldq] == 1 else min(vs)

        def rec_with(hh, order2, plays):
            if len(plays) == 4:
                w = v1.trick_winner(plays)
                base = F(v1.team_sign(w))
                tt = {t for _, t in plays}
                if d in tt:
                    base += lam * (1 if v1.TEAM[w] == 1 else -1)
                return base + val(hh, w)
            seat = order2[len(plays)]
            led = v1.led_suit(plays[0][1])
            opts = v1.legal(hh[seat], led)
            vs = []
            for t in opts:
                h2 = {s: set(x) for s, x in hh.items()}
                h2[seat].discard(t)
                vs.append(rec_with(h2, order2, plays + [(seat, t)]))
            return max(vs) if v1.TEAM[seat] == 1 else min(vs)

        return top(hs, ldr)

    order = [(leadr + k) % 4 for k in range(4)]

    def rec0(hh, plays, idx):
        if len(plays) == 4:
            w = v1.trick_winner(plays)
            base = F(v1.team_sign(w))
            tt = {t for _, t in plays}
            if d in tt:
                base += lam * (1 if v1.TEAM[w] == 1 else -1)
            return base + val(hh, w)
        seat = order[idx]
        led = v1.led_suit(plays[0][1])
        opts = v1.legal(hh[seat], led)
        vs = []
        for t in opts:
            h2 = {s: set(x) for s, x in hh.items()}
            h2[seat].discard(t)
            vs.append(rec0(h2, plays + [(seat, t)], idx + 1))
        return max(vs) if v1.TEAM[seat] == 1 else min(vs)

    return rec0(h, [(leadr, lead_tile)], 1)


print("\n== independent scalar validation of PI minimax (spot worlds) ==")
import random  # ordering only; seeded for determinism, no probabilities
random.seed(42)
picks = random.sample(range(len(worlds)), 6)
ok = True
for wi in picks:
    w = worlds[wi]
    for root in roots:
        for d in [(4, 1), (1, 1), (5, 2)]:
            for lam in [F(0), F(1, 2), F(2), F(7)]:
                got = v3.pwl_eval(v3.mm3_root(w, 1, root, d), lam)
                ind = scalar_minimax(w, 1, root, d, lam)
                if got != ind:
                    ok = False
                    print(f"  MISMATCH world {wi} root {root} d {d} L={lam}:"
                          f" pwl={got} scalar={ind}")
print("  all spot checks OK" if ok else "  FAILURES above")
