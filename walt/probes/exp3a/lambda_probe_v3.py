#!/usr/bin/env python3
"""
lambda_probe_v3.py -- v0.31 integration-doc experiments (Sec. 16.14 / 16.15),
extending lambda_probe.py (v0.1 Sec. 14) and lambda_probe_v2.py (v0.2 Sec. 16).
Exploratory probe tier: stdlib only, exact Fractions/integers, no floats.

Part 0  -- independent verification of every table v0.31 DERIVED from the
           reported v0.1/v0.2 data (class census with holder splits, action-
           correspondence census, holder-ladder audit, entropy bound, feature-
           to-line conversions, per-master capture-differential vectors,
           control-vs-location counterexample counts).
Part 1  -- Experiment 3A (Sec. 16.14): counterexample-guided descriptor
           synthesis on the existing 90-world corpus against the 8-class
           parametric root-Q target and the 3-class action-correspondence
           target, over a registered vocabulary of mechanical and local-
           continuation relations.
Part 2  -- Experiment 3B (Sec. 16.15): horizon 3 (trick-5 start of receipt
           hand 0; S1 leads holding the last live trump; 1680-world fiber).
           Per-world parametric minimax for every root lead and every live
           valued-tile direction, hunting genuine interior breakpoints
           (lambda* > 0); plus the fixed-field information-set solve, whose
           per-action Q is a sum of per-info-set convex envelopes (so its
           breakpoints are exposed-vertex changes of the action polytope).
"""
import sys, os
from fractions import Fraction
from itertools import combinations

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lambda_probe as v1
import lambda_probe_v2 as v2

F = Fraction


# ---------------------------------------------------------------- PWL helpers

def pwl_add_line(f, slope, intercept):
    """Add the line intercept + slope*L to every segment of PWL f."""
    return [(xs, sl + slope, ic + intercept) for xs, sl, ic in f]


def pwl_eval(f, x):
    return v1._eval(f, x)


def pwl_segments_on(f):
    """Number of distinct affine pieces on [0, inf)."""
    return len(f)


def interior_breaks(f):
    """Breakpoints strictly inside (0, inf)."""
    return [xs for xs, _, _ in f if xs > 0]


def side_points(f_breaks, lam):
    """Safe evaluation points just left/right of breakpoint lam, guaranteed to
    stay inside the adjacent segments even when breakpoints are close."""
    below = [x for x in f_breaks if x < lam]
    above = [x for x in f_breaks if x > lam]
    lo = (max(below) + lam) / 2 if below else lam / 2
    hi = (lam + min(above)) / 2 if above else lam + 1
    return lo, hi


# =========================================================================
# Part 0 -- verify the v0.31-derived tables on the 90-world corpus
# =========================================================================

