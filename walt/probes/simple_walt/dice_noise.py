"""EXPLORATORY: is there shuffle-only noise, and is the dice noise averageable?

Follow-up to noise_source.py (same level-1 player: fixed n0=8, 30 root deals,
same frozen positions, same root-deal seeding Random(base_of(seed, state))).
noise_source.py showed that reseeding only the level-0 dice reproduces the whole
~33% floor, but its shuffle-only arm was confounded: dice were keyed by world
SLOT, so a shuffle reseed re-mapped most realized opponent plays. Changes here:

 1. Dice are keyed by (dice_base, tape index, acting seat, the acting seat's
    REMAINING HAND in that world, public record). A dice draw therefore depends
    on the hand that plays and the record, never on the world slot: under a
    shuffle reseed every realized play in a given (tape, seat, hand, record)
    situation is bit-identical (the `check` subcommand instruments this).
    Shuffle keying is unchanged: Random(key(shuffle_base, seat, hand, record)).
 2. K tapes: in a modeled mind's solve each of its n0 imagined worlds appears K
    times, tape index 0..K-1 (same hands, independent dice stream), so the mind
    best-responds to K playouts per world; its option values are counts over the
    n0*K world-copies. The root is unchanged (30 root deals, identical across
    arms and across A/B).

Arms (A/B pair of independent evaluations over every frozen position; s,d are
seed indices 1/2):
  shuffle_k1: K=1  A=(s1,d1) B=(s2,d1)   clean Q1: shuffle-only noise
  dice_k1:    K=1  A=(s1,d1) B=(s1,d2)   calibration (~35% in noise_source)
  both_k1:    K=1  A=(s1,d1) B=(s2,d2)
  both_k4:    K=4  A=(s1,d1) B=(s2,d2)
  both_k16:   K=16 A=(s1,d1) B=(s2,d2)   Q2: does the floor fall with K?
Output: dice_ARM.jsonl, one row per position: picks, option values (A, B),
seconds, and mean world-copies per modeled decision (A_copies, B_copies; the
total copies and the number of multi-option modeled decisions are A_mcopies /
A_models).

Usage:
  python3 dice_noise.py run [ARM ...] [--chunk START COUNT]   # default: all arms
  python3 dice_noise.py summarize [ARM ...]                   # default: all arms
  python3 dice_noise.py check [--positions I J ...]           # shuffle-reseed dice-site check
"""
import argparse, json, os, random, sys, time
from fractions import Fraction
from multiprocessing import Pool

from walt42 import DEALS, outcome, play, sample, tiles, turn
_argv, sys.argv = sys.argv, sys.argv[:1]  # crn_experiment reads argv at import
from crn_experiment import contract, mix
sys.argv = _argv
from seq_experiment import POSITIONS, base_of, dec, regret

HERE = os.path.dirname(os.path.abspath(__file__))
N0 = 8
# arm -> (K, A seed indices (s, d), B seed indices (s, d))
ARMS = {"shuffle_k1": (1, (1, 1), (2, 1)), "dice_k1": (1, (1, 1), (1, 2)),
        "both_k1": (1, (1, 1), (2, 2)), "both_k4": (4, (1, 1), (2, 2)),
        "both_k16": (16, (1, 1), (2, 2))}


def key(*xs):
    """A 64-bit key from ints; a pure function of its arguments."""
    k = 0
    for x in xs:
        k = mix(k ^ (x & (1 << 62) - 1))
    return k


