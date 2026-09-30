"""walt: a ladder of best responses that bottoms out at random. Plays one hand
of straight 42 (pip trumps, bid 30) from trick 1 with walt in all four seats.

    python3 walt42.py [seed]
"""

import random
import sys

TILES = [(h, l) for h in range(7) for l in range(h + 1)]  # 28 dominoes
BID, DEALS, INNER_DEALS = 30, 30, 8


def tiles(mask):
    out = []
    while mask:
        low = mask & -mask
        out.append(low.bit_length() - 1)
        mask ^= low
    return out


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
        # power[led][t]: the highest power in a trick wins.
        # Trumps beat the led suit, which beats everything else; within a
        # suit a double is highest, then the larger pip sum.
        self.power = [[(2 if trumps >> t & 1 else 1 if self.suit[q] >> t & 1 else 0) * 16
                       + (12 if h == l else h + l) for t, (h, l) in enumerate(TILES)]
                      for q in range(8)]

    def legal(self, hand, trick):
        if not trick:
            return hand
        return hand & self.suit[self.lead[trick[0]]] or hand


POINTS = [sum(d) if sum(d) in (5, 10) else 0 for d in TILES]


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
    power = [rules.power[rules.lead[trick[0]]][x] for x in trick]
    winner = (leader + power.index(max(power))) % 4
    won = 1 + POINTS[trick[0]] + POINTS[trick[1]] + POINTS[trick[2]] + POINTS[trick[3]]
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
    options = tiles(rules.legal(hand, state[2]))
    if len(options) == 1:
        return options[0]
    deals = sample(seat, hand, state, n, rng, voids)
    made = {t: value(rules, seat, play(rules, state, t), deals, level, rng) for t in options}
    return (max if seat % 2 else min)(options, key=made.get)


def value(rules, me, state, deals, level, rng):
    """How many of `deals` make the bid from here."""
    done = outcome(state)
    if done is not None:
        return len(deals) if done else 0
    seat, played, trick = turn(state), state[0], state[2]
    if seat == me:  # one tile for every deal I can't tell apart
        better, best, goal = (max, 0, len(deals)) if me % 2 else (min, len(deals), 0)
        for t in tiles(rules.legal(deals[0][me] & ~played, trick)):
            best = better(best, value(rules, me, play(rules, state, t), deals, level, rng))
            if best == goal:
                break  # can't do better than that
        return best
    groups = {}  # what does `seat` play in each deal?
    for deal in deals:
        hand = deal[seat] & ~played
        if level == 0:
            t = rng.choice(tiles(rules.legal(hand, trick)))
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
    # Contract: whoever holds the most of one pip bids 30 with it as trump.
    # Rotate so the bidder is seat 1.
    _, bidder, trump = max((sum(p in TILES[t] for t in tiles(hands[s])), s, p)
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
