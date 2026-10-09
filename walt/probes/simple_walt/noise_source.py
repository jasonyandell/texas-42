"""EXPLORATORY: which random source carries walt's ~30% root-move floor?

The level-1 player of seq_experiment.py (fixed n0=8, 30 root deals, same frozen
positions, same root-deal seeding: Random(base_of(seed, state))) has exactly two
random sources inside its model of the other seats:
  (S) shuffles: which worlds a modeled level-0 mind imagines, from its own chair
  (D) dice: the uniformly random legal tile each non-mind seat plays in each
      imagined world
Here both are pure functions of a key and a base (no running rng anywhere):
  shuffle stream = Random(key(shuffle_base, modeled seat, its hand, public record))
  dice           = options[key(dice_base, world id, seat, public record) % len]
  world id       = key(modeled seat, its hand, public record at the model, index)
                   (a context hash, independent of both bases)
So reseeding one base leaves the other source bit-identical. Four arms, each an
A/B pair over every frozen position (s,d are seed indices 1/2):
  both:    A=(s1,d1) B=(s2,d2)
  shuffle: A=(s1,d1) B=(s2,d1)
  dice:    A=(s1,d1) B=(s1,d2)
  none:    A=(s1,d1) B=(s1,d1)   (determinism check: must give 0 flips)
Each A and B is an independent evaluation (the none arm really re-runs).
Output: noise_ARM.jsonl, one row per position (picks, option values, seconds).

Usage:
  python3 noise_source.py run [ARM ...] [--chunk START COUNT]   # default: all arms
  python3 noise_source.py summarize [ARM ...]                   # default: all arms
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
ARMS = {"both": ((1, 1), (2, 2)), "shuffle": ((1, 1), (2, 1)),
        "dice": ((1, 1), (1, 2)), "none": ((1, 1), (1, 1))}


def key(*xs):
    """A 64-bit key from ints; a pure function of its arguments."""
    k = 0
    for x in xs:
        k = mix(k ^ (x & (1 << 62) - 1))
    return k


class Walt:
    """Level-1 player, fixed n0, shuffles and dice on independent keyed streams."""

    def __init__(self, rules, root, shuffle_base, dice_base):
        self.rules = rules
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
        deals = sample(seat, hand, state, N0, random.Random(key(self.sb, seat, hand, ph)))
        for i, d in enumerate(deals):
            d.append(key(seat, hand, ph, i))  # world id: context only, no base
        made = {t: self.value0(seat, play(self.rules, state, t), deals) for t in options}
        return (max if seat % 2 else min)(options, key=made.get)

    def value0(self, me, state, deals):
        """walt42.value at level 0, dice keyed (dice_base, world id, seat, public record)."""
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
            legal = tiles(self.rules.legal(deal[seat] & ~played, trick))
            t = legal[(key(self.db, deal[4], seat, ph) >> 8) % len(legal)]
            groups.setdefault(t, []).append(deal)
        return sum(self.value0(me, play(self.rules, state, t), g) for t, g in groups.items())


def evaluate(job):
    idx, p, arm = job
    rules, _ = contract(p["seed"])
    state = tuple(p["state"][:2]) + (tuple(p["state"][2]),) + tuple(p["state"][3:])
    base = base_of(p["seed"], state)
    sb = {k: key(base, 0x5348, k) for k in (1, 2)}   # shuffle bases s1, s2
    db = {k: key(base, 0x4449, k) for k in (1, 2)}   # dice bases d1, d2
    row = {"pos": idx, "arm": arm, "seed": p["seed"], "seat": p["seat"]}
    for tag, (s, d) in zip("AB", ARMS[arm]):
        w = Walt(rules, base, sb[s], db[d])
        t0 = time.time()
        row[tag] = w.decide(p["seat"], p["hand"], state, p["voids"])
        row[tag + "_s"] = round(time.time() - t0, 3)
        row[tag + "_made"] = {str(k): v for k, v in w.made.items()}
    return row


def run(arms, start, count):
    with open(POSITIONS) as f:
        positions = [json.loads(l) for l in f]
    stop = len(positions) if count is None else start + count
    jobs = [(i, positions[i], a) for i in range(start, min(stop, len(positions))) for a in arms]
    outs = {a: open(os.path.join(HERE, "noise_%s.jsonl" % a), "a") for a in arms}
    with Pool(4) as pool:
        for row in pool.imap_unordered(evaluate, jobs):
            outs[row["arm"]].write(json.dumps(row) + "\n")
            outs[row["arm"]].flush()
    print("arms", ",".join(arms), "positions", start, "..", min(stop, len(positions)) - 1, flush=True)


def summarize(arms):
    for arm in arms:
        rows = {}
        for line in open(os.path.join(HERE, "noise_%s.jsonl" % arm)):
            r = json.loads(line)
            rows[r["pos"]] = r  # a re-run position replaces its earlier row
        rows = list(rows.values())
        flips = [r for r in rows if r["A"] != r["B"]]
        reg = sum(max(regret(r, "A", "B"), regret(r, "B", "A")) for r in flips)
        n, secs = max(len(rows), 1), sum(r["A_s"] + r["B_s"] for r in rows)
        print("%s: decisions %d, flips %d (%s%%), cross-regret %s deals/decision "
              "(%s per flip), %s s/evaluation" % (
                  arm, len(rows), len(flips), dec(Fraction(100 * len(flips), n), 1),
                  dec(Fraction(reg, n)), dec(Fraction(reg, max(len(flips), 1))),
                  dec(Fraction(round(secs * 1000), 2000 * n))))


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run"); r.add_argument("arms", nargs="*", choices=list(ARMS) + [[]])
    r.add_argument("--chunk", nargs=2, type=int, metavar=("START", "COUNT"))
    s = sub.add_parser("summarize"); s.add_argument("arms", nargs="*", choices=list(ARMS) + [[]])
    a = ap.parse_args()
    arms = a.arms or list(ARMS)
    if a.cmd == "run":
        run(arms, *(a.chunk or (0, None)))
    else:
        summarize(arms)