class Walt:
    """Level-1 player, fixed n0, shuffles and dice on independent keyed streams;
    each modeled mind imagines every world K times (K dice tapes)."""

    def __init__(self, rules, root, shuffle_base, dice_base, k=1, trace=None):
        self.rules = rules
        self.k = k
        self.trace = trace  # optional {(tape, seat, hand, record): tile} instrumentation
        self.models = self.copies = 0
        self.root = random.Random(root)  # own-seat deals: same as seq_experiment
        self.sb, self.db = shuffle_base, dice_base

    def decide(self, seat, hand, state, voids=(0, 0, 0, 0), n=DEALS):
        options = tiles(self.rules.legal(hand, state[2]))
        if len(options) == 1:
            return options[0]
        deals = sample(seat, hand, state, n, self.root, voids)
        self.made = {t: self.value(seat, play(self.rules, state, t), deals) for t in options}
        return (max if seat % 2 else min)(options, key=self.made.get)

    def value(self, me, state, deals):
        """Level-1 value: modeled seats are level-0 minds."""
        done = outcome(state)
        if done is not None:
            return len(deals) if done else 0
        seat, played, trick = turn(state), state[0], state[2]
        if seat == me:
            better, best, goal = (max, 0, len(deals)) if me % 2 else (min, len(deals), 0)
            for t in tiles(self.rules.legal(deals[0][me] & ~played, trick)):
                best = better(best, self.value(me, play(self.rules, state, t), deals))
                if best == goal:
                    break
            return best
        groups = {}
        for deal in deals:
            t = self.model(seat, deal[seat] & ~played, state)
            groups.setdefault(t, []).append(deal)
        return sum(self.value(me, play(self.rules, state, t), g) for t, g in groups.items())

    def model(self, seat, hand, state):
        """Level-0 mind: shuffles (S) from its own chair, then best response to dice (D)."""
        options = tiles(self.rules.legal(hand, state[2]))
        if len(options) == 1:
            return options[0]
        ph = hash(state)
        worlds = sample(seat, hand, state, N0, random.Random(key(self.sb, seat, hand, ph)))
        # each world K times: same hands, tape index 0..K-1 (deal[4]) picks the dice stream
        deals = [w + [tape] for w in worlds for tape in range(self.k)]
        self.models += 1
        self.copies += len(deals)
        made = {t: self.value0(seat, play(self.rules, state, t), deals) for t in options}
        return (max if seat % 2 else min)(options, key=made.get)

    def value0(self, me, state, deals):
        """walt42.value at level 0, dice keyed (dice_base, tape, seat, its remaining hand,
        public record): never the world slot."""
        done = outcome(state)
        if done is not None:
            return len(deals) if done else 0
        seat, played, trick = turn(state), state[0], state[2]
        if seat == me:
            better, best, goal = (max, 0, len(deals)) if me % 2 else (min, len(deals), 0)
            for t in tiles(self.rules.legal(deals[0][me] & ~played, trick)):
                best = better(best, self.value0(me, play(self.rules, state, t), deals))
                if best == goal:
                    break
            return best
        groups, ph = {}, hash(state)
        for deal in deals:
            rest = deal[seat] & ~played
            legal = tiles(self.rules.legal(rest, trick))
            t = legal[(key(self.db, deal[4], seat, rest, ph) >> 8) % len(legal)]
            if self.trace is not None:
                self.trace[(deal[4], seat, rest, ph)] = (
                    t if self.trace.get((deal[4], seat, rest, ph), t) == t else None)
            groups.setdefault(t, []).append(deal)
        return sum(self.value0(me, play(self.rules, state, t), g) for t, g in groups.items())


def evaluate(job):
    idx, p, arm = job
    k, specA, specB = ARMS[arm]
    rules, _ = contract(p["seed"])
    state = tuple(p["state"][:2]) + (tuple(p["state"][2]),) + tuple(p["state"][3:])
    base = base_of(p["seed"], state)
    sb = {k: key(base, 0x5348, k) for k in (1, 2)}   # shuffle bases s1, s2
    db = {k: key(base, 0x4449, k) for k in (1, 2)}   # dice bases d1, d2
    row = {"pos": idx, "arm": arm, "K": k, "seed": p["seed"], "seat": p["seat"]}
    for tag, (s, d) in zip("AB", (specA, specB)):
        w = Walt(rules, base, sb[s], db[d], k)
        t0 = time.time()
        row[tag] = w.decide(p["seat"], p["hand"], state, p["voids"])
        row[tag + "_s"] = round(time.time() - t0, 3)
        row[tag + "_made"] = {str(t): v for t, v in w.made.items()}
        row[tag + "_models"], row[tag + "_mcopies"] = w.models, w.copies
        row[tag + "_copies"] = round(w.copies / w.models, 3) if w.models else 0
    return row


def run(arms, start, count):
    with open(POSITIONS) as f:
        positions = [json.loads(l) for l in f]
    stop = len(positions) if count is None else start + count
    jobs = [(i, positions[i], a) for i in range(start, min(stop, len(positions))) for a in arms]
    outs = {a: open(os.path.join(HERE, "dice_%s.jsonl" % a), "a") for a in arms}
    with Pool(4) as pool:
        for row in pool.imap_unordered(evaluate, jobs):
            outs[row["arm"]].write(json.dumps(row) + "\n")
            outs[row["arm"]].flush()
    print("arms", ",".join(arms), "positions", start, "..", min(stop, len(positions)) - 1, flush=True)


