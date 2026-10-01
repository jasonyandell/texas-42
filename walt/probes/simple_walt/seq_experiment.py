"""EXPLORATORY: does a modeled mind that samples SEQUENTIALLY (blocks of worlds
from its own chair, stop when its top two options separate) give walt a lower
root noise floor than a fixed n0 at the same mean cost?

Variants (the name is the spec; output goes to seq_<variant>.jsonl):
  fixedN              N worlds per modeled decision (walt42: fixed8)
  seqBxMAXmM          blocks of B worlds, at most MAX, stop at a lead of M deals
                      (e.g. seq4x16m2)
A modeled mind always re-solves ONE joint problem over every world drawn so far
(never per-block counts: that would let it pick different continuations per
block, i.e. strategy fusion), and samples only from its own hand + public record.
The root draws 30 own-seat deals from a seed that depends on the position only;
inner randomness (modeled shuffles, dice) is one running rng from base ^ salt,
salt A=1, B=2. Noise floor = how often the root's pick changes between A and B.

Usage:
  python3 seq_experiment.py genpos FIRST_SEED COUNT       # append positions (self-play, fixed8)
  python3 seq_experiment.py run VARIANT [--chunk START COUNT]
  python3 seq_experiment.py summarize VARIANT [VARIANT ...]
"""
import argparse, json, os, random, re, sys, time
from fractions import Fraction
from multiprocessing import Pool

from walt42 import DEALS, INNER_DEALS, outcome, play, sample, tiles, turn, value as value0
_argv, sys.argv = sys.argv, sys.argv[:1]  # crn_experiment reads argv at import
from crn_experiment import contract
sys.argv = _argv

HERE = os.path.dirname(os.path.abspath(__file__))
POSITIONS = os.path.join(HERE, "seq_positions.jsonl")


def parse(variant):
    m = re.fullmatch(r"fixed(\d+)", variant)
    if m:
        return {"mode": "fixed", "n0": int(m[1])}
    m = re.fullmatch(r"seq(\d+)x(\d+)m(\d+)", variant)
    if m:
        return {"mode": "seq", "block": int(m[1]), "budget": int(m[2]), "margin": int(m[3])}
    raise SystemExit("bad variant %r (fixedN or seqBxMAXmM)" % variant)


def base_of(seed, public):
    return hash((seed, public)) & (1 << 40) - 1  # ints only: stable across processes


class Walt:
    """A level-1 player: 30 root deals, modeled seats are level-0 minds."""

    def __init__(self, rules, spec, root, inner):
        self.rules, self.spec = rules, spec
        self.root = random.Random(root)    # own-seat deals: fixed across variants and A/B
        self.running = random.Random(inner)  # modeled shuffles and dice
        self.worlds = self.mdecs = 0       # worlds drawn / sampled modeled decisions

    def decide(self, seat, hand, state, voids=(0, 0, 0, 0), n=DEALS):
        options = tiles(self.rules.legal(hand, state[2]))
        if len(options) == 1:
            return options[0]
        deals = sample(seat, hand, state, n, self.root, voids)
        self.made = {t: self.value(seat, play(self.rules, state, t), deals) for t in options}
        return (max if seat % 2 else min)(options, key=self.made.get)

    def value(self, me, state, deals):
        """walt42.value at level 1, with the modeled seat's decision swapped in."""
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
        """The modeled level-0 mind: best response to dice over worlds from its own chair."""
        options = tiles(self.rules.legal(hand, state[2]))
        if len(options) == 1:
            return options[0]
        s, deals = self.spec, []
        while True:
            more = s["n0"] if s["mode"] == "fixed" else min(s["block"], s["budget"] - len(deals))
            deals += sample(seat, hand, state, more, self.running)
            n = len(deals)
            made = {t: value0(self.rules, seat, play(self.rules, state, t), deals, 0, self.running)
                    for t in options}  # one joint solve over every world so far
            if s["mode"] == "fixed" or n >= s["budget"]:
                break
            lead = sorted((made[t] if seat % 2 else -made[t] for t in options), reverse=True)
            if lead[0] - lead[1] >= s["margin"] or all(v in (0, n) for v in made.values()):
                break
        self.worlds, self.mdecs = self.worlds + n, self.mdecs + 1
        return (max if seat % 2 else min)(options, key=made.get)


# ---- positions: fixed-8 self-play, every non-forced decision ----------------