def part0(tricks):
    print("=" * 72)
    print("Part 0: independent verification of v0.31-derived tables")
    print("=" * 72)

    hands, leader = v1.endgame_state(tricks)
    unseen = sorted(hands[0] | hands[2] | hands[3])
    leads = sorted(hands[1])
    live8 = frozenset(hands[0] | hands[1] | hands[2] | hands[3])
    worlds = []
    for s2 in combinations(unseen, 2):
        rest = [t for t in unseen if t not in s2]
        for s3 in combinations(rest, 2):
            s0 = tuple(t for t in rest if t not in s3)
            worlds.append({2: set(s2), 3: set(s3), 0: set(s0), 1: set(hands[1])})
    assert len(worlds) == 90

    # per-world minimax lines (each Q is a single line in this domain)
    info = []
    for w in worlds:
        qs = {ld: v1.minimax_pwl(w, 1, ld) for ld in leads}
        for ld in leads:
            assert len(qs[ld]) == 1  # doc: no interior breakpoints at 2 tricks
        lines = {ld: (qs[ld][0][2], qs[ld][0][1]) for ld in leads}  # (a, b)
        holder = next(s for s in (0, 2, 3) if v1.D_TILE in w[s])
        info.append((w, lines, holder))

    # ---- class census: sizes, lines, holder splits (doc Sec. 16.5 table)
    census = {}
    for w, lines, holder in info:
        key = (lines[leads[0]], lines[leads[1]])
        census.setdefault(key, []).append(holder)
    got = sorted(((len(hs), key, tuple(hs.count(s) for s in (0, 2, 3)))
                  for key, hs in census.items()), reverse=True)
    L = lambda a, b: (F(a), F(b))  # (intercept, slope)
    expected = [
        (26, (L(0, -1), L(-2, -1)), (10, 10, 6)),
        (22, (L(0, 1), L(-2, -1)), (2, 2, 18)),
        (16, (L(2, 1), L(2, 1)), (8, 8, 0)),
        (12, (L(2, 1), L(0, -1)), (3, 3, 6)),
        (8, (L(0, -1), L(0, 1)), (4, 4, 0)),
        (2, (L(0, -1), L(0, -1)), (1, 1, 0)),
        (2, (L(0, 1), L(0, 1)), (1, 1, 0)),
        (2, (L(2, 1), L(0, 1)), (1, 1, 0)),
    ]
    assert sorted(got) == sorted(expected), got
    print("  class census (sizes, both lines, holder splits S0/S2/S3):"
          " MATCHES doc Sec. 16.5 table exactly")
    for n, (l1, l2), hs in got:
        print(f"    {n:2d} worlds  Q({leads[0]})={l1[0]}+{l1[1]}L "
              f"Q({leads[1]})={l2[0]}+{l2[1]}L  holders {hs}")

    # ---- baseline root-Q vectors (4 classes)
    basevecs = {(lines[leads[0]][0], lines[leads[1]][0]) for _, lines, _ in info}
    assert basevecs == {(F(0), F(-2)), (F(2), F(2)), (F(2), F(0)), (F(0), F(0))}
    print("  baseline root-Q vectors: {(0,-2),(2,2),(2,0),(0,0)} -- 4 classes OK")

    # ---- action-correspondence census (2 -> 3)
    corr = {"strict00": 0, "tie_all": 0, "tie0_then21": 0}
    for _, lines, _ in info:
        (a1, b1), (a2, b2) = lines[leads[0]], lines[leads[1]]
        if a1 == a2 and b1 == b2:
            corr["tie_all"] += 1
        elif a1 == a2 and b2 > b1:
            corr["tie0_then21"] += 1
        elif a1 > a2 or (a1 == a2 and b1 > b2):
            # 0-0 weakly better at 0; check strictness over the whole ray
            assert a1 > a2 or b1 > b2
            corr["strict00"] += 1
        else:
            raise AssertionError((a1, b1, a2, b2))
    assert corr == {"strict00": 62, "tie_all": 20, "tie0_then21": 8}, corr
    base_classes = {("tie" if a1 == a2 else "strict") for (_, lines, _) in info
                    for (a1, _), (a2, _) in [(lines[leads[0]], lines[leads[1]])]}
    assert base_classes == {"tie", "strict"}
    print(f"  action correspondence: 62 strict 0-0 / 20 tie always / 8 tie-at-0"
          f" then 2-1  (2 classes at L=0 -> 3 parametric) OK")

    # ---- holder-ladder audit (doc Sec. 16.7 table)
    def cells_worst(fact_tiles):
        cells = {}
        for w, lines, _ in info:
            key = tuple(next(s for s in (0, 2, 3) if t in w[s]) for t in fact_tiles)
            sig = (lines[leads[0]], lines[leads[1]])
            cells.setdefault(key, set()).add(sig)
        return len(cells), max(len(s) for s in cells.values())

    ladder = [
        ([v1.T("4-1")], (3, 8)),
        ([v1.T("4-1"), v1.T("2-2")], (9, 4)),
        ([v1.T("4-1"), v1.T("2-2"), v1.T("2-0")], (24, 3)),
        ([v1.T("4-1"), v1.T("2-2"), v1.T("4-2")], (24, 3)),
    ]
    for tiles, exp in ladder:
        gotcw = cells_worst(tiles)
        assert gotcw == exp, (tiles, gotcw, exp)
    print("  holder ladder: (3 cells, worst 8) -> (9, 4) -> (24, 3) -> (24, 3)"
          "  MATCHES doc Sec. 16.7")

    # smallest purpose-sound holder set is 5 of 6 (re-verified)
    found = None
    for r in range(1, 7):
        for facts in combinations(unseen, r):
            n, worst = cells_worst(list(facts))
            if worst == 1:
                found = facts
                break
        if found:
            break
    assert found is not None and len(found) == 5
    print(f"  smallest purpose-sound holder set: size 5 (e.g. {list(found)}) OK")

    # ---- entropy bound, exact integer arithmetic only (doc: ~2.52 bits)
    sizes = [n for n, _, _ in got]
    num = 90 ** 90
    den = 1
    for n in sizes:
        den *= n ** n
    # H = log2(num/den)/90; bracket via bit_length of the exact ratio scaled
    # 2^(90H) = num/den. Use integer scaling to get tight bounds.
    scale = 2 ** 200
    q = (num * scale) // den  # floor(2^200 * 2^(90H))
    b = q.bit_length()  # 2^(b-1) <= q < 2^b
    lo_num, hi_num = b - 1 - 200, b - 200  # 90H in [lo_num, hi_num)
    assert lo_num <= 227 and hi_num >= 226
    print(f"  class-size entropy: 90*H in [{lo_num}, {hi_num}] bits exactly ->"
          f" H in [{lo_num}/90, {hi_num}/90] ~ [2.51, 2.53]; doc's 2.52 OK"
          f" (exact integer bracket, no floats)")

    # ---- feature-to-line conversion identities (doc Sec. 16.4)
    assert 2 * (F(13, 3) - 3) - 2 == F(2, 3)
    assert 2 * F(3, 5) - 1 == F(1, 5)
    assert 2 * (F(11, 3) - 3) - 2 == F(-2, 3)
    assert 2 * F(1, 3) - 1 == F(-1, 3)
    print("  feature-to-line conversions (Sec. 16.4): all four identities OK")

    # ---- per-master capture-differential vectors (doc Sec. 16.12)
    agg = {}
    for ld in leads:
        acc = {}
        for w in worlds:
            for k, p in v2.field_law(w, 1, ld).items():
                acc[k] = acc.get(k, F(0)) + p / 90
        agg[ld] = acc
    M0 = sorted(v2.masters(live8))
    g00 = tuple(v2.law_slope(agg[leads[0]], live8, m) for m in M0)
    g21 = tuple(v2.law_slope(agg[leads[1]], live8, m) for m in M0)
    assert g00 == (F(1), F(1, 5), F(1, 5), F(1, 5)), g00
    assert g21 == (F(-1, 3),) * 4, g21
    print(f"  per-master capture-differential vectors: g(0-0)={g00},"
          f" g(2-1)={g21}  MATCH doc Sec. 16.12")

    # ---- control vs location counterexample counts (doc Sec. 16.7 / 12.12)
    c0_s3 = sum(1 for w, lines, h in info
                if lines[leads[0]] == L(0, -1) and lines[leads[1]] == L(-2, -1)
                and h == 3)
    c1_opp = sum(1 for w, lines, h in info
                 if lines[leads[0]] == L(0, 1) and lines[leads[1]] == L(-2, -1)
                 and h in (0, 2))
    assert c0_s3 == 6 and c1_opp == 4
    print(f"  control != location: class-0 has {c0_s3} worlds where partner"
          f" holds 4-1 yet opponents capture it; class-1 has {c1_opp} worlds"
          f" where an opponent holds it yet T1 captures it  OK")

    return worlds, info, leads, live8, unseen