def summarize(arms):
    for arm in arms:
        path = os.path.join(HERE, "dice_%s.jsonl" % arm)
        if not os.path.exists(path):
            print("%s: no rows" % arm)
            continue
        rows = {}
        for line in open(path):
            r = json.loads(line)
            rows[r["pos"]] = r  # a re-run position replaces its earlier row
        rows = list(rows.values())
        flips = [r for r in rows if r["A"] != r["B"]]
        reg = sum(max(regret(r, "A", "B"), regret(r, "B", "A")) for r in flips)
        n, secs = max(len(rows), 1), sum(r["A_s"] + r["B_s"] for r in rows)
        models = sum(r["A_models"] + r["B_models"] for r in rows)
        copies = sum(r["A_mcopies"] + r["B_mcopies"] for r in rows)
        print("%s (K=%d): decisions %d, flips %d (%s%%), cross-regret %s deals/decision "
              "(%s per flip), %s s/evaluation, %s world-copies per modeled decision "
              "(%s modeled decisions per evaluation)" % (
                  arm, ARMS[arm][0], len(rows), len(flips),
                  dec(Fraction(100 * len(flips), n), 1),
                  dec(Fraction(reg, n)), dec(Fraction(reg, max(len(flips), 1))),
                  dec(Fraction(round(secs * 1000), 2000 * n)),
                  dec(Fraction(copies, max(models, 1)), 1), dec(Fraction(models, 2 * n), 1)))


def check(idxs):
    """Instrumented check: under a shuffle reseed (s1 -> s2, d1, K=1 and K=4), every
    dice site with the same (tape, seat, hand, record) key realizes the same tile.
    Contrast: under a dice reseed (d1 -> d2) the shared sites mostly differ."""
    with open(POSITIONS) as f:
        positions = [json.loads(l) for l in f]
    for i in idxs:
        p = positions[i]
        rules, _ = contract(p["seed"])
        state = tuple(p["state"][:2]) + (tuple(p["state"][2]),) + tuple(p["state"][3:])
        base = base_of(p["seed"], state)
        sb = {k: key(base, 0x5348, k) for k in (1, 2)}
        db = {k: key(base, 0x4449, k) for k in (1, 2)}
        for k in (1, 4):
            traces = {}
            for tag, (s, d) in (("s1d1", (1, 1)), ("s2d1", (2, 1)), ("s1d2", (1, 2))):
                traces[tag] = {}
                Walt(rules, base, sb[s], db[d], k, traces[tag]).decide(
                    p["seat"], p["hand"], state, p["voids"])
            A = traces["s1d1"]
            for tag in ("s2d1", "s1d2"):
                B = traces[tag]
                shared = A.keys() & B.keys()
                same = sum(A[x] == B[x] for x in shared)
                inner = sum(v is None for t in (A, B) for v in t.values())
                print("pos %d K=%d s1d1 vs %s: sites %d/%d, shared %d, same tile %d, "
                      "differ %d, within-run conflicts %d" % (
                          i, k, tag, len(A), len(B), len(shared), same,
                          len(shared) - same, inner), flush=True)
                if tag == "s2d1":
                    assert same == len(shared) and not inner and shared, "KEYING BROKEN"
    print("check ok: shuffle reseed leaves every shared (tape, seat, hand, record) "
          "site bit-identical")


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run"); r.add_argument("arms", nargs="*", choices=list(ARMS) + [[]])
    r.add_argument("--chunk", nargs=2, type=int, metavar=("START", "COUNT"))
    s = sub.add_parser("summarize"); s.add_argument("arms", nargs="*", choices=list(ARMS) + [[]])
    c = sub.add_parser("check"); c.add_argument("--positions", nargs="+", type=int, default=[0, 1])
    a = ap.parse_args()
    if a.cmd == "check":
        check(a.positions)
    else:
        arms = a.arms or list(ARMS)
        if a.cmd == "run":
            run(arms, *(a.chunk or (0, None)))
        else:
            summarize(arms)
