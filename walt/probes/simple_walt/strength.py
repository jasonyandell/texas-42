"""EXPLORATORY: strength on mirrored deals. Does reseeding walt's inner
randomness (which flips ~30% of root moves) cost any strength?

Players (all level-1 walt: 30 own-seat deals, n0=8 modeled level-0 minds, voids
observed at the root, as in walt42.main):
  L1(seed)  inner rng seeded from (seed, deal, position); ROOT deals drawn from a
            stream seeded by (deal, position) alone, so two L1 players with
            different seeds see the same 30 own-seat deals at the same position
            and differ ONLY in inner randomness.
  L0        level 0 at the root: modeled seats play uniformly random legal tiles;
            same root deals; dice seeded from (deal, position).
A pair = one deal (contract: walt42 deal-and-contract logic, bidder at seat 1)
played twice, X in seats 1,3 then X in seats 0,2. Pair score =
made(X declaring) - made(Y declaring).

Matchups: L1a_vs_L1b = L1(1) vs L1(2);  L1_vs_L0 = L1(1) vs L0;
          L1_same = L1(1) vs L1(1) (determinism/mirror control, used by `check`).

Usage:
  python3 strength.py run MATCHUP START COUNT   # deal seeds START..START+COUNT-1, appends strength_MATCHUP.jsonl
  python3 strength.py summarize MATCHUP
  python3 strength.py check [COUNT]             # smoke: seconds/pair + mirror/determinism checks
"""
import json, os, sys, time
from fractions import Fraction
from math import comb, isqrt
from multiprocessing import Pool

from walt42 import DEALS, outcome, play, sample, tiles, turn, value as value0

_argv, sys.argv = sys.argv, sys.argv[:1]  # crn_experiment reads argv at import
from crn_experiment import contract
from seq_experiment import Walt, base_of
sys.argv = _argv

HERE = os.path.dirname(os.path.abspath(__file__))
FIXED8 = {"mode": "fixed", "n0": 8}
MASK = (1 << 40) - 1
MATCHUPS = {"L1a_vs_L1b": (("L1", 1), ("L1", 2)),
            "L1_vs_L0": (("L1", 1), ("L0", 0)),
            "L1_same": (("L1", 1), ("L1", 1))}


class L0Root:
    """Level 0 at the root: best response to uniformly random modeled seats."""

    def __init__(self, rules, root, dice):
        self.rules = rules
        import random
        self.root, self.dice = random.Random(root), random.Random(dice)

    def decide(self, seat, hand, state, voids):
        options = tiles(self.rules.legal(hand, state[2]))
        if len(options) == 1:
            return options[0]
        deals = sample(seat, hand, state, DEALS, self.root, voids)
        made = {t: value0(self.rules, seat, play(self.rules, state, t), deals, 0, self.dice)
                for t in options}
        return (max if seat % 2 else min)(options, key=made.get)


def choose(player, rules, dealseed, seat, hand, state, voids):
    kind, pseed = player
    base = base_of(dealseed, state)  # root stream: position only, same for every player
    inner = hash((pseed, base)) & MASK  # ints only: stable across processes
    if kind == "L1":
        return Walt(rules, FIXED8, base, inner).decide(seat, hand, state, voids)
    return L0Root(rules, base, inner).decide(seat, hand, state, voids)


def game(dealseed, players):
    """players[seat] = (kind, seed). Returns (made, t1, t0, record)."""
    rules, hands = contract(dealseed)
    state, voids, record = (0, 1, (), 0, 0), [0] * 4, []
    while outcome(state) is None:
        seat, trick = turn(state), state[2]
        hand = hands[seat] & ~state[0]
        t = choose(players[seat], rules, dealseed, seat, hand, state, voids)
        if trick and not rules.suit[rules.lead[trick[0]]] >> t & 1:
            voids[seat] |= rules.suit[rules.lead[trick[0]]]  # couldn't follow
        state = play(rules, state, t)
        record.append((seat, t))
    return bool(outcome(state)), state[3], state[4], record, rules


def pair(job):
    matchup, dealseed = job
    X, Y = MATCHUPS[matchup]
    t0 = time.time()
    m1, a1, b1, rec1, rules = game(dealseed, [Y, X, Y, X])  # X declares (seats 1,3)
    m2, a2, b2, rec2, _ = game(dealseed, [X, Y, X, Y])      # Y declares
    return {"seed": dealseed, "trump": trump_of(dealseed), "x_declares_made": m1, "y_declares_made": m2,
            "pair": int(m1) - int(m2), "game1_scores": [a1, b1], "game2_scores": [a2, b2],
            "game1": rec1, "game2": rec2, "seconds": round(time.time() - t0, 2)}


def trump_of(dealseed):
    import random
    from walt42 import TILES
    rng = random.Random(dealseed)
    order = list(range(28))
    rng.shuffle(order)
    hands = [sum(1 << t for t in order[7 * i:7 * i + 7]) for i in range(4)]
    return max((sum(p in TILES[t] for t in tiles(hands[s])), s, p) for s in range(4) for p in range(7))[2]


def path(matchup):
    return os.path.join(HERE, "strength_%s.jsonl" % matchup)