# =========================================================================
# Part 1 -- Experiment 3A: descriptor synthesis on the 90-world corpus
# =========================================================================

SUIT2_RANK = {v1.T("2-2"): 100, v1.T("5-2"): 5, v1.T("4-2"): 4,
              v1.T("2-1"): 1, v1.T("2-0"): 0}


def build_atoms(unseen):
    """Registered candidate vocabulary (doc Sec. 16.14): mechanical and
    local-continuation relations, each a function world -> small value."""
    T = v1.T
    atoms = {}

    def holder(w, t):
        return next(s for s in (0, 2, 3) if t in w[s])

    # location atoms (the baseline the doc says is nearly maximally inefficient)
    for t in unseen:
        name = f"h({t[0]}-{t[1]})"
        atoms[name] = (lambda w, t=t: holder(w, t))
    for t in unseen:
        name = f"team({t[0]}-{t[1]})"
        atoms[name] = (lambda w, t=t: 1 if holder(w, t) == 3 else 0)

    # companion of 4-1's holder (which tile shares that hand)
    def companion41(w):
        s = holder(w, T("4-1"))
        others = [t for t in w[s] if t != T("4-1")]
        return others[0]
    atoms["comp41"] = companion41
    atoms["comp41_in_suit2"] = lambda w: 1 if companion41(w) in SUIT2_RANK else 0
    atoms["comp41_is_2-0"] = lambda w: 1 if companion41(w) == T("2-0") else 0

    # suit-2 strength relations (trick 7 after a 0-0 lead is a forced suit-2
    # trick led by 2-1; trick 6 after a 2-1 lead is a suit-2 trick)
    def max2(w, s):
        vals = [SUIT2_RANK[t] for t in w[s] if t in SUIT2_RANK]
        return max(vals) if vals else -1
    atoms["s3max2"] = lambda w: max2(w, 3)
    atoms["oppmax2"] = lambda w: max(max2(w, 0), max2(w, 2))
    atoms["s3_top2"] = lambda w: 1 if max2(w, 3) > max(max2(w, 0), max2(w, 2)) else 0
    atoms["nbeat_opp"] = lambda w: sum(
        1 for u in (T("2-2"), T("5-2"), T("4-2")) for s in (0, 2) if u in w[s])

    # continuation motif: winner of the forced trick 7 (led 2-1) when every
    # hidden seat keeps its best suit-2 tile through trick 6
    def t7_bestkeep_team(w):
        best_rank, best_seat = 1, 1  # S1 leads 2-1 (rank 1)
        for s in (2, 3, 0):
            r = max2(w, s)
            if r > best_rank:
                best_rank, best_seat = r, s
        return v1.TEAM[best_seat]
    atoms["t7w_bestkeep"] = t7_bestkeep_team

    # can 4-1's holder win trick 7 with the kept companion?
    def comp41_rank2(w):
        c = companion41(w)
        return SUIT2_RANK.get(c, -1)
    atoms["comp41_rank2"] = comp41_rank2

    # does 4-1 sit with the suit-2 boss?
    atoms["41_with_22"] = lambda w: 1 if holder(w, T("4-1")) == holder(w, T("2-2")) else 0
    return atoms


