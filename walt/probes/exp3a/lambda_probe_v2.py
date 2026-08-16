#!/usr/bin/env python3
"""
lambda_probe_v2.py -- v0.2 integration-doc experiment (Sec. 16), extending the
v0.1 run in lambda_probe.py. Same domain (receipt hand 0, last two tricks,
viewer S1, valued tile 4-1, lambda >= 0), same tier (exploratory probe,
stdlib + exact Fractions, no floats).

New in v0.2 / Sec. 16:
  [16.3] holder marginals under the selected belief
  [16.3] Scheme event counts: extensional world count vs witnessed realization
         count, witness-multiplicity distribution, Fix-branch overlap
  [16.4 q8] naive witness-weighting counterfactual
  [16.4 q9] rigid role transport vs fresh re-query through the actual trick-6
         observation, plus exact hindsight anchoring
  [16.3] banked-mode check: Q_full = ev_w(bank) + Q_future identity
  [8.3]  additive gauge invariance check on enumerated terminal outcomes
  [16.4 q10] cone-exposed policy census (degenerate here; reported honestly)
  [16.5] second experiment: role-valued defect (role = absolute master at
         trick-6 start) under five role-selection lenses
"""
import sys, os
from fractions import Fraction
from itertools import combinations

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lambda_probe as v1

F = Fraction


# ------------------------------------------------------------- role machinery

def beats(u, d):
    """Raw beating capability: would u beat d if d were led (2-tile trick)."""
    return v1.trick_winner([(0, d), (1, u)]) == 1


def masters(live):
    """AbsoluteMaster_K over a live set: nothing live beats it when led."""
    return {d for d in sorted(live) if not any(beats(u, d) for u in live if u != d)}


# ------------------------------------------------- terminal law per (world, lead)

def field_law(hands0, leader, lead_tile):
    """Exact terminal law over uniform-random legal field plays for one world:
    {(trick_diff, frozenset(tiles captured by T1)): prob}. Trick 7 forced."""
    out = {}

    def rec(hands, order, plays, prob, tdiff, cap1):
        if len(plays) == 4:
            w = v1.trick_winner(plays)
            td = tdiff + v1.team_sign(w)
            tt = frozenset(t for _, t in plays)
            c1 = cap1 | tt if v1.TEAM[w] == 1 else cap1
            order7 = [(w + k) % 4 for k in range(4)]
            plays7 = [(s, next(iter(hands[s]))) for s in order7]
            w7 = v1.trick_winner(plays7)
            td7 = td + v1.team_sign(w7)
            t7 = frozenset(t for _, t in plays7)
            c1f = c1 | t7 if v1.TEAM[w7] == 1 else c1
            key = (td7, c1f)
            out[key] = out.get(key, F(0)) + prob
            return
        seat = order[len(plays)]
        led = v1.led_suit(plays[0][1])
        opts = v1.legal(hands[seat], led)
        for t in opts:
            h2 = {s: set(x) for s, x in hands.items()}
            h2[seat].discard(t)
            rec(h2, order, plays + [(seat, t)], prob / len(opts), tdiff, cap1)

    h = {s: set(x) for s, x in hands0.items()}
    h[leader].discard(lead_tile)
    order = [(leader + k) % 4 for k in range(4)]
    rec(h, order, [(leader, lead_tile)], F(1), 0, frozenset())
    assert sum(out.values()) == 1
    return out


def law_line(agg, live8, w):
    """Team-differential expected value of additive valuation dict w (tiles ->
    Fraction) plus trick coefficient 1, as (constant, ) -- fully evaluated."""
    tot = F(0)
    for (td, cap1), p in agg.items():
        cap0 = live8 - cap1
        tot += p * (td + sum(w.get(t, F(0)) for t in cap1)
                    - sum(w.get(t, F(0)) for t in cap0))
    return tot


def law_slope(agg, live8, tile):
    """d/d(lambda) of the team differential when only `tile` carries lambda."""
    return sum(p * (1 if tile in cap1 else -1) for (td, cap1), p in agg.items())


def law_const(agg):
    return sum(p * td for (td, cap1), p in agg.items())


# ---------------------------------------------------------------------- main