def run(matchup, start, count):
    if matchup not in MATCHUPS or matchup == "L1_same":
        raise SystemExit("matchup must be L1a_vs_L1b or L1_vs_L0")
    t0 = time.time()
    with Pool(4) as pool, open(path(matchup), "a") as f:
        n = 0
        for row in pool.imap_unordered(pair, [(matchup, s) for s in range(start, start + count)]):
            f.write(json.dumps(row) + "\n")
            f.flush()
            n += 1
    print("%s: %d pairs, seeds %d..%d, wall %.1f s (%.1f s/pair on 4 cores)" % (
        matchup, n, start, start + count - 1, time.time() - t0, (time.time() - t0) / max(n, 1)))


# ---- summary (exact arithmetic; no floats near values) ---------------------

def dec(x, places=3):
    x = Fraction(x)
    q = round(abs(x) * 10 ** places)
    return "%s%d.%0*d" % ("-" if x < 0 else "", q // 10 ** places, places, q % 10 ** places)


def sign_test_p(w, l):
    """Exact two-sided binomial p, H0: decisive pairs are 50/50."""
    m = w + l
    if m == 0:
        return Fraction(1)
    tail = sum(comb(m, k) for k in range(min(w, l) + 1))
    return min(Fraction(1), Fraction(2 * tail, 2 ** m))


def summarize(matchup):
    rows = {}
    for line in open(path(matchup)):
        r = json.loads(line)
        rows[r["seed"]] = r  # a re-run seed replaces its earlier row
    rows = list(rows.values())
    n = len(rows)
    xw = sum(r["pair"] == 1 for r in rows)
    yw = sum(r["pair"] == -1 for r in rows)
    ties = n - xw - yw
    total = sum(r["pair"] for r in rows)
    mean = Fraction(total, n)
    var = Fraction(sum((r["pair"] - mean) ** 2 for r in rows), n - 1) if n > 1 else Fraction(0)
    se2 = Fraction(isqrt(int(4 * var / n * 10 ** 12)), 10 ** 6)  # 2*sqrt(var/n), to 6 places
    xm, ym = sum(r["x_declares_made"] for r in rows), sum(r["y_declares_made"] for r in rows)
    print("%s: pairs %d, X wins %d, Y wins %d, ties %d" % (matchup, n, xw, yw, ties))
    print("  pair-score mean (X-Y) %s  2SE +-%s  (95%% interval %s .. %s)" % (
        dec(mean), dec(se2), dec(mean - se2), dec(mean + se2)))
    print("  declaring make rate: X %d/%d = %s, Y %d/%d = %s" % (
        xm, n, dec(Fraction(xm, n)), ym, n, dec(Fraction(ym, n))))
    print("  sign test on decisive pairs (%d vs %d): two-sided exact p = %s" % (
        xw, yw, dec(sign_test_p(xw, yw), 4)))
    print("  mean seconds/pair (one core) %s" % dec(Fraction(round(sum(r["seconds"] for r in rows) * 100), 100 * n), 1))


# ---- smoke ------------------------------------------------------------------

def check(count):
    t0 = time.time()
    with Pool(4) as pool:
        rows = pool.map(pair, [("L1a_vs_L1b", 5000 + i) for i in range(count)])
    wall = time.time() - t0
    print("L1a_vs_L1b smoke: %d pairs in %.1f s wall = %.1f s/pair (4 cores); one-core mean %.1f s" % (
        count, wall, wall / count, sum(r["seconds"] for r in rows) / count))
    for r in rows:
        print("  seed", r["seed"], "trump", r["trump"], "made X/Y", r["x_declares_made"], r["y_declares_made"],
              "pair", r["pair"], "plays", len(r["game1"]), len(r["game2"]),
              "games identical" if r["game1"] == r["game2"] else "games differ", "%.1f s" % r["seconds"])
    # mirror: seed 1 vs seed 1 -> both games must be move-for-move identical
    t0 = time.time()
    same = Pool(4).map(pair, [("L1_same", 5000 + i) for i in range(count)])
    ok = all(r["game1"] == r["game2"] and r["pair"] == 0 and r["game1_scores"] == r["game2_scores"] for r in same)
    print("L1 seed1 vs seed1: %d/%d pairs with game1 == game2 move-for-move, scores equal, pair 0 (%.1f s): %s" % (
        sum(r["game1"] == r["game2"] for r in same), count, time.time() - t0, "MIRROR EXACT" if ok else "MIRROR BROKEN"))
    # determinism: a fresh process replay of the same pairs is byte-identical
    again = Pool(4).map(pair, [("L1_same", 5000 + i) for i in range(count)])
    print("replay of the same pairs: %s" % ("identical" if all(a["game1"] == b["game1"] and a["game2"] == b["game2"]
                                                              for a, b in zip(same, again)) else "DIFFERENT"))
    again = Pool(4).map(pair, [("L1a_vs_L1b", 5000 + i) for i in range(count)])
    print("replay of the L1a_vs_L1b pairs: %s" % ("identical" if all(a["game1"] == b["game1"] and a["game2"] == b["game2"]
                                                                    for a, b in zip(rows, again)) else "DIFFERENT"))
    # L0 sanity: runs at all
    t0 = time.time()
    r = pair(("L1_vs_L0", 5000))
    print("L1_vs_L0 seed 5000: pair %d, %.1f s one core" % (r["pair"], time.time() - t0))


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "run":
        run(sys.argv[2], int(sys.argv[3]), int(sys.argv[4]))
    elif cmd == "summarize":
        summarize(sys.argv[2])
    elif cmd == "check":
        check(int(sys.argv[2]) if len(sys.argv) > 2 else 4)
    else:
        raise SystemExit(__doc__)