def descriptor_partition(worlds, atom_fns):
    cells = {}
    for i, w in enumerate(worlds):
        key = tuple(fn(w) for fn in atom_fns)
        cells.setdefault(key, []).append(i)
    return cells


def is_sound(cells, target):
    for idxs in cells.values():
        if len({target[i] for i in idxs}) > 1:
            return False
    return True


def first_counterexample(cells, target):
    for idxs in cells.values():
        seen = {}
        for i in idxs:
            if target[i] in seen:
                continue
            for j, tj in seen.items():
                if tj != target[i]:
                    return (j, i)
            seen[target[i]] = i
    return None


def part1(worlds, info, leads):
    print()
    print("=" * 72)
    print("Part 1: Experiment 3A -- mine the existing compression (Sec. 16.14)")
    print("=" * 72)

    unseen = sorted(set().union(*(w[s] for w in worlds[:1] for s in (0, 2, 3))))
    atoms = build_atoms(unseen)
    names = sorted(atoms)
    print(f"  registered atom vocabulary ({len(names)}):")
    print("    " + ", ".join(names))

    # targets
    sig8 = {}
    target8 = []
    for w, lines, _ in info:
        key = (lines[leads[0]], lines[leads[1]])
        sig8.setdefault(key, len(sig8))
        target8.append(sig8[key])
    target3 = []
    for w, lines, _ in info:
        (a1, b1), (a2, b2) = lines[leads[0]], lines[leads[1]]
        if (a1, b1) == (a2, b2):
            target3.append(0)
        elif a1 == a2 and b2 > b1:
            target3.append(1)
        else:
            target3.append(2)
    print(f"  targets: R8 = 8-class parametric root-Q signature;"
          f" R3 = 3-class action correspondence")
    print("  note: with single-line Q's the first-order jet target equals R8")

    # precompute atom value vectors
    vals = {n: [atoms[n](w) for w, _, _ in info] for n in names}

    def cells_of(subset):
        cells = {}
        for i in range(len(info)):
            key = tuple(vals[n][i] for n in subset)
            cells.setdefault(key, []).append(i)
        return cells

    for label, target in [("R8", target8), ("R3", target3)]:
        print(f"\n  -- exhaustive minimum-size search, target {label} --")
        best = None
        for r in range(1, 5):
            hits = []
            for subset in combinations(names, r):
                if is_sound(cells_of(subset), target):
                    hits.append(subset)
            if hits:
                best = (r, hits)
                break
        if best:
            r, hits = best
            print(f"    smallest sound descriptor size: {r} atom(s);"
                  f" {len(hits)} minimal solution(s)")
            for h in hits[:8]:
                cells = cells_of(h)
                print(f"      {list(h)}  ({len(cells)} cells)")
            if len(hits) > 8:
                print(f"      ... and {len(hits) - 8} more")
        else:
            print("    NO sound descriptor of size <= 4 in this vocabulary")
            # report the best size-4 candidates and their counterexamples
            scored = []
            for subset in combinations(names, 4):
                cells = cells_of(subset)
                worst = max(len({target[i] for i in idxs})
                            for idxs in cells.values())
                bad = sum(1 for idxs in cells.values()
                          if len({target[i] for i in idxs}) > 1)
                scored.append((worst, bad, subset))
            scored.sort()
            worst, bad, subset = scored[0]
            print(f"    best size-4 candidate {list(subset)}:"
                  f" worst cell holds {worst} classes, {bad} impure cells")
            cex = first_counterexample(cells_of(subset), target)
            if cex:
                i, j = cex
                wi, wj = info[i][0], info[j][0]
                print(f"    counterexample pair: worlds {i} and {j}")
                for tag, w in (("A", wi), ("B", wj)):
                    print(f"      {tag}: S2={sorted(w[2])} S3={sorted(w[3])}"
                          f" S0={sorted(w[0])}  class {target[i] if tag=='A' else target[j]}")

    # greedy counterexample-guided loop for R8, starting from the natural
    # first guess (team of the valued tile's holder)
    print("\n  -- counterexample-guided refinement loop (target R8) --")
    chosen = ["team(4-1)"]
    step = 0
    while True:
        cells = cells_of(tuple(chosen))
        cex = first_counterexample(cells, target8)
        if cex is None:
            print(f"    SOUND after {step} refinements: {chosen}"
                  f" ({len(cells)} cells for 8 classes)")
            break
        i, j = cex
        wi, wj = info[i][0], info[j][0]
        seps = [n for n in names if n not in chosen and vals[n][i] != vals[n][j]]
        # pick the separating atom that minimizes total impurity when added
        best_n, best_imp = None, None
        for n in seps:
            c2 = cells_of(tuple(chosen + [n]))
            imp = sum(len({target8[k] for k in idxs}) - 1
                      for idxs in c2.values())
            if best_imp is None or imp < best_imp:
                best_n, best_imp = n, imp
        step += 1
        print(f"    step {step}: worlds {i}/{j} share descriptor, classes"
              f" {target8[i]} vs {target8[j]};"
              f" separating atoms {seps[:6]}{'...' if len(seps) > 6 else ''};"
              f" add '{best_n}'")
        chosen.append(best_n)
        if step > 8:
            print("    (loop cap reached)")
            break
    return


