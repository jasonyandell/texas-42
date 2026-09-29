"""walt: a ladder of best responses that bottoms out at random. Plays one hand
of straight 42 (pip trumps, bid 30) from trick 1 with walt in all four seats.

    python3 walt42.py [seed]
"""

import random
import sys

TILES = [(h, l) for h in range(7) for l in range(h + 1)]  # 28 dominoes
BID, DEALS, INNER_DEALS = 30, 30, 8


def tiles(mask):
    return [t for t in range(28) if mask >> t & 1]


def name(t):
    return "%d-%d" % TILES[t]


# ---- the rules (the suit algebra) -------------------------------------------

class Rules:
    def __init__(self, trump):
        trumps = sum(1 << t for t, d in enumerate(TILES) if trump in d)
        # suit[q]: tiles that follow a lead of pip q (trumps excluded); suit[7] = trumps
        self.suit = [sum(1 << t for t, d in enumerate(TILES) if q in d) & ~trumps
                     for q in range(7)] + [trumps]
        self.lead = [7 if trumps >> t & 1 else TILES[t][0] for t in range(28)]
        self.trumps = trumps

    def legal(self, hand, trick):
        if not trick:
            return hand
        return hand & self.suit[self.lead[trick[0]]] or hand

    def strength(self, t, led):
        tier = 2 if self.trumps >> t & 1 else 1 if self.suit[led] >> t & 1 else 0
        h, l = TILES[t]
        return tier, 12 if h == l else h + l


def points(t):
    s = sum(TILES[t])
    return s if s in (5, 10) else 0


# ---- the public state: (played, leader, trick so far, points T1, points T0) --
# Seats 1 and 3 (team T1) hold the bid.

def turn(state):
    return (state[1] + len(state[2])) % 4


def outcome(state):
    return True if state[3] >= BID else False if state[4] > 42 - BID else None


def play(rules, state, t):
    played, leader, trick, t1, t0 = state
    trick, played = trick + (t,), played | 1 << t
    if len(trick) < 4:
        return played, leader, trick, t1, t0
    led = rules.lead[trick[0]]
    winner = (leader + max(range(4), key=lambda i: rules.strength(trick[i], led))) % 4
    won = 1 + sum(points(x) for x in trick)
    if winner % 2:
        return played, winner, (), t1 + won, t0
    return played, winner, (), t1, t0 + won


def sample(seat, hand, state, n, rng, voids=(0, 0, 0, 0)):
    """Deals consistent with what `seat` can see. Never the real deal."""
    played, leader, trick = state[:3]
    unseen = tiles((1 << 28) - 1 & ~played & ~hand)
    size = [7 - (bin(played).count("1") - len(trick)) // 4] * 4
    for i in range(len(trick)):
        size[(leader + i) % 4] -= 1
    deals = []
    while len(deals) < n:
        rng.shuffle(unseen)
        deal, at = [0] * 4, 0
        deal[seat] = hand
        for s in range(4):
            if s != seat:
                deal[s] = sum(1 << t for t in unseen[at:at + size[s]])
                at += size[s]
        if not any(deal[s] & voids[s] for s in range(4) if s != seat):
            deals.append(deal)
    return deals


# ---- walt -------------------------------------------------------------------

def decide(rules, seat, hand, state, level, rng, voids=(0, 0, 0, 0), n=INNER_DEALS):
    """The tile that makes (or sets) the bid most often, across the deals
    `seat` cannot rule out."""
    deals = sample(seat, hand, state, n, rng, voids)
    options = tiles(rules.legal(hand, state[2]))
    made = {t: value(rules, seat, play(rules, state, t), deals, level, rng) for t in options}
    return (max if seat % 2 else min)(options, key=made.get)


def value(rules, me, state, deals, level, rng):
    """How many of `deals` make the bid from here."""
    done = outcome(state)
    if done is not None:
        return len(deals) if done else 0
    seat, played, trick = turn(state), state[0], state[2]
    if seat == me:  # one tile for every deal I can't tell apart
        results = [value(rules, me, play(rules, state, t), deals, level, rng)
                   for t in tiles(rules.legal(deals[0][me] & ~played, trick))]
        return max(results) if me % 2 else min(results)
    groups = {}  # what does `seat` play in each deal?
    for deal in deals:
        hand = deal[seat] & ~played
        options = tiles(rules.legal(hand, trick))
        if len(options) == 1:
            t = options[0]
        elif level == 0:
            t = rng.choice(options)
        else:
            t = decide(rules, seat, hand, state, level - 1, rng)
        groups.setdefault(t, []).append(deal)
    return sum(value(rules, me, play(rules, state, t), g, level, rng) for t, g in groups.items())


# ---- a hand -----------------------------------------------------------------

def main(seed):
    rng = random.Random(seed)
    order = list(range(28))
    rng.shuffle(order)
    hands = [sum(1 << t for t in order[7 * i:7 * i + 7]) for i in range(4)]
    # Contract: the seat and trump with the most trumps (then the trump double,
    # then the most doubles) bids 30. Rotate so the bidder is seat 1.
    score, bidder, trump = max(
        ((sum(p in TILES[t] for t in tiles(hands[s])), hands[s] >> TILES.index((p, p)) & 1,
          sum(TILES[t][0] == TILES[t][1] for t in tiles(hands[s]))), s, p)
        for s in range(4) for p in range(7))
    hands = [hands[(bidder + i + 3) % 4] for i in range(4)]
    rules = Rules(trump)
    print("trump %ds, seats 1+3 bid %d, seat 1 leads" % (trump, BID))
    for s in range(4):
        print("  seat %d: %s" % (s, " ".join(name(t) for t in tiles(hands[s]))))

    state, voids = (0, 1, (), 0, 0), [0] * 4
    while outcome(state) is None:
        seat, trick = turn(state), state[2]
        hand = hands[seat] & ~state[0]
        t = decide(rules, seat, hand, state, 1, rng, voids, DEALS)
        if trick and not rules.suit[rules.lead[trick[0]]] >> t & 1:
            voids[seat] |= rules.suit[rules.lead[trick[0]]]  # couldn't follow
        state = play(rules, state, t)
        print("seat %d plays %s" % (seat, name(t)), end="")
        print("   -> T1 %d, T0 %d" % (state[3], state[4]) if not state[2] else "", flush=True)
    print("made" if outcome(state) else "set")


if __name__ == "__main__":
    main(int(sys.argv[1]) if len(sys.argv) > 1 else 1)
