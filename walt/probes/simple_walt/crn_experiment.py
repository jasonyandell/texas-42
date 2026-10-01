"""EXPLORATORY: does keying the model's randomness on (world, seat, trick)
instead of the public record lower walt's decision noise?

Every source of randomness inside the model (level-0 dice, modeled seats'
belief shuffles) is drawn from a stream seeded by a KEY:
  running  one rng runs on through everything (walt42.py as committed)
  record   seed ^ world ^ hash(public record)      (the Rust ticker tape)
  trick    seed ^ world ^ seat ^ trick number      (common across my candidate tiles)
Noise floor = how often the chosen move changes when only the base seed
changes, same 30 own-seat deals. Positions come from self-play (trick keying).

Usage: python3 crn_experiment.py FIRST_SEED COUNT OUT.jsonl [DEALS] [KEYINGS]
"""
import json, random, sys, time
from multiprocessing import Pool
from walt42 import Rules, TILES, outcome, play, sample, tiles, turn

DEALS, INNER = int(sys.argv[4]) if len(sys.argv) > 4 else 30, 8
KEYINGS = sys.argv[5].split(",") if len(sys.argv) > 5 else ["running", "record", "trick"]


def mix(x):
    x = (x ^ 0x9E3779B97F4A7C15) * 0xBF58476D1CE4E5B9 % (1 << 64)
    x = (x ^ (x >> 31)) * 0x94D049BB133111EB % (1 << 64)
    return x ^ (x >> 29)


class Walt:
    def __init__(self, rules, keying, base, root):
        self.rules, self.keying, self.base = rules, keying, base
        self.root = random.Random(root)  # the own-seat deals: fixed across A/B
        self.running = random.Random(base)

    def key(self, world, seat, public):
        if self.keying == "record":
            return mix(self.base ^ mix(world) ^ hash(public) & (1 << 62) - 1)
        trick = (bin(public[0]).count("1") - len(public[2])) // 4
        return mix(self.base ^ mix(world) ^ mix(seat << 8 | trick))

    def stream(self, world, seat, public):
        return self.running if self.keying == "running" else random.Random(self.key(world, seat, public))

    def pick(self, options, world, seat, public):
        if self.keying == "running":
            return self.running.choice(options)
        return options[(self.key(world, seat, public) >> 8) % len(options)]

    def decide(self, seat, hand, public, level, voids=(0, 0, 0, 0), world=0, n=INNER):
        options = tiles(self.rules.legal(hand, public[2]))
        if len(options) == 1:
            return options[0]
        rng = self.stream(world, seat, public) if world else self.root
        deals = sample(seat, hand, public, n, rng, voids)
        for i, d in enumerate(deals):
            d.append(world * 64 + i + 1)  # a world id for keying, never read as a hand
        made = {t: self.value(seat, play(self.rules, public, t), deals, level) for t in options}
        if world == 0:
            self.made = made  # the root's option values, for the record
        return (max if seat % 2 else min)(options, key=made.get)

    def value(self, me, public, deals, level):
        done = outcome(public)
        if done is not None:
            return len(deals) if done else 0
        seat, played, trick = turn(public), public[0], public[2]
        if seat == me:
            better, best, goal = (max, 0, len(deals)) if me % 2 else (min, len(deals), 0)
            for t in tiles(self.rules.legal(deals[0][me] & ~played, trick)):
                best = better(best, self.value(me, play(self.rules, public, t), deals, level))
                if best == goal:
                    break
            return best
        groups = {}
        for deal in deals:
            hand = deal[seat] & ~played
            if level == 0:
                t = self.pick(tiles(self.rules.legal(hand, trick)), deal[4], seat, public)
            else:
                t = self.decide(seat, hand, public, level - 1, world=deal[4])
            groups.setdefault(t, []).append(deal)
        return sum(self.value(me, play(self.rules, public, t), g, level) for t, g in groups.items())


def contract(seed):
    rng = random.Random(seed)
    order = list(range(28))
    rng.shuffle(order)
    hands = [sum(1 << t for t in order[7 * i:7 * i + 7]) for i in range(4)]
    _, bidder, trump = max((sum(p in TILES[t] for t in tiles(hands[s])), s, p)
                           for s in range(4) for p in range(7))
    return Rules(trump), [hands[(bidder + i + 3) % 4] for i in range(4)]


def run_deal(seed):
    rules, hands = contract(seed)
    public, voids, rows = (0, 1, (), 0, 0), [0] * 4, []
    while outcome(public) is None:
        seat, trick = turn(public), public[2]
        hand = hands[seat] & ~public[0]
        legal = tiles(rules.legal(hand, trick))
        if len(legal) == 1:
            t = legal[0]
        else:
            base = hash((seed, public)) & (1 << 40) - 1
            row = {"seed": seed, "deals": DEALS, "trick": bin(public[0]).count("1") // 4 + 1, "legal": len(legal)}
            for keying in KEYINGS:
                for tag, salt in (("A", 1), ("B", 2)):
                    w = Walt(rules, keying, base ^ salt, base)
                    t0 = time.time()
                    row[keying + tag] = w.decide(seat, hand, public, 1, voids, 0, DEALS)
                    row[keying + tag + "_s"] = round(time.time() - t0, 2)
                    row[keying + tag + "_made"] = {str(k): v for k, v in w.made.items()}
            rows.append(row)
            t = row[KEYINGS[-1] + "A"]
        if trick and not rules.suit[rules.lead[trick[0]]] >> t & 1:
            voids[seat] |= rules.suit[rules.lead[trick[0]]]
        public = play(rules, public, t)
    return rows


if __name__ == "__main__":
    first, count, out = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3]
    with Pool(4) as pool, open(out, "a") as f:
        for rows in pool.imap_unordered(run_deal, range(first, first + count)):
            for r in rows:
                f.write(json.dumps(r) + "\n")
            f.flush()
            print("deal", rows[0]["seed"] if rows else "?", len(rows), "decisions", flush=True)