# =========================================================================
# Part 2 -- Experiment 3B: horizon 3 (trick-5 start), breakpoint hunt
# =========================================================================

def trick5_state(tricks):
    hands = {s: set() for s in range(4)}
    for plays, _, _ in tricks:
        for s, t in plays:
            hands[s].add(t)
    leader = tricks[0][0][0][0]
    for plays, w, _ in tricks[:4]:
        for s, t in plays:
            hands[s].discard(t)
        leader = w
    return hands, leader


def voids_after4(tricks):
    """(seat, suit) voids observable from tricks 1-4."""
    hands = {s: set() for s in range(4)}
    for plays, _, _ in tricks:
        for s, t in plays:
            hands[s].add(t)
    out = set()
    for plays, w, _ in tricks[:4]:
        led = v1.led_suit(plays[0][1])
        for s, t in plays:
            if s != plays[0][0] and not v1.in_suit(t, led):
                out.add((s, led))
            hands[s].discard(t)
    return out


# ---- per-world parametric minimax over 3 tricks, valued tile d, with cache

CACHE = {}


def mm3(hands, leader, d, dcap):
    """PWL (on L >= 0) of future (trick diff + L * capture sign of d) under
    perfect-information minimax from a trick boundary. dcap: True if d already
    captured (then slope contribution is settled upstream)."""
    key = (tuple(tuple(sorted(hands[s])) for s in range(4)), leader,
           None if dcap else d)
    if key in CACHE:
        return CACHE[key]
    n = len(hands[leader])
    if n == 0:
        res = [(F(0), F(0), F(0))]
        CACHE[key] = res
        return res

    def rec(hs, order, plays, idx):
        if len(plays) == 4:
            w = v1.trick_winner(plays)
            td = v1.team_sign(w)
            trick_tiles = {t for _, t in plays}
            cap_now = (not dcap) and d in trick_tiles
            sub = mm3(hs, w, d, dcap or cap_now)
            f = pwl_add_line(sub, F(0), F(td))
            if cap_now:
                f = pwl_add_line(f, F(1) if v1.TEAM[w] == 1 else F(-1), F(0))
            return f
        seat = order[idx]
        led = v1.led_suit(plays[0][1])
        opts = v1.legal(hs[seat], led)
        vals = []
        for t in opts:
            h2 = {s: set(x) for s, x in hs.items()}
            h2[seat].discard(t)
            vals.append(rec(h2, order, plays + [(seat, t)], idx + 1))
        return v1.pwl_max(vals) if v1.TEAM[seat] == 1 else v1.pwl_min(vals)

    order = [(leader + k) % 4 for k in range(4)]
    opts = list(hands[leader])
    vals = []
    for t in opts:
        h2 = {s: set(x) for s, x in hands.items()}
        h2[leader].discard(t)
        vals.append(rec(h2, order, [(leader, t)], 1))
    res = v1.pwl_max(vals) if v1.TEAM[leader] == 1 else v1.pwl_min(vals)
    CACHE[key] = res
    return res