def selfplay(seed):
    rules, hands = contract(seed)
    state, voids, rows = (0, 1, (), 0, 0), [0] * 4, []
    while outcome(state) is None:
        seat, trick = turn(state), state[2]
        hand = hands[seat] & ~state[0]
        if len(tiles(rules.legal(hand, trick))) == 1:
            t = tiles(rules.legal(hand, trick))[0]
        else:
            rows.append({"seed": seed, "state": list(state), "seat": seat, "hand": hand,
                         "voids": list(voids), "trick": list(trick)})
            base = base_of(seed, state)
            t = Walt(rules, {"mode": "fixed", "n0": INNER_DEALS}, base, base ^ 1).decide(seat, hand, state, voids)
        if trick and not rules.suit[rules.lead[trick[0]]] >> t & 1:
            voids[seat] |= rules.suit[rules.lead[trick[0]]]
        state = play(rules, state, t)
    return rows


def genpos(first, count):
    with Pool(4) as pool, open(POSITIONS, "a") as f:
        for rows in pool.imap(selfplay, range(first, first + count)):
            for r in rows:
                f.write(json.dumps(r) + "\n")
            f.flush()
            print("deal", rows[0]["seed"], len(rows), "decisions", flush=True)


# ---- one evaluation row per position ----------------------------------------

def evaluate(job):
    idx, p, spec = job
    rules, _ = contract(p["seed"])
    state = tuple(p["state"][:2]) + (tuple(p["state"][2]),) + tuple(p["state"][3:])
    base, row = base_of(p["seed"], state), {"pos": idx, "seed": p["seed"], "seat": p["seat"]}
    for tag, salt in (("A", 1), ("B", 2)):
        w = Walt(rules, spec, base, base ^ salt)
        t0 = time.time()
        row[tag] = w.decide(p["seat"], p["hand"], state, p["voids"])
        row[tag + "_s"] = round(time.time() - t0, 3)
        row[tag + "_made"] = {str(k): v for k, v in w.made.items()}
        row[tag + "_worlds"], row[tag + "_mdecs"] = w.worlds, w.mdecs
    return row


def run(variant, start, count):
    spec = parse(variant)
    with open(POSITIONS) as f:
        positions = [json.loads(l) for l in f]
    stop = len(positions) if count is None else start + count
    jobs = [(i, positions[i], spec) for i in range(start, min(stop, len(positions)))]
    with Pool(4) as pool, open(os.path.join(HERE, "seq_%s.jsonl" % variant), "a") as f:
        for row in pool.imap_unordered(evaluate, jobs):
            f.write(json.dumps(row) + "\n")
            f.flush()
    print(variant, "positions", jobs[0][0], "..", jobs[-1][0], flush=True)


# ---- summary ----------------------------------------------------------------

def dec(x, places=2):  # exact Fraction printed to `places` decimals, no floats
    x = Fraction(x)
    q = round(abs(x) * 10 ** places)
    return "%s%d.%0*d" % ("-" if x < 0 else "", q // 10 ** places, places, q % 10 ** places)


def regret(row, mine, theirs):
    """What `mine`'s pick loses under `theirs`' option values, in deals."""
    v, a, b = row[theirs + "_made"], str(row[mine]), str(row[theirs])
    return (v[b] - v[a]) * (1 if row["seat"] % 2 else -1)


def summarize(variants):
    for variant in variants:
        path = os.path.join(HERE, "seq_%s.jsonl" % variant)
        rows = {}
        for line in open(path):
            r = json.loads(line)
            rows[r["pos"]] = r  # a re-run position replaces its earlier row
        rows = list(rows.values())
        flips = [r for r in rows if r["A"] != r["B"]]
        cost = sum(r["A_s"] + r["B_s"] for r in rows)
        worlds = sum(r[t + "_worlds"] for r in rows for t in "AB")
        mdecs = sum(r[t + "_mdecs"] for r in rows for t in "AB")
        reg = [max(regret(r, "A", "B"), regret(r, "B", "A")) for r in flips]
        print("%s: decisions %d, flips %d (%s%%), cross-regret %s deals/flip, %s s/decision, "
              "%s inner worlds/modeled decision (%d modeled decisions, A+B)" % (
                  variant, len(rows), len(flips), dec(Fraction(100 * len(flips), max(len(rows), 1)), 1),
                  dec(Fraction(sum(reg), max(len(flips), 1))), dec(Fraction(round(cost * 1000), 2000 * max(len(rows), 1))),
                  dec(Fraction(worlds, max(mdecs, 1)), 2), mdecs))


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    g = sub.add_parser("genpos"); g.add_argument("first", type=int); g.add_argument("count", type=int)
    r = sub.add_parser("run"); r.add_argument("variant"); r.add_argument("--chunk", nargs=2, type=int, metavar=("START", "COUNT"))
    s = sub.add_parser("summarize"); s.add_argument("variants", nargs="+")
    a = ap.parse_args()
    if a.cmd == "genpos":
        genpos(a.first, a.count)
    elif a.cmd == "run":
        run(a.variant, *(a.chunk or (0, None)))
    else:
        summarize(a.variants)
