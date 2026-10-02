"""EXPLORATORY controls for strength.py (reuses it by import; nothing re-implemented).

  python3 strength_ctrl.py rand START COUNT    # L1(seed 1) vs uniform-random legal tile: the true positive control
  python3 strength_ctrl.py agree START COUNT   # on positions from strength_L1_vs_L0.jsonl games (seeds START..): how often do
                                               # L1(1), L1(2), L0 pick the same tile at the same position?
rand appends strength_L1_vs_RAND.jsonl (summarize with strength_audit.py).
agree prints counts only.
"""
import json, os, random, sys, time
from multiprocessing import Pool

import strength as S
from walt42 import play, tiles, turn, outcome
from crn_experiment import contract

S.MATCHUPS["L1_vs_RAND"] = (("L1", 1), ("RAND", 0))
_choose = S.choose


def choose(player, rules, dealseed, seat, hand, state, voids):
    if player[0] == "RAND":
        opts = tiles(rules.legal(hand, state[2]))
        rng = random.Random(hash((dealseed, state, 99)) & S.MASK)
        return opts[rng.randrange(len(opts))]
    return _choose(player, rules, dealseed, seat, hand, state, voids)


S.choose = choose


def rand_run(start, count):
    t0 = time.time()
    with Pool(4) as pool, open(os.path.join(S.HERE, "strength_L1_vs_RAND.jsonl"), "a") as f:
        for row in pool.imap_unordered(S.pair, [("L1_vs_RAND", s) for s in range(start, start + count)]):
            f.write(json.dumps(row) + "\n"); f.flush()
    print("L1_vs_RAND seeds %d..%d wall %.1f s (%.1f s/pair)" % (start, start + count - 1, time.time() - t0, (time.time() - t0) / count))


def agree_job(seed):
    row = [json.loads(l) for l in open(S.path("L1_vs_L0")) if json.loads(l)["seed"] == seed][0]
    out = []
    for gname in ("game1", "game2"):
        rules, hands = contract(seed)
        state, voids = (0, 1, (), 0, 0), [0] * 4
        for seat, t in row[gname]:
            hand = hands[seat] & ~state[0]
            trick = state[2]
            if len(tiles(rules.legal(hand, trick))) > 1:
                a = S.choose(("L1", 1), rules, seed, seat, hand, state, voids)
                b = S.choose(("L1", 2), rules, seed, seat, hand, state, voids)
                c = S.choose(("L0", 0), rules, seed, seat, hand, state, voids)
                out.append((seed, gname, 0, seat, a, b, c, t))
            if trick and not rules.suit[rules.lead[trick[0]]] >> t & 1:
                voids[seat] |= rules.suit[rules.lead[trick[0]]]
            state = play(rules, state, t)
    return out


def agree(start, count):
    t0 = time.time()
    with Pool(4) as pool:
        res = [x for r in pool.map(agree_job, range(start, start + count), chunksize=1) for x in r]
    n = len(res)
    f = lambda i, j: sum(x[i] != x[j] for x in res)
    print("agree seeds %d..%d: %d non-forced positions (%.1f s wall)" % (start, start + count - 1, n, time.time() - t0))
    print("  L1(1) != L1(2): %d   L1(1) != L0: %d   L1(2) != L0: %d   all three equal: %d" % (
        f(4, 5), f(4, 6), f(5, 6), sum(x[4] == x[5] == x[6] for x in res)))
    json.dump(res, open(os.path.join(S.HERE, "strength_agree_%d_%d.json" % (start, count)), "w"))


if __name__ == "__main__":
    {"rand": lambda: rand_run(int(sys.argv[2]), int(sys.argv[3])),
     "agree": lambda: agree(int(sys.argv[2]), int(sys.argv[3]))}[sys.argv[1]]()