def mm3_root(world, leader, lead_tile, d):
    """Q(lead_tile; L) for one world and valued tile d."""
    h = {s: set(world[s]) for s in range(4)}
    h[leader].discard(lead_tile)
    order = [(leader + k) % 4 for k in range(4)]

    def rec(hs, plays, idx):
        if len(plays) == 4:
            w = v1.trick_winner(plays)
            td = v1.team_sign(w)
            trick_tiles = {t for _, t in plays}
            cap_now = d in trick_tiles
            sub = mm3(hs, w, d, cap_now)
            f = pwl_add_line(sub, F(0), F(td))
            if cap_now:
                f = pwl_add_line(f, F(1) if v1.TEAM[w] == 1 else F(-1), F(0))
            return f
        seat = order[idx]
        led = v1.led_suit(plays[0][1])
        opts = v1.legal(hs[seat], led)
        vals = []
        for t in opts:
            h2 = {s: set(x) for s, x in hs.items()}
            h2[seat].discard(t)
            vals.append(rec(h2, plays + [(seat, t)], idx + 1))
        return v1.pwl_max(vals) if v1.TEAM[seat] == 1 else v1.pwl_min(vals)

    return rec(h, [(leader, lead_tile)], 1)


# ---- fixed-field information-set solve at horizon 3

WSCALE = 6 ** 6  # integer weight per world; each field choice divides by <= 3