def main():
    tricks = v1.parse_hand0()
    assert v1.replay_validate(tricks)
    print("receipt replay validated (winners, points, follow-legality)")

    hands, leader = v1.endgame_state(tricks)
    assert leader == 1
    unseen = sorted(hands[0] | hands[2] | hands[3])
    live8 = frozenset(hands[0] | hands[1] | hands[2] | hands[3])
    leads = sorted(hands[1])
    worlds = []
    for s2 in combinations(unseen, 2):
        rest = [t for t in unseen if t not in s2]
        for s3 in combinations(rest, 2):
            s0 = tuple(t for t in rest if t not in s3)
            worlds.append({2: set(s2), 3: set(s3), 0: set(s0), 1: set(hands[1])})
    assert len(worlds) == 90
    true_world = {s: set(hands[s]) for s in range(4)}
    print(f"|Phi(K)| = {len(worlds)}; live carrier = {sorted(live8)}")

    # aggregated terminal law per root action (uniform belief x uniform field)
    agg = {}
    for ld in leads:
        acc = {}
        for w in worlds:
            for k, p in field_law(w, 1, ld).items():
                acc[k] = acc.get(k, F(0)) + p / 90
        agg[ld] = acc
    # cross-check against the v0.1 Part A lines
    for ld in leads:
        a = law_const(agg[ld])
        b = law_slope(agg[ld], live8, v1.D_TILE)
        print(f"  law({ld}): E[td] = {a}, slope(4-1) = {b}  "
              f"[{len(agg[ld])} distinct terminal outcomes]")
    assert law_const(agg[(0, 0)]) == F(2, 3) and law_slope(agg[(0, 0)], live8, v1.D_TILE) == F(1, 5)
    assert law_const(agg[(2, 1)]) == F(-2, 3) and law_slope(agg[(2, 1)], live8, v1.D_TILE) == F(-1, 3)
    print("  cross-check vs v0.1 Part A lines: EXACT MATCH")

    # ---------------- 16.3 holder marginals
    print("\n== holder marginals under uniform belief ==")
    for t in unseen:
        marg = {s: sum(1 for w in worlds if t in w[s]) for s in (0, 2, 3)}
        assert all(v == 30 for v in marg.values())
    print("  every unseen tile: P(S0) = P(S2) = P(S3) = 30/90 = 1/3 (verified)")

    # ---------------- 16.3 Scheme event counts + multiplicity (q7)
    print("\n== Scheme event census: extensional vs witnessed (q7) ==")
    M0 = masters(live8)
    print(f"  F_master  (exists tile role e: Live(e) & AbsoluteMaster(e)):")
    print(f"    witnesses (kernel-level, holder-free): {sorted(M0)}")
    print(f"    extensional worlds 90/90 (rule-certain); witnesses per world = {len(M0)};")
    print(f"    total witnessed realizations = {90 * len(M0)}  (4x the world count)")

    beaters21 = {u for u in live8 if u != (2, 1) and beats(u, (2, 1))}
    print(f"  F_beat21  (exists chair role c in opponents, tile role e:"
          f" Holds(c,e) & Beats(e, 2-1 led)):")
    print(f"    beater tiles of 2-1: {sorted(beaters21)}")
    mult = {}
    for w in worlds:
        m = sum(1 for u in beaters21 for s in (0, 2) if u in w[s])
        mult[id(w)] = m
    hist = {}
    for m in mult.values():
        hist[m] = hist.get(m, 0) + 1
    ext = sum(c for m, c in hist.items() if m >= 1)
    wit = sum(m * c for m, c in hist.items())
    print(f"    multiplicity histogram (witnesses -> worlds): "
          f"{dict(sorted(hist.items()))}")
    print(f"    extensional world count = {ext}/90; total witnessed realizations = {wit}")

    # Fix-branch overlap: (opp holds 2-2) OR (opp holds 5-2)
    ea = sum(1 for w in worlds if any((2, 2) in w[s] for s in (0, 2)))
    eb = sum(1 for w in worlds if any((5, 2) in w[s] for s in (0, 2)))
    eab = sum(1 for w in worlds if any((2, 2) in w[s] for s in (0, 2))
              and any((5, 2) in w[s] for s in (0, 2)))
    print(f"  Fix overlap: |E_a(opp holds 2-2)| = {ea}, |E_b(opp holds 5-2)| = {eb},"
          f" |E_a ^ E_b| = {eab}")
    print(f"    union = {ea + eb - eab} != naive branch sum {ea + eb}"
          f"  (a Fix is a union, not a mixture)")

    # ---------------- 16.4 q8: naive witness weighting
    print("\n== naive witness-weighting counterfactual (q8) ==")
    per_world = {ld: [v1.field_expect(w, 1, ld) for w in worlds] for ld in leads}
    ext_idx = [i for i, w in enumerate(worlds) if mult[id(w)] >= 1]
    wsum = sum(mult[id(w)] for w in worlds)
    for label, weights in [
        ("uniform | Ext(F_beat21)",
         {i: F(1, len(ext_idx)) for i in ext_idx}),
        ("witness-weighted (naive lift)",
         {i: F(mult[id(worlds[i])], wsum) for i in range(90) if mult[id(worlds[i])] > 0}),
    ]:
        out = {}
        for ld in leads:
            a = sum(q * per_world[ld][i][0] for i, q in weights.items())
            b = sum(q * per_world[ld][i][1] for i, q in weights.items())
            out[ld] = (a, b)
        (a1, b1), (a2, b2) = out[leads[0]], out[leads[1]]
        cross = "parallel" if b1 == b2 else F(a1 - a2, b2 - b1)
        print(f"  {label}:")
        for ld in leads:
            print(f"    lead {ld}: {out[ld][0]} + {out[ld][1]}*L")
        print(f"    crossing at L = {cross}")

    # ---------------- 16.4 q9: rigid transport vs fresh re-query
    print("\n== rigid transport vs fresh re-query through actual trick 6 (q9) ==")
    t6_plays = tricks[5][0]
    dead6 = frozenset(t for _, t in t6_plays)
    live_after = live8 - dead6
    Mf = masters(live_after)
    print(f"  observed trick 6: {[(f'S{s}', t) for s, t in t6_plays]}")
    print(f"  masters before: {sorted(M0)}")
    for m in sorted(M0):
        alive = m in live_after
        still = m in Mf
        tag = ("dead -- Live(e) now FALSE" if not alive
               else ("still an absolute master" if still else "alive but mastery LOST"))
        print(f"    rigid transport of witness e = {m}: {tag}")
    print(f"  fresh re-query after trick 6: masters = {sorted(Mf)}")
    switched = sorted(Mf - M0)
    msg = ("succeeds via surviving witnesses" if Mf and not switched
           else f"finds NEW witnesses {switched}")
    print(f"  witness switching: fresh query {msg}; the two operators disagree "
          f"on {sorted((M0 - live_after) | (Mf - M0))}")

    # exact support cut by the trick-6 observation + hindsight anchoring
    led6 = v1.led_suit(t6_plays[0][1])
    surv = []
    for w in worlds:
        ok = True
        for s, t in t6_plays[1:]:
            if t not in w[s] or t not in v1.legal(w[s], led6):
                ok = False
                break
        surv.append(ok)
    n_surv = sum(surv)
    anchored = sum(1 for w, ok in zip(worlds, surv) if ok and v1.D_TILE in w[2])
    print(f"  support cut by trick-6 observation: 90 -> {n_surv} worlds")
    print(f"  hindsight anchor 'role(4-1 holder) = S2' after trick 7: "
          f"{anchored}/{n_surv} surviving worlds (exact backward preimage)")

    # ---------------- 16.3 banked-mode check
    print("\n== banked-mode check (Sec. 16.3) ==")
    bank_t = {0: 0, 1: 0}
    bank_cap = {0: set(), 1: set()}
    for plays, w, _ in tricks[:5]:
        th = v1.TEAM[w]
        bank_t[th] += 1
        bank_cap[th] |= {t for _, t in plays}
    print(f"  banked after 5 tricks: T1 tricks = {bank_t[1]}, T0 tricks = {bank_t[0]};"
          f" T1 count pts = {sum(v1.count_pts(t) for t in bank_cap[1])},"
          f" T0 count pts = {sum(v1.count_pts(t) for t in bank_cap[0])}")
    w_count = {t: F(v1.count_pts(t)) for t in live8}
    for label, wv in [("straight count", w_count),
                      ("lambda*e_{4-1}, lambda=7/2", {v1.D_TILE: F(7, 2)})]:
        ev_bank = (bank_t[1] - bank_t[0]) + \
            sum(wv.get(t, F(0)) for t in bank_cap[1]) - \
            sum(wv.get(t, F(0)) for t in bank_cap[0])
        # wv only values live tiles here except straight count: extend to banked
        if label == "straight count":
            ev_bank = (bank_t[1] - bank_t[0]) + \
                sum(F(v1.count_pts(t)) for t in bank_cap[1]) - \
                sum(F(v1.count_pts(t)) for t in bank_cap[0])
        qf = {ld: law_line(agg[ld], live8, wv) for ld in leads}
        qfull = {}
        for ld in leads:
            tot = F(0)
            for (td, cap1), p in agg[ld].items():
                cap0 = live8 - cap1
                full_t1 = bank_t[1] + (td + 2) // 2
                full_t0 = bank_t[0] + (2 - td) // 2
                c1 = bank_cap[1] | cap1
                c0 = bank_cap[0] | cap0
                if label == "straight count":
                    val = (full_t1 - full_t0) + \
                        sum(F(v1.count_pts(t)) for t in c1) - \
                        sum(F(v1.count_pts(t)) for t in c0)
                else:
                    val = (full_t1 - full_t0) + \
                        sum(wv.get(t, F(0)) for t in c1) - \
                        sum(wv.get(t, F(0)) for t in c0)
                tot += p * val
            qfull[ld] = tot
        ok = all(qfull[ld] == ev_bank + qf[ld] for ld in leads)
        order_f = sorted(leads, key=lambda ld: qf[ld], reverse=True)
        order_full = sorted(leads, key=lambda ld: qfull[ld], reverse=True)
        print(f"  {label}: Q_full = ev_w(bank) + Q_future exactly: {ok};"
              f" action order identical: {order_f == order_full}")
        assert ok and order_f == order_full

    # ---------------- 8.3 gauge invariance on enumerated terminal outcomes
    print("\n== additive gauge check (Sec. 8.3) ==")
    checked = 0
    for ld in leads:
        for (td, cap1), p in agg[ld].items():
            cap0 = live8 - cap1
            t1 = bank_t[1] + (td + 2) // 2
            t0 = bank_t[0] + (2 - td) // 2
            x1 = bank_cap[1] | cap1
            x0 = bank_cap[0] | cap0
            assert len(x1) == 4 * t1 and len(x0) == 4 * t0
            for c in (F(1), F(3), F(-2)):
                base = (t1 - t0) + sum(F(v1.count_pts(t)) for t in x1) \
                    - sum(F(v1.count_pts(t)) for t in x0)
                shifted = (1 - 4 * c) * (t1 - t0) + \
                    sum(F(v1.count_pts(t)) + c for t in x1) - \
                    sum(F(v1.count_pts(t)) + c for t in x0)
                assert base == shifted
            u = F(5, 7)
            assert (t1 - t0) + u * (len(x1) - len(x0)) == (1 + 4 * u) * (t1 - t0)
            checked += 1
    print(f"  conservation (4 tiles per trick) + gauge (b,w) ~ (b-4c, w+c*1)"
          f" verified on {checked} full-hand terminal outcomes; uniform value"
          f" collapse P = (1+4u)*t verified")

    # ---------------- 16.4 q10 + information consistency + feature collapse
    print("\n== policy census (q10) ==")
    print("  focal deterministic contingent policies: trick 7 is forced, so")
    print("  exactly 1 per root action -> full set = 2, one feature point each:")
    for ld in leads:
        et = sum(p * (bank_t[1] + (td + 2) // 2) for (td, c), p in agg[ld].items())
        pc = sum(p for (td, c1), p in agg[ld].items() if v1.D_TILE in c1)
        print(f"    lead {ld}: (E[t_T1], P(T1 captures 4-1)) = ({et}, {pc})")
    print("  cone-exposed for lambda >= 0: 1 of 2 (the 0-0 line); the domain is")
    print("  degenerate for hull structure, as the v0.1 run already showed")
    print("  information consistency: each policy plays one action across all 90")
    print("  worlds by construction (root action fixed, continuation forced)")

    # ---------------- 16.5 role-valued defect
    print("\n== Sec. 16.5: role-valued defect -- role = absolute master at trick-6"
          " start ==")
    print(f"  witness set (all worlds): {sorted(M0)}; lens comparison, trick b=1:")
    slopes = {ld: {m: law_slope(agg[ld], live8, m) for m in sorted(M0)} for ld in leads}
    consts = {ld: law_const(agg[ld]) for ld in leads}
    for ld in leads:
        per = ", ".join(f"{m}: {slopes[ld][m]}" for m in sorted(M0))
        print(f"  per-witness capture slopes, lead {ld}:  {per}")
    lenses = {}
    for ld in leads:
        sl = slopes[ld]
        lenses.setdefault("ALL witnesses carry lambda", {})[ld] = \
            (consts[ld], sum(sl.values()))
        lenses.setdefault("CANONICAL witness (least tile = 0-0)", {})[ld] = \
            (consts[ld], sl[(0, 0)])
        lenses.setdefault("PROBABILISTIC selector (uniform chi)", {})[ld] = \
            (consts[ld], sum(sl.values()) / 4)
        lenses.setdefault("ADVERSARIAL witness (min slope)", {})[ld] = \
            (consts[ld], min(sl.values()))
        alo = sum(p * ((1 if cap1 & M0 else 0) - (1 if (live8 - cap1) & M0 else 0))
                  for (td, cap1), p in agg[ld].items())
        lenses.setdefault("AT-LEAST-ONE witness captured", {})[ld] = (consts[ld], alo)
    for name, lns in lenses.items():
        (a1, b1), (a2, b2) = lns[leads[0]], lns[leads[1]]
        cross = None if b1 == b2 else F(a1 - a2, b2 - b1)
        pref = leads[0] if a1 > a2 or (a1 == a2 and b1 >= b2) else leads[1]
        note = ("parallel" if cross is None else
                f"cross at L = {cross}" + ("  ** IN DOMAIN **" if cross > 0 else " (outside L>=0)"))
        print(f"  {name}:")
        print(f"    Q({leads[0]}) = {a1} + {b1}*L   Q({leads[1]}) = {a2} + {b2}*L   [{note}]")


if __name__ == "__main__":
    main()