def fixed_field_h3(worlds, leader, hands1, live12):
    """Enumerate (world, field randomness) forward with exact integer weights.
    S1's decisions: root lead (3 options) and its single trick-6 action.
    Returns, per root lead: dict infoset -> {s1_option: [wsum, a_sum, capmass]}
    where wsum = sum of integer path weights, a_sum = sum of weight * trick
    differential, capmass[t] = sum of weights on paths where T1 captures t.
    True probability = weight / (len(worlds) * WSCALE)."""
    out = {}
    for root in sorted(hands1):
        groups = {}

        def record(infoset, opt, wgt, td, cap1):
            cell = groups.setdefault(infoset, {}).setdefault(
                opt, [0, 0, {}])
            cell[0] += wgt
            cell[1] += wgt * td
            for t in cap1:
                cell[2][t] = cell[2].get(t, 0) + wgt

        def finish_forced(hs, ldr, td, cap1, wgt, infoset, opt):
            order = [(ldr + k) % 4 for k in range(4)]
            plays = [(s, next(iter(hs[s]))) for s in order]
            w = v1.trick_winner(plays)
            td2 = td + v1.team_sign(w)
            c2 = cap1 | {t for _, t in plays} if v1.TEAM[w] == 1 else cap1
            record(infoset, opt, wgt, td2, c2)

        def trick6(hs, ldr, td, cap1, wgt, t5pub):
            order = [(ldr + k) % 4 for k in range(4)]

            def rec6(hs6, plays, idx, wg, infoset, opt):
                if len(plays) == 4:
                    w = v1.trick_winner(plays)
                    td6 = td + v1.team_sign(w)
                    c6 = cap1 | {t for _, t in plays} \
                        if v1.TEAM[w] == 1 else cap1
                    finish_forced(hs6, w, td6, c6, wg, infoset, opt)
                    return
                seat = order[idx]
                led = v1.led_suit(plays[0][1]) if plays else None
                opts = v1.legal(hs6[seat], led)
                if seat == 1:
                    pre = tuple(plays)
                    for t in opts:
                        h2 = {s: set(x) for s, x in hs6.items()}
                        h2[1].discard(t)
                        rec6(h2, plays + [(seat, t)], idx + 1, wg,
                             (root, t5pub, pre), t)
                else:
                    for t in opts:
                        h2 = {s: set(x) for s, x in hs6.items()}
                        h2[seat].discard(t)
                        rec6(h2, plays + [(seat, t)], idx + 1,
                             wg // len(opts), infoset, opt)

            rec6(hs, [], 0, wgt, None, None)

        for world in worlds:
            h = {s: set(world[s]) for s in range(4)}
            h[1] = set(hands1)
            h[1].discard(root)
            order5 = [(leader + k) % 4 for k in range(4)]

            def rec5(hs5, plays, idx, wg):
                if len(plays) == 4:
                    w = v1.trick_winner(plays)
                    td5 = v1.team_sign(w)
                    c5 = {t for _, t in plays} if v1.TEAM[w] == 1 else set()
                    trick6(hs5, w, td5, c5, wg, tuple(plays))
                    return
                seat = order5[idx]
                led = v1.led_suit(plays[0][1])
                opts = v1.legal(hs5[seat], led)
                for t in opts:
                    h2 = {s: set(x) for s, x in hs5.items()}
                    h2[seat].discard(t)
                    rec5(h2, plays + [(seat, t)], idx + 1, wg // len(opts))

            rec5(h, [(leader, root)], 1, WSCALE)
        out[root] = groups
    return out


def part2(tricks):
    print()
    print("=" * 72)
    print("Part 2: Experiment 3B -- horizon 3 interior-breakpoint hunt"
          " (Sec. 16.15)")
    print("=" * 72)

    hands, leader = trick5_state(tricks)
    assert leader == 1, leader
    s1_hand = sorted(hands[1])
    unseen9 = sorted(hands[0] | hands[2] | hands[3])
    live12 = frozenset(hands[0] | hands[1] | hands[2] | hands[3])
    print(f"  trick-5 start: S1 leads holding {s1_hand}; unseen: {unseen9}")
    assert v1.T("3-2") in hands[1]
    assert not any(v1.is_trump(t) for t in unseen9)
    vs = voids_after4(tricks)
    print(f"  observed voids after tricks 1-4: {sorted(vs)} -- all trump,"
          f" vacuous for the 9 unseen tiles (none contains a 3)")
    masters5 = v2.masters(live12)
    print(f"  absolute masters at trick-5 start: {sorted(masters5)}"
          f"  (the last live trump)")

    worlds = []
    for c2 in combinations(unseen9, 3):
        rest = [t for t in unseen9 if t not in c2]
        for c3 in combinations(rest, 3):
            c0 = tuple(t for t in rest if t not in c3)
            worlds.append({2: set(c2), 3: set(c3), 0: set(c0),
                           1: set(hands[1])})
    assert len(worlds) == 1680
    true_world = {s: set(hands[s]) for s in range(4)}
    assert any(all(w[s] == true_world[s] for s in range(4)) for w in worlds)
    print(f"  |Phi(K)| = {len(worlds)} (voids vacuous; true world present)")

    roots = s1_hand
    dirs = sorted(live12)

    # ---------- per-world parametric minimax census
    print("\n  -- per-world parametric minimax census"
          " (1680 worlds x 3 roots x 12 valued-tile directions) --")
    stats = {}
    examples = {}
    for d in dirs:
        n_seg = 0          # worlds with >= 2 segments in some root Q
        n_cross = 0        # worlds whose argmax switches at some L* > 0
        best_ex = None
        for wi, w in enumerate(worlds):
            qs = {r: mm3_root(w, 1, r, d) for r in roots}
            has_seg = any(len(q) > 1 for q in qs.values())
            if has_seg:
                n_seg += 1
            # argmax switch at positive L: compare best sets at 0 and beyond
            env = v1.pwl_max(list(qs.values()))
            all_breaks = sorted({x for q in qs.values()
                                 for x in interior_breaks(q)}
                                | set(interior_breaks(env)))
            switched = None
            for lam in interior_breaks(env):
                lo_pt, hi_pt = side_points(all_breaks, lam)
                lo_best = {r for r in roots
                           if pwl_eval(qs[r], lo_pt) == pwl_eval(env, lo_pt)}
                hi_best = {r for r in roots
                           if pwl_eval(qs[r], hi_pt) == pwl_eval(env, hi_pt)}
                if hi_best - lo_best:
                    switched = lam
                    break
            if switched is not None:
                n_cross += 1
                if best_ex is None or switched < best_ex[0]:
                    best_ex = (switched, wi, {r: list(qs[r]) for r in roots})
        stats[d] = (n_seg, n_cross)
        if best_ex:
            examples[d] = best_ex
        print(f"    d = {d}: worlds with multi-segment root Q: {n_seg}/1680;"
              f" worlds with interior argmax switch: {n_cross}/1680"
              + (f"; smallest L* = {best_ex[0]}" if best_ex else ""))

    tot_seg = sum(1 for d in dirs if stats[d][0] > 0)
    tot_cross = sum(1 for d in dirs if stats[d][1] > 0)
    print(f"\n  directions with any multi-segment Q: {tot_seg}/12;"
          f" with any interior argmax switch: {tot_cross}/12")
    if examples:
        d, (lam, wi, qs) = min(examples.items(), key=lambda kv: kv[1][0])
        w = worlds[wi]
        print(f"\n  sharpest interior crossing: d = {d}, L* = {lam},"
              f" world S2={sorted(w[2])} S3={sorted(w[3])} S0={sorted(w[0])}")
        for r in roots:
            print(f"    Q({r}) = {v1.pwl_str(qs[r])}")

    # ---------- fixed-field information-set solve
    print("\n  -- fixed-field information-set solve (uniform belief,"
          " uniform-random field) --")
    print("  (S1's remaining decision after the root is exactly its trick-6"
          " play, so Q(root; L) = sum over info-sets of the max over options"
          " -- a convex PWL whose interior breakpoints are exposed-vertex"
          " changes of the action polytope)")
    groups = fixed_field_h3(worlds, 1, hands[1], live12)
    denom = len(worlds) * WSCALE
    for root in roots:
        mass = sum(next(iter(opts.values()))[0]
                   for opts in groups[root].values())
        assert mass == denom, (root, mass, denom)
    n_infosets = {root: len(groups[root]) for root in roots}
    print(f"  info-set counts per root: "
          + ", ".join(f"{r}: {n_infosets[r]}" for r in roots))
    ff_results = {}
    for d in dirs:
        qlines = {}
        for root in roots:
            total = None
            for infoset, opts in groups[root].items():
                seg = []
                for opt, (wsum, a_sum, capmass) in opts.items():
                    a = F(a_sum, denom)
                    b = F(2 * capmass.get(d, 0) - wsum, denom)
                    seg.append(v1.line(a, b))
                best = v1.pwl_max(seg)
                total = best if total is None else _pwl_sum(total, best)
            qlines[root] = total
        ff_results[d] = qlines

    any_vertex = False
    any_cross = False
    for d in dirs:
        qlines = ff_results[d]
        seg_info = {r: pwl_segments_on(qlines[r]) for r in roots}
        env = v1.pwl_max(list(qlines.values()))
        all_breaks = sorted({x for q in qlines.values()
                             for x in interior_breaks(q)}
                            | set(interior_breaks(env)))
        crossings = []
        for lam in interior_breaks(env):
            lo_pt, hi_pt = side_points(all_breaks, lam)
            lo_best = {r for r in roots
                       if pwl_eval(qlines[r], lo_pt) == pwl_eval(env, lo_pt)}
            hi_best = {r for r in roots
                       if pwl_eval(qlines[r], hi_pt) == pwl_eval(env, hi_pt)}
            if hi_best != lo_best:
                crossings.append((lam, sorted(lo_best), sorted(hi_best)))
        multi = {r: n for r, n in seg_info.items() if n > 1}
        if multi or crossings:
            print(f"    d = {d}:")
            for r in roots:
                q = qlines[r]
                note = f"{seg_info[r]} exposed segment(s)"
                if seg_info[r] > 1:
                    any_vertex = True
                    note += f"; breakpoints at {interior_breaks(q)}"
                print(f"      Q({r}) = {v1.pwl_str(q)}   [{note}]")
            for lam, lob, hib in crossings:
                any_cross = True
                print(f"      ROOT SWITCH at L* = {lam}: {lob} -> {hib}")
    if not any_vertex and not any_cross:
        print("    every root Q is a single line for every valued direction;"
              " the fixed-field horizon-3 domain is still degenerate")

    # trick-vs-capture trade at the sharpest fixed-field vertex
    print("\n  -- trick-versus-capture trade at exposed-vertex changes --")
    traded = False
    for d in dirs:
        for root in roots:
            q = ff_results[d][root]
            for lam in interior_breaks(q):
                # feature points on either side: slope = capture differential,
                # intercept = trick component of that exposed policy
                lo, hi = side_points(interior_breaks(q), lam)
                lo_seg = [(sl, ic) for xs, sl, ic in q if xs <= lo][-1]
                hi_seg = [(sl, ic) for xs, sl, ic in q if xs <= hi][-1]
                if lo_seg[0] < hi_seg[0] and lo_seg[1] > hi_seg[1]:
                    traded = True
                    print(f"    d = {d}, root {root}, L* = {lam}: policy trades"
                          f" trick value {lo_seg[1]} -> {hi_seg[1]} for capture"
                          f" slope {lo_seg[0]} -> {hi_seg[0]}"
                          f"  (genuine trick-vs-capture trade)")
    if not traded:
        print("    no exposed-vertex change trades tricks for capture")

    print(f"\n  minimax subgame cache size: {len(CACHE)}")
    return stats, examples, ff_results


def _pwl_sum(f, g):
    """Sum of two PWL functions."""
    xs = sorted(set(v1._breaks(f)) | set(v1._breaks(g)))
    out = []
    for x0 in xs:
        fs = [(sl, ic) for xx, sl, ic in f if xx <= x0][-1]
        gs = [(sl, ic) for xx, sl, ic in g if xx <= x0][-1]
        sl, ic = fs[0] + gs[0], fs[1] + gs[1]
        if out and out[-1][1] == sl and out[-1][2] == ic:
            continue
        out.append((x0, sl, ic))
    return out


# ---------------------------------------------------------------------- main

def main():
    tricks = v1.parse_hand0()
    assert v1.replay_validate(tricks)
    print("receipt replay validated (winners, points, follow-legality)\n")
    worlds, info, leads, live8, unseen = part0(tricks)
    part1(worlds, info, leads)
    part2(tricks)


if __name__ == "__main__":
    main()
